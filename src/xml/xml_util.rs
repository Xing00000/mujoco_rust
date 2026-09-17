//! Port of: xml/xml_util.cc
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: ParseInfOrNan (xml/xml_util.cc:59)
#[allow(unused_variables, non_snake_case)]
pub fn parse_inf_or_nan__710379b3524f8e4b(s: *const anonymous_namespace___string) -> *const () {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_util.cc:__ZN12_GLOBAL__N_113ParseInfOrNanIdEENSt3__18optionalIT_EERKNS1_12basic_stringIcNS1_11char_traitsIcEENS1_9allocatorIcEEEE
}

/// C: ParseInfOrNan (xml/xml_util.cc:59)
#[allow(unused_variables, non_snake_case)]
pub fn parse_inf_or_nan__294bcdd3b68b5f45(s: *const anonymous_namespace___string) -> *const () {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_util.cc:__ZN12_GLOBAL__N_113ParseInfOrNanIfEENSt3__18optionalIT_EERKNS1_12basic_stringIcNS1_11char_traitsIcEENS1_9allocatorIcEEEE
}

/// C: ParseInfOrNan (xml/xml_util.cc:59)
#[allow(unused_variables, non_snake_case)]
pub fn parse_inf_or_nan__b033f087c3fec07c(s: *const anonymous_namespace___string) -> *const () {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_util.cc:__ZN12_GLOBAL__N_113ParseInfOrNanIhEENSt3__18optionalIT_EERKNS1_12basic_stringIcNS1_11char_traitsIcEENS1_9allocatorIcEEEE
}

/// C: ParseInfOrNan (xml/xml_util.cc:59)
#[allow(unused_variables, non_snake_case)]
pub fn parse_inf_or_nan__848eec1f9ba79dc3(s: *const anonymous_namespace___string) -> *const () {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_util.cc:__ZN12_GLOBAL__N_113ParseInfOrNanIiEENSt3__18optionalIT_EERKNS1_12basic_stringIcNS1_11char_traitsIcEENS1_9allocatorIcEEEE
}

/// C: ResolveFilePath (xml/xml_util.cc:84)
/// Calls: cxx:__ZNK6mujoco4user8FilePath5IsAbsEv, cxx:__ZNK6mujoco4user8FilePath5c_strEv, cxx:_mju_closeResource, cxx:_mju_openResource
#[allow(unused_variables, non_snake_case)]
pub fn resolve_file_path(e: *mut tinyxml2__XMLElement, filename: *const mujoco__user__FilePath, dir: *const mujoco__user__FilePath, vfs: *const mjVFS) -> mujoco__user__FilePath {
    todo!() // cxx-internal:xml_util.cc.o:__ZN12_GLOBAL__N_115ResolveFilePathEPN8tinyxml210XMLElementERKN6mujoco4user8FilePathES7_PK6mjVFS_
}

/// C: AccumulateFiles (xml/xml_util.cc:111)
/// Calls: cxx:__ZNK6mujoco4user8FilePath3StrEv, cxx:__ZNK6mujoco4user8FilePath5IsAbsEv, cxx:_mju_getXMLDependencies
#[allow(unused_variables, non_snake_case)]
pub fn accumulate_files(files: *const (), root: *mut tinyxml2__XMLElement, model_dir: *const mujoco__user__FilePath) {
    todo!() // cxx-internal:xml_util.cc.o:__ZN12_GLOBAL__N_115AccumulateFilesERNSt3__113unordered_setINS0_12basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEENS0_4hashIS7_EENS0_8equal_toIS7_EENS5_IS7_EEEEPN8tinyxml210XMLElementERKN6mujoco4user8FilePathE
}

/// C: mjCopyError (xml/xml_util.cc:209)
#[allow(unused_variables, non_snake_case)]
pub fn mj_copy_error(dst: *mut i8, src: *const i8, maxlen: i32) {
    todo!() // cxx:__Z11mjCopyErrorPcPKci
}

