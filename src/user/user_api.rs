//! Port of: user/user_api.cc
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_parse (user/user_api.cc:78)
/// Calls: mj_defaultVFS, mj_deleteVFS, mj_parseXML, mju_closeResource, mju_decodeResource, mju_free, mju_malloc, mju_openResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_parse(filename: *const i8, content_type: *const i8, vfs: *const mjVFS, error: *mut i8, error_sz: i32) -> *mut mjSpec {
    // WARNING: signature changed — verify body
    // Previous params: (filename : * const i8, content_type : * const i8, vfs : * const mjVFS, error : * mut i8, error_sz : i32)
    // Previous return: * mut mjSpec
    todo ! ()
}

/// C: mj_encode (user/user_api.cc:151)
/// Calls: mj_saveXML, mjp_findEncoder
#[allow(unused_variables, non_snake_case)]
pub fn mj_encode(s: *const mjSpec, m: *const mjModel, filename: *const i8, content_type: *const i8, vfs: *const mjVFS, error: *mut i8, error_sz: i32) -> i32 {
    extern "C" { fn mj_encode_impl(s: *const mjSpec, m: *const mjModel, filename: *const i8, content_type: *const i8, vfs: *const mjVFS, error: *mut i8, error_sz: i32) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mj_encode_impl(s, m, filename, content_type, vfs, error, error_sz) }
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
    extern "C" { fn LogCompileTime_impl(t: *const f64); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { LogCompileTime_impl(t) }
}

/// C: SetFrame (user/user_api.cc:293)
/// Calls: mjs_firstChild, mjs_getFrame, mjs_nextChild, mjs_setFrame
#[allow(unused_variables, non_snake_case)]
pub fn set_frame(body: *mut mjsBody, objtype: mjtObj, frame: *mut mjsFrame) {
    // WARNING: signature changed — verify body
    // Previous params: (body : * mut mjsBody, objtype : mjtObj, frame : * mut mjsFrame)
    // Previous return: ()
    todo ! ()
}

/// C: attachBody (user/user_api.cc:306)
#[allow(unused_variables, non_snake_case)]
pub fn attach_body(parent: *mut mjCFrame, child: *const mjCBody, prefix: *const i8, suffix: *const i8) -> *mut mjsElement {
    extern "C" { fn attachBody_impl(parent: *mut mjCFrame, child: *const mjCBody, prefix: *const i8, suffix: *const i8) -> *mut mjsElement; }
    // SAFETY: delegates to C implementation
    unsafe { attachBody_impl(parent, child, prefix, suffix) }
}

/// C: attachFrame (user/user_api.cc:325)
#[allow(unused_variables, non_snake_case)]
pub fn attach_frame(parent: *mut mjCBody, child: *const mjCFrame, prefix: *const i8, suffix: *const i8) -> *mut mjsElement {
    extern "C" { fn attachFrame_impl(parent: *mut mjCBody, child: *const mjCFrame, prefix: *const i8, suffix: *const i8) -> *mut mjsElement; }
    // SAFETY: delegates to C implementation
    unsafe { attachFrame_impl(parent, child, prefix, suffix) }
}

/// C: attachToSite (user/user_api.cc:344)
/// Calls: attachBody, mjCBody::AddFrame, mjCFrame::SetParent, mjCSite::Body
#[allow(unused_variables, non_snake_case)]
pub fn attach_to_site(parent: *mut mjCSite, child: *const mjCBody, prefix: *const i8, suffix: *const i8) -> *mut mjsElement {
    extern "C" { fn attachToSite_impl(parent: *mut mjCSite, child: *const mjCBody, prefix: *const i8, suffix: *const i8) -> *mut mjsElement; }
    // SAFETY: delegates to C implementation
    unsafe { attachToSite_impl(parent, child, prefix, suffix) }
}

/// C: attachFrameToSite (user/user_api.cc:365)
/// Calls: attachFrame, mjCBody::AddFrame, mjCFrame::SetParent, mjCSite::Body
#[allow(unused_variables, non_snake_case)]
pub fn attach_frame_to_site(parent: *mut mjCSite, child: *const mjCFrame, prefix: *const i8, suffix: *const i8) -> *mut mjsElement {
    extern "C" { fn attachFrameToSite_impl(parent: *mut mjCSite, child: *const mjCFrame, prefix: *const i8, suffix: *const i8) -> *mut mjsElement; }
    // SAFETY: delegates to C implementation
    unsafe { attachFrameToSite_impl(parent, child, prefix, suffix) }
}

/// C: mjs_getTimer (user/user_api.cc:515)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_timer(s: *mut mjSpec) -> *const f64 {
    extern "C" { fn mjs_getTimer_impl(s: *mut mjSpec) -> *const f64; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_getTimer_impl(s) }
}

/// C: mjs_numWarnings (user/user_api.cc:535)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_num_warnings(spec: *const mjSpec) -> i32 {
    extern "C" { fn mjs_numWarnings_impl(spec: *const mjSpec) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_numWarnings_impl(spec) }
}

/// C: mjs_getWarning (user/user_api.cc:544)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_warning(spec: *const mjSpec, index: i32) -> *const i8 {
    extern "C" { fn mjs_getWarning_impl(spec: *const mjSpec, index: i32) -> *const i8; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_getWarning_impl(spec, index) }
}

/// C: FlexcompTypeFromStr (user/user_api.cc:709)
#[allow(unused_variables, non_snake_case)]
pub fn flexcomp_type_from_str(r#type: *const i8) -> mjtFcompType {
    // WARNING: signature changed — verify body
    // Previous params: (r#type : * const i8)
    // Previous return: mjtFcompType
    extern "C" { fn FlexcompTypeFromStr_impl (r#type : * const i8) -> mjtFcompType ; } unsafe { FlexcompTypeFromStr_impl (r#type) }
}

/// C: FlexcompDofFromStr (user/user_api.cc:724)
#[allow(unused_variables, non_snake_case)]
pub fn flexcomp_dof_from_str(dof: *const i8) -> mjtDof {
    // WARNING: signature changed — verify body
    // Previous params: (dof : * const i8)
    // Previous return: mjtDof
    extern "C" { fn FlexcompDofFromStr_impl (dof : * const i8) -> mjtDof ; } unsafe { FlexcompDofFromStr_impl (dof) }
}

/// C: mjs_getCompiler (user/user_api.cc:1553)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_compiler(element: *const mjsElement) -> *mut mjsCompiler {
    // WARNING: signature changed — verify body
    // Previous params: (element : * const mjsElement)
    // Previous return: * mut mjsCompiler
    extern "C" { fn mjs_getCompiler_impl (element : * const mjsElement) -> * mut mjsCompiler ; } unsafe { mjs_getCompiler_impl (element) }
}

/// C: mjs_setBuffer (user/user_api.cc:2230)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_buffer(dest: *mut i32, array: *const (), size: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (dest : * mut i32, array : * const (), size : i32)
    // Previous return: ()
    todo ! ()
}

