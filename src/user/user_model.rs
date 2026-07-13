//! Port of: user/user_model.cc
//! IR hash: d3ac8715281cd691
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: IsSameVec (user/user_model.cc:72)
#[allow(unused_variables, non_snake_case)]
pub fn is_same_vec(pos1: *const type_parameter_0_0, pos2: *const type_parameter_0_0) -> bool {
    // NOTE: signature changed from previous IR version
    // Previous params: (pos1 : * const type_parameter_0_0, pos2 : * const type_parameter_0_0)
    // Previous return: bool
    todo!("re-translate: params renamed")
}

/// C: NumCompilerThreads (user/user_model.cc:79)
#[allow(unused_variables, non_snake_case)]
pub fn num_compiler_threads(upper_bound: i32) -> u32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (upper_bound : i32)
    // Previous return: u32
    todo!("re-translate: params renamed")
}

/// C: IsSameQuat (user/user_model.cc:93)
#[allow(unused_variables, non_snake_case)]
pub fn is_same_quat(quat1: *const type_parameter_0_0, quat2: *const type_parameter_0_0) -> bool {
    // NOTE: signature changed from previous IR version
    // Previous params: (quat1 : * const type_parameter_0_0, quat2 : * const type_parameter_0_0)
    // Previous return: bool
    todo!("re-translate: params renamed")
}

/// C: IsSamePose (user/user_model.cc:111)
#[allow(unused_variables, non_snake_case)]
pub fn is_same_pose(pos1: *const type_parameter_0_0, pos2: *const type_parameter_0_0, quat1: *const type_parameter_0_0, quat2: *const type_parameter_0_0) -> bool {
    // NOTE: signature changed from previous IR version
    // Previous params: (pos1 : * const type_parameter_0_0, pos2 : * const type_parameter_0_0, quat1 : * const type_parameter_0_0, quat2 : * const type_parameter_0_0)
    // Previous return: bool
    todo!("re-translate: params renamed")
}

/// C: IsNullPose (user/user_model.cc:127)
#[allow(unused_variables, non_snake_case)]
pub fn is_null_pose(pos: *const type_parameter_0_0, quat: *const type_parameter_0_0) -> bool {
    // NOTE: signature changed from previous IR version
    // Previous params: (pos : * const type_parameter_0_0, quat : * const type_parameter_0_0)
    // Previous return: bool
    todo!("re-translate: params renamed")
}