/// C: mju_getXMLDependencies (xml/xml_util.cc:216)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_util.cc:__ZN12_GLOBAL__N_115AccumulateFilesERNSt3__113unordered_setINS0_12basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEENS0_4hashIS7_EENS0_8equal_toIS7_EENS5_IS7_EEEEPN8tinyxml210XMLElementERKN6mujoco4user8FilePathE, cxx:_mju_closeResource, cxx:_mju_error, cxx:_mju_getResourceDir, cxx:_mju_openResource, cxx:_mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn mju_get_xml_dependencies(filename: *const i8, dependencies: *mut mjStringVec) {
    todo!() // cxx:_mju_getXMLDependencies
}

/// C: FirstChildElement (xml/xml_util.cc:278)
#[allow(unused_variables, non_snake_case)]
pub fn first_child_element(e: *mut tinyxml2__XMLElement, name: *const i8) -> *mut tinyxml2__XMLElement {
    todo!() // cxx:__Z17FirstChildElementPN8tinyxml210XMLElementEPKc
}

/// C: NextSiblingElement (xml/xml_util.cc:292)
/// Calls: cxx:__Z17FirstChildElementPN8tinyxml210XMLElementEPKc
#[allow(unused_variables, non_snake_case)]
pub fn next_sibling_element(e: *mut tinyxml2__XMLElement, name: *const i8) -> *mut tinyxml2__XMLElement {
    todo!() // cxx:__Z18NextSiblingElementPN8tinyxml210XMLElementEPKc
}

/// C: BundleList (xml/xml_util.cc:369)
#[allow(unused_variables, non_snake_case)]
pub fn bundle_list(bundles: *const ()) -> anonymous_namespace___string {
    todo!() // cxx-internal:xml_util.cc.o:__ZL10BundleListRKNSt3__16vectorINS0_INS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEENS4_IS6_EEEENS4_IS8_EEEE
}

/// C: mjXSchema::CheckConstraints (xml/xml_util.cc:388)
/// Calls: cxx-internal:xml_util.cc.o:__ZL10BundleListRKNSt3__16vectorINS0_INS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEENS4_IS6_EEEENS4_IS8_EEEE
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_schema_check_constraints(self_ptr: *mut mjXSchema, elem: *mut tinyxml2__XMLElement) -> *mut tinyxml2__XMLElement {
    todo!() // cxx:__ZN9mjXSchema16CheckConstraintsEPN8tinyxml210XMLElementE
}

/// C: mjXSchema::GetError (xml/xml_util.cc:456)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_schema_get_error(self_ptr: *mut mjXSchema) -> anonymous_namespace___string {
    todo!() // cxx:__ZN9mjXSchema8GetErrorEv
}

/// C: printspace (xml/xml_util.cc:462)
#[allow(unused_variables, non_snake_case)]
pub fn printspace(str: *mut anonymous_namespace___stringstream, n: i32, space: *const i8) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_util.cc:__ZL10printspaceRNSt3__118basic_stringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEEEiPKc
}

/// C: mjXSchema::Print (xml/xml_util.cc:468)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_util.cc:__ZL10printspaceRNSt3__118basic_stringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEEEiPKc
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_schema_print(self_ptr: *mut mjXSchema, str: *mut anonymous_namespace___stringstream, level: i32) {
    todo!() // cxx:__ZNK9mjXSchema5PrintERNSt3__118basic_stringstreamIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEi
}

/// C: mjXSchema::PrintHTML (xml/xml_util.cc:497)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_util.cc:__ZL10printspaceRNSt3__118basic_stringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEEEiPKc
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_schema_print_html(self_ptr: *mut mjXSchema, str: *mut anonymous_namespace___stringstream, level: i32, pad: bool) {
    todo!() // cxx:__ZNK9mjXSchema9PrintHTMLERNSt3__118basic_stringstreamIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEib
}

/// C: mjXSchema::NameMatch (xml/xml_util.cc:538)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_schema_name_match(self_ptr: *mut mjXSchema, elem: *mut tinyxml2__XMLElement, level: i32) -> bool {
    todo!() // cxx:__ZN9mjXSchema9NameMatchEPN8tinyxml210XMLElementEi
}