/// C: mjs_setString (user/user_api.cc:2240)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_string(dest: *mut i32, text: *const i8) {
    extern "C" { fn mjs_setString_impl(dest: *mut i32, text: *const i8); }
    // SAFETY: delegates to C implementation
    unsafe { mjs_setString_impl(dest, text) }
}

/// C: mjs_setInStringVec (user/user_api.cc:2248)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_in_string_vec(dest: *mut i32, i: i32, text: *const i8) -> mjtBool {
    extern "C" { fn mjs_setInStringVec_impl(dest: *mut i32, i: i32, text: *const i8) -> mjtBool; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_setInStringVec_impl(dest, i, text) }
}

/// C: mjs_setStringVec (user/user_api.cc:2260)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_string_vec(dest: *mut i32, text: *const i8) {
    extern "C" { fn mjs_setStringVec_impl(dest: *mut i32, text: *const i8); }
    // SAFETY: delegates to C implementation
    unsafe { mjs_setStringVec_impl(dest, text) }
}

/// C: mjs_appendString (user/user_api.cc:2268)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_append_string(dest: *mut i32, text: *const i8) {
    extern "C" { fn mjs_appendString_impl(dest: *mut i32, text: *const i8); }
    // SAFETY: delegates to C implementation
    unsafe { mjs_appendString_impl(dest, text) }
}

/// C: mjs_setInt (user/user_api.cc:2274)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_int(dest: *mut i32, array: *const i32, size: i32) {
    extern "C" { fn mjs_setInt_impl(dest: *mut i32, array: *const i32, size: i32); }
    // SAFETY: delegates to C implementation
    unsafe { mjs_setInt_impl(dest, array, size) }
}

/// C: mjs_appendIntVec (user/user_api.cc:2284)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_append_int_vec(dest: *mut i32, array: *const i32, size: i32) {
    extern "C" { fn mjs_appendIntVec_impl(dest: *mut i32, array: *const i32, size: i32); }
    // SAFETY: delegates to C implementation
    unsafe { mjs_appendIntVec_impl(dest, array, size) }
}

/// C: mjs_setFloat (user/user_api.cc:2291)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_float(dest: *mut i32, array: *const f32, size: i32) {
    extern "C" { fn mjs_setFloat_impl(dest: *mut i32, array: *const f32, size: i32); }
    // SAFETY: delegates to C implementation
    unsafe { mjs_setFloat_impl(dest, array, size) }
}

/// C: mjs_appendFloatVec (user/user_api.cc:2302)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_append_float_vec(dest: *mut i32, array: *const f32, size: i32) {
    extern "C" { fn mjs_appendFloatVec_impl(dest: *mut i32, array: *const f32, size: i32); }
    // SAFETY: delegates to C implementation
    unsafe { mjs_appendFloatVec_impl(dest, array, size) }
}

/// C: mjs_setDouble (user/user_api.cc:2309)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_double(dest: *mut i32, array: *const f64, size: i32) {
    extern "C" { fn mjs_setDouble_impl(dest: *mut i32, array: *const f64, size: i32); }
    // SAFETY: delegates to C implementation
    unsafe { mjs_setDouble_impl(dest, array, size) }
}

/// C: mjs_getName (user/user_api.cc:2319)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_name(element: *mut mjsElement) -> *mut i32 {
    extern "C" { fn mjs_getName_impl(element: *mut mjsElement) -> *mut i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_getName_impl(element) }
}

/// C: mjs_getString (user/user_api.cc:2329)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_string(source: *const i32) -> *const i8 {
    extern "C" { fn mjs_getString_impl(source: *const i32) -> *const i8; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_getString_impl(source) }
}

/// C: mjs_getDouble (user/user_api.cc:2336)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_double(source: *const i32, size: *mut i32) -> *const f64 {
    extern "C" { fn mjs_getDouble_impl(source: *const i32, size: *mut i32) -> *const f64; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_getDouble_impl(source, size) }
}

/// C: mj_makeSpec (user/user_api.h:40)
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_spec() -> *mut mjSpec {
    extern "C" { fn mj_makeSpec_impl() -> *mut mjSpec; }
    // SAFETY: delegates to C implementation
    unsafe { mj_makeSpec_impl() }
}

/// C: mj_compile (user/user_api.h:43)
/// Calls: LogCompileTime, mjCModel::Compile
#[allow(unused_variables, non_snake_case)]
pub fn mj_compile(s: *mut mjSpec, vfs: *const mjVFS) -> *mut mjModel {
    // WARNING: signature changed — verify body
    // Previous params: (s : * mut mjSpec, vfs : * const mjVFS)
    // Previous return: * mut mjModel
    todo ! ()
}

/// C: mj_recompile (user/user_api.h:46)
/// Calls: mjCModel::Compile, mjCModel::MakeData, mjCModel::SetError, mj_deleteData
#[allow(unused_variables, non_snake_case)]
pub fn mj_recompile(s: *mut mjSpec, vfs: *const mjVFS, m: *mut mjModel, d: *mut mjData) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (s : * mut mjSpec, vfs : * const mjVFS, m : * mut mjModel, d : * mut mjData)
    // Previous return: i32
    todo ! ()
}

/// C: mj_copySpec (user/user_api.h:49)
#[allow(unused_variables, non_snake_case)]
pub fn mj_copy_spec(s: *const mjSpec) -> *mut mjSpec {
    extern "C" { fn mj_copySpec_impl(s: *const mjSpec) -> *mut mjSpec; }
    // SAFETY: delegates to C implementation
    unsafe { mj_copySpec_impl(s) }
}

/// C: mjs_getError (user/user_api.h:52)
/// Calls: mjCModel::GetError, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_error(s: *mut mjSpec) -> *const i8 {
    extern "C" { fn mjs_getError_impl(s: *mut mjSpec) -> *const i8; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_getError_impl(s) }
}

/// C: mjs_isWarning (user/user_api.h:55)
/// Calls: mjCModel::GetError
#[allow(unused_variables, non_snake_case)]
pub fn mjs_is_warning(s: *mut mjSpec) -> i32 {
    extern "C" { fn mjs_isWarning_impl(s: *mut mjSpec) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_isWarning_impl(s) }
}

/// C: mj_deleteSpec (user/user_api.h:58)
/// Calls: mjCModel::Release
#[allow(unused_variables, non_snake_case)]
pub fn mj_delete_spec(s: *mut mjSpec) {
    // WARNING: signature changed — verify body
    // Previous params: (s : * mut mjSpec)
    // Previous return: ()
    extern "C" { fn mj_deleteSpec_impl (s : * mut mjSpec) ; } unsafe { mj_deleteSpec_impl (s) }
}

/// C: mjs_addSpec (user/user_api.h:61)
/// Calls: mjCModel::AppendSpec
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_spec(s: *mut mjSpec, child: *mut mjSpec) {
    extern "C" { fn mjs_addSpec_impl(s: *mut mjSpec, child: *mut mjSpec); }
    // SAFETY: delegates to C implementation
    unsafe { mjs_addSpec_impl(s, child) }
}

