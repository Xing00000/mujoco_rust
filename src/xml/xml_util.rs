//! Port of: xml/xml_util.cc
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: ParseInfOrNan (xml/xml_util.cc:54)
#[allow(unused_variables, non_snake_case)]
pub fn parse_inf_or_nan(s: *const std__string) -> *const () {
    extern "C" { fn ParseInfOrNan_impl(s: *const std__string) -> *const (); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { ParseInfOrNan_impl(s) }
}

/// C: ResolveFilePath (xml/xml_util.cc:79)
/// Calls: FilePath::IsAbs, FilePath::c_str, mju_closeResource, mju_openResource
#[allow(unused_variables, non_snake_case)]
pub fn resolve_file_path(e: *mut XMLElement, filename: *const FilePath, dir: *const FilePath, vfs: *const mjVFS) -> FilePath {
    extern "C" { fn ResolveFilePath_impl(e: *mut XMLElement, filename: *const FilePath, dir: *const FilePath, vfs: *const mjVFS) -> FilePath; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { ResolveFilePath_impl(e, filename, dir, vfs) }
}

/// C: AccumulateFiles (xml/xml_util.cc:109)
#[allow(unused_variables, non_snake_case)]
pub fn accumulate_files(files: *mut i32, root: *mut tinyxml2__XMLElement, model_dir: *const FilePath) {
    // WARNING: signature changed — verify body
    // Previous params: (files : * mut i32, root : * mut tinyxml2__XMLElement, model_dir : * const FilePath)
    // Previous return: ()
    extern "C" { fn AccumulateFiles_impl (files : * mut i32 , root : * mut tinyxml2__XMLElement , model_dir : * const FilePath) ; } unsafe { AccumulateFiles_impl (files , root , model_dir) }
}

/// C: mju_getXMLDependencies (xml/xml_util.cc:224)
/// Calls: mju_closeResource, mju_error, mju_getResourceDir, mju_openResource, mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn mju_get_xml_dependencies(filename: *const i8, dependencies: *mut i32) {
    extern "C" { fn mju_getXMLDependencies_impl(filename: *const i8, dependencies: *mut i32); }
    // SAFETY: delegates to C implementation
    unsafe { mju_getXMLDependencies_impl(filename, dependencies) }
}

/// C: printspace (xml/xml_util.cc:391)
#[allow(unused_variables, non_snake_case)]
pub fn printspace(str: *mut std__stringstream, n: i32, space: *const i8) {
    extern "C" { fn printspace_impl(str: *mut std__stringstream, n: i32, space: *const i8); }
    // SAFETY: delegates to C implementation
    unsafe { printspace_impl(str, n, space) }
}

/// C: mjXUtil::ReadAttrVec (xml/xml_util.cc:714)
/// Calls: mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_vec(self_ptr: *mut mjXUtil, elem: *mut XMLElement, attr: *const i8, required: bool) -> i32 {
    extern "C" { fn mjXUtil_ReadAttrVec_impl(self_ptr: *mut mjXUtil, elem: *mut XMLElement, attr: *const i8, required: bool) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjXUtil_ReadAttrVec_impl(self_ptr, elem, attr, required) }
}

/// C: mjXUtil::ReadVector (xml/xml_util.cc:860)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_vector(self_ptr: *mut mjXUtil, elem: *mut XMLElement, attr: *const i8, vec: *mut i32, text: *mut std__string, required: bool) -> i32 {
    extern "C" { fn mjXUtil_ReadVector_impl(self_ptr: *mut mjXUtil, elem: *mut XMLElement, attr: *const i8, vec: *mut i32, text: *mut std__string, required: bool) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjXUtil_ReadVector_impl(self_ptr, elem, attr, vec, text, required) }
}

/// C: mjXUtil::Vector2String (xml/xml_util.cc:898)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_vector2string(self_ptr: *mut mjXUtil, txt: *mut std__string, vec: *const i32, ncol: i32) {
    extern "C" { fn mjXUtil_Vector2String_impl(self_ptr: *mut mjXUtil, txt: *mut std__string, vec: *const i32, ncol: i32); }
    // SAFETY: delegates to C implementation
    unsafe { mjXUtil_Vector2String_impl(self_ptr, txt, vec, ncol) }
}