/// C: mjXSchema::Check (xml/xml_util.cc:553)
/// Calls: cxx:__Z17FirstChildElementPN8tinyxml210XMLElementEPKc, cxx:__Z18NextSiblingElementPN8tinyxml210XMLElementEPKc
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_schema_check(self_ptr: *mut mjXSchema, elem: *mut tinyxml2__XMLElement, level: i32) -> *mut tinyxml2__XMLElement {
    todo!() // cxx:__ZN9mjXSchema5CheckEPN8tinyxml210XMLElementEi
}

/// C: mjXUtil::ReadAttrValues (xml/xml_util.cc:667)
/// Calls: cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_values__76eb02c229306fb4(elem: *mut tinyxml2__XMLElement, attr: *const i8, push: *const (), max: i32) -> bool {
    todo!() // cxx:__ZN7mjXUtil14ReadAttrValuesIdEEbPN8tinyxml210XMLElementEPKcNSt3__18functionIFviT_EEEi
}

/// C: mjXUtil::ReadAttrValues (xml/xml_util.cc:667)
/// Calls: cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_values__2a87e73a60f7e080(elem: *mut tinyxml2__XMLElement, attr: *const i8, push: *const (), max: i32) -> bool {
    todo!() // cxx:__ZN7mjXUtil14ReadAttrValuesIfEEbPN8tinyxml210XMLElementEPKcNSt3__18functionIFviT_EEEi
}

/// C: mjXUtil::ReadAttrValues (xml/xml_util.cc:667)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_values__f2da1fd5b257fc96(elem: *mut tinyxml2__XMLElement, attr: *const i8, push: *const (), max: i32) -> bool {
    todo!() // cxx:__ZN7mjXUtil14ReadAttrValuesIhEEbPN8tinyxml210XMLElementEPKcNSt3__18functionIFviT_EEEi
}

/// C: mjXUtil::ReadAttrValues (xml/xml_util.cc:667)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_values__2f7a69f5d8d051c3(elem: *mut tinyxml2__XMLElement, attr: *const i8, push: *const (), max: i32) -> bool {
    todo!() // cxx:__ZN7mjXUtil14ReadAttrValuesIiEEbPN8tinyxml210XMLElementEPKcNSt3__18functionIFviT_EEEi
}

/// C: mjXUtil::SameVector (xml/xml_util.cc:727)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_same_vector__3c3c892281a48ab1(vec1: *const f64, vec2: *const f64, n: i32) -> bool {
    todo!() // cxx:__ZN7mjXUtil10SameVectorIdEEbPKT_S3_i
}

/// C: mjXUtil::SameVector (xml/xml_util.cc:727)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_same_vector__6f1576b134171c35(vec1: *const f32, vec2: *const f32, n: i32) -> bool {
    todo!() // cxx:__ZN7mjXUtil10SameVectorIfEEbPKT_S3_i
}

/// C: mjXUtil::SameVector (xml/xml_util.cc:727)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_same_vector__b73a3b0acefd4f83(vec1: *const u8, vec2: *const u8, n: i32) -> bool {
    todo!() // cxx:__ZN7mjXUtil10SameVectorIhEEbPKT_S3_i
}

/// C: mjXUtil::SameVector (xml/xml_util.cc:727)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_same_vector__89bee67c7c70014f(vec1: *const i32, vec2: *const i32, n: i32) -> bool {
    todo!() // cxx:__ZN7mjXUtil10SameVectorIiEEbPKT_S3_i
}

/// C: mjXUtil::FindKey (xml/xml_util.cc:744)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_find_key(self_ptr: *mut mjXUtil, map: *const mjMap, mapsz: i32, key: std__string) -> i32 {
    todo!() // cxx:__ZN7mjXUtil7FindKeyEPK6_mjMapiNSt3__112basic_stringIcNS3_11char_traitsIcEENS3_9allocatorIcEEEE
}

/// C: mjXUtil::FindValue (xml/xml_util.cc:754)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_find_value(self_ptr: *mut mjXUtil, map: *const mjMap, mapsz: i32, value: i32) -> anonymous_namespace___string {
    todo!() // cxx:__ZN7mjXUtil9FindValueEPK6_mjMapii
}