/// C: GetBodyIdFromWrap (user/user_model.cc:134)
/// Calls: mjCWrap::Type
#[allow(unused_variables, non_snake_case)]
pub fn get_body_id_from_wrap(wrap: *const mjCWrap) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (wrap : * const mjCWrap)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjCModel::CopyList (user/user_model.cc:261)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_list(self_ptr: *mut mjCModel, dest: *mut i32, source: *const i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, dest : * mut i32, source : * const i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: resetlist (user/user_model.cc:304)
#[allow(unused_variables, non_snake_case)]
pub fn resetlist(list: *mut i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (list : * mut i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::CopyPlugin (user/user_model.cc:396)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_plugin(self_ptr: *mut mjCModel, source: *const i32, list: *const i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, source : * const i32, list : * const i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: IsPluginActive (user/user_model.cc:440)
#[allow(unused_variables, non_snake_case)]
pub fn is_plugin_active(plugin: *const mjpPlugin, active_plugins: *const i32) -> bool {
    // NOTE: signature changed from previous IR version
    // Previous params: (plugin : * const mjpPlugin, active_plugins : * const i32)
    // Previous return: bool
    todo!("re-translate: params renamed")
}

/// C: mjCModel::RemoveFromList (user/user_model.cc:508)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_remove_from_list(self_ptr: *mut mjCModel, list: *mut i32, other: *const mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, list : * mut i32, other : * const mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::DeleteAll (user/user_model.cc:545)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_delete_all(self_ptr: *mut mjCModel, elements: *mut i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, elements : * mut i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::MarkPluginInstance (user/user_model.cc:555)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_mark_plugin_instance(self_ptr: *mut mjCModel, instances: *mut i32, list: *const i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, instances : * mut i32, list : * const i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: deletefromlist (user/user_model.cc:715)
#[allow(unused_variables, non_snake_case)]
pub fn deletefromlist(list: *mut i32, element: *mut mjsElement) {
    // NOTE: signature changed from previous IR version
    // Previous params: (list : * mut i32, element : * mut mjsElement)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::AddObject (user/user_model.cc:1252)
/// Calls: mjCModel::Signature
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_object(self_ptr: *mut mjCModel, list: *mut i32, r#type: string) -> *mut T {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, list : * mut i32, r#type : string)
    // Previous return: * mut T
    todo!("re-translate: params renamed")
}

/// C: mjCModel::AddObjectDefault (user/user_model.cc:1263)
/// Calls: mjCModel::Signature
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_object_default(self_ptr: *mut mjCModel, list: *mut i32, r#type: string, def: *mut mjCDef) -> *mut T {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, list : * mut i32, r#type : string, def : * mut mjCDef)
    // Previous return: * mut T
    todo!("re-translate: params renamed")
}

/// C: GetNext (user/user_model.cc:1411)
#[allow(unused_variables, non_snake_case)]
pub fn get_next(list: *const i32, child: *const mjsElement) -> *mut mjsElement {
    // NOTE: signature changed from previous IR version
    // Previous params: (list : * const i32, child : * const mjsElement)
    // Previous return: * mut mjsElement
    todo!("re-translate: params renamed")
}

/// C: findobject (user/user_model.cc:1596)
#[allow(unused_variables, non_snake_case)]
pub fn findobject(name: string_view, list: *const i32, ids: *const mjKeyMap) -> *mut T {
    // NOTE: signature changed from previous IR version
    // Previous params: (name : string_view, list : * const i32, ids : * const mjKeyMap)
    // Previous return: * mut T
    todo!("re-translate: params renamed")
}

/// C: mjCModel::FindAsset (user/user_model.cc:1621)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_find_asset(self_ptr: *mut mjCModel, name: string_view, list: *const i32) -> *mut mjCBase {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, name : string_view, list : * const i32)
    // Previous return: * mut mjCBase
    todo!("re-translate: params renamed")
}

/// C: mjCModel::DeleteMaterial (user/user_model.cc:1778)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_delete_material(self_ptr: *mut mjCModel, list: *mut i32, name: string_view) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, list : * mut i32, name : string_view)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: DeleteAllTextures (user/user_model.cc:1790)
#[allow(unused_variables, non_snake_case)]
pub fn delete_all_textures(list: *mut i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (list : * mut i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: DeleteTexcoord (user/user_model.cc:1799)
#[allow(unused_variables, non_snake_case)]
pub fn delete_texcoord(list: *mut i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (list : * mut i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: DeleteElements (user/user_model.cc:1810)
#[allow(unused_variables, non_snake_case)]
pub fn delete_elements(elements: *mut i32, discard: *const i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (elements : * mut i32, discard : * const i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::Delete (user/user_model.cc:1848)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_delete(self_ptr: *mut mjCModel, elements: *mut i32, discard: *const i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, elements : * mut i32, discard : * const i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: getpathslength (user/user_model.cc:2128)
#[allow(unused_variables, non_snake_case)]
pub fn getpathslength(list: i32) -> usize {
    // NOTE: signature changed from previous IR version
    // Previous params: (list : i32)
    // Previous return: usize
    todo!("re-translate: params renamed")
}

/// C: LRfunc (user/user_model.cc:2459)
/// Calls: mj_setLengthRange
#[allow(unused_variables, non_snake_case)]
pub fn l_rfunc(arg: *mut ()) -> *mut () {
    // NOTE: signature changed from previous IR version
    // Previous params: (arg : * mut ())
    // Previous return: * mut ()
    todo!("re-translate: params renamed")
}

/// C: addtolist (user/user_model.cc:2577)
#[allow(unused_variables, non_snake_case)]
pub fn addtolist(input: *const std__string, adr: i32, output_adr_field: *mut i32, output_buffer: *mut i8) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (input : * const std__string, adr : i32, output_adr_field : * mut i32, output_buffer : * mut i8)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: namelist (user/user_model.cc:2593)
#[allow(unused_variables, non_snake_case)]
pub fn namelist(list: *mut i32, adr: i32, name_adr: *mut i32, names: *mut i8, map: *mut i32) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (list : * mut i32, adr : i32, name_adr : * mut i32, names : * mut i8, map : * mut i32)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: pathlist (user/user_model.cc:2702)
#[allow(unused_variables, non_snake_case)]
pub fn pathlist(list: *mut i32, adr: i32, path_adr: *mut i32, paths: *mut i8) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (list : * mut i32, adr : i32, path_adr : * mut i32, paths : * mut i8)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: makelistid (user/user_model.cc:4311)
#[allow(unused_variables, non_snake_case)]
pub fn makelistid(dest: *mut i32, source: *mut i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (dest : * mut i32, source : * mut i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: changeframe (user/user_model.cc:4319)
/// Calls: mjuu_frameaccum
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn changeframe(childpos: *mut f64, childquat: *mut f64, bodypos: *const f64, bodyquat: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (childpos : * mut f64, childquat : * mut f64, bodypos : * const f64, bodyquat : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::ReassignChild (user/user_model.cc:4355)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_reassign_child(self_ptr: *mut mjCModel, dest: *mut i32, list: *mut i32, parent: *mut mjCBody, body: *mut mjCBody) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, dest : * mut i32, list : * mut i32, parent : * mut mjCBody, body : * mut mjCBody)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::ResolveReferences (user/user_model.cc:4371)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_resolve_references(self_ptr: *mut mjCModel, list: *mut i32, body: *mut mjCBody) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, list : * mut i32, body : * mut mjCBody)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: comparePair (user/user_model.cc:4565)
/// Calls: mjCPair::GetSignature
#[allow(unused_variables, non_snake_case)]
pub fn compare_pair(el1: *mut mjCPair, el2: *mut mjCPair) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (el1 : * mut mjCPair, el2 : * mut mjCPair)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: compareBodyPair (user/user_model.cc:4568)
/// Calls: mjCBodyPair::GetSignature
#[allow(unused_variables, non_snake_case)]
pub fn compare_body_pair(el1: *mut mjCBodyPair, el2: *mut mjCBodyPair) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (el1 : * mut mjCBodyPair, el2 : * mut mjCBodyPair)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: reassignid (user/user_model.cc:4575)
#[allow(unused_variables, non_snake_case)]
pub fn reassignid(list: *mut i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (list : * mut i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::ProcessList_ (user/user_model.cc:4600)
/// Calls: mjCModel::CheckRepeat, mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_process_list(self_ptr: *mut mjCModel, ids: *mut mjListKeyMap, list: *mut i32, r#type: u32, checkrepeat: bool) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, ids : * mut mjListKeyMap, list : * mut i32, r#type : u32, checkrepeat : bool)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: compilerLogHandler (user/user_model.cc:4665)
#[allow(unused_variables, non_snake_case)]
pub fn compiler_log_handler(msg: *const mjLogMessage) {
    // NOTE: signature changed from previous IR version
    // Previous params: (msg : * const mjLogMessage)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: CompileMesh (user/user_model.cc:4770)
/// Calls: _mjPRIVATE_setTlsLogHandler, mjCMesh::Compile
#[allow(unused_variables, non_snake_case)]
pub fn compile_mesh(mesh: *mut mjCMesh, vfs: *const mjVFS, exception: *mut std__exception_ptr, exception_mutex: *mut std__mutex, warningtext: *mut string) {
    // NOTE: signature changed from previous IR version
    // Previous params: (mesh : * mut mjCMesh, vfs : * const mjVFS, exception : * mut std__exception_ptr, exception_mutex : * mut std__mutex, warningtext : * mut string)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: CompileTexture (user/user_model.cc:4790)
/// Calls: _mjPRIVATE_setTlsLogHandler, mjCTexture::Compile
#[allow(unused_variables, non_snake_case)]
pub fn compile_texture(texture: *mut mjCTexture, vfs: *const mjVFS, exception: *mut std__exception_ptr, exception_mutex: *mut std__mutex, warningtext: *mut string) {
    // NOTE: signature changed from previous IR version
    // Previous params: (texture : * mut mjCTexture, vfs : * const mjVFS, exception : * mut std__exception_ptr, exception_mutex : * mut std__mutex, warningtext : * mut string)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: PrintIndent (user/user_model.cc:5457)
#[allow(unused_variables, non_snake_case)]
pub fn print_indent(ss: *mut std__stringstream, depth: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (ss : * mut std__stringstream, depth : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::CopyFromSpec (user/user_model.h:191)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_from_spec(self_ptr: *mut mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::PointToLocal (user/user_model.h:192)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_point_to_local(self_ptr: *mut mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::Compile (user/user_model.h:203)
/// Calls: _mjPRIVATE_setTlsLogHandler, mjCModel::Clear, mjCModel::ClearCompileWarnings, mjCModel::CopyFromSpec, mjCModel::TryCompile, mj_deleteData, mj_deleteModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_compile(self_ptr: *mut mjCModel, vfs: *const mjVFS, m: *mut *mut mjModel) -> *mut mjModel {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, vfs : * const mjVFS, m : * mut * mut mjModel)
    // Previous return: * mut mjModel
    todo!("re-translate: params renamed")
}

/// C: mjCModel::CopyBack (user/user_model.h:204)
/// Calls: mjCMesh::GetPosPtr, mjCMesh::GetQuatPtr
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_back(self_ptr: *mut mjCModel, arg0: *const mjModel) -> bool {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, arg0 : * const mjModel)
    // Previous return: bool
    todo!("re-translate: params renamed")
}

/// C: mjCModel::FuseStatic (user/user_model.h:205)
/// Calls: mjCBody::ComputeBVH, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_fuse_static(self_ptr: *mut mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::FuseReindex (user/user_model.h:206)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_fuse_reindex(self_ptr: *mut mjCModel, body: *mut mjCBody) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, body : * mut mjCBody)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::AddFlex (user/user_model.h:209)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_flex(self_ptr: *mut mjCModel) -> *mut mjCFlex {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * mut mjCFlex
    todo!("re-translate: params renamed")
}

/// C: mjCModel::AddMesh (user/user_model.h:210)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_mesh(self_ptr: *mut mjCModel, def: *mut mjCDef) -> *mut mjCMesh {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, def : * mut mjCDef)
    // Previous return: * mut mjCMesh
    todo!("re-translate: params renamed")
}

/// C: mjCModel::AddSkin (user/user_model.h:211)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_skin(self_ptr: *mut mjCModel) -> *mut mjCSkin {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * mut mjCSkin
    todo!("re-translate: params renamed")
}

/// C: mjCModel::AddHField (user/user_model.h:212)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_h_field(self_ptr: *mut mjCModel) -> *mut mjCHField {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * mut mjCHField
    todo!("re-translate: params renamed")
}

/// C: mjCModel::AddTexture (user/user_model.h:213)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_texture(self_ptr: *mut mjCModel) -> *mut mjCTexture {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * mut mjCTexture
    todo!("re-translate: params renamed")
}

/// C: mjCModel::AddMaterial (user/user_model.h:214)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_material(self_ptr: *mut mjCModel, def: *mut mjCDef) -> *mut mjCMaterial {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, def : * mut mjCDef)
    // Previous return: * mut mjCMaterial
    todo!("re-translate: params renamed")
}

/// C: mjCModel::AddPair (user/user_model.h:215)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_pair(self_ptr: *mut mjCModel, def: *mut mjCDef) -> *mut mjCPair {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, def : * mut mjCDef)
    // Previous return: * mut mjCPair
    todo!("re-translate: params renamed")
}

/// C: mjCModel::AddExclude (user/user_model.h:216)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_exclude(self_ptr: *mut mjCModel) -> *mut mjCBodyPair {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * mut mjCBodyPair
    todo!("re-translate: params renamed")
}

/// C: mjCModel::AddEquality (user/user_model.h:217)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_equality(self_ptr: *mut mjCModel, def: *mut mjCDef) -> *mut mjCEquality {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, def : * mut mjCDef)
    // Previous return: * mut mjCEquality
    todo!("re-translate: params renamed")
}

/// C: mjCModel::AddTendon (user/user_model.h:218)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_tendon(self_ptr: *mut mjCModel, def: *mut mjCDef) -> *mut mjCTendon {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, def : * mut mjCDef)
    // Previous return: * mut mjCTendon
    todo!("re-translate: params renamed")
}

/// C: mjCModel::AddActuator (user/user_model.h:219)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_actuator(self_ptr: *mut mjCModel, def: *mut mjCDef) -> *mut mjCActuator {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, def : * mut mjCDef)
    // Previous return: * mut mjCActuator
    todo!("re-translate: params renamed")
}

/// C: mjCModel::AddSensor (user/user_model.h:220)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_sensor(self_ptr: *mut mjCModel) -> *mut mjCSensor {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * mut mjCSensor
    todo!("re-translate: params renamed")
}

/// C: mjCModel::AddNumeric (user/user_model.h:221)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_numeric(self_ptr: *mut mjCModel) -> *mut mjCNumeric {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * mut mjCNumeric
    todo!("re-translate: params renamed")
}

/// C: mjCModel::AddText (user/user_model.h:222)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_text(self_ptr: *mut mjCModel) -> *mut mjCText {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * mut mjCText
    todo!("re-translate: params renamed")
}

/// C: mjCModel::AddTuple (user/user_model.h:223)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_tuple(self_ptr: *mut mjCModel) -> *mut mjCTuple {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * mut mjCTuple
    todo!("re-translate: params renamed")
}

/// C: mjCModel::AddKey (user/user_model.h:224)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_key(self_ptr: *mut mjCModel) -> *mut mjCKey {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * mut mjCKey
    todo!("re-translate: params renamed")
}

/// C: mjCModel::AddPlugin (user/user_model.h:225)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_plugin(self_ptr: *mut mjCModel) -> *mut mjCPlugin {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * mut mjCPlugin
    todo!("re-translate: params renamed")
}

/// C: mjCModel::AppendSpec (user/user_model.h:228)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_append_spec(self_ptr: *mut mjCModel, spec: *mut mjSpec, compiler: *const mjsCompiler) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, spec : * mut mjSpec, compiler : * const mjsCompiler)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::NumObjects (user/user_model.h:244)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_num_objects(self_ptr: *mut mjCModel, r#type: u32) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, r#type : u32)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjCModel::GetObject (user/user_model.h:245)
/// Calls: mjCModel::NumObjects
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_object(self_ptr: *mut mjCModel, r#type: u32, id: i32) -> *mut mjCBase {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, r#type : u32, id : i32)
    // Previous return: * mut mjCBase
    todo!("re-translate: params renamed")
}

/// C: mjCModel::NextObject (user/user_model.h:246)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_next_object(self_ptr: *mut mjCModel, object: *const mjsElement, r#type: u32) -> *mut mjsElement {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, object : * const mjsElement, r#type : u32)
    // Previous return: * mut mjsElement
    todo!("re-translate: params renamed")
}

/// C: mjCModel::IsCompiled (user/user_model.h:249)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_is_compiled(self_ptr: *mut mjCModel) -> bool {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: bool
    todo!("re-translate: params renamed")
}

/// C: mjCModel::GetError (user/user_model.h:250)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_error(self_ptr: *mut mjCModel) -> *const mjCError {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * const mjCError
    todo!("re-translate: params renamed")
}