/// C: mjs_activatePlugin (user/user_api.h:64)
/// Calls: mjCModel::ActivatePlugin, mjp_getPlugin
#[allow(unused_variables, non_snake_case)]
pub fn mjs_activate_plugin(s: *mut mjSpec, name: *const i8) -> i32 {
    extern "C" { fn mjs_activatePlugin_impl(s: *mut mjSpec, name: *const i8) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_activatePlugin_impl(s, name) }
}

/// C: mjs_setDeepCopy (user/user_api.h:67)
/// Calls: mjCModel::SetDeepCopy
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_deep_copy(s: *mut mjSpec, deepcopy: i32) -> i32 {
    extern "C" { fn mjs_setDeepCopy_impl(s: *mut mjSpec, deepcopy: i32) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_setDeepCopy_impl(s, deepcopy) }
}

/// C: mj_copyBack (user/user_api.h:70)
/// Calls: mjCModel::CopyBack
#[allow(unused_variables, non_snake_case)]
pub fn mj_copy_back(s: *mut mjSpec, m: *const mjModel) -> i32 {
    extern "C" { fn mj_copyBack_impl(s: *mut mjSpec, m: *const mjModel) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mj_copyBack_impl(s, m) }
}

/// C: mjs_attach (user/user_api.h:76)
/// Calls: SetFrame, mjCModel::SetAttachWarningBoundary, mjCModel::SetError, mjs_addFrame, mjs_getParent, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjs_attach(parent: *mut mjsElement, child: *const mjsElement, prefix: *const i8, suffix: *const i8) -> *mut mjsElement {
    // WARNING: signature changed — verify body
    // Previous params: (parent : * mut mjsElement, child : * const mjsElement, prefix : * const i8, suffix : * const i8)
    // Previous return: * mut mjsElement
    todo ! ()
}

/// C: mjs_addBody (user/user_api.h:83)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_body(body: *mut mjsBody, def: *const mjsDefault) -> *mut mjsBody {
    extern "C" { fn mjs_addBody_impl(body: *mut mjsBody, def: *const mjsDefault) -> *mut mjsBody; }
    // SAFETY: delegates to C++ implementation; caller guarantees body is valid, def may be null
    unsafe { mjs_addBody_impl(body, def) }
}

/// C: mjs_addSite (user/user_api.h:86)
/// Calls: mjCBody::AddSite
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_site(body: *mut mjsBody, def: *const mjsDefault) -> *mut mjsSite {
    extern "C" { fn mjs_addSite_impl(body: *mut mjsBody, def: *const mjsDefault) -> *mut mjsSite; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_addSite_impl(body, def) }
}

/// C: mjs_addJoint (user/user_api.h:89)
/// Calls: mjCBody::AddJoint
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_joint(body: *mut mjsBody, def: *const mjsDefault) -> *mut mjsJoint {
    extern "C" { fn mjs_addJoint_impl(body: *mut mjsBody, def: *const mjsDefault) -> *mut mjsJoint; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_addJoint_impl(body, def) }
}

/// C: mjs_addFreeJoint (user/user_api.h:92)
/// Calls: mjCBody::AddFreeJoint
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_free_joint(body: *mut mjsBody) -> *mut mjsJoint {
    extern "C" { fn mjs_addFreeJoint_impl(body: *mut mjsBody) -> *mut mjsJoint; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_addFreeJoint_impl(body) }
}

/// C: mjs_addGeom (user/user_api.h:95)
/// Calls: mjCBody::AddGeom
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_geom(body: *mut mjsBody, def: *const mjsDefault) -> *mut mjsGeom {
    extern "C" { fn mjs_addGeom_impl(body: *mut mjsBody, def: *const mjsDefault) -> *mut mjsGeom; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_addGeom_impl(body, def) }
}

/// C: mjs_addCamera (user/user_api.h:98)
/// Calls: mjCBody::AddCamera
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_camera(body: *mut mjsBody, def: *const mjsDefault) -> *mut mjsCamera {
    extern "C" { fn mjs_addCamera_impl(body: *mut mjsBody, def: *const mjsDefault) -> *mut mjsCamera; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_addCamera_impl(body, def) }
}

/// C: mjs_addLight (user/user_api.h:101)
/// Calls: mjCBody::AddLight
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_light(body: *mut mjsBody, def: *const mjsDefault) -> *mut mjsLight {
    extern "C" { fn mjs_addLight_impl(body: *mut mjsBody, def: *const mjsDefault) -> *mut mjsLight; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_addLight_impl(body, def) }
}

/// C: mjs_addFrame (user/user_api.h:104)
/// Calls: mjCBody::AddFrame, mjCFrame::SetParent
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_frame(body: *mut mjsBody, parentframe: *mut mjsFrame) -> *mut mjsFrame {
    extern "C" { fn mjs_addFrame_impl(body: *mut mjsBody, parentframe: *mut mjsFrame) -> *mut mjsFrame; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_addFrame_impl(body, parentframe) }
}

/// C: mjs_delete (user/user_api.h:107)
/// Calls: mjCModel::IsAttached, mjCModel::SetError
#[allow(unused_variables, non_snake_case)]
pub fn mjs_delete(s: *mut mjSpec, element: *mut mjsElement) -> i32 {
    extern "C" { fn mjs_delete_impl(s: *mut mjSpec, element: *mut mjsElement) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_delete_impl(s, element) }
}

/// C: mjs_addActuator (user/user_api.h:113)
/// Calls: mjCModel::AddActuator
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_actuator(s: *mut mjSpec, def: *const mjsDefault) -> *mut mjsActuator {
    extern "C" { fn mjs_addActuator_impl(s: *mut mjSpec, def: *const mjsDefault) -> *mut mjsActuator; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_addActuator_impl(s, def) }
}

/// C: mjs_addSensor (user/user_api.h:116)
/// Calls: mjCModel::AddSensor
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_sensor(s: *mut mjSpec) -> *mut mjsSensor {
    extern "C" { fn mjs_addSensor_impl(s: *mut mjSpec) -> *mut mjsSensor; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_addSensor_impl(s) }
}

/// C: mjs_addFlex (user/user_api.h:119)
/// Calls: mjCModel::AddFlex
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_flex(s: *mut mjSpec) -> *mut mjsFlex {
    extern "C" { fn mjs_addFlex_impl(s: *mut mjSpec) -> *mut mjsFlex; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_addFlex_impl(s) }
}

/// C: mjs_makeFlex (user/user_api.h:122)
/// Calls: FlexcompDofFromStr, FlexcompTypeFromStr, mjCFlexcomp::Make, mjCModel::SetError, mju_error
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_make_flex(body: *mut mjsBody, name: *const i8, r#type: *const i8, dim: i32, dof: *const i8, count: [i32; 3], cellcount: [i32; 3], spacing: [f64; 3], scale: [f64; 3], radius: f64, mass: f64, inertiabox: f64, equality: i32, rigid: i32, flatskin: i32, elastic2d: i32, pos: [f64; 3], quat: [f64; 4], origin: [f64; 3], file: *const i8, vfs: *const mjVFS) -> *mut mjsFlex {
    // WARNING: signature changed — verify body
    // Previous params: (body : * mut mjsBody, name : * const i8, r#type : * const i8, dim : i32, dof : * const i8, count : [i32 ; 3], cellcount : [i32 ; 3], spacing : [f64 ; 3], scale : [f64 ; 3], radius : f64, mass : f64, inertiabox : f64, equality : i32, rigid : i32, flatskin : i32, elastic2d : i32, pos : [f64 ; 3], quat : [f64 ; 4], origin : [f64 ; 3], file : * const i8, vfs : * const mjVFS)
    // Previous return: * mut mjsFlex
    todo ! ()
}