/// C: mjXUtil::ReadAttrVec (xml/xml_util.cc:765)
/// Calls: cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_vec__d0574b8ad6e080d5(elem: *mut tinyxml2__XMLElement, attr: *const i8, required: bool) -> *const () {
    todo!() // cxx:__ZN7mjXUtil11ReadAttrVecIdEENSt3__18optionalINS1_6vectorIT_NS1_9allocatorIS4_EEEEEEPN8tinyxml210XMLElementEPKcb
}

/// C: mjXUtil::ReadAttrVec (xml/xml_util.cc:765)
/// Calls: cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_vec__2e3109540bc6a9f0(elem: *mut tinyxml2__XMLElement, attr: *const i8, required: bool) -> *const () {
    todo!() // cxx:__ZN7mjXUtil11ReadAttrVecIfEENSt3__18optionalINS1_6vectorIT_NS1_9allocatorIS4_EEEEEEPN8tinyxml210XMLElementEPKcb
}

/// C: mjXUtil::ReadAttrVec (xml/xml_util.cc:765)
/// Calls: cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_vec__4f9ba9ef948210f1(elem: *mut tinyxml2__XMLElement, attr: *const i8, required: bool) -> *const () {
    todo!() // cxx:__ZN7mjXUtil11ReadAttrVecIhEENSt3__18optionalINS1_6vectorIT_NS1_9allocatorIS4_EEEEEEPN8tinyxml210XMLElementEPKcb
}

/// C: mjXUtil::ReadAttrVec (xml/xml_util.cc:765)
/// Calls: cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_vec__f2c363c0b191419d(elem: *mut tinyxml2__XMLElement, attr: *const i8, required: bool) -> *const () {
    todo!() // cxx:__ZN7mjXUtil11ReadAttrVecIiEENSt3__18optionalINS1_6vectorIT_NS1_9allocatorIS4_EEEEEEPN8tinyxml210XMLElementEPKcb
}

/// C: mjXUtil::ReadAttrStr (xml/xml_util.cc:807)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_str(self_ptr: *mut mjXUtil, elem: *mut tinyxml2__XMLElement, attr: *const i8, required: bool) -> *const () {
    todo!() // cxx:__ZN7mjXUtil11ReadAttrStrEPN8tinyxml210XMLElementEPKcb
}

/// C: mjXUtil::ReadAttrFile (xml/xml_util.cc:823)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_util.cc:__ZN12_GLOBAL__N_115ResolveFilePathEPN8tinyxml210XMLElementERKN6mujoco4user8FilePathES7_PK6mjVFS_
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_file(self_ptr: *mut mjXUtil, elem: *mut tinyxml2__XMLElement, attr: *const i8, vfs: *const mjVFS, dir: *const mujoco__user__FilePath, required: bool) -> *const () {
    todo!() // cxx:__ZN7mjXUtil12ReadAttrFileEPN8tinyxml210XMLElementEPKcPK6mjVFS_RKN6mujoco4user8FilePathEb
}

/// C: mjXUtil::ReadAttrNum (xml/xml_util.cc:833)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_num__2fc9ebbdb49ec653(elem: *mut tinyxml2__XMLElement, attr: *const i8, required: bool) -> *const () {
    todo!() // cxx:__ZN7mjXUtil11ReadAttrNumIdEENSt3__18optionalIT_EEPN8tinyxml210XMLElementEPKcb
}

/// C: mjXUtil::ReadAttrNum (xml/xml_util.cc:833)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_num__12e89f3703ebb4d2(elem: *mut tinyxml2__XMLElement, attr: *const i8, required: bool) -> *const () {
    todo!() // cxx:__ZN7mjXUtil11ReadAttrNumIfEENSt3__18optionalIT_EEPN8tinyxml210XMLElementEPKcb
}