/// C: mjCModel::SetError (user/user_model.h:251)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_set_error(self_ptr: *mut mjCModel, error: *const mjCError) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, error : * const mjCError)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::AddWarning (user/user_model.h:252)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_warning(self_ptr: *mut mjCModel, msg: string, obj: *const mjCBase) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, msg : string, obj : * const mjCBase)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::AddGroupedWarning (user/user_model.h:254)
/// Calls: mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_grouped_warning(self_ptr: *mut mjCModel, subject: *const std__string, body: *const std__string) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, subject : * const std__string, body : * const std__string)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::GetWarnings (user/user_model.h:256)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_warnings(self_ptr: *mut mjCModel) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCModel::ClearWarnings (user/user_model.h:260)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_clear_warnings(self_ptr: *mut mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::ClearCompileWarnings (user/user_model.h:264)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_clear_compile_warnings(self_ptr: *mut mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::SetAttachWarningBoundary (user/user_model.h:267)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_set_attach_warning_boundary(self_ptr: *mut mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::GetWorld (user/user_model.h:271)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_world(self_ptr: *mut mjCModel) -> *mut mjCBody {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * mut mjCBody
    todo!("re-translate: params renamed")
}

/// C: mjCModel::FindDefault (user/user_model.h:272)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_find_default(self_ptr: *mut mjCModel, name: *const std__string) -> *mut mjCDef {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, name : * const std__string)
    // Previous return: * mut mjCDef
    todo!("re-translate: params renamed")
}