/// C: mjs_addPair (user/user_api.h:130)
/// Calls: mjCModel::AddPair
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_pair(s: *mut mjSpec, def: *const mjsDefault) -> *mut mjsPair {
    extern "C" { fn mjs_addPair_impl(s: *mut mjSpec, def: *const mjsDefault) -> *mut mjsPair; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_addPair_impl(s, def) }
}

/// C: mjs_addExclude (user/user_api.h:133)
/// Calls: mjCModel::AddExclude
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_exclude(s: *mut mjSpec) -> *mut mjsExclude {
    extern "C" { fn mjs_addExclude_impl(s: *mut mjSpec) -> *mut mjsExclude; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_addExclude_impl(s) }
}

/// C: mjs_addEquality (user/user_api.h:136)
/// Calls: mjCModel::AddEquality
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_equality(s: *mut mjSpec, def: *const mjsDefault) -> *mut mjsEquality {
    extern "C" { fn mjs_addEquality_impl(s: *mut mjSpec, def: *const mjsDefault) -> *mut mjsEquality; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_addEquality_impl(s, def) }
}

/// C: mjs_addTendon (user/user_api.h:139)
/// Calls: mjCModel::AddTendon
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_tendon(s: *mut mjSpec, def: *const mjsDefault) -> *mut mjsTendon {
    extern "C" { fn mjs_addTendon_impl(s: *mut mjSpec, def: *const mjsDefault) -> *mut mjsTendon; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_addTendon_impl(s, def) }
}

/// C: mjs_wrapSite (user/user_api.h:142)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_wrap_site(tendon: *mut mjsTendon, name: *const i8) -> *mut mjsWrap {
    extern "C" { fn mjs_wrapSite_impl(tendon: *mut mjsTendon, name: *const i8) -> *mut mjsWrap; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_wrapSite_impl(tendon, name) }
}

/// C: mjs_wrapGeom (user/user_api.h:145)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_wrap_geom(tendon: *mut mjsTendon, name: *const i8, sidesite: *const i8) -> *mut mjsWrap {
    extern "C" { fn mjs_wrapGeom_impl(tendon: *mut mjsTendon, name: *const i8, sidesite: *const i8) -> *mut mjsWrap; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_wrapGeom_impl(tendon, name, sidesite) }
}

/// C: mjs_wrapJoint (user/user_api.h:148)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_wrap_joint(tendon: *mut mjsTendon, name: *const i8, coef: f64) -> *mut mjsWrap {
    extern "C" { fn mjs_wrapJoint_impl(tendon: *mut mjsTendon, name: *const i8, coef: f64) -> *mut mjsWrap; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_wrapJoint_impl(tendon, name, coef) }
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
    extern "C" { fn mjs_wrapPulley_impl(tendon: *mut mjsTendon, divisor: f64) -> *mut mjsWrap; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_wrapPulley_impl(tendon, divisor) }
}

/// C: mjs_addNumeric (user/user_api.h:154)
/// Calls: mjCModel::AddNumeric
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_numeric(s: *mut mjSpec) -> *mut mjsNumeric {
    extern "C" { fn mjs_addNumeric_impl(s: *mut mjSpec) -> *mut mjsNumeric; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_addNumeric_impl(s) }
}

/// C: mjs_addText (user/user_api.h:157)
/// Calls: mjCModel::AddText
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_text(s: *mut mjSpec) -> *mut mjsText {
    extern "C" { fn mjs_addText_impl(s: *mut mjSpec) -> *mut mjsText; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_addText_impl(s) }
}

/// C: mjs_addTuple (user/user_api.h:160)
/// Calls: mjCModel::AddTuple
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_tuple(s: *mut mjSpec) -> *mut mjsTuple {
    extern "C" { fn mjs_addTuple_impl(s: *mut mjSpec) -> *mut mjsTuple; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_addTuple_impl(s) }
}

/// C: mjs_addKey (user/user_api.h:163)
/// Calls: mjCModel::AddKey
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_key(s: *mut mjSpec) -> *mut mjsKey {
    extern "C" { fn mjs_addKey_impl(s: *mut mjSpec) -> *mut mjsKey; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_addKey_impl(s) }
}

/// C: mjs_addPlugin (user/user_api.h:166)
/// Calls: mjCModel::AddPlugin
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_plugin(s: *mut mjSpec) -> *mut mjsPlugin {
    extern "C" { fn mjs_addPlugin_impl(s: *mut mjSpec) -> *mut mjsPlugin; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_addPlugin_impl(s) }
}

/// C: mjs_addDefault (user/user_api.h:169)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_default(s: *mut mjSpec, classname: *const i8, parent: *const mjsDefault) -> *mut mjsDefault {
    extern "C" { fn mjs_addDefault_impl(s: *mut mjSpec, classname: *const i8, parent: *const mjsDefault) -> *mut mjsDefault; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_addDefault_impl(s, classname, parent) }
}

/// C: mjs_setToMotor (user/user_api.h:175)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_to_motor(actuator: *mut mjsActuator) -> *const i8 {
    extern "C" { fn mjs_setToMotor_impl(actuator: *mut mjsActuator) -> *const i8; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_setToMotor_impl(actuator) }
}

/// C: mjs_setToPosition (user/user_api.h:178)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_to_position(actuator: *mut mjsActuator, kp: f64, kv: [f64; 1], dampratio: [f64; 1], timeconst: [f64; 1], inheritrange: f64) -> *const i8 {
    extern "C" { fn mjs_setToPosition_impl(actuator: *mut mjsActuator, kp: f64, kv: [f64; 1], dampratio: [f64; 1], timeconst: [f64; 1], inheritrange: f64) -> *const i8; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_setToPosition_impl(actuator, kp, kv, dampratio, timeconst, inheritrange) }
}

/// C: mjs_setToIntVelocity (user/user_api.h:182)
/// Calls: mjs_setToPosition
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_to_int_velocity(actuator: *mut mjsActuator, kp: f64, kv: [f64; 1], dampratio: [f64; 1], timeconst: [f64; 1], inheritrange: f64) -> *const i8 {
    extern "C" { fn mjs_setToIntVelocity_impl(actuator: *mut mjsActuator, kp: f64, kv: [f64; 1], dampratio: [f64; 1], timeconst: [f64; 1], inheritrange: f64) -> *const i8; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_setToIntVelocity_impl(actuator, kp, kv, dampratio, timeconst, inheritrange) }
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
    extern "C" { fn mjs_setToVelocity_impl(actuator: *mut mjsActuator, kv: f64) -> *const i8; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_setToVelocity_impl(actuator, kv) }
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
    extern "C" { fn mjs_setToDamper_impl(actuator: *mut mjsActuator, kv: f64) -> *const i8; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_setToDamper_impl(actuator, kv) }
}