/// C: mjXUtil::ReadAttrNum (xml/xml_util.cc:833)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_num__8410ca45eca73f45(elem: *mut tinyxml2__XMLElement, attr: *const i8, required: bool) -> *const () {
    todo!() // cxx:__ZN7mjXUtil11ReadAttrNumIhEENSt3__18optionalIT_EEPN8tinyxml210XMLElementEPKcb
}

/// C: mjXUtil::ReadAttrNum (xml/xml_util.cc:833)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_num__d58e52b1f1592b16(elem: *mut tinyxml2__XMLElement, attr: *const i8, required: bool) -> *const () {
    todo!() // cxx:__ZN7mjXUtil11ReadAttrNumIiEENSt3__18optionalIT_EEPN8tinyxml210XMLElementEPKcb
}

/// C: mjXUtil::ReadAttr (xml/xml_util.cc:853)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr__22014830fa93e82c(elem: *mut tinyxml2__XMLElement, attr: *const i8, len: i32, data: *mut f64, text: *mut anonymous_namespace___string, required: bool, exact: bool) -> i32 {
    todo!() // cxx:__ZN7mjXUtil8ReadAttrIdEEiPN8tinyxml210XMLElementEPKciPT_RNSt3__112basic_stringIcNS8_11char_traitsIcEENS8_9allocatorIcEEEEbb
}

/// C: mjXUtil::ReadAttr (xml/xml_util.cc:853)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr__19f4ef690d6a20cd(elem: *mut tinyxml2__XMLElement, attr: *const i8, len: i32, data: *mut f32, text: *mut anonymous_namespace___string, required: bool, exact: bool) -> i32 {
    todo!() // cxx:__ZN7mjXUtil8ReadAttrIfEEiPN8tinyxml210XMLElementEPKciPT_RNSt3__112basic_stringIcNS8_11char_traitsIcEENS8_9allocatorIcEEEEbb
}

/// C: mjXUtil::ReadAttr (xml/xml_util.cc:853)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr__1eae55baddc6e945(elem: *mut tinyxml2__XMLElement, attr: *const i8, len: i32, data: *mut u8, text: *mut anonymous_namespace___string, required: bool, exact: bool) -> i32 {
    todo!() // cxx:__ZN7mjXUtil8ReadAttrIhEEiPN8tinyxml210XMLElementEPKciPT_RNSt3__112basic_stringIcNS8_11char_traitsIcEENS8_9allocatorIcEEEEbb
}

/// C: mjXUtil::ReadAttr (xml/xml_util.cc:853)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr__bcc935075ff4abf6(elem: *mut tinyxml2__XMLElement, attr: *const i8, len: i32, data: *mut i32, text: *mut anonymous_namespace___string, required: bool, exact: bool) -> i32 {
    todo!() // cxx:__ZN7mjXUtil8ReadAttrIiEEiPN8tinyxml210XMLElementEPKciPT_RNSt3__112basic_stringIcNS8_11char_traitsIcEENS8_9allocatorIcEEEEbb
}

/// C: mjXUtil::ReadQuat (xml/xml_util.cc:909)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_quat(self_ptr: *mut mjXUtil, elem: *mut tinyxml2__XMLElement, attr: *const i8, data: *mut f64, text: *mut anonymous_namespace___string, required: bool) -> i32 {
    todo!() // cxx:__ZN7mjXUtil8ReadQuatEPN8tinyxml210XMLElementEPKcPdRNSt3__112basic_stringIcNS6_11char_traitsIcEENS6_9allocatorIcEEEEb
}

/// C: mjXUtil::ReadVector (xml/xml_util.cc:923)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_vector(self_ptr: *mut mjXUtil, elem: *mut tinyxml2__XMLElement, attr: *const i8, vec: *const (), text: *mut anonymous_namespace___string, required: bool) -> i32 {
    todo!() // cxx:__ZN7mjXUtil10ReadVectorEPN8tinyxml210XMLElementEPKcRNSt3__16vectorIdNS5_9allocatorIdEEEERNS5_12basic_stringIcNS5_11char_traitsIcEENS7_IcEEEEb
}