/// C: mjCModel::AddDefault (user/user_model.h:273)
/// Calls: mjCDef::CopyFromSpec, mjCDef::CopyWithoutChildren
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_default(self_ptr: *mut mjCModel, name: string, parent: *mut mjCDef) -> *mut mjCDef {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, name : string, parent : * mut mjCDef)
    // Previous return: * mut mjCDef
    todo!("re-translate: params renamed")
}

/// C: mjCModel::FindObject (user/user_model.h:274)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_find_object(self_ptr: *mut mjCModel, r#type: u32, name: string) -> *mut mjCBase {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, r#type : u32, name : string)
    // Previous return: * mut mjCBase
    todo!("re-translate: params renamed")
}

/// C: mjCModel::FindTree (user/user_model.h:275)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_find_tree(self_ptr: *mut mjCModel, body: *mut mjCBody, r#type: u32, name: string) -> *mut mjCBase {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, body : * mut mjCBody, r#type : u32, name : string)
    // Previous return: * mut mjCBase
    todo!("re-translate: params renamed")
}

/// C: mjCModel::FindSpec (user/user_model.h:276)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_find_spec(self_ptr: *mut mjCModel, name: string) -> *mut mjSpec {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, name : string)
    // Previous return: * mut mjSpec
    todo!("re-translate: params renamed")
}