/// C: isint (xml/xml_util.cc:1004)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn isint(x: f64) -> bool {
    extern "C" { fn isint_impl(x: f64) -> bool; }
    // SAFETY: delegates to C implementation
    unsafe { isint_impl(x) }
}

/// C: Round (xml/xml_util.cc:1010)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn round(x: f64) -> i32 {
    extern "C" { fn Round_impl(x: f64) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { Round_impl(x) }
}

/// C: mjXUtil::WriteVector (xml/xml_util.cc:1084)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_write_vector(self_ptr: *mut mjXUtil, elem: *mut XMLElement, name: string, vec: *const i32) {
    extern "C" { fn mjXUtil_WriteVector_impl(self_ptr: *mut mjXUtil, elem: *mut XMLElement, name: string, vec: *const i32); }
    // SAFETY: delegates to C implementation
    unsafe { mjXUtil_WriteVector_impl(self_ptr, elem, name, vec) }
}

/// C: mjCopyError (xml/xml_util.h:32)
#[allow(unused_variables, non_snake_case)]
pub fn mj_copy_error(dst: *mut i8, src: *const i8, maxlen: i32) {
    if dst.is_null() || maxlen <= 0 {
        return;
    }
    // SAFETY: dst is non-null and maxlen > 0 (checked above). src is a valid C string.
    // We replicate strncpy semantics: copy up to maxlen bytes, then null-terminate.
    unsafe {
        let n = maxlen as usize;
        let mut i: usize = 0;
        while i < n {
            let c = *src.add(i);
            *dst.add(i) = c;
            if c == 0 {
                break;
            }
            i += 1;
        }
        // Ensure null termination at last position
        *dst.add(n - 1) = 0;
    }
}

/// C: FirstChildElement (xml/xml_util.h:36)
#[allow(unused_variables, non_snake_case)]
pub fn first_child_element(e: *mut XMLElement, name: *const i8) -> *mut XMLElement {
    // WARNING: signature changed — verify body
    // Previous params: (e : * mut XMLElement, name : * const i8)
    // Previous return: * mut XMLElement
    extern "C" { fn FirstChildElement_impl (e : * mut XMLElement , name : * const i8) -> * mut XMLElement ; } unsafe { FirstChildElement_impl (e , name) }
}

/// C: NextSiblingElement (xml/xml_util.h:37)
/// Calls: FirstChildElement
#[allow(unused_variables, non_snake_case)]
pub fn next_sibling_element(e: *mut XMLElement, name: *const i8) -> *mut XMLElement {
    extern "C" { fn NextSiblingElement_impl(e: *mut XMLElement, name: *const i8) -> *mut XMLElement; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { NextSiblingElement_impl(e, name) }
}

/// C: mjXSchema::GetError (xml/xml_util.h:57)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_schema_get_error(self_ptr: *mut mjXSchema) -> std__string {
    extern "C" { fn mjXSchema_GetError_impl(self_ptr: *mut mjXSchema) -> std__string; }
    // SAFETY: delegates to C implementation
    unsafe { mjXSchema_GetError_impl(self_ptr) }
}

/// C: mjXSchema::Print (xml/xml_util.h:58)
/// Calls: printspace
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_schema_print(self_ptr: *mut mjXSchema, str: *mut std__stringstream, level: i32) {
    extern "C" { fn mjXSchema_Print_impl(self_ptr: *mut mjXSchema, str: *mut std__stringstream, level: i32); }
    // SAFETY: delegates to C implementation
    unsafe { mjXSchema_Print_impl(self_ptr, str, level) }
}

/// C: mjXSchema::PrintHTML (xml/xml_util.h:59)
/// Calls: printspace
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_schema_print_html(self_ptr: *mut mjXSchema, str: *mut std__stringstream, level: i32, pad: bool) {
    extern "C" { fn mjXSchema_PrintHTML_impl(self_ptr: *mut mjXSchema, str: *mut std__stringstream, level: i32, pad: bool); }
    // SAFETY: delegates to C implementation
    unsafe { mjXSchema_PrintHTML_impl(self_ptr, str, level, pad) }
}