/// C: mjXUtil::ReadAttrTxt (xml/xml_util.cc:934)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_txt(self_ptr: *mut mjXUtil, elem: *mut tinyxml2__XMLElement, attr: *const i8, text: *mut anonymous_namespace___string, required: bool) -> bool {
    todo!() // cxx:__ZN7mjXUtil11ReadAttrTxtEPN8tinyxml210XMLElementEPKcRNSt3__112basic_stringIcNS5_11char_traitsIcEENS5_9allocatorIcEEEEb
}

/// C: mjXUtil::ReadAttrInt (xml/xml_util.cc:943)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_int(self_ptr: *mut mjXUtil, elem: *mut tinyxml2__XMLElement, attr: *const i8, data: *mut i32, required: bool) -> bool {
    todo!() // cxx:__ZN7mjXUtil11ReadAttrIntEPN8tinyxml210XMLElementEPKcPib
}

/// C: mjXUtil::Vector2String (xml/xml_util.cc:953)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_vector2string(self_ptr: *mut mjXUtil, txt: *mut anonymous_namespace___string, vec: *const (), ncol: i32) {
    todo!() // cxx:__ZN7mjXUtil13Vector2StringERNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEERKNS0_6vectorIfNS4_IfEEEEi
}

/// C: mjXUtil::FindSubElem (xml/xml_util.cc:969)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_find_sub_elem(self_ptr: *mut mjXUtil, elem: *mut tinyxml2__XMLElement, name: std__string, required: bool) -> *mut tinyxml2__XMLElement {
    todo!() // cxx:__ZN7mjXUtil11FindSubElemEPN8tinyxml210XMLElementENSt3__112basic_stringIcNS3_11char_traitsIcEENS3_9allocatorIcEEEEb
}

/// C: mjXUtil::MapValue (xml/xml_util.cc:994)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_map_value__7ca53f010a2dca64(self_ptr: *mut mjXUtil, elem: *mut tinyxml2__XMLElement, attr: *const i8, data: *mut i32, map: *const mjMap, mapSz: i32, required: bool) -> bool {
    todo!() // cxx:__ZN7mjXUtil8MapValueEPN8tinyxml210XMLElementEPKcPiPK6_mjMapib
}

/// C: mjXUtil::MapValues (xml/xml_util.cc:1011)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_map_values(self_ptr: *mut mjXUtil, elem: *mut tinyxml2__XMLElement, attr: *const i8, data: *mut i32, map: *const mjMap, mapSz: i32, required: bool) -> i32 {
    todo!() // cxx:__ZN7mjXUtil9MapValuesEPN8tinyxml210XMLElementEPKcPiPK6_mjMapib
}

/// C: isint (xml/xml_util.cc:1046)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn isint(x: f64) -> bool {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_util.cc:__ZL5isintd
}

/// C: Round (xml/xml_util.cc:1052)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn round(x: f64) -> i32 {
    todo!() // cxx-internal:xml_util.cc.o:__ZL5Roundd
}

/// C: mjXUtil::WriteAttr (xml/xml_util.cc:1063)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_util.cc:__ZL5Roundd, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_util.cc:__ZL5isintd, cxx:__mjPRIVATE__get_xml_precision
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_write_attr__37c22e1581ea2cac(elem: *mut tinyxml2__XMLElement, name: std__string, n: i32, data: *const f64, def: *const f64, trim: bool) {
    todo!() // cxx:__ZN7mjXUtil9WriteAttrIdEEvPN8tinyxml210XMLElementENSt3__112basic_stringIcNS4_11char_traitsIcEENS4_9allocatorIcEEEEiPKT_SD_b
}

/// C: mjXUtil::WriteAttr (xml/xml_util.cc:1063)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_util.cc:__ZL5Roundd, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_util.cc:__ZL5isintd, cxx:__mjPRIVATE__get_xml_precision
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_write_attr__2fa148a0734ac99f(elem: *mut tinyxml2__XMLElement, name: std__string, n: i32, data: *const f32, def: *const f32, trim: bool) {
    todo!() // cxx:__ZN7mjXUtil9WriteAttrIfEEvPN8tinyxml210XMLElementENSt3__112basic_stringIcNS4_11char_traitsIcEENS4_9allocatorIcEEEEiPKT_SD_b
}