/// C: mjCModel::ActivatePlugin (user/user_model.h:278)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_activate_plugin(self_ptr: *mut mjCModel, plugin: *const mjpPlugin, slot: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, plugin : * const mjpPlugin, slot : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::get_meshdir (user/user_model.h:285)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_meshdir(self_ptr: *mut mjCModel) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjCModel::get_texturedir (user/user_model.h:286)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_texturedir(self_ptr: *mut mjCModel) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjCModel::Default (user/user_model.h:288)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_default(self_ptr: *mut mjCModel) -> *mut mjCDef {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * mut mjCDef
    todo!("re-translate: params renamed")
}

/// C: mjCModel::NumDefaults (user/user_model.h:289)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_num_defaults(self_ptr: *mut mjCModel) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjCModel::ActivePlugins (user/user_model.h:291)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_active_plugins(self_ptr: *mut mjCModel) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCModel::Flexes (user/user_model.h:295)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_flexes(self_ptr: *mut mjCModel) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCModel::Meshes (user/user_model.h:296)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_meshes(self_ptr: *mut mjCModel) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCModel::Skins (user/user_model.h:297)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_skins(self_ptr: *mut mjCModel) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCModel::HFields (user/user_model.h:298)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_h_fields(self_ptr: *mut mjCModel) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCModel::Textures (user/user_model.h:299)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_textures(self_ptr: *mut mjCModel) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCModel::Materials (user/user_model.h:300)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_materials(self_ptr: *mut mjCModel) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCModel::Pairs (user/user_model.h:301)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_pairs(self_ptr: *mut mjCModel) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCModel::Excludes (user/user_model.h:302)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_excludes(self_ptr: *mut mjCModel) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCModel::Equalities (user/user_model.h:303)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_equalities(self_ptr: *mut mjCModel) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCModel::Tendons (user/user_model.h:304)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_tendons(self_ptr: *mut mjCModel) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCModel::Actuators (user/user_model.h:305)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_actuators(self_ptr: *mut mjCModel) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCModel::Sensors (user/user_model.h:306)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_sensors(self_ptr: *mut mjCModel) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCModel::Numerics (user/user_model.h:307)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_numerics(self_ptr: *mut mjCModel) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCModel::Texts (user/user_model.h:308)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_texts(self_ptr: *mut mjCModel) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCModel::Tuples (user/user_model.h:309)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_tuples(self_ptr: *mut mjCModel) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCModel::Keys (user/user_model.h:310)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_keys(self_ptr: *mut mjCModel) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCModel::Plugins (user/user_model.h:311)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_plugins(self_ptr: *mut mjCModel) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCModel::Bodies (user/user_model.h:312)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_bodies(self_ptr: *mut mjCModel) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCModel::Geoms (user/user_model.h:313)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_geoms(self_ptr: *mut mjCModel) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCModel::ResolvePlugin (user/user_model.h:316)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_resolve_plugin(self_ptr: *mut mjCModel, obj: *mut mjCBase, plugin_name: *const std__string, plugin_instance_name: *const std__string, plugin_instance: *mut *mut mjCPlugin) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, obj : * mut mjCBase, plugin_name : * const std__string, plugin_instance_name : * const std__string, plugin_instance : * mut * mut mjCPlugin)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::Clear (user/user_model.h:321)
/// Calls: mjCModel::ClearCompileWarnings
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_clear(self_ptr: *mut mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::SaveState (user/user_model.h:329)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_save_state(self_ptr: *mut mjCModel, state_name: *const std__string, qpos: *const T, qvel: *const T, act: *const T, ctrl: *const T, mpos: *const T, mquat: *const T) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, state_name : * const std__string, qpos : * const T, qvel : * const T, act : * const T, ctrl : * const T, mpos : * const T, mquat : * const T)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::RestoreState (user/user_model.h:334)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_restore_state(self_ptr: *mut mjCModel, state_name: *const std__string, pos0: *const f64, mpos0: *const f64, mquat0: *const f64, qpos: *mut T, qvel: *mut T, act: *mut T, ctrl: *mut T, mpos: *mut T, mquat: *mut T) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, state_name : * const std__string, pos0 : * const f64, mpos0 : * const f64, mquat0 : * const f64, qpos : * mut T, qvel : * mut T, act : * mut T, ctrl : * mut T, mpos : * mut T, mquat : * mut T)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::MakeData (user/user_model.h:338)
/// Calls: mj_initPlugin, mj_makeRawData, mj_resetData
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_make_data(self_ptr: *mut mjCModel, m: *const mjModel, dest: *mut *mut mjData) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, m : * const mjModel, dest : * mut * mut mjData)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::StoreKeyframes (user/user_model.h:341)
/// Calls: mjCModel::ComputeReference, mjCModel::SaveDofOffsets
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_store_keyframes(self_ptr: *mut mjCModel, dest: *mut mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, dest : * mut mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::SetDeepCopy (user/user_model.h:347)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_set_deep_copy(self_ptr: *mut mjCModel, deepcopy: bool) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, deepcopy : bool)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::SetAttached (user/user_model.h:350)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_set_attached(self_ptr: *mut mjCModel, deepcopy: bool) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, deepcopy : bool)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::IsAttached (user/user_model.h:353)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_is_attached(self_ptr: *mut mjCModel) -> bool {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: bool
    todo!("re-translate: params renamed")
}