/// C: mjXSchema::NameMatch (xml/xml_util.h:61)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_schema_name_match(self_ptr: *mut mjXSchema, elem: *mut tinyxml2__XMLElement, level: i32) -> bool {
    extern "C" { fn mjXSchema_NameMatch_impl(self_ptr: *mut mjXSchema, elem: *mut tinyxml2__XMLElement, level: i32) -> bool; }
    // SAFETY: delegates to C implementation
    unsafe { mjXSchema_NameMatch_impl(self_ptr, elem, level) }
}

/// C: mjXSchema::Check (xml/xml_util.h:62)
/// Calls: FirstChildElement, NextSiblingElement, mjXSchema::NameMatch
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_schema_check(self_ptr: *mut mjXSchema, elem: *mut tinyxml2__XMLElement, level: i32) -> *mut tinyxml2__XMLElement {
    extern "C" { fn mjXSchema_Check_impl(self_ptr: *mut mjXSchema, elem: *mut tinyxml2__XMLElement, level: i32) -> *mut tinyxml2__XMLElement; }
    // SAFETY: delegates to C implementation
    unsafe { mjXSchema_Check_impl(self_ptr, elem, level) }
}

/// C: mjXUtil::SameVector (xml/xml_util.h:91)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_same_vector(vec1: *const T, vec2: *const T, n: i32) -> bool {
    extern "C" { fn mjXUtil_SameVector_impl(vec1: *const T, vec2: *const T, n: i32) -> bool; }
    // SAFETY: delegates to C implementation
    unsafe { mjXUtil_SameVector_impl(vec1, vec2, n) }
}

/// C: mjXUtil::FindKey (xml/xml_util.h:94)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_find_key(map: *const mjMap, mapsz: i32, key: string) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (map : * const mjMap, mapsz : i32, key : string)
    // Previous return: i32
    extern "C" { fn mjXUtil_FindKey_impl(map: *const mjMap, mapsz: i32, key: string) -> i32; }
    // SAFETY: delegates to C++ implementation; caller guarantees map is valid
    unsafe { mjXUtil_FindKey_impl(map, mapsz, key) }
}

/// C: mjXUtil::FindValue (xml/xml_util.h:97)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_find_value(map: *const mjMap, mapsz: i32, value: i32) -> std__string {
    extern "C" { fn mjXUtil_FindValue_impl(map: *const mjMap, mapsz: i32, value: i32) -> std__string; }
    // SAFETY: delegates to C implementation
    unsafe { mjXUtil_FindValue_impl(map, mapsz, value) }
}

/// C: mjXUtil::ReadAttrStr (xml/xml_util.h:106)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_str(elem: *mut tinyxml2__XMLElement, attr: *const i8, required: bool) -> *const () {
    extern "C" { fn mjXUtil_ReadAttrStr_impl(elem: *mut tinyxml2__XMLElement, attr: *const i8, required: bool) -> *const (); }
    // SAFETY: delegates to C implementation
    unsafe { mjXUtil_ReadAttrStr_impl(elem, attr, required) }
}

/// C: mjXUtil::ReadAttrFile (xml/xml_util.h:112)
/// Calls: ResolveFilePath
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_file(elem: *mut tinyxml2__XMLElement, attr: *const i8, vfs: *const mjVFS, dir: *const mujoco__user__FilePath, required: bool) -> *const () {
    extern "C" { fn mjXUtil_ReadAttrFile_impl(elem: *mut tinyxml2__XMLElement, attr: *const i8, vfs: *const mjVFS, dir: *const mujoco__user__FilePath, required: bool) -> *const (); }
    // SAFETY: delegates to C implementation
    unsafe { mjXUtil_ReadAttrFile_impl(elem, attr, vfs, dir, required) }
}

/// C: mjXUtil::ReadAttrNum (xml/xml_util.h:119)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_num(elem: *mut tinyxml2__XMLElement, attr: *const i8, required: bool) -> *const () {
    extern "C" { fn mjXUtil_ReadAttrNum_impl(elem: *mut tinyxml2__XMLElement, attr: *const i8, required: bool) -> *const (); }
    // SAFETY: delegates to C implementation
    unsafe { mjXUtil_ReadAttrNum_impl(elem, attr, required) }
}