/// C: mjXUtil::WriteAttr (xml/xml_util.cc:1063)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_util.cc:__ZL5Roundd, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_util.cc:__ZL5isintd, cxx:__mjPRIVATE__get_xml_precision
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_write_attr__e73191c06403a697(elem: *mut tinyxml2__XMLElement, name: std__string, n: i32, data: *const u8, def: *const u8, trim: bool) {
    todo!() // cxx:__ZN7mjXUtil9WriteAttrIhEEvPN8tinyxml210XMLElementENSt3__112basic_stringIcNS4_11char_traitsIcEENS4_9allocatorIcEEEEiPKT_SD_b
}

/// C: mjXUtil::WriteAttr (xml/xml_util.cc:1063)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_util.cc:__ZL5Roundd, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_util.cc:__ZL5isintd, cxx:__mjPRIVATE__get_xml_precision
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_write_attr__f17873c42dc98b20(elem: *mut tinyxml2__XMLElement, name: std__string, n: i32, data: *const i32, def: *const i32, trim: bool) {
    todo!() // cxx:__ZN7mjXUtil9WriteAttrIiEEvPN8tinyxml210XMLElementENSt3__112basic_stringIcNS4_11char_traitsIcEENS4_9allocatorIcEEEEiPKT_SD_b
}

/// C: mjXUtil::WriteVector (xml/xml_util.cc:1121)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_write_vector__27d529d1308384c8(self_ptr: *mut mjXUtil, elem: *mut tinyxml2__XMLElement, name: std__string, vec: *const ()) {
    todo!() // cxx:__ZN7mjXUtil11WriteVectorEPN8tinyxml210XMLElementENSt3__112basic_stringIcNS3_11char_traitsIcEENS3_9allocatorIcEEEERKNS3_6vectorIdNS7_IdEEEE
}

/// C: mjXUtil::WriteVector (xml/xml_util.cc:1138)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_write_vector__02c6ec43fd330f56(self_ptr: *mut mjXUtil, elem: *mut tinyxml2__XMLElement, name: std__string, vec: *const (), def: *const ()) {
    todo!() // cxx:__ZN7mjXUtil11WriteVectorEPN8tinyxml210XMLElementENSt3__112basic_stringIcNS3_11char_traitsIcEENS3_9allocatorIcEEEERKNS3_6vectorIdNS7_IdEEEESE_
}

/// C: mjXUtil::WriteAttrTxt (xml/xml_util.cc:1158)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_write_attr_txt(self_ptr: *mut mjXUtil, elem: *mut tinyxml2__XMLElement, name: std__string, value: std__string) {
    todo!() // cxx:__ZN7mjXUtil12WriteAttrTxtEPN8tinyxml210XMLElementENSt3__112basic_stringIcNS3_11char_traitsIcEENS3_9allocatorIcEEEES9_
}

/// C: mjXUtil::WriteAttrInt (xml/xml_util.cc:1168)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_write_attr_int(self_ptr: *mut mjXUtil, elem: *mut tinyxml2__XMLElement, name: std__string, data: i32, def: i32) {
    todo!() // cxx:__ZN7mjXUtil12WriteAttrIntEPN8tinyxml210XMLElementENSt3__112basic_stringIcNS3_11char_traitsIcEENS3_9allocatorIcEEEEii
}

/// C: mjXUtil::WriteAttrKey (xml/xml_util.cc:1177)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_write_attr_key(self_ptr: *mut mjXUtil, elem: *mut tinyxml2__XMLElement, name: std__string, map: *const mjMap, mapsz: i32, data: i32, def: i32) {
    todo!() // cxx:__ZN7mjXUtil12WriteAttrKeyEPN8tinyxml210XMLElementENSt3__112basic_stringIcNS3_11char_traitsIcEENS3_9allocatorIcEEEEPK6_mjMapiii
}

