//! Port of: user/user_api.cc
//! IR hash: 32301b9dc9774d55
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_parse (user/user_api.cc:78)
/// Calls: mj_defaultVFS, mj_deleteVFS, mj_parseXML, mju_closeResource, mju_decodeResource, mju_free, mju_malloc, mju_openResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_parse(filename: *const i8, content_type: *const i8, vfs: *const mjVFS, error: *mut i8, error_sz: i32) -> *mut mjSpec {
    // NOTE: signature changed from previous IR version
    // Previous params: (filename : * const i8, content_type : * const i8, vfs : * const mjVFS, error : * mut i8, error_sz : i32)
    // Previous return: * mut mjSpec
    todo!("re-translate: params renamed")
}

/// C: mj_encode (user/user_api.cc:151)
/// Calls: mj_saveXML, mjp_findEncoder
#[allow(unused_variables, non_snake_case)]
pub fn mj_encode(s: *const mjSpec, m: *const mjModel, filename: *const i8, content_type: *const i8, vfs: *const mjVFS, error: *mut i8, error_sz: i32) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (s : * const mjSpec, m : * const mjModel, filename : * const i8, content_type : * const i8, vfs : * const mjVFS, error : * mut i8, error_sz : i32)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: LogCompileTime (user/user_api.cc:212)
/// Calls: mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn log_compile_time(t: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (t : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: SetFrame (user/user_api.cc:293)
/// Calls: mjs_firstChild, mjs_getFrame, mjs_nextChild, mjs_setFrame
#[allow(unused_variables, non_snake_case)]
pub fn set_frame(body: *mut mjsBody, objtype: u32, frame: *mut mjsFrame) {
    // NOTE: signature changed from previous IR version
    // Previous params: (body : * mut mjsBody, objtype : u32, frame : * mut mjsFrame)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: attachBody (user/user_api.cc:306)
#[allow(unused_variables, non_snake_case)]
pub fn attach_body(parent: *mut mjCFrame, child: *const mjCBody, prefix: *const i8, suffix: *const i8) -> *mut mjsElement {
    // NOTE: signature changed from previous IR version
    // Previous params: (parent : * mut mjCFrame, child : * const mjCBody, prefix : * const i8, suffix : * const i8)
    // Previous return: * mut mjsElement
    todo!("re-translate: params renamed")
}

/// C: attachFrame (user/user_api.cc:325)
#[allow(unused_variables, non_snake_case)]
pub fn attach_frame(parent: *mut mjCBody, child: *const mjCFrame, prefix: *const i8, suffix: *const i8) -> *mut mjsElement {
    // NOTE: signature changed from previous IR version
    // Previous params: (parent : * mut mjCBody, child : * const mjCFrame, prefix : * const i8, suffix : * const i8)
    // Previous return: * mut mjsElement
    todo!("re-translate: params renamed")
}

/// C: attachToSite (user/user_api.cc:344)
/// Calls: attachBody, mjCBody::AddFrame, mjCFrame::SetParent, mjCSite::Body
#[allow(unused_variables, non_snake_case)]
pub fn attach_to_site(parent: *mut mjCSite, child: *const mjCBody, prefix: *const i8, suffix: *const i8) -> *mut mjsElement {
    // NOTE: signature changed from previous IR version
    // Previous params: (parent : * mut mjCSite, child : * const mjCBody, prefix : * const i8, suffix : * const i8)
    // Previous return: * mut mjsElement
    todo!("re-translate: params renamed")
}

/// C: attachFrameToSite (user/user_api.cc:365)
/// Calls: attachFrame, mjCBody::AddFrame, mjCFrame::SetParent, mjCSite::Body
#[allow(unused_variables, non_snake_case)]
pub fn attach_frame_to_site(parent: *mut mjCSite, child: *const mjCFrame, prefix: *const i8, suffix: *const i8) -> *mut mjsElement {
    // NOTE: signature changed from previous IR version
    // Previous params: (parent : * mut mjCSite, child : * const mjCFrame, prefix : * const i8, suffix : * const i8)
    // Previous return: * mut mjsElement
    todo!("re-translate: params renamed")
}

/// C: mjs_getTimer (user/user_api.cc:515)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_timer(s: *mut mjSpec) -> *const f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (s : * mut mjSpec)
    // Previous return: * const f64
    todo!("re-translate: params renamed")
}

/// C: mjs_numWarnings (user/user_api.cc:535)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_num_warnings(spec: *const mjSpec) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (spec : * const mjSpec)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjs_getWarning (user/user_api.cc:544)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_warning(spec: *const mjSpec, index: i32) -> *const i8 {
    // NOTE: signature changed from previous IR version
    // Previous params: (spec : * const mjSpec, index : i32)
    // Previous return: * const i8
    todo!("re-translate: params renamed")
}

/// C: FlexcompTypeFromStr (user/user_api.cc:709)
#[allow(unused_variables, non_snake_case)]
pub fn flexcomp_type_from_str(r#type: *const i8) -> mjtFcompType {
    // NOTE: signature changed from previous IR version
    // Previous params: (r#type : * const i8)
    // Previous return: mjtFcompType
    todo!("re-translate: params renamed")
}

/// C: FlexcompDofFromStr (user/user_api.cc:724)
#[allow(unused_variables, non_snake_case)]
pub fn flexcomp_dof_from_str(dof: *const i8) -> mjtDof {
    // NOTE: signature changed from previous IR version
    // Previous params: (dof : * const i8)
    // Previous return: mjtDof
    todo!("re-translate: params renamed")
}

/// C: mjs_getCompiler (user/user_api.cc:1553)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_compiler(element: *const mjsElement) -> *mut mjsCompiler {
    // NOTE: signature changed from previous IR version
    // Previous params: (element : * const mjsElement)
    // Previous return: * mut mjsCompiler
    todo!("re-translate: params renamed")
}

/// C: mjs_setBuffer (user/user_api.cc:2230)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_buffer(dest: *mut i32, array: *const (), size: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (dest : * mut i32, array : * const (), size : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjs_setString (user/user_api.cc:2240)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_string(dest: *mut i32, text: *const i8) {
    // NOTE: signature changed from previous IR version
    // Previous params: (dest : * mut i32, text : * const i8)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjs_setInStringVec (user/user_api.cc:2248)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_in_string_vec(dest: *mut i32, i: i32, text: *const i8) -> mjtBool {
    // NOTE: signature changed from previous IR version
    // Previous params: (dest : * mut i32, i : i32, text : * const i8)
    // Previous return: mjtBool
    todo!("re-translate: params renamed")
}