/// C: mjs_setToCylinder (user/user_api.h:192)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_to_cylinder(actuator: *mut mjsActuator, timeconst: f64, bias: f64, area: f64, diameter: f64) -> *const i8 {
    extern "C" { fn mjs_setToCylinder_impl(actuator: *mut mjsActuator, timeconst: f64, bias: f64, area: f64, diameter: f64) -> *const i8; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_setToCylinder_impl(actuator, timeconst, bias, area, diameter) }
}

/// C: mjs_setToMuscle (user/user_api.h:196)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_to_muscle(actuator: *mut mjsActuator, timeconst: [f64; 2], tausmooth: f64, range: [f64; 2], force: f64, scale: f64, lmin: f64, lmax: f64, vmax: f64, fpmax: f64, fvmax: f64) -> *const i8 {
    extern "C" { fn mjs_setToMuscle_impl(actuator: *mut mjsActuator, timeconst: [f64; 2], tausmooth: f64, range: [f64; 2], force: f64, scale: f64, lmin: f64, lmax: f64, vmax: f64, fpmax: f64, fvmax: f64) -> *const i8; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_setToMuscle_impl(actuator, timeconst, tausmooth, range, force, scale, lmin, lmax, vmax, fpmax, fvmax) }
}

/// C: mjs_setToAdhesion (user/user_api.h:201)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_to_adhesion(actuator: *mut mjsActuator, gain: f64) -> *const i8 {
    extern "C" { fn mjs_setToAdhesion_impl(actuator: *mut mjsActuator, gain: f64) -> *const i8; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_setToAdhesion_impl(actuator, gain) }
}

/// C: mjs_setToDCMotor (user/user_api.h:204)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_to_dc_motor(actuator: *mut mjsActuator, motorconst: [f64; 2], resistance: f64, nominal: [f64; 3], saturation: [f64; 3], inductance: [f64; 2], cogging: [f64; 3], controller: [f64; 6], thermal: [f64; 6], lugre: [f64; 5], input_mode: i32) -> *const i8 {
    extern "C" { fn mjs_setToDCMotor_impl(actuator: *mut mjsActuator, motorconst: [f64; 2], resistance: f64, nominal: [f64; 3], saturation: [f64; 3], inductance: [f64; 2], cogging: [f64; 3], controller: [f64; 6], thermal: [f64; 6], lugre: [f64; 5], input_mode: i32) -> *const i8; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_setToDCMotor_impl(actuator, motorconst, resistance, nominal, saturation, inductance, cogging, controller, thermal, lugre, input_mode) }
}

/// C: mjs_addMesh (user/user_api.h:213)
/// Calls: mjCModel::AddMesh
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_mesh(s: *mut mjSpec, def: *const mjsDefault) -> *mut mjsMesh {
    extern "C" { fn mjs_addMesh_impl(s: *mut mjSpec, def: *const mjsDefault) -> *mut mjsMesh; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_addMesh_impl(s, def) }
}

/// C: mjs_addHField (user/user_api.h:216)
/// Calls: mjCModel::AddHField
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_h_field(s: *mut mjSpec) -> *mut mjsHField {
    extern "C" { fn mjs_addHField_impl(s: *mut mjSpec) -> *mut mjsHField; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_addHField_impl(s) }
}

/// C: mjs_addSkin (user/user_api.h:219)
/// Calls: mjCModel::AddSkin
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_skin(s: *mut mjSpec) -> *mut mjsSkin {
    extern "C" { fn mjs_addSkin_impl(s: *mut mjSpec) -> *mut mjsSkin; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_addSkin_impl(s) }
}

/// C: mjs_addTexture (user/user_api.h:222)
/// Calls: mjCModel::AddTexture
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_texture(s: *mut mjSpec) -> *mut mjsTexture {
    extern "C" { fn mjs_addTexture_impl(s: *mut mjSpec) -> *mut mjsTexture; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_addTexture_impl(s) }
}

/// C: mjs_addMaterial (user/user_api.h:225)
/// Calls: mjCModel::AddMaterial
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_material(s: *mut mjSpec, def: *const mjsDefault) -> *mut mjsMaterial {
    extern "C" { fn mjs_addMaterial_impl(s: *mut mjSpec, def: *const mjsDefault) -> *mut mjsMaterial; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_addMaterial_impl(s, def) }
}

/// C: mjs_makeMesh (user/user_api.h:228)
/// Calls: mjCMesh::MakeCone, mjCMesh::MakeHemisphere, mjCMesh::MakeRect, mjCMesh::MakeSphere, mjCMesh::MakeSupersphere, mjCMesh::MakeSupertorus, mjCMesh::MakeWedge, mjCModel::SetError
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_make_mesh(mesh: *mut mjsMesh, builtin: mjtMeshBuiltin, params: *mut f64, nparams: i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (mesh : * mut mjsMesh, builtin : mjtMeshBuiltin, params : * mut f64, nparams : i32)
    // Previous return: i32
    todo ! ()
}

/// C: mjs_getSpec (user/user_api.h:233)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_spec(element: *const mjsElement) -> *mut mjSpec {
    extern "C" { fn mjs_getSpec_impl(element: *const mjsElement) -> *mut mjSpec; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_getSpec_impl(element) }
}

/// C: mjs_getOriginSpec (user/user_api.h:237)
/// Calls: mjCModel::FindSpec
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_origin_spec(element: *const mjsElement) -> *mut mjSpec {
    // WARNING: signature changed — verify body
    // Previous params: (element : * const mjsElement)
    // Previous return: * mut mjSpec
    extern "C" { fn mjs_getOriginSpec_impl (element : * const mjsElement) -> * mut mjSpec ; } unsafe { mjs_getOriginSpec_impl (element) }
}

/// C: mjs_findSpec (user/user_api.h:240)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_find_spec(spec: *const mjSpec, name: *const i8) -> *mut mjSpec {
    extern "C" { fn mjs_findSpec_impl(spec: *const mjSpec, name: *const i8) -> *mut mjSpec; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_findSpec_impl(spec, name) }
}

/// C: mjs_findBody (user/user_api.h:243)
/// Calls: mjs_findElement
#[allow(unused_variables, non_snake_case)]
pub fn mjs_find_body(s: *const mjSpec, name: *const i8) -> *mut mjsBody {
    // WARNING: signature changed — verify body
    // Previous params: (s : * const mjSpec, name : * const i8)
    // Previous return: * mut mjsBody
    todo ! ()
}