/// C: mjXUtil::ReadAttrArr (xml/xml_util.h:126)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_arr(elem: *mut tinyxml2__XMLElement, attr: *const i8, required: bool) -> *const () {
    extern "C" { fn mjXUtil_ReadAttrArr_impl(elem: *mut tinyxml2__XMLElement, attr: *const i8, required: bool) -> *const (); }
    // SAFETY: delegates to C implementation
    unsafe { mjXUtil_ReadAttrArr_impl(elem, attr, required) }
}

/// C: mjXUtil::ReadAttr (xml/xml_util.h:151)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr(elem: *mut tinyxml2__XMLElement, attr: *const i8, len: i32, data: *mut T, text: *mut std__string, required: bool, exact: bool) -> i32 {
    extern "C" { fn mjXUtil_ReadAttr_impl(elem: *mut tinyxml2__XMLElement, attr: *const i8, len: i32, data: *mut T, text: *mut std__string, required: bool, exact: bool) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjXUtil_ReadAttr_impl(elem, attr, len, data, text, required, exact) }
}

/// C: mjXUtil::ReadQuat (xml/xml_util.h:155)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_quat(elem: *mut tinyxml2__XMLElement, attr: *const i8, data: *mut f64, text: *mut std__string, required: bool) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (elem : * mut tinyxml2__XMLElement, attr : * const i8, data : * mut f64, text : * mut std__string, required : bool)
    // Previous return: i32
    extern "C" { fn mjXUtil_ReadQuat_impl(elem: *mut tinyxml2__XMLElement, attr: *const i8, data: *mut f64, text: *mut std__string, required: bool) -> i32; }
    // SAFETY: delegates to C++ implementation; caller guarantees elem, attr, data, text are valid
    unsafe { mjXUtil_ReadQuat_impl(elem, attr, data, text, required) }
}

/// C: mjXUtil::ReadAttrTxt (xml/xml_util.h:163)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_txt(elem: *mut tinyxml2__XMLElement, attr: *const i8, text: *mut std__string, required: bool) -> bool {
    extern "C" { fn mjXUtil_ReadAttrTxt_impl(elem: *mut tinyxml2__XMLElement, attr: *const i8, text: *mut std__string, required: bool) -> bool; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjXUtil_ReadAttrTxt_impl(elem, attr, text, required) }
}

/// C: mjXUtil::ReadAttrInt (xml/xml_util.h:167)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_int(elem: *mut tinyxml2__XMLElement, attr: *const i8, data: *mut i32, required: bool) -> bool {
    extern "C" { fn mjXUtil_ReadAttrInt_impl(elem: *mut tinyxml2__XMLElement, attr: *const i8, data: *mut i32, required: bool) -> bool; }
    // SAFETY: delegates to C++ implementation, all pointers valid per caller contract
    unsafe { mjXUtil_ReadAttrInt_impl(elem, attr, data, required) }
}

/// C: mjXUtil::FindSubElem (xml/xml_util.h:175)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_find_sub_elem(elem: *mut tinyxml2__XMLElement, name: string, required: bool) -> *mut tinyxml2__XMLElement {
    extern "C" { fn mjXUtil_FindSubElem_impl(elem: *mut tinyxml2__XMLElement, name: string, required: bool) -> *mut tinyxml2__XMLElement; }
    // SAFETY: delegates to C implementation
    unsafe { mjXUtil_FindSubElem_impl(elem, name, required) }
}

/// C: mjXUtil::MapValue (xml/xml_util.h:179)
/// Calls: mjXUtil::FindKey
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_map_value(elem: *mut tinyxml2__XMLElement, attr: *const i8, data: *mut i32, map: *const mjMap, mapSz: i32, required: bool) -> bool {
    extern "C" { fn mjXUtil_MapValue_impl(elem: *mut tinyxml2__XMLElement, attr: *const i8, data: *mut i32, map: *const mjMap, mapSz: i32, required: bool) -> bool; }
    // SAFETY: delegates to C++ implementation, all pointers valid per caller contract
    unsafe { mjXUtil_MapValue_impl(elem, attr, data, map, mapSz, required) }
}