/// C: mjs_setStringVec (user/user_api.cc:2260)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_string_vec(dest: *mut i32, text: *const i8) {
    // NOTE: signature changed from previous IR version
    // Previous params: (dest : * mut i32, text : * const i8)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjs_appendString (user/user_api.cc:2268)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_append_string(dest: *mut i32, text: *const i8) {
    // NOTE: signature changed from previous IR version
    // Previous params: (dest : * mut i32, text : * const i8)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjs_setInt (user/user_api.cc:2274)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_int(dest: *mut i32, array: *const i32, size: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (dest : * mut i32, array : * const i32, size : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjs_appendIntVec (user/user_api.cc:2284)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_append_int_vec(dest: *mut i32, array: *const i32, size: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (dest : * mut i32, array : * const i32, size : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjs_setFloat (user/user_api.cc:2291)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_float(dest: *mut i32, array: *const f32, size: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (dest : * mut i32, array : * const f32, size : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjs_appendFloatVec (user/user_api.cc:2302)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_append_float_vec(dest: *mut i32, array: *const f32, size: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (dest : * mut i32, array : * const f32, size : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjs_setDouble (user/user_api.cc:2309)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_double(dest: *mut i32, array: *const f64, size: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (dest : * mut i32, array : * const f64, size : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjs_getName (user/user_api.cc:2319)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_name(element: *mut mjsElement) -> *mut i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (element : * mut mjsElement)
    // Previous return: * mut i32
    todo!("re-translate: params renamed")
}

/// C: mjs_getString (user/user_api.cc:2329)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_string(source: *const i32) -> *const i8 {
    // NOTE: signature changed from previous IR version
    // Previous params: (source : * const i32)
    // Previous return: * const i8
    todo!("re-translate: params renamed")
}

/// C: mjs_getDouble (user/user_api.cc:2336)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_double(source: *const i32, size: *mut i32) -> *const f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (source : * const i32, size : * mut i32)
    // Previous return: * const f64
    todo!("re-translate: params renamed")
}

/// C: mj_makeSpec (user/user_api.h:40)
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_spec() -> *mut mjSpec {
    todo!() // mj_makeSpec
}

/// C: mj_compile (user/user_api.h:43)
/// Calls: LogCompileTime, mjCModel::Compile
#[allow(unused_variables, non_snake_case)]
pub fn mj_compile(s: *mut mjSpec, vfs: *const mjVFS) -> *mut mjModel {
    // NOTE: signature changed from previous IR version
    // Previous params: (s : * mut mjSpec, vfs : * const mjVFS)
    // Previous return: * mut mjModel
    todo!("re-translate: params renamed")
}

/// C: mj_recompile (user/user_api.h:46)
/// Calls: mjCModel::Compile, mjCModel::MakeData, mjCModel::SetError, mj_deleteData
#[allow(unused_variables, non_snake_case)]
pub fn mj_recompile(s: *mut mjSpec, vfs: *const mjVFS, m: *mut mjModel, d: *mut mjData) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (s : * mut mjSpec, vfs : * const mjVFS, m : * mut mjModel, d : * mut mjData)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mj_copySpec (user/user_api.h:49)
#[allow(unused_variables, non_snake_case)]
pub fn mj_copy_spec(s: *const mjSpec) -> *mut mjSpec {
    // NOTE: signature changed from previous IR version
    // Previous params: (s : * const mjSpec)
    // Previous return: * mut mjSpec
    todo!("re-translate: params renamed")
}

/// C: mjs_getError (user/user_api.h:52)
/// Calls: mjCModel::GetError, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_error(s: *mut mjSpec) -> *const i8 {
    // NOTE: signature changed from previous IR version
    // Previous params: (s : * mut mjSpec)
    // Previous return: * const i8
    todo!("re-translate: params renamed")
}

/// C: mjs_isWarning (user/user_api.h:55)
/// Calls: mjCModel::GetError
#[allow(unused_variables, non_snake_case)]
pub fn mjs_is_warning(s: *mut mjSpec) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (s : * mut mjSpec)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mj_deleteSpec (user/user_api.h:58)
/// Calls: mjCModel::Release
#[allow(unused_variables, non_snake_case)]
pub fn mj_delete_spec(s: *mut mjSpec) {
    // NOTE: signature changed from previous IR version
    // Previous params: (s : * mut mjSpec)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjs_addSpec (user/user_api.h:61)
/// Calls: mjCModel::AppendSpec
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_spec(s: *mut mjSpec, child: *mut mjSpec) {
    // NOTE: signature changed from previous IR version
    // Previous params: (s : * mut mjSpec, child : * mut mjSpec)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjs_activatePlugin (user/user_api.h:64)
/// Calls: mjCModel::ActivatePlugin, mjp_getPlugin
#[allow(unused_variables, non_snake_case)]
pub fn mjs_activate_plugin(s: *mut mjSpec, name: *const i8) -> i32 {
    todo!() // mjs_activatePlugin
}

/// C: mjs_setDeepCopy (user/user_api.h:67)
/// Calls: mjCModel::SetDeepCopy
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_deep_copy(s: *mut mjSpec, deepcopy: i32) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (s : * mut mjSpec, deepcopy : i32)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mj_copyBack (user/user_api.h:70)
/// Calls: mjCModel::CopyBack
#[allow(unused_variables, non_snake_case)]
pub fn mj_copy_back(s: *mut mjSpec, m: *const mjModel) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (s : * mut mjSpec, m : * const mjModel)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjs_attach (user/user_api.h:76)
/// Calls: SetFrame, mjCModel::SetAttachWarningBoundary, mjCModel::SetError, mjs_addFrame, mjs_getParent, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjs_attach(parent: *mut mjsElement, child: *const mjsElement, prefix: *const i8, suffix: *const i8) -> *mut mjsElement {
    // NOTE: signature changed from previous IR version
    // Previous params: (parent : * mut mjsElement, child : * const mjsElement, prefix : * const i8, suffix : * const i8)
    // Previous return: * mut mjsElement
    todo!("re-translate: params renamed")
}

/// C: mjs_addBody (user/user_api.h:83)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_body(body: *mut mjsBody, def: *const mjsDefault) -> *mut mjsBody {
    // NOTE: signature changed from previous IR version
    // Previous params: (body : * mut mjsBody, def : * const mjsDefault)
    // Previous return: * mut mjsBody
    todo!("re-translate: params renamed")
}

/// C: mjs_addSite (user/user_api.h:86)
/// Calls: mjCBody::AddSite
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_site(body: *mut mjsBody, def: *const mjsDefault) -> *mut mjsSite {
    // NOTE: signature changed from previous IR version
    // Previous params: (body : * mut mjsBody, def : * const mjsDefault)
    // Previous return: * mut mjsSite
    todo!("re-translate: params renamed")
}