/// C: mjs_findElement (user/user_api.h:246)
/// Calls: mjCModel::GetWorld, mjCModel::IsCompiled
#[allow(unused_variables, non_snake_case)]
pub fn mjs_find_element(s: *const mjSpec, r#type: mjtObj, name: *const i8) -> *mut mjsElement {
    extern "C" { fn mjs_findElement_impl(s: *const mjSpec, r#type: mjtObj, name: *const i8) -> *mut mjsElement; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_findElement_impl(s, r#type, name) }
}

/// C: mjs_findChild (user/user_api.h:249)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_find_child(body: *const mjsBody, name: *const i8) -> *mut mjsBody {
    extern "C" { fn mjs_findChild_impl(body: *const mjsBody, name: *const i8) -> *mut mjsBody; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_findChild_impl(body, name) }
}

/// C: mjs_getParent (user/user_api.h:252)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_parent(element: *const mjsElement) -> *mut mjsBody {
    extern "C" { fn mjs_getParent_impl(element: *const mjsElement) -> *mut mjsBody; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_getParent_impl(element) }
}

/// C: mjs_getFrame (user/user_api.h:255)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_frame(element: *const mjsElement) -> *mut mjsFrame {
    extern "C" { fn mjs_getFrame_impl(element: *const mjsElement) -> *mut mjsFrame; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_getFrame_impl(element) }
}

/// C: mjs_findFrame (user/user_api.h:258)
/// Calls: mjs_findElement
#[allow(unused_variables, non_snake_case)]
pub fn mjs_find_frame(s: *const mjSpec, name: *const i8) -> *mut mjsFrame {
    // WARNING: signature changed — verify body
    // Previous params: (s : * const mjSpec, name : * const i8)
    // Previous return: * mut mjsFrame
    todo ! ()
}

/// C: mjs_getDefault (user/user_api.h:261)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_default(element: *const mjsElement) -> *mut mjsDefault {
    extern "C" { fn mjs_getDefault_impl(element: *const mjsElement) -> *mut mjsDefault; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_getDefault_impl(element) }
}

/// C: mjs_findDefault (user/user_api.h:264)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_find_default(s: *const mjSpec, classname: *const i8) -> *mut mjsDefault {
    extern "C" { fn mjs_findDefault_impl(s: *const mjSpec, classname: *const i8) -> *mut mjsDefault; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_findDefault_impl(s, classname) }
}

/// C: mjs_getSpecDefault (user/user_api.h:267)
/// Calls: mjCModel::Default
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_spec_default(s: *const mjSpec) -> *mut mjsDefault {
    extern "C" { fn mjs_getSpecDefault_impl(s: *const mjSpec) -> *mut mjsDefault; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_getSpecDefault_impl(s) }
}

/// C: mjs_getId (user/user_api.h:270)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_id(element: *const mjsElement) -> i32 {
    extern "C" { fn mjs_getId_impl(element: *const mjsElement) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_getId_impl(element) }
}

/// C: mjs_firstChild (user/user_api.h:276)
/// Calls: mjCBody::NextChild
#[allow(unused_variables, non_snake_case)]
pub fn mjs_first_child(body: *const mjsBody, r#type: mjtObj, recurse: i32) -> *mut mjsElement {
    extern "C" { fn mjs_firstChild_impl(body: *const mjsBody, r#type: mjtObj, recurse: i32) -> *mut mjsElement; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_firstChild_impl(body, r#type, recurse) }
}

/// C: mjs_nextChild (user/user_api.h:280)
/// Calls: mjCBody::NextChild
#[allow(unused_variables, non_snake_case)]
pub fn mjs_next_child(body: *const mjsBody, child: *const mjsElement, recurse: i32) -> *mut mjsElement {
    extern "C" { fn mjs_nextChild_impl(body: *const mjsBody, child: *const mjsElement, recurse: i32) -> *mut mjsElement; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_nextChild_impl(body, child, recurse) }
}

/// C: mjs_firstElement (user/user_api.h:283)
/// Calls: mjCModel::NextObject
#[allow(unused_variables, non_snake_case)]
pub fn mjs_first_element(s: *const mjSpec, r#type: mjtObj) -> *mut mjsElement {
    // WARNING: signature changed — verify body
    // Previous params: (s : * const mjSpec, r#type : mjtObj)
    // Previous return: * mut mjsElement
    extern "C" { fn mjs_firstElement_impl (s : * const mjSpec , r#type : mjtObj) -> * mut mjsElement ; } unsafe { mjs_firstElement_impl (s , r#type) }
}

/// C: mjs_nextElement (user/user_api.h:286)
/// Calls: mjCModel::NextObject
#[allow(unused_variables, non_snake_case)]
pub fn mjs_next_element(s: *const mjSpec, element: *const mjsElement) -> *mut mjsElement {
    // WARNING: signature changed — verify body
    // Previous params: (s : * const mjSpec, element : * const mjsElement)
    // Previous return: * mut mjsElement
    extern "C" { fn mjs_nextElement_impl (s : * const mjSpec , element : * const mjsElement) -> * mut mjsElement ; } unsafe { mjs_nextElement_impl (s , element) }
}

/// C: mjs_getWrapTarget (user/user_api.h:289)
/// Calls: mjCWrap::Type, mjs_getSpec
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_wrap_target(wrap: *const mjsWrap) -> *mut mjsElement {
    extern "C" { fn mjs_getWrapTarget_impl(wrap: *const mjsWrap) -> *mut mjsElement; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_getWrapTarget_impl(wrap) }
}

/// C: mjs_getWrapSideSite (user/user_api.h:292)
/// Calls: mjCWrap::Type, mjs_asSite, mjs_getSpec
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_wrap_side_site(wrap: *const mjsWrap) -> *mut mjsSite {
    extern "C" { fn mjs_getWrapSideSite_impl(wrap: *const mjsWrap) -> *mut mjsSite; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_getWrapSideSite_impl(wrap) }
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
    extern "C" { fn mjs_getWrapDivisor_impl(wrap: *const mjsWrap) -> f64; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_getWrapDivisor_impl(wrap) }
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
    extern "C" { fn mjs_getWrapCoef_impl(wrap: *const mjsWrap) -> f64; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_getWrapCoef_impl(wrap) }
}

/// C: mjs_asBody (user/user_api.h:301)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_body(element: *mut mjsElement) -> *mut mjsBody {
    extern "C" { fn mjs_asBody_impl(element: *mut mjsElement) -> *mut mjsBody; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_asBody_impl(element) }
}

/// C: mjs_asGeom (user/user_api.h:304)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_geom(element: *mut mjsElement) -> *mut mjsGeom {
    extern "C" { fn mjs_asGeom_impl(element: *mut mjsElement) -> *mut mjsGeom; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_asGeom_impl(element) }
}

/// C: mjs_asJoint (user/user_api.h:307)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_joint(element: *mut mjsElement) -> *mut mjsJoint {
    extern "C" { fn mjs_asJoint_impl(element: *mut mjsElement) -> *mut mjsJoint; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_asJoint_impl(element) }
}

/// C: mjs_asSite (user/user_api.h:310)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_site(element: *mut mjsElement) -> *mut mjsSite {
    extern "C" { fn mjs_asSite_impl(element: *mut mjsElement) -> *mut mjsSite; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_asSite_impl(element) }
}

/// C: mjs_asCamera (user/user_api.h:313)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_camera(element: *mut mjsElement) -> *mut mjsCamera {
    extern "C" { fn mjs_asCamera_impl(element: *mut mjsElement) -> *mut mjsCamera; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_asCamera_impl(element) }
}