/// C: mjXUtil::MapValues (xml/xml_util.h:183)
/// Calls: mjXUtil::FindKey
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_map_values(elem: *mut tinyxml2__XMLElement, attr: *const i8, data: *mut i32, map: *const mjMap, mapSz: i32, required: bool) -> i32 {
    extern "C" { fn mjXUtil_MapValues_impl(elem: *mut tinyxml2__XMLElement, attr: *const i8, data: *mut i32, map: *const mjMap, mapSz: i32, required: bool) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjXUtil_MapValues_impl(elem, attr, data, map, mapSz, required) }
}

/// C: mjXUtil::WriteAttr (xml/xml_util.h:188)
/// Calls: _mjPRIVATE__get_xml_precision, mjXUtil::WriteAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_write_attr(elem: *mut tinyxml2__XMLElement, name: string, n: i32, data: *const T, def: *const T, trim: bool) {
    extern "C" { fn mjXUtil_WriteAttr_impl(elem: *mut tinyxml2__XMLElement, name: string, n: i32, data: *const T, def: *const T, trim: bool); }
    // SAFETY: delegates to C implementation
    unsafe { mjXUtil_WriteAttr_impl(elem, name, n, data, def, trim) }
}

/// C: mjXUtil::WriteAttrTxt (xml/xml_util.h:198)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_write_attr_txt(elem: *mut tinyxml2__XMLElement, name: string, value: string) {
    extern "C" { fn mjXUtil_WriteAttrTxt_impl(elem: *mut tinyxml2__XMLElement, name: string, value: string); }
    // SAFETY: delegates to C implementation
    unsafe { mjXUtil_WriteAttrTxt_impl(elem, name, value) }
}

/// C: mjXUtil::WriteAttrInt (xml/xml_util.h:201)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_write_attr_int(elem: *mut tinyxml2__XMLElement, name: string, data: i32, def: i32) {
    extern "C" { fn mjXUtil_WriteAttrInt_impl(elem: *mut tinyxml2__XMLElement, name: string, data: i32, def: i32); }
    // SAFETY: delegates to C implementation
    unsafe { mjXUtil_WriteAttrInt_impl(elem, name, data, def) }
}

/// C: mjXUtil::WriteAttrKey (xml/xml_util.h:204)
/// Calls: mjXUtil::WriteAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_write_attr_key(elem: *mut tinyxml2__XMLElement, name: string, map: *const mjMap, mapsz: i32, data: i32, def: i32) {
    extern "C" { fn mjXUtil_WriteAttrKey_impl(elem: *mut tinyxml2__XMLElement, name: string, map: *const mjMap, mapsz: i32, data: i32, def: i32); }
    // SAFETY: delegates to C implementation
    unsafe { mjXUtil_WriteAttrKey_impl(elem, name, map, mapsz, data, def) }
}

/// C: mjXUtil::WriteAttrKeys (xml/xml_util.h:208)
/// Calls: mjXUtil::WriteAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_write_attr_keys(elem: *mut XMLElement, name: string, map: *const mjMap, mapsz: i32, data: *mut i32, ndata: i32, def: i32) {
    extern "C" { fn mjXUtil_WriteAttrKeys_impl(elem: *mut XMLElement, name: string, map: *const mjMap, mapsz: i32, data: *mut i32, ndata: i32, def: i32); }
    // SAFETY: delegates to C implementation
    unsafe { mjXUtil_WriteAttrKeys_impl(elem, name, map, mapsz, data, ndata, def) }
}

/// C: mjXUtil::ReadAttrValues (xml/xml_util.h:213)
/// Calls: mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_values(elem: *mut tinyxml2__XMLElement, attr: *const i8, push: *const (), max: i32) -> bool {
    extern "C" { fn mj_x_util_read_attr_values_impl(elem: *mut tinyxml2__XMLElement, attr: *const i8, push: *const (), max: i32) -> bool; }
    // SAFETY: delegates to C implementation
    unsafe { mj_x_util_read_attr_values_impl(elem, attr, push, max) }
}