/// C: mjs_addJoint (user/user_api.h:89)
/// Calls: mjCBody::AddJoint
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_joint(body: *mut mjsBody, def: *const mjsDefault) -> *mut mjsJoint {
    // NOTE: signature changed from previous IR version
    // Previous params: (body : * mut mjsBody, def : * const mjsDefault)
    // Previous return: * mut mjsJoint
    todo!("re-translate: params renamed")
}

/// C: mjs_addFreeJoint (user/user_api.h:92)
/// Calls: mjCBody::AddFreeJoint
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_free_joint(body: *mut mjsBody) -> *mut mjsJoint {
    // NOTE: signature changed from previous IR version
    // Previous params: (body : * mut mjsBody)
    // Previous return: * mut mjsJoint
    todo!("re-translate: params renamed")
}

/// C: mjs_addGeom (user/user_api.h:95)
/// Calls: mjCBody::AddGeom
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_geom(body: *mut mjsBody, def: *const mjsDefault) -> *mut mjsGeom {
    // NOTE: signature changed from previous IR version
    // Previous params: (body : * mut mjsBody, def : * const mjsDefault)
    // Previous return: * mut mjsGeom
    todo!("re-translate: params renamed")
}

/// C: mjs_addCamera (user/user_api.h:98)
/// Calls: mjCBody::AddCamera
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_camera(body: *mut mjsBody, def: *const mjsDefault) -> *mut mjsCamera {
    // NOTE: signature changed from previous IR version
    // Previous params: (body : * mut mjsBody, def : * const mjsDefault)
    // Previous return: * mut mjsCamera
    todo!("re-translate: params renamed")
}

/// C: mjs_addLight (user/user_api.h:101)
/// Calls: mjCBody::AddLight
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_light(body: *mut mjsBody, def: *const mjsDefault) -> *mut mjsLight {
    // NOTE: signature changed from previous IR version
    // Previous params: (body : * mut mjsBody, def : * const mjsDefault)
    // Previous return: * mut mjsLight
    todo!("re-translate: params renamed")
}

/// C: mjs_addFrame (user/user_api.h:104)
/// Calls: mjCBody::AddFrame, mjCFrame::SetParent
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_frame(body: *mut mjsBody, parentframe: *mut mjsFrame) -> *mut mjsFrame {
    // NOTE: signature changed from previous IR version
    // Previous params: (body : * mut mjsBody, parentframe : * mut mjsFrame)
    // Previous return: * mut mjsFrame
    todo!("re-translate: params renamed")
}

/// C: mjs_delete (user/user_api.h:107)
/// Calls: mjCModel::IsAttached, mjCModel::SetError
#[allow(unused_variables, non_snake_case)]
pub fn mjs_delete(s: *mut mjSpec, element: *mut mjsElement) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (s : * mut mjSpec, element : * mut mjsElement)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjs_addActuator (user/user_api.h:113)
/// Calls: mjCModel::AddActuator
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_actuator(s: *mut mjSpec, def: *const mjsDefault) -> *mut mjsActuator {
    // NOTE: signature changed from previous IR version
    // Previous params: (s : * mut mjSpec, def : * const mjsDefault)
    // Previous return: * mut mjsActuator
    todo!("re-translate: params renamed")
}

/// C: mjs_addSensor (user/user_api.h:116)
/// Calls: mjCModel::AddSensor
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_sensor(s: *mut mjSpec) -> *mut mjsSensor {
    // NOTE: signature changed from previous IR version
    // Previous params: (s : * mut mjSpec)
    // Previous return: * mut mjsSensor
    todo!("re-translate: params renamed")
}

/// C: mjs_addFlex (user/user_api.h:119)
/// Calls: mjCModel::AddFlex
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_flex(s: *mut mjSpec) -> *mut mjsFlex {
    // NOTE: signature changed from previous IR version
    // Previous params: (s : * mut mjSpec)
    // Previous return: * mut mjsFlex
    todo!("re-translate: params renamed")
}

/// C: mjs_makeFlex (user/user_api.h:122)
/// Calls: FlexcompDofFromStr, FlexcompTypeFromStr, mjCFlexcomp::Make, mjCModel::SetError, mju_error
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_make_flex(body: *mut mjsBody, name: *const i8, r#type: *const i8, dim: i32, dof: *const i8, count: *const i32, cellcount: *const i32, spacing: *const f64, scale: *const f64, radius: f64, mass: f64, inertiabox: f64, equality: i32, rigid: i32, flatskin: i32, elastic2d: i32, pos: *const f64, quat: *const f64, origin: *const f64, file: *const i8, vfs: *const mjVFS) -> *mut mjsFlex {
    // NOTE: signature changed from previous IR version
    // Previous params: (body : * mut mjsBody, name : * const i8, r#type : * const i8, dim : i32, dof : * const i8, count : * const i32, cellcount : * const i32, spacing : * const f64, scale : * const f64, radius : f64, mass : f64, inertiabox : f64, equality : i32, rigid : i32, flatskin : i32, elastic2d : i32, pos : * const f64, quat : * const f64, origin : * const f64, file : * const i8, vfs : * const mjVFS)
    // Previous return: * mut mjsFlex
    todo!("re-translate: params renamed")
}

/// C: mjs_addPair (user/user_api.h:130)
/// Calls: mjCModel::AddPair
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_pair(s: *mut mjSpec, def: *const mjsDefault) -> *mut mjsPair {
    // NOTE: signature changed from previous IR version
    // Previous params: (s : * mut mjSpec, def : * const mjsDefault)
    // Previous return: * mut mjsPair
    todo!("re-translate: params renamed")
}

/// C: mjs_addExclude (user/user_api.h:133)
/// Calls: mjCModel::AddExclude
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_exclude(s: *mut mjSpec) -> *mut mjsExclude {
    // NOTE: signature changed from previous IR version
    // Previous params: (s : * mut mjSpec)
    // Previous return: * mut mjsExclude
    todo!("re-translate: params renamed")
}

/// C: mjs_addEquality (user/user_api.h:136)
/// Calls: mjCModel::AddEquality
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_equality(s: *mut mjSpec, def: *const mjsDefault) -> *mut mjsEquality {
    // NOTE: signature changed from previous IR version
    // Previous params: (s : * mut mjSpec, def : * const mjsDefault)
    // Previous return: * mut mjsEquality
    todo!("re-translate: params renamed")
}