/// C: mjs_asLight (user/user_api.h:316)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_light(element: *mut mjsElement) -> *mut mjsLight {
    extern "C" { fn mjs_asLight_impl(element: *mut mjsElement) -> *mut mjsLight; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_asLight_impl(element) }
}

/// C: mjs_asFrame (user/user_api.h:319)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_frame(element: *mut mjsElement) -> *mut mjsFrame {
    extern "C" { fn mjs_asFrame_impl(element: *mut mjsElement) -> *mut mjsFrame; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_asFrame_impl(element) }
}

/// C: mjs_asActuator (user/user_api.h:322)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_actuator(element: *mut mjsElement) -> *mut mjsActuator {
    extern "C" { fn mjs_asActuator_impl(element: *mut mjsElement) -> *mut mjsActuator; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_asActuator_impl(element) }
}

/// C: mjs_asSensor (user/user_api.h:325)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_sensor(element: *mut mjsElement) -> *mut mjsSensor {
    extern "C" { fn mjs_asSensor_impl(element: *mut mjsElement) -> *mut mjsSensor; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_asSensor_impl(element) }
}

/// C: mjs_asFlex (user/user_api.h:328)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_flex(element: *mut mjsElement) -> *mut mjsFlex {
    extern "C" { fn mjs_asFlex_impl(element: *mut mjsElement) -> *mut mjsFlex; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_asFlex_impl(element) }
}

/// C: mjs_asPair (user/user_api.h:331)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_pair(element: *mut mjsElement) -> *mut mjsPair {
    extern "C" { fn mjs_asPair_impl(element: *mut mjsElement) -> *mut mjsPair; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_asPair_impl(element) }
}

/// C: mjs_asEquality (user/user_api.h:334)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_equality(element: *mut mjsElement) -> *mut mjsEquality {
    extern "C" { fn mjs_asEquality_impl(element: *mut mjsElement) -> *mut mjsEquality; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_asEquality_impl(element) }
}

/// C: mjs_asExclude (user/user_api.h:337)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_exclude(element: *mut mjsElement) -> *mut mjsExclude {
    extern "C" { fn mjs_asExclude_impl(element: *mut mjsElement) -> *mut mjsExclude; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_asExclude_impl(element) }
}

/// C: mjs_asTendon (user/user_api.h:340)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_tendon(element: *mut mjsElement) -> *mut mjsTendon {
    extern "C" { fn mjs_asTendon_impl(element: *mut mjsElement) -> *mut mjsTendon; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_asTendon_impl(element) }
}

/// C: mjs_asNumeric (user/user_api.h:343)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_numeric(element: *mut mjsElement) -> *mut mjsNumeric {
    extern "C" { fn mjs_asNumeric_impl(element: *mut mjsElement) -> *mut mjsNumeric; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_asNumeric_impl(element) }
}

/// C: mjs_asText (user/user_api.h:346)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_text(element: *mut mjsElement) -> *mut mjsText {
    extern "C" { fn mjs_asText_impl(element: *mut mjsElement) -> *mut mjsText; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_asText_impl(element) }
}

/// C: mjs_asTuple (user/user_api.h:349)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_tuple(element: *mut mjsElement) -> *mut mjsTuple {
    extern "C" { fn mjs_asTuple_impl(element: *mut mjsElement) -> *mut mjsTuple; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_asTuple_impl(element) }
}

/// C: mjs_asKey (user/user_api.h:352)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_key(element: *mut mjsElement) -> *mut mjsKey {
    extern "C" { fn mjs_asKey_impl(element: *mut mjsElement) -> *mut mjsKey; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_asKey_impl(element) }
}

/// C: mjs_asMesh (user/user_api.h:355)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_mesh(element: *mut mjsElement) -> *mut mjsMesh {
    // WARNING: signature changed — verify body
    // Previous params: (element : * mut mjsElement)
    // Previous return: * mut mjsMesh
    extern "C" { fn mjs_asMesh_impl (element : * mut mjsElement) -> * mut mjsMesh ; } unsafe { mjs_asMesh_impl (element) }
}

/// C: mjs_asHField (user/user_api.h:358)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_h_field(element: *mut mjsElement) -> *mut mjsHField {
    // WARNING: signature changed — verify body
    // Previous params: (element : * mut mjsElement)
    // Previous return: * mut mjsHField
    extern "C" { fn mjs_asHField_impl (element : * mut mjsElement) -> * mut mjsHField ; } unsafe { mjs_asHField_impl (element) }
}

/// C: mjs_asSkin (user/user_api.h:361)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_skin(element: *mut mjsElement) -> *mut mjsSkin {
    // WARNING: signature changed — verify body
    // Previous params: (element : * mut mjsElement)
    // Previous return: * mut mjsSkin
    extern "C" { fn mjs_asSkin_impl (element : * mut mjsElement) -> * mut mjsSkin ; } unsafe { mjs_asSkin_impl (element) }
}

/// C: mjs_asTexture (user/user_api.h:364)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_texture(element: *mut mjsElement) -> *mut mjsTexture {
    // WARNING: signature changed — verify body
    // Previous params: (element : * mut mjsElement)
    // Previous return: * mut mjsTexture
    extern "C" { fn mjs_asTexture_impl (element : * mut mjsElement) -> * mut mjsTexture ; } unsafe { mjs_asTexture_impl (element) }
}

/// C: mjs_asMaterial (user/user_api.h:367)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_material(element: *mut mjsElement) -> *mut mjsMaterial {
    extern "C" { fn mjs_asMaterial_impl(element: *mut mjsElement) -> *mut mjsMaterial; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_asMaterial_impl(element) }
}

/// C: mjs_asPlugin (user/user_api.h:370)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_plugin(element: *mut mjsElement) -> *mut mjsPlugin {
    extern "C" { fn mjs_asPlugin_impl(element: *mut mjsElement) -> *mut mjsPlugin; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_asPlugin_impl(element) }
}

/// C: mjs_setName (user/user_api.h:376)
/// Calls: mjCModel::CheckRepeat, mjCModel::SetError
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_name(element: *mut mjsElement, name: *const i8) -> i32 {
    extern "C" { fn mjs_setName_impl(element: *mut mjsElement, name: *const i8) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_setName_impl(element, name) }
}

/// C: mjs_setPluginAttributes (user/user_api.h:409)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_plugin_attributes(plugin: *mut mjsPlugin, attributes: *mut ()) {
    // WARNING: signature changed — verify body
    // Previous params: (plugin : * mut mjsPlugin, attributes : * mut ())
    // Previous return: ()
    todo ! ()
}

/// C: mjs_getWrapNum (user/user_api.h:424)
/// Calls: mjCTendon::NumWraps
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_wrap_num(tendonspec: *const mjsTendon) -> i32 {
    extern "C" { fn mjs_getWrapNum_impl(tendonspec: *const mjsTendon) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_getWrapNum_impl(tendonspec) }
}