/// C: mjCModel::CheckRepeat (user/user_model.h:356)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_check_repeat(self_ptr: *mut mjCModel, r#type: u32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, r#type : u32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::AddRef (user/user_model.h:359)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_ref(self_ptr: *mut mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::GetRef (user/user_model.h:360)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_ref(self_ptr: *mut mjCModel) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjCModel::Release (user/user_model.h:361)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_release(self_ptr: *mut mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::MakeTreeLists (user/user_model.h:377)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_make_tree_lists(self_ptr: *mut mjCModel, body: *mut mjCBody) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, body : * mut mjCBody)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::TryCompile (user/user_model.h:380)
/// Calls: mjCModel::AddKey, mjCModel::AddWarning, mjCModel::AutoSpringDamper, mjCModel::CheckEmptyNames, mjCModel::ClearCompileWarnings, mjCModel::CompileMeshesAndTextures, mjCModel::ComputeSparseSizes, mjCModel::CopyNames, mjCModel::CopyObjects, mjCModel::CopyPaths, mjCModel::CopyPlugins, mjCModel::CopyTree, mjCModel::ExpandAllKeyframes, mjCModel::FinalizeSimple, mjCModel::FuseStatic, mjCModel::IndexAssets, mjCModel::LengthRange, mjCModel::ProcessLists, mjCModel::ResolveKeyframes, mjCModel::SaveDofOffsets, mjCModel::SetNuser, mjCModel::SetSizes, mjCModel::Signature, mj_deleteData, mj_makeData, mj_makeRawData, mj_normalizeQuat, mj_resetData, mj_setConst, mj_step, mj_validateReferences, mjuu_defined
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_try_compile(self_ptr: *mut mjCModel, m: *mut *mut mjModel, d: *mut *mut mjData, vfs: *const mjVFS) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, m : * mut * mut mjModel, d : * mut * mut mjData, vfs : * const mjVFS)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::CompileMeshesAndTextures (user/user_model.h:381)
/// Calls: NumCompilerThreads, ThreadPool::WaitCount
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_compile_meshes_and_textures(self_ptr: *mut mjCModel, vfs: *const mjVFS) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, vfs : * const mjVFS)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::SetNuser (user/user_model.h:383)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_set_nuser(self_ptr: *mut mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::IndexAssets (user/user_model.h:384)
/// Calls: mjCGeom::IsVisual, mjCGeom::get_hfieldname, mjCGeom::get_material, mjCGeom::get_meshname, mjCMesh::IsVisual, mjCMesh::SetNotVisual, mjCModel::FindObject, mjCSite::get_material
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_index_assets(self_ptr: *mut mjCModel, discard: bool) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, discard : bool)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::CheckEmptyNames (user/user_model.h:385)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_check_empty_names(self_ptr: *mut mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::SetSizes (user/user_model.h:386)
/// Calls: mjCBody::GetParent
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_set_sizes(self_ptr: *mut mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::ComputeSparseSizes (user/user_model.h:387)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_compute_sparse_sizes(self_ptr: *mut mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::AutoSpringDamper (user/user_model.h:388)
/// Calls: mjCJoint::nv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_auto_spring_damper(self_ptr: *mut mjCModel, arg0: *mut mjModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, arg0 : * mut mjModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::LengthRange (user/user_model.h:389)
/// Calls: NumCompilerThreads, ThreadPool::WaitCount, mj_makeData
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_length_range(self_ptr: *mut mjCModel, arg0: *mut mjModel, arg1: *mut mjData) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, arg0 : * mut mjModel, arg1 : * mut mjData)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::CopyNames (user/user_model.h:390)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_names(self_ptr: *mut mjCModel, arg0: *mut mjModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, arg0 : * mut mjModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::CopyPaths (user/user_model.h:391)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_paths(self_ptr: *mut mjCModel, arg0: *mut mjModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, arg0 : * mut mjModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::CopyObjects (user/user_model.h:392)
/// Calls: mjCActuator::is_actlimited, mjCActuator::is_ctrllimited, mjCActuator::is_forcelimited, mjCBoundingVolumeHierarchy::Nbvh, mjCBoundingVolumeHierarchy::Nodeid, mjCMesh::CopyFace, mjCMesh::CopyFaceNormal, mjCMesh::CopyFaceTexcoord, mjCMesh::CopyGraph, mjCMesh::CopyNormal, mjCMesh::CopyPolygonMap, mjCMesh::CopyPolygonNormals, mjCMesh::CopyPolygons, mjCMesh::CopyTexcoord, mjCMesh::CopyVert, mjCMesh::GetPosPtr, mjCMesh::GetQuatPtr, mjCMesh::HasTexcoord, mjCMesh::Scale, mjCMesh::nface, mjCMesh::nnormal, mjCMesh::npolygon, mjCMesh::npolygonmap, mjCMesh::npolygonvert, mjCMesh::ntexcoord, mjCMesh::nvert, mjCMesh::octree, mjCMesh::szgraph, mjCMesh::tree, mjCModel::AddWarning, mjCOctree::CopyAabb, mjCOctree::CopyChild, mjCOctree::CopyCoeff, mjCOctree::CopyLevel, mjCOctree::NumNodes, mjCTendon::is_actfrclimited, mjCTendon::is_limited
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_objects(self_ptr: *mut mjCModel, arg0: *mut mjModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, arg0 : * mut mjModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::CopyTree (user/user_model.h:393)
/// Calls: mjCGeom::GetRBound, mjCJoint::is_actfrclimited, mjCJoint::is_limited, mjCJoint::nq, mjCJoint::nv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_tree(self_ptr: *mut mjCModel, arg0: *mut mjModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, arg0 : * mut mjModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::FinalizeSimple (user/user_model.h:394)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_finalize_simple(self_ptr: *mut mjCModel, m: *mut mjModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, m : * mut mjModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::CopyPlugins (user/user_model.h:395)
/// Calls: mjp_getPluginAtSlot, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_plugins(self_ptr: *mut mjCModel, arg0: *mut mjModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, arg0 : * mut mjModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::CountTendonDofs (user/user_model.h:396)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_count_tendon_dofs(self_ptr: *mut mjCModel, m: *const mjModel, id: i32) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, m : * const mjModel, id : i32)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjCModel::CountNJmom (user/user_model.h:398)
/// Calls: mjCModel::CountTendonDofs
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_count_n_jmom(self_ptr: *mut mjCModel, m: *const mjModel) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, m : * const mjModel)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjCModel::CountNJten (user/user_model.h:399)
/// Calls: mjCModel::CountTendonDofs
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_count_n_jten(self_ptr: *mut mjCModel, m: *const mjModel) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, m : * const mjModel)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjCModel::RemovePlugins (user/user_model.h:402)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_remove_plugins(self_ptr: *mut mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::CopyExplicitPlugin (user/user_model.h:448)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_explicit_plugin(self_ptr: *mut mjCModel, obj: *mut T) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, obj : * mut T)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::CreateObjectLists (user/user_model.h:458)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_create_object_lists(self_ptr: *mut mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::ProcessLists (user/user_model.h:461)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_process_lists(self_ptr: *mut mjCModel, checkrepeat: bool) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, checkrepeat : bool)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::ResetTreeLists (user/user_model.h:468)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_reset_tree_lists(self_ptr: *mut mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::SaveDofOffsets (user/user_model.h:471)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_save_dof_offsets(self_ptr: *mut mjCModel, computesize: bool) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, computesize : bool)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::ResolveKeyframes (user/user_model.h:474)
/// Calls: mjCModel::SaveDofOffsets
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_resolve_keyframes(self_ptr: *mut mjCModel, m: *const mjModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, m : * const mjModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::ExpandKeyframe (user/user_model.h:477)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_expand_keyframe(self_ptr: *mut mjCModel, key: *mut mjCKey, qpos0_: *const f64, bpos: *const f64, bquat: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, key : * mut mjCKey, qpos0_ : * const f64, bpos : * const f64, bquat : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::ComputeReference (user/user_model.h:480)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_compute_reference(self_ptr: *mut mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::CheckBodyMassInertia (user/user_model.h:483)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_check_body_mass_inertia(self_ptr: *mut mjCModel, body: *mut mjCBody) -> bool {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, body : * mut mjCBody)
    // Previous return: bool
    todo!("re-translate: params renamed")
}

/// C: mjCModel::PrintTree (user/user_model.h:491)
/// Calls: PrintIndent
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_print_tree(self_ptr: *mut mjCModel, tree: *mut std__stringstream, body: *const mjCBody, depth: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, tree : * mut std__stringstream, body : * const mjCBody, depth : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::Signature (user/user_model.h:494)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_signature(self_ptr: *mut mjCModel) -> u64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: u64
    todo!("re-translate: params renamed")
}

/// C: mjCModel::DeleteSubtreePlugin (user/user_model.h:505)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_delete_subtree_plugin(self_ptr: *mut mjCModel, subtree: *mut mjCBody) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel, subtree : * mut mjCBody)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCModel::ExpandAllKeyframes (user/user_model.h:508)
/// Calls: mjCModel::ComputeReference, mjCModel::SaveDofOffsets
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_expand_all_keyframes(self_ptr: *mut mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