/// C: mjs_addTendon (user/user_api.h:139)
/// Calls: mjCModel::AddTendon
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_tendon(s: *mut mjSpec, def: *const mjsDefault) -> *mut mjsTendon {
    // NOTE: signature changed from previous IR version
    // Previous params: (s : * mut mjSpec, def : * const mjsDefault)
    // Previous return: * mut mjsTendon
    todo!("re-translate: params renamed")
}

/// C: mjs_wrapSite (user/user_api.h:142)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_wrap_site(tendon: *mut mjsTendon, name: *const i8) -> *mut mjsWrap {
    // NOTE: signature changed from previous IR version
    // Previous params: (tendon : * mut mjsTendon, name : * const i8)
    // Previous return: * mut mjsWrap
    todo!("re-translate: params renamed")
}

/// C: mjs_wrapGeom (user/user_api.h:145)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_wrap_geom(tendon: *mut mjsTendon, name: *const i8, sidesite: *const i8) -> *mut mjsWrap {
    // NOTE: signature changed from previous IR version
    // Previous params: (tendon : * mut mjsTendon, name : * const i8, sidesite : * const i8)
    // Previous return: * mut mjsWrap
    todo!("re-translate: params renamed")
}

/// C: mjs_wrapJoint (user/user_api.h:148)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_wrap_joint(tendon: *mut mjsTendon, name: *const i8, coef: f64) -> *mut mjsWrap {
    // NOTE: signature changed from previous IR version
    // Previous params: (tendon : * mut mjsTendon, name : * const i8, coef : f64)
    // Previous return: * mut mjsWrap
    todo!("re-translate: params renamed")
}

/// C: mjs_wrapPulley (user/user_api.h:151)
/// Calls: mjCTendon::WrapPulley
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_wrap_pulley(tendon: *mut mjsTendon, divisor: f64) -> *mut mjsWrap {
    // NOTE: signature changed from previous IR version
    // Previous params: (tendon : * mut mjsTendon, divisor : f64)
    // Previous return: * mut mjsWrap
    todo!("re-translate: params renamed")
}

/// C: mjs_addNumeric (user/user_api.h:154)
/// Calls: mjCModel::AddNumeric
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_numeric(s: *mut mjSpec) -> *mut mjsNumeric {
    // NOTE: signature changed from previous IR version
    // Previous params: (s : * mut mjSpec)
    // Previous return: * mut mjsNumeric
    todo!("re-translate: params renamed")
}

/// C: mjs_addText (user/user_api.h:157)
/// Calls: mjCModel::AddText
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_text(s: *mut mjSpec) -> *mut mjsText {
    // NOTE: signature changed from previous IR version
    // Previous params: (s : * mut mjSpec)
    // Previous return: * mut mjsText
    todo!("re-translate: params renamed")
}

/// C: mjs_addTuple (user/user_api.h:160)
/// Calls: mjCModel::AddTuple
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_tuple(s: *mut mjSpec) -> *mut mjsTuple {
    // NOTE: signature changed from previous IR version
    // Previous params: (s : * mut mjSpec)
    // Previous return: * mut mjsTuple
    todo!("re-translate: params renamed")
}

/// C: mjs_addKey (user/user_api.h:163)
/// Calls: mjCModel::AddKey
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_key(s: *mut mjSpec) -> *mut mjsKey {
    // NOTE: signature changed from previous IR version
    // Previous params: (s : * mut mjSpec)
    // Previous return: * mut mjsKey
    todo!("re-translate: params renamed")
}

/// C: mjs_addPlugin (user/user_api.h:166)
/// Calls: mjCModel::AddPlugin
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_plugin(s: *mut mjSpec) -> *mut mjsPlugin {
    // NOTE: signature changed from previous IR version
    // Previous params: (s : * mut mjSpec)
    // Previous return: * mut mjsPlugin
    todo!("re-translate: params renamed")
}

/// C: mjs_addDefault (user/user_api.h:169)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_default(s: *mut mjSpec, classname: *const i8, parent: *const mjsDefault) -> *mut mjsDefault {
    // NOTE: signature changed from previous IR version
    // Previous params: (s : * mut mjSpec, classname : * const i8, parent : * const mjsDefault)
    // Previous return: * mut mjsDefault
    todo!("re-translate: params renamed")
}

/// C: mjs_setToMotor (user/user_api.h:175)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_to_motor(actuator: *mut mjsActuator) -> *const i8 {
    // NOTE: signature changed from previous IR version
    // Previous params: (actuator : * mut mjsActuator)
    // Previous return: * const i8
    todo!("re-translate: params renamed")
}

/// C: mjs_setToPosition (user/user_api.h:178)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_to_position(actuator: *mut mjsActuator, kp: f64, kv: *mut f64, dampratio: *mut f64, timeconst: *mut f64, inheritrange: f64) -> *const i8 {
    // NOTE: signature changed from previous IR version
    // Previous params: (actuator : * mut mjsActuator, kp : f64, kv : * mut f64, dampratio : * mut f64, timeconst : * mut f64, inheritrange : f64)
    // Previous return: * const i8
    todo!("re-translate: params renamed")
}

/// C: mjs_setToIntVelocity (user/user_api.h:182)
/// Calls: mjs_setToPosition
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_to_int_velocity(actuator: *mut mjsActuator, kp: f64, kv: *mut f64, dampratio: *mut f64, timeconst: *mut f64, inheritrange: f64) -> *const i8 {
    // NOTE: signature changed from previous IR version
    // Previous params: (actuator : * mut mjsActuator, kp : f64, kv : * mut f64, dampratio : * mut f64, timeconst : * mut f64, inheritrange : f64)
    // Previous return: * const i8
    todo!("re-translate: params renamed")
}

/// C: mjs_setToVelocity (user/user_api.h:186)
/// Calls: mjuu_zerovec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_to_velocity(actuator: *mut mjsActuator, kv: f64) -> *const i8 {
    // NOTE: signature changed from previous IR version
    // Previous params: (actuator : * mut mjsActuator, kv : f64)
    // Previous return: * const i8
    todo!("re-translate: params renamed")
}

/// C: mjs_setToDamper (user/user_api.h:189)
/// Calls: mjuu_zerovec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_to_damper(actuator: *mut mjsActuator, kv: f64) -> *const i8 {
    // NOTE: signature changed from previous IR version
    // Previous params: (actuator : * mut mjsActuator, kv : f64)
    // Previous return: * const i8
    todo!("re-translate: params renamed")
}