/// C: mjs_getWrap (user/user_api.h:426)
/// Calls: mjCTendon::GetWrap, mjCTendon::NumWraps, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_wrap(tendonspec: *const mjsTendon, i: i32) -> *mut mjsWrap {
    extern "C" { fn mjs_getWrap_impl(tendonspec: *const mjsTendon, i: i32) -> *mut mjsWrap; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_getWrap_impl(tendonspec, i) }
}

/// C: mjs_getPluginAttributes (user/user_api.h:429)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_plugin_attributes(plugin: *const mjsPlugin) -> *const () {
    extern "C" { fn mjs_getPluginAttributes_impl(plugin: *const mjsPlugin) -> *const (); }
    // SAFETY: delegates to C implementation
    unsafe { mjs_getPluginAttributes_impl(plugin) }
}

/// C: mjs_isAuthored (user/user_api.h:435)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_is_authored(elem_ptr: *const (), field_ptr: *const ()) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (elem_ptr : * const (), field_ptr : * const ())
    // Previous return: i32
    todo ! ()
}

/// C: mjs_setAuthored (user/user_api.h:438)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_authored(elem_ptr: *const (), field_ptr: *const (), authored: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (elem_ptr : * const (), field_ptr : * const (), authored : i32)
    // Previous return: ()
    todo ! ()
}

/// C: mjs_setDefault (user/user_api.h:441)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_default(element: *mut mjsElement, def: *const mjsDefault) {
    // WARNING: signature changed — verify body
    // Previous params: (element : * mut mjsElement, def : * const mjsDefault)
    // Previous return: ()
    extern "C" { fn mjs_setDefault_impl(element: *mut mjsElement, def: *const mjsDefault); }
    // SAFETY: delegates to C++ implementation; caller guarantees element is valid, def may be null
    unsafe { mjs_setDefault_impl(element, def) }
}

/// C: mjs_setFrame (user/user_api.h:444)
/// Calls: mjCBase::SetFrame, mjCModel::SetError
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_frame(dest: *mut mjsElement, frame: *mut mjsFrame) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (dest : * mut mjsElement, frame : * mut mjsFrame)
    // Previous return: i32
    todo ! ()
}

/// C: mjs_resolveOrientation (user/user_api.h:447)
/// Calls: ResolveOrientation
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_resolve_orientation(quat: [f64; 4], degree: u8, sequence: *const i8, orientation: *const mjsOrientation) -> *const i8 {
    extern "C" { fn mjs_resolveOrientation_impl(quat: [f64; 4], degree: u8, sequence: *const i8, orientation: *const mjsOrientation) -> *const i8; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_resolveOrientation_impl(quat, degree, sequence, orientation) }
}

/// C: mjs_bodyToFrame (user/user_api.h:451)
/// Calls: mjCBody::ToFrame
#[allow(unused_variables, non_snake_case)]
pub fn mjs_body_to_frame(body: *mut *mut mjsBody) -> *mut mjsFrame {
    extern "C" { fn mjs_bodyToFrame_impl(body: *mut *mut mjsBody) -> *mut mjsFrame; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_bodyToFrame_impl(body) }
}

/// C: mjs_setUserValue (user/user_api.h:454)
/// Calls: mjs_setUserValueWithCleanup
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_user_value(element: *mut mjsElement, key: *const i8, data: *const ()) {
    // WARNING: signature changed — verify body
    // Previous params: (element : * mut mjsElement, key : * const i8, data : * const ())
    // Previous return: ()
    todo ! ()
}

/// C: mjs_setUserValueWithCleanup (user/user_api.h:457)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_user_value_with_cleanup(element: *mut mjsElement, key: *const i8, data: *const (), cleanup: Option<unsafe extern "C" fn()>) {
    // WARNING: signature changed — verify body
    // Previous params: (element : * mut mjsElement, key : * const i8, data : * const (), cleanup : Option < unsafe extern "C" fn () >)
    // Previous return: ()
    todo ! ()
}

/// C: mjs_getUserValue (user/user_api.h:462)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_user_value(element: *mut mjsElement, key: *const i8) -> *const () {
    extern "C" { fn mjs_getUserValue_impl(element: *mut mjsElement, key: *const i8) -> *const (); }
    // SAFETY: delegates to C implementation
    unsafe { mjs_getUserValue_impl(element, key) }
}

/// C: mjs_deleteUserValue (user/user_api.h:465)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_delete_user_value(element: *mut mjsElement, key: *const i8) {
    extern "C" { fn mjs_deleteUserValue_impl(element: *mut mjsElement, key: *const i8); }
    // SAFETY: delegates to C implementation
    unsafe { mjs_deleteUserValue_impl(element, key) }
}

/// C: mjs_sensorDim (user/user_api.h:468)
/// Calls: mju_condataSize, mju_raydataSize
#[allow(unused_variables, non_snake_case)]
pub fn mjs_sensor_dim(sensor: *const mjsSensor) -> i32 {
    extern "C" { fn mjs_sensorDim_impl(sensor: *const mjsSensor) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjs_sensorDim_impl(sensor) }
}

/// C: mj_getCacheCapacity (user/user_api.h:551)
/// Calls: mjCCache::Capacity
#[allow(unused_variables, non_snake_case)]
pub fn mj_get_cache_capacity(cache: *const mjCache) -> usize {
    extern "C" { fn mj_getCacheCapacity_impl(cache: *const mjCache) -> usize; }
    // SAFETY: delegates to C implementation
    unsafe { mj_getCacheCapacity_impl(cache) }
}

/// C: mj_setCacheCapacity (user/user_api.h:554)
/// Calls: mjCCache::Capacity, mjCCache::SetCapacity
#[allow(unused_variables, non_snake_case)]
pub fn mj_set_cache_capacity(cache: *mut mjCache, size: usize) -> usize {
    // WARNING: signature changed — verify body
    // Previous params: (cache : * mut mjCache, size : usize)
    // Previous return: usize
    todo ! ()
}

/// C: mj_getCacheSize (user/user_api.h:557)
/// Calls: mjCCache::Size
#[allow(unused_variables, non_snake_case)]
pub fn mj_get_cache_size(cache: *const mjCache) -> usize {
    extern "C" { fn mj_getCacheSize_impl(cache: *const mjCache) -> usize; }
    // SAFETY: delegates to C implementation
    unsafe { mj_getCacheSize_impl(cache) }
}

/// C: mj_clearCache (user/user_api.h:560)
/// Calls: mjCCache::Reset
#[allow(unused_variables, non_snake_case)]
pub fn mj_clear_cache(cache: *mut mjCache) {
    extern "C" { fn mj_clearCache_impl(cache: *mut mjCache); }
    // SAFETY: delegates to C implementation
    unsafe { mj_clearCache_impl(cache) }
}

/// C: mj_getCache (user/user_api.h:563)
/// Calls: mjCCache::Capacity
#[allow(unused_variables, non_snake_case)]
pub fn mj_get_cache() -> *mut mjCache {
    // WARNING: signature changed — verify body
    // Previous params: ()
    // Previous return: * mut mjCache
    extern "C" { fn mj_getCache_impl () -> * mut mjCache ; } unsafe { mj_getCache_impl () }
}