/// C: mjXUtil::WriteAttrKeys (xml/xml_util.cc:1187)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_write_attr_keys(self_ptr: *mut mjXUtil, elem: *mut tinyxml2__XMLElement, name: std__string, map: *const mjMap, mapsz: i32, data: *mut i32, ndata: i32, def: i32) {
    todo!() // cxx:__ZN7mjXUtil13WriteAttrKeysEPN8tinyxml210XMLElementENSt3__112basic_stringIcNS3_11char_traitsIcEENS3_9allocatorIcEEEEPK6_mjMapiPiii
}

/// C: mjXUtil::ReadAttrArr (xml/xml_util.h:148)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_arr__48d358e625458b02(elem: *mut tinyxml2__XMLElement, attr: *const i8, required: bool) -> *const () {
    todo!() // cxx:__ZN7mjXUtil11ReadAttrArrIdLi1EEENSt3__18optionalINS1_5arrayIT_XT0_EEEEEPN8tinyxml210XMLElementEPKcb
}

/// C: mjXUtil::ReadAttrArr (xml/xml_util.h:148)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_arr__185dd5d86f16b6b9(elem: *mut tinyxml2__XMLElement, attr: *const i8, required: bool) -> *const () {
    todo!() // cxx:__ZN7mjXUtil11ReadAttrArrIdLi3EEENSt3__18optionalINS1_5arrayIT_XT0_EEEEEPN8tinyxml210XMLElementEPKcb
}

/// C: mjXUtil::ReadAttrArr (xml/xml_util.h:148)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_arr__f584df3b06fcb333(elem: *mut tinyxml2__XMLElement, attr: *const i8, required: bool) -> *const () {
    todo!() // cxx:__ZN7mjXUtil11ReadAttrArrIfLi1EEENSt3__18optionalINS1_5arrayIT_XT0_EEEEEPN8tinyxml210XMLElementEPKcb
}

/// C: mjXUtil::ReadAttrArr (xml/xml_util.h:148)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_arr__aa481060c95ac66a(elem: *mut tinyxml2__XMLElement, attr: *const i8, required: bool) -> *const () {
    todo!() // cxx:__ZN7mjXUtil11ReadAttrArrIhLi1EEENSt3__18optionalINS1_5arrayIT_XT0_EEEEEPN8tinyxml210XMLElementEPKcb
}

/// C: mjXUtil::ReadAttrArr (xml/xml_util.h:148)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_arr__8ab2197b3bcced6e(elem: *mut tinyxml2__XMLElement, attr: *const i8, required: bool) -> *const () {
    todo!() // cxx:__ZN7mjXUtil11ReadAttrArrIiLi1EEENSt3__18optionalINS1_5arrayIT_XT0_EEEEEPN8tinyxml210XMLElementEPKcb
}

/// C: mjXUtil::MapValue (xml/xml_util.h:229)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_map_value__2b4e38d6a1c52459(elem: *mut tinyxml2__XMLElement, attr: *const i8, data: *mut u32, map: *const mjMap, mapSz: i32, required: bool) -> bool {
    todo!() // cxx:__ZN7mjXUtil8MapValueI10mjtLimitedEEbPN8tinyxml210XMLElementEPKcPT_PK6_mjMapib
}

/// C: mjXUtil::MapValue (xml/xml_util.h:229)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_map_value__438a881cf37d9230(elem: *mut tinyxml2__XMLElement, attr: *const i8, data: *mut u32, map: *const mjMap, mapSz: i32, required: bool) -> bool {
    todo!() // cxx:__ZN7mjXUtil8MapValueI11mjtFlexSelfEEbPN8tinyxml210XMLElementEPKcPT_PK6_mjMapib
}

/// C: mjXUtil::MapValue (xml/xml_util.h:229)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_map_value__484757dbd1d38d17(elem: *mut tinyxml2__XMLElement, attr: *const i8, data: *mut u32, map: *const mjMap, mapSz: i32, required: bool) -> bool {
    todo!() // cxx:__ZN7mjXUtil8MapValueI12mjtAlignFreeEEbPN8tinyxml210XMLElementEPKcPT_PK6_mjMapib
}