/// C: mjs_setToCylinder (user/user_api.h:192)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_to_cylinder(actuator: *mut mjsActuator, timeconst: f64, bias: f64, area: f64, diameter: f64) -> *const i8 {
    // NOTE: signature changed from previous IR version
    // Previous params: (actuator : * mut mjsActuator, timeconst : f64, bias : f64, area : f64, diameter : f64)
    // Previous return: * const i8
    todo!("re-translate: params renamed")
}

/// C: mjs_setToMuscle (user/user_api.h:196)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_to_muscle(actuator: *mut mjsActuator, timeconst: *mut f64, tausmooth: f64, range: *mut f64, force: f64, scale: f64, lmin: f64, lmax: f64, vmax: f64, fpmax: f64, fvmax: f64) -> *const i8 {
    // NOTE: signature changed from previous IR version
    // Previous params: (actuator : * mut mjsActuator, timeconst : * mut f64, tausmooth : f64, range : * mut f64, force : f64, scale : f64, lmin : f64, lmax : f64, vmax : f64, fpmax : f64, fvmax : f64)
    // Previous return: * const i8
    todo!("re-translate: params renamed")
}

/// C: mjs_setToAdhesion (user/user_api.h:201)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_to_adhesion(actuator: *mut mjsActuator, gain: f64) -> *const i8 {
    // NOTE: signature changed from previous IR version
    // Previous params: (actuator : * mut mjsActuator, gain : f64)
    // Previous return: * const i8
    todo!("re-translate: params renamed")
}

/// C: mjs_setToDCMotor (user/user_api.h:204)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_to_dc_motor(actuator: *mut mjsActuator, motorconst: *mut f64, resistance: f64, nominal: *mut f64, saturation: *mut f64, inductance: *mut f64, cogging: *mut f64, controller: *mut f64, thermal: *mut f64, lugre: *mut f64, input_mode: i32) -> *const i8 {
    // NOTE: signature changed from previous IR version
    // Previous params: (actuator : * mut mjsActuator, motorconst : * mut f64, resistance : f64, nominal : * mut f64, saturation : * mut f64, inductance : * mut f64, cogging : * mut f64, controller : * mut f64, thermal : * mut f64, lugre : * mut f64, input_mode : i32)
    // Previous return: * const i8
    todo!("re-translate: params renamed")
}

/// C: mjs_addMesh (user/user_api.h:213)
/// Calls: mjCModel::AddMesh
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_mesh(s: *mut mjSpec, def: *const mjsDefault) -> *mut mjsMesh {
    // NOTE: signature changed from previous IR version
    // Previous params: (s : * mut mjSpec, def : * const mjsDefault)
    // Previous return: * mut mjsMesh
    todo!("re-translate: params renamed")
}

/// C: mjs_addHField (user/user_api.h:216)
/// Calls: mjCModel::AddHField
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_h_field(s: *mut mjSpec) -> *mut mjsHField {
    // NOTE: signature changed from previous IR version
    // Previous params: (s : * mut mjSpec)
    // Previous return: * mut mjsHField
    todo!("re-translate: params renamed")
}

/// C: mjs_addSkin (user/user_api.h:219)
/// Calls: mjCModel::AddSkin
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_skin(s: *mut mjSpec) -> *mut mjsSkin {
    // NOTE: signature changed from previous IR version
    // Previous params: (s : * mut mjSpec)
    // Previous return: * mut mjsSkin
    todo!("re-translate: params renamed")
}

/// C: mjs_addTexture (user/user_api.h:222)
/// Calls: mjCModel::AddTexture
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_texture(s: *mut mjSpec) -> *mut mjsTexture {
    // NOTE: signature changed from previous IR version
    // Previous params: (s : * mut mjSpec)
    // Previous return: * mut mjsTexture
    todo!("re-translate: params renamed")
}

/// C: mjs_addMaterial (user/user_api.h:225)
/// Calls: mjCModel::AddMaterial
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_material(s: *mut mjSpec, def: *const mjsDefault) -> *mut mjsMaterial {
    // NOTE: signature changed from previous IR version
    // Previous params: (s : * mut mjSpec, def : * const mjsDefault)
    // Previous return: * mut mjsMaterial
    todo!("re-translate: params renamed")
}

/// C: mjs_makeMesh (user/user_api.h:228)
/// Calls: mjCMesh::MakeCone, mjCMesh::MakeHemisphere, mjCMesh::MakeRect, mjCMesh::MakeSphere, mjCMesh::MakeSupersphere, mjCMesh::MakeSupertorus, mjCMesh::MakeWedge, mjCModel::SetError
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_make_mesh(mesh: *mut mjsMesh, builtin: u32, params: *mut f64, nparams: i32) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (mesh : * mut mjsMesh, builtin : u32, params : * mut f64, nparams : i32)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjs_getSpec (user/user_api.h:233)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_spec(element: *const mjsElement) -> *mut mjSpec {
    // NOTE: signature changed from previous IR version
    // Previous params: (element : * const mjsElement)
    // Previous return: * mut mjSpec
    todo!("re-translate: params renamed")
}

/// C: mjs_getOriginSpec (user/user_api.h:237)
/// Calls: mjCModel::FindSpec
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_origin_spec(element: *const mjsElement) -> *mut mjSpec {
    // NOTE: signature changed from previous IR version
    // Previous params: (element : * const mjsElement)
    // Previous return: * mut mjSpec
    todo!("re-translate: params renamed")
}

/// C: mjs_findSpec (user/user_api.h:240)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_find_spec(spec: *const mjSpec, name: *const i8) -> *mut mjSpec {
    // NOTE: signature changed from previous IR version
    // Previous params: (spec : * const mjSpec, name : * const i8)
    // Previous return: * mut mjSpec
    todo!("re-translate: params renamed")
}

/// C: mjs_findBody (user/user_api.h:243)
/// Calls: mjs_findElement
#[allow(unused_variables, non_snake_case)]
pub fn mjs_find_body(s: *const mjSpec, name: *const i8) -> *mut mjsBody {
    // NOTE: signature changed from previous IR version
    // Previous params: (s : * const mjSpec, name : * const i8)
    // Previous return: * mut mjsBody
    todo!("re-translate: params renamed")
}

/// C: mjs_findElement (user/user_api.h:246)
/// Calls: mjCModel::GetWorld, mjCModel::IsCompiled
#[allow(unused_variables, non_snake_case)]
pub fn mjs_find_element(s: *const mjSpec, r#type: u32, name: *const i8) -> *mut mjsElement {
    // NOTE: signature changed from previous IR version
    // Previous params: (s : * const mjSpec, r#type : u32, name : * const i8)
    // Previous return: * mut mjsElement
    todo!("re-translate: params renamed")
}

/// C: mjs_findChild (user/user_api.h:249)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_find_child(body: *const mjsBody, name: *const i8) -> *mut mjsBody {
    // NOTE: signature changed from previous IR version
    // Previous params: (body : * const mjsBody, name : * const i8)
    // Previous return: * mut mjsBody
    todo!("re-translate: params renamed")
}

/// C: mjs_getParent (user/user_api.h:252)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_parent(element: *const mjsElement) -> *mut mjsBody {
    // NOTE: signature changed from previous IR version
    // Previous params: (element : * const mjsElement)
    // Previous return: * mut mjsBody
    todo!("re-translate: params renamed")
}

/// C: mjs_getFrame (user/user_api.h:255)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_frame(element: *const mjsElement) -> *mut mjsFrame {
    // NOTE: signature changed from previous IR version
    // Previous params: (element : * const mjsElement)
    // Previous return: * mut mjsFrame
    todo!("re-translate: params renamed")
}

/// C: mjs_findFrame (user/user_api.h:258)
/// Calls: mjs_findElement
#[allow(unused_variables, non_snake_case)]
pub fn mjs_find_frame(s: *const mjSpec, name: *const i8) -> *mut mjsFrame {
    // NOTE: signature changed from previous IR version
    // Previous params: (s : * const mjSpec, name : * const i8)
    // Previous return: * mut mjsFrame
    todo!("re-translate: params renamed")
}

/// C: mjs_getDefault (user/user_api.h:261)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_default(element: *const mjsElement) -> *mut mjsDefault {
    // NOTE: signature changed from previous IR version
    // Previous params: (element : * const mjsElement)
    // Previous return: * mut mjsDefault
    todo!("re-translate: params renamed")
}

/// C: mjs_findDefault (user/user_api.h:264)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_find_default(s: *const mjSpec, classname: *const i8) -> *mut mjsDefault {
    // NOTE: signature changed from previous IR version
    // Previous params: (s : * const mjSpec, classname : * const i8)
    // Previous return: * mut mjsDefault
    todo!("re-translate: params renamed")
}

/// C: mjs_getSpecDefault (user/user_api.h:267)
/// Calls: mjCModel::Default
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_spec_default(s: *const mjSpec) -> *mut mjsDefault {
    // NOTE: signature changed from previous IR version
    // Previous params: (s : * const mjSpec)
    // Previous return: * mut mjsDefault
    todo!("re-translate: params renamed")
}

/// C: mjs_getId (user/user_api.h:270)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_id(element: *const mjsElement) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (element : * const mjsElement)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjs_firstChild (user/user_api.h:276)
/// Calls: mjCBody::NextChild
#[allow(unused_variables, non_snake_case)]
pub fn mjs_first_child(body: *const mjsBody, r#type: u32, recurse: i32) -> *mut mjsElement {
    // NOTE: signature changed from previous IR version
    // Previous params: (body : * const mjsBody, r#type : u32, recurse : i32)
    // Previous return: * mut mjsElement
    todo!("re-translate: params renamed")
}

/// C: mjs_nextChild (user/user_api.h:280)
/// Calls: mjCBody::NextChild
#[allow(unused_variables, non_snake_case)]
pub fn mjs_next_child(body: *const mjsBody, child: *const mjsElement, recurse: i32) -> *mut mjsElement {
    // NOTE: signature changed from previous IR version
    // Previous params: (body : * const mjsBody, child : * const mjsElement, recurse : i32)
    // Previous return: * mut mjsElement
    todo!("re-translate: params renamed")
}

/// C: mjs_firstElement (user/user_api.h:283)
/// Calls: mjCModel::NextObject
#[allow(unused_variables, non_snake_case)]
pub fn mjs_first_element(s: *const mjSpec, r#type: u32) -> *mut mjsElement {
    // NOTE: signature changed from previous IR version
    // Previous params: (s : * const mjSpec, r#type : u32)
    // Previous return: * mut mjsElement
    todo!("re-translate: params renamed")
}

/// C: mjs_nextElement (user/user_api.h:286)
/// Calls: mjCModel::NextObject
#[allow(unused_variables, non_snake_case)]
pub fn mjs_next_element(s: *const mjSpec, element: *const mjsElement) -> *mut mjsElement {
    // NOTE: signature changed from previous IR version
    // Previous params: (s : * const mjSpec, element : * const mjsElement)
    // Previous return: * mut mjsElement
    todo!("re-translate: params renamed")
}

/// C: mjs_getWrapTarget (user/user_api.h:289)
/// Calls: mjCWrap::Type, mjs_getSpec
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_wrap_target(wrap: *const mjsWrap) -> *mut mjsElement {
    // NOTE: signature changed from previous IR version
    // Previous params: (wrap : * const mjsWrap)
    // Previous return: * mut mjsElement
    todo!("re-translate: params renamed")
}

/// C: mjs_getWrapSideSite (user/user_api.h:292)
/// Calls: mjCWrap::Type, mjs_asSite, mjs_getSpec
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_wrap_side_site(wrap: *const mjsWrap) -> *mut mjsSite {
    // NOTE: signature changed from previous IR version
    // Previous params: (wrap : * const mjsWrap)
    // Previous return: * mut mjsSite
    todo!("re-translate: params renamed")
}

/// C: mjs_getWrapDivisor (user/user_api.h:295)
/// Calls: mjCWrap::Type
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_wrap_divisor(wrap: *const mjsWrap) -> f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (wrap : * const mjsWrap)
    // Previous return: f64
    todo!("re-translate: params renamed")
}

/// C: mjs_getWrapCoef (user/user_api.h:298)
/// Calls: mjCWrap::Type
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_wrap_coef(wrap: *const mjsWrap) -> f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (wrap : * const mjsWrap)
    // Previous return: f64
    todo!("re-translate: params renamed")
}

/// C: mjs_asBody (user/user_api.h:301)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_body(element: *mut mjsElement) -> *mut mjsBody {
    // NOTE: signature changed from previous IR version
    // Previous params: (element : * mut mjsElement)
    // Previous return: * mut mjsBody
    todo!("re-translate: params renamed")
}

/// C: mjs_asGeom (user/user_api.h:304)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_geom(element: *mut mjsElement) -> *mut mjsGeom {
    // NOTE: signature changed from previous IR version
    // Previous params: (element : * mut mjsElement)
    // Previous return: * mut mjsGeom
    todo!("re-translate: params renamed")
}

/// C: mjs_asJoint (user/user_api.h:307)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_joint(element: *mut mjsElement) -> *mut mjsJoint {
    // NOTE: signature changed from previous IR version
    // Previous params: (element : * mut mjsElement)
    // Previous return: * mut mjsJoint
    todo!("re-translate: params renamed")
}

/// C: mjs_asSite (user/user_api.h:310)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_site(element: *mut mjsElement) -> *mut mjsSite {
    // NOTE: signature changed from previous IR version
    // Previous params: (element : * mut mjsElement)
    // Previous return: * mut mjsSite
    todo!("re-translate: params renamed")
}

/// C: mjs_asCamera (user/user_api.h:313)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_camera(element: *mut mjsElement) -> *mut mjsCamera {
    // NOTE: signature changed from previous IR version
    // Previous params: (element : * mut mjsElement)
    // Previous return: * mut mjsCamera
    todo!("re-translate: params renamed")
}

/// C: mjs_asLight (user/user_api.h:316)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_light(element: *mut mjsElement) -> *mut mjsLight {
    // NOTE: signature changed from previous IR version
    // Previous params: (element : * mut mjsElement)
    // Previous return: * mut mjsLight
    todo!("re-translate: params renamed")
}

/// C: mjs_asFrame (user/user_api.h:319)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_frame(element: *mut mjsElement) -> *mut mjsFrame {
    // NOTE: signature changed from previous IR version
    // Previous params: (element : * mut mjsElement)
    // Previous return: * mut mjsFrame
    todo!("re-translate: params renamed")
}

/// C: mjs_asActuator (user/user_api.h:322)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_actuator(element: *mut mjsElement) -> *mut mjsActuator {
    // NOTE: signature changed from previous IR version
    // Previous params: (element : * mut mjsElement)
    // Previous return: * mut mjsActuator
    todo!("re-translate: params renamed")
}

/// C: mjs_asSensor (user/user_api.h:325)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_sensor(element: *mut mjsElement) -> *mut mjsSensor {
    // NOTE: signature changed from previous IR version
    // Previous params: (element : * mut mjsElement)
    // Previous return: * mut mjsSensor
    todo!("re-translate: params renamed")
}

/// C: mjs_asFlex (user/user_api.h:328)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_flex(element: *mut mjsElement) -> *mut mjsFlex {
    // NOTE: signature changed from previous IR version
    // Previous params: (element : * mut mjsElement)
    // Previous return: * mut mjsFlex
    todo!("re-translate: params renamed")
}

/// C: mjs_asPair (user/user_api.h:331)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_pair(element: *mut mjsElement) -> *mut mjsPair {
    // NOTE: signature changed from previous IR version
    // Previous params: (element : * mut mjsElement)
    // Previous return: * mut mjsPair
    todo!("re-translate: params renamed")
}

/// C: mjs_asEquality (user/user_api.h:334)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_equality(element: *mut mjsElement) -> *mut mjsEquality {
    // NOTE: signature changed from previous IR version
    // Previous params: (element : * mut mjsElement)
    // Previous return: * mut mjsEquality
    todo!("re-translate: params renamed")
}

/// C: mjs_asExclude (user/user_api.h:337)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_exclude(element: *mut mjsElement) -> *mut mjsExclude {
    // NOTE: signature changed from previous IR version
    // Previous params: (element : * mut mjsElement)
    // Previous return: * mut mjsExclude
    todo!("re-translate: params renamed")
}

/// C: mjs_asTendon (user/user_api.h:340)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_tendon(element: *mut mjsElement) -> *mut mjsTendon {
    // NOTE: signature changed from previous IR version
    // Previous params: (element : * mut mjsElement)
    // Previous return: * mut mjsTendon
    todo!("re-translate: params renamed")
}

/// C: mjs_asNumeric (user/user_api.h:343)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_numeric(element: *mut mjsElement) -> *mut mjsNumeric {
    // NOTE: signature changed from previous IR version
    // Previous params: (element : * mut mjsElement)
    // Previous return: * mut mjsNumeric
    todo!("re-translate: params renamed")
}

/// C: mjs_asText (user/user_api.h:346)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_text(element: *mut mjsElement) -> *mut mjsText {
    // NOTE: signature changed from previous IR version
    // Previous params: (element : * mut mjsElement)
    // Previous return: * mut mjsText
    todo!("re-translate: params renamed")
}

/// C: mjs_asTuple (user/user_api.h:349)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_tuple(element: *mut mjsElement) -> *mut mjsTuple {
    // NOTE: signature changed from previous IR version
    // Previous params: (element : * mut mjsElement)
    // Previous return: * mut mjsTuple
    todo!("re-translate: params renamed")
}

/// C: mjs_asKey (user/user_api.h:352)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_key(element: *mut mjsElement) -> *mut mjsKey {
    // NOTE: signature changed from previous IR version
    // Previous params: (element : * mut mjsElement)
    // Previous return: * mut mjsKey
    todo!("re-translate: params renamed")
}

/// C: mjs_asMesh (user/user_api.h:355)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_mesh(element: *mut mjsElement) -> *mut mjsMesh {
    // NOTE: signature changed from previous IR version
    // Previous params: (element : * mut mjsElement)
    // Previous return: * mut mjsMesh
    todo!("re-translate: params renamed")
}

/// C: mjs_asHField (user/user_api.h:358)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_h_field(element: *mut mjsElement) -> *mut mjsHField {
    // NOTE: signature changed from previous IR version
    // Previous params: (element : * mut mjsElement)
    // Previous return: * mut mjsHField
    todo!("re-translate: params renamed")
}

/// C: mjs_asSkin (user/user_api.h:361)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_skin(element: *mut mjsElement) -> *mut mjsSkin {
    // NOTE: signature changed from previous IR version
    // Previous params: (element : * mut mjsElement)
    // Previous return: * mut mjsSkin
    todo!("re-translate: params renamed")
}

/// C: mjs_asTexture (user/user_api.h:364)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_texture(element: *mut mjsElement) -> *mut mjsTexture {
    // NOTE: signature changed from previous IR version
    // Previous params: (element : * mut mjsElement)
    // Previous return: * mut mjsTexture
    todo!("re-translate: params renamed")
}

/// C: mjs_asMaterial (user/user_api.h:367)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_material(element: *mut mjsElement) -> *mut mjsMaterial {
    // NOTE: signature changed from previous IR version
    // Previous params: (element : * mut mjsElement)
    // Previous return: * mut mjsMaterial
    todo!("re-translate: params renamed")
}

/// C: mjs_asPlugin (user/user_api.h:370)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_plugin(element: *mut mjsElement) -> *mut mjsPlugin {
    // NOTE: signature changed from previous IR version
    // Previous params: (element : * mut mjsElement)
    // Previous return: * mut mjsPlugin
    todo!("re-translate: params renamed")
}

/// C: mjs_setName (user/user_api.h:376)
/// Calls: mjCModel::CheckRepeat, mjCModel::SetError
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_name(element: *mut mjsElement, name: *const i8) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (element : * mut mjsElement, name : * const i8)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjs_setPluginAttributes (user/user_api.h:409)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_plugin_attributes(plugin: *mut mjsPlugin, attributes: *mut ()) {
    // NOTE: signature changed from previous IR version
    // Previous params: (plugin : * mut mjsPlugin, attributes : * mut ())
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjs_getWrapNum (user/user_api.h:424)
/// Calls: mjCTendon::NumWraps
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_wrap_num(tendonspec: *const mjsTendon) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (tendonspec : * const mjsTendon)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjs_getWrap (user/user_api.h:426)
/// Calls: mjCTendon::GetWrap, mjCTendon::NumWraps, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_wrap(tendonspec: *const mjsTendon, i: i32) -> *mut mjsWrap {
    // NOTE: signature changed from previous IR version
    // Previous params: (tendonspec : * const mjsTendon, i : i32)
    // Previous return: * mut mjsWrap
    todo!("re-translate: params renamed")
}

/// C: mjs_getPluginAttributes (user/user_api.h:429)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_plugin_attributes(plugin: *const mjsPlugin) -> *const () {
    // NOTE: signature changed from previous IR version
    // Previous params: (plugin : * const mjsPlugin)
    // Previous return: * const ()
    todo!("re-translate: params renamed")
}

/// C: mjs_isAuthored (user/user_api.h:435)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_is_authored(elem_ptr: *const (), field_ptr: *const ()) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (elem_ptr : * const (), field_ptr : * const ())
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjs_setAuthored (user/user_api.h:438)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_authored(elem_ptr: *const (), field_ptr: *const (), authored: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (elem_ptr : * const (), field_ptr : * const (), authored : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjs_setDefault (user/user_api.h:441)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_default(element: *mut mjsElement, def: *const mjsDefault) {
    // NOTE: signature changed from previous IR version
    // Previous params: (element : * mut mjsElement, def : * const mjsDefault)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjs_setFrame (user/user_api.h:444)
/// Calls: mjCBase::SetFrame, mjCModel::SetError
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_frame(dest: *mut mjsElement, frame: *mut mjsFrame) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (dest : * mut mjsElement, frame : * mut mjsFrame)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjs_resolveOrientation (user/user_api.h:447)
/// Calls: ResolveOrientation
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_resolve_orientation(quat: *mut f64, degree: u8, sequence: *const i8, orientation: *const mjsOrientation) -> *const i8 {
    todo!() // mjs_resolveOrientation
}

/// C: mjs_bodyToFrame (user/user_api.h:451)
/// Calls: mjCBody::ToFrame
#[allow(unused_variables, non_snake_case)]
pub fn mjs_body_to_frame(body: *mut *mut mjsBody) -> *mut mjsFrame {
    // NOTE: signature changed from previous IR version
    // Previous params: (body : * mut * mut mjsBody)
    // Previous return: * mut mjsFrame
    todo!("re-translate: params renamed")
}

/// C: mjs_setUserValue (user/user_api.h:454)
/// Calls: mjs_setUserValueWithCleanup
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_user_value(element: *mut mjsElement, key: *const i8, data: *const ()) {
    // NOTE: signature changed from previous IR version
    // Previous params: (element : * mut mjsElement, key : * const i8, data : * const ())
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjs_setUserValueWithCleanup (user/user_api.h:457)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_user_value_with_cleanup(element: *mut mjsElement, key: *const i8, data: *const (), cleanup: Option<unsafe extern "C" fn()>) {
    // NOTE: signature changed from previous IR version
    // Previous params: (element : * mut mjsElement, key : * const i8, data : * const (), cleanup : Option < unsafe extern "C" fn () >)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjs_getUserValue (user/user_api.h:462)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_user_value(element: *mut mjsElement, key: *const i8) -> *const () {
    // NOTE: signature changed from previous IR version
    // Previous params: (element : * mut mjsElement, key : * const i8)
    // Previous return: * const ()
    todo!("re-translate: params renamed")
}

/// C: mjs_deleteUserValue (user/user_api.h:465)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_delete_user_value(element: *mut mjsElement, key: *const i8) {
    // NOTE: signature changed from previous IR version
    // Previous params: (element : * mut mjsElement, key : * const i8)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjs_sensorDim (user/user_api.h:468)
/// Calls: mju_condataSize, mju_raydataSize
#[allow(unused_variables, non_snake_case)]
pub fn mjs_sensor_dim(sensor: *const mjsSensor) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (sensor : * const mjsSensor)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mj_getCacheCapacity (user/user_api.h:551)
/// Calls: mjCCache::Capacity
#[allow(unused_variables, non_snake_case)]
pub fn mj_get_cache_capacity(cache: *const mjCache) -> usize {
    // NOTE: signature changed from previous IR version
    // Previous params: (cache : * const mjCache)
    // Previous return: usize
    todo!("re-translate: params renamed")
}

/// C: mj_setCacheCapacity (user/user_api.h:554)
/// Calls: mjCCache::Capacity, mjCCache::SetCapacity
#[allow(unused_variables, non_snake_case)]
pub fn mj_set_cache_capacity(cache: *mut mjCache, size: usize) -> usize {
    // NOTE: signature changed from previous IR version
    // Previous params: (cache : * mut mjCache, size : usize)
    // Previous return: usize
    todo!("re-translate: params renamed")
}

/// C: mj_getCacheSize (user/user_api.h:557)
/// Calls: mjCCache::Size
#[allow(unused_variables, non_snake_case)]
pub fn mj_get_cache_size(cache: *const mjCache) -> usize {
    // NOTE: signature changed from previous IR version
    // Previous params: (cache : * const mjCache)
    // Previous return: usize
    todo!("re-translate: params renamed")
}

/// C: mj_clearCache (user/user_api.h:560)
/// Calls: mjCCache::Reset
#[allow(unused_variables, non_snake_case)]
pub fn mj_clear_cache(cache: *mut mjCache) {
    // NOTE: signature changed from previous IR version
    // Previous params: (cache : * mut mjCache)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mj_getCache (user/user_api.h:563)
/// Calls: mjCCache::Capacity
#[allow(unused_variables, non_snake_case)]
pub fn mj_get_cache() -> *mut mjCache {
    // NOTE: signature changed from previous IR version
    // Previous params: ()
    // Previous return: * mut mjCache
    todo!("re-translate: params renamed")
}

