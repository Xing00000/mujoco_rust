//! Port of: user/user_model.cc
//! IR hash: 699b5f0da57e8d78
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: IsSameVec (user/user_model.cc:72)
#[allow(unused_variables, non_snake_case)]
pub fn is_same_vec(pos1: *const T, pos2: *const T) -> bool {
    // WARNING: signature changed — verify body
    // Previous params: (pos1 : * const T, pos2 : * const T)
    // Previous return: bool
    todo ! ()
}

/// C: NumCompilerThreads (user/user_model.cc:79)
#[allow(unused_variables, non_snake_case)]
pub fn num_compiler_threads(upper_bound: i32) -> u32 {
    // WARNING: signature changed — verify body
    // Previous params: (upper_bound : i32)
    // Previous return: u32
    todo ! ()
}

/// C: IsSameQuat (user/user_model.cc:93)
#[allow(unused_variables, non_snake_case)]
pub fn is_same_quat(quat1: *const T, quat2: *const T) -> bool {
    // WARNING: signature changed — verify body
    // Previous params: (quat1 : * const T, quat2 : * const T)
    // Previous return: bool
    todo ! ()
}

/// C: IsSamePose (user/user_model.cc:111)
#[allow(unused_variables, non_snake_case)]
pub fn is_same_pose(pos1: *const T, pos2: *const T, quat1: *const T, quat2: *const T) -> bool {
    // WARNING: signature changed — verify body
    // Previous params: (pos1 : * const T, pos2 : * const T, quat1 : * const T, quat2 : * const T)
    // Previous return: bool
    todo ! ()
}

/// C: IsNullPose (user/user_model.cc:127)
#[allow(unused_variables, non_snake_case)]
pub fn is_null_pose(pos: *const T, quat: *const T) -> bool {
    // WARNING: signature changed — verify body
    // Previous params: (pos : * const T, quat : * const T)
    // Previous return: bool
    todo ! ()
}

/// C: GetBodyIdFromWrap (user/user_model.cc:134)
/// Calls: mjCWrap::Type
#[allow(unused_variables, non_snake_case)]
pub fn get_body_id_from_wrap(wrap: *const mjCWrap) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (wrap : * const mjCWrap)
    // Previous return: i32
    todo ! ()
}

/// C: mjCModel::CopyList (user/user_model.cc:261)
/// Calls: mjCModel::FindSpec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_list(dest: *mut i32, source: *const i32) {
    // WARNING: signature changed — verify body
    // Previous params: (dest : * mut i32, source : * const i32)
    // Previous return: ()
    todo ! ()
}

/// C: resetlist (user/user_model.cc:304)
#[allow(unused_variables, non_snake_case)]
pub fn resetlist(list: *mut i32) {
    // WARNING: signature changed — verify body
    // Previous params: (list : * mut i32)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::CopyPlugin (user/user_model.cc:396)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_plugin(source: *const i32, list: *const i32) {
    // WARNING: signature changed — verify body
    // Previous params: (source : * const i32, list : * const i32)
    // Previous return: ()
    todo ! ()
}

/// C: IsPluginActive (user/user_model.cc:440)
#[allow(unused_variables, non_snake_case)]
pub fn is_plugin_active(plugin: *const mjpPlugin, active_plugins: *const i32) -> bool {
    // WARNING: signature changed — verify body
    // Previous params: (plugin : * const mjpPlugin, active_plugins : * const i32)
    // Previous return: bool
    todo ! ()
}

/// C: mjCModel::RemoveFromList (user/user_model.cc:508)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_remove_from_list(list: *mut i32, other: *const mjCModel) {
    // WARNING: signature changed — verify body
    // Previous params: (list : * mut i32, other : * const mjCModel)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::DeleteAll (user/user_model.cc:545)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_delete_all(elements: *mut i32) {
    // WARNING: signature changed — verify body
    // Previous params: (elements : * mut i32)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::MarkPluginInstance (user/user_model.cc:555)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_mark_plugin_instance(instances: *mut i32, list: *const i32) {
    // WARNING: signature changed — verify body
    // Previous params: (instances : * mut i32, list : * const i32)
    // Previous return: ()
    todo ! ()
}

/// C: deletefromlist (user/user_model.cc:715)
#[allow(unused_variables, non_snake_case)]
pub fn deletefromlist(list: *mut i32, element: *mut mjsElement) {
    // WARNING: signature changed — verify body
    // Previous params: (list : * mut i32, element : * mut mjsElement)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::AddObject (user/user_model.cc:1252)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_object(list: *mut i32, r#type: i32) -> *mut T {
    // WARNING: signature changed — verify body
    // Previous params: (list : * mut i32, r#type : i32)
    // Previous return: * mut T
    todo ! ()
}

/// C: mjCModel::AddObjectDefault (user/user_model.cc:1263)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_object_default(list: *mut i32, r#type: i32, def: *mut mjCDef) -> *mut T {
    // WARNING: signature changed — verify body
    // Previous params: (list : * mut i32, r#type : i32, def : * mut mjCDef)
    // Previous return: * mut T
    todo ! ()
}

/// C: GetNext (user/user_model.cc:1411)
#[allow(unused_variables, non_snake_case)]
pub fn get_next(list: *const i32, child: *const mjsElement) -> *mut mjsElement {
    // WARNING: signature changed — verify body
    // Previous params: (list : * const i32, child : * const mjsElement)
    // Previous return: * mut mjsElement
    todo ! ()
}

/// C: mjCModel::AddWarning (user/user_model.cc:1513)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_warning(msg: i32, obj: *const mjCBase) {
    // WARNING: signature changed — verify body
    // Previous params: (msg : i32, obj : * const mjCBase)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::AddGroupedWarning (user/user_model.cc:1530)
/// Calls: mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_grouped_warning(subject: *const i32, body: *const i32) {
    // WARNING: signature changed — verify body
    // Previous params: (subject : * const i32, body : * const i32)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::FindDefault (user/user_model.cc:1552)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_find_default(name: *const i32) -> *mut mjCDef {
    // WARNING: signature changed — verify body
    // Previous params: (name : * const i32)
    // Previous return: * mut mjCDef
    todo ! ()
}

/// C: mjCModel::AddDefault (user/user_model.cc:1564)
/// Calls: mjCDef::CopyFromSpec, mjCDef::CopyWithoutChildren
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_default(name: i32, parent: *mut mjCDef) -> *mut mjCDef {
    // WARNING: signature changed — verify body
    // Previous params: (name : i32, parent : * mut mjCDef)
    // Previous return: * mut mjCDef
    todo ! ()
}

/// C: findobject (user/user_model.cc:1596)
#[allow(unused_variables, non_snake_case)]
pub fn findobject(name: std__string_view, list: *const i32, ids: *const i32) -> *mut T {
    // WARNING: signature changed — verify body
    // Previous params: (name : std__string_view, list : * const i32, ids : * const i32)
    // Previous return: * mut T
    todo ! ()
}

/// C: mjCModel::FindAsset (user/user_model.cc:1621)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_find_asset(name: std__string_view, list: *const i32) -> *mut mjCBase {
    // WARNING: signature changed — verify body
    // Previous params: (name : std__string_view, list : * const i32)
    // Previous return: * mut mjCBase
    todo ! ()
}

/// C: mjCModel::FindObject (user/user_model.cc:1642)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_find_object(r#type: mjtObj, name: i32) -> *mut mjCBase {
    // WARNING: signature changed — verify body
    // Previous params: (r#type : mjtObj, name : i32)
    // Previous return: * mut mjCBase
    todo ! ()
}

/// C: mjCModel::FindTree (user/user_model.cc:1652)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_find_tree(body: *mut mjCBody, r#type: mjtObj, name: i32) -> *mut mjCBase {
    // WARNING: signature changed — verify body
    // Previous params: (body : * mut mjCBody, r#type : mjtObj, name : i32)
    // Previous return: * mut mjCBase
    todo ! ()
}

/// C: mjCModel::DeleteMaterial (user/user_model.cc:1778)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_delete_material(list: *mut i32, name: std__string_view) {
    // WARNING: signature changed — verify body
    // Previous params: (list : * mut i32, name : std__string_view)
    // Previous return: ()
    todo ! ()
}

/// C: DeleteAllTextures (user/user_model.cc:1790)
#[allow(unused_variables, non_snake_case)]
pub fn delete_all_textures(list: *mut i32) {
    // WARNING: signature changed — verify body
    // Previous params: (list : * mut i32)
    // Previous return: ()
    todo ! ()
}

/// C: DeleteTexcoord (user/user_model.cc:1799)
#[allow(unused_variables, non_snake_case)]
pub fn delete_texcoord(list: *mut i32) {
    // WARNING: signature changed — verify body
    // Previous params: (list : * mut i32)
    // Previous return: ()
    todo ! ()
}

/// C: DeleteElements (user/user_model.cc:1810)
#[allow(unused_variables, non_snake_case)]
pub fn delete_elements(elements: *mut i32, discard: *const i32) {
    // WARNING: signature changed — verify body
    // Previous params: (elements : * mut i32, discard : * const i32)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::Delete (user/user_model.cc:1848)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_delete(elements: *mut i32, discard: *const i32) {
    // WARNING: signature changed — verify body
    // Previous params: (elements : * mut i32, discard : * const i32)
    // Previous return: ()
    todo ! ()
}

/// C: getpathslength (user/user_model.cc:2128)
#[allow(unused_variables, non_snake_case)]
pub fn getpathslength(list: i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (list : i32)
    // Previous return: i32
    todo ! ()
}

/// C: LRfunc (user/user_model.cc:2459)
/// Calls: mj_setLengthRange
#[allow(unused_variables, non_snake_case)]
pub fn l_rfunc(arg: *mut ()) -> *mut () {
    // WARNING: signature changed — verify body
    // Previous params: (arg : * mut ())
    // Previous return: * mut ()
    todo ! ()
}

/// C: addtolist (user/user_model.cc:2577)
#[allow(unused_variables, non_snake_case)]
pub fn addtolist(input: *const i32, adr: i32, output_adr_field: *mut i32, output_buffer: *mut i8) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (input : * const i32, adr : i32, output_adr_field : * mut i32, output_buffer : * mut i8)
    // Previous return: i32
    todo ! ()
}

/// C: namelist (user/user_model.cc:2593)
#[allow(unused_variables, non_snake_case)]
pub fn namelist(list: *mut i32, adr: i32, name_adr: *mut i32, names: *mut i8, map: *mut i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (list : * mut i32, adr : i32, name_adr : * mut i32, names : * mut i8, map : * mut i32)
    // Previous return: i32
    todo ! ()
}

/// C: pathlist (user/user_model.cc:2702)
#[allow(unused_variables, non_snake_case)]
pub fn pathlist(list: *mut i32, adr: i32, path_adr: *mut i32, paths: *mut i8) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (list : * mut i32, adr : i32, path_adr : * mut i32, paths : * mut i8)
    // Previous return: i32
    todo ! ()
}

/// C: mjCModel::SaveState (user/user_model.cc:4131)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_save_state(state_name: *const i32, qpos: *const T, qvel: *const T, act: *const T, ctrl: *const T, mpos: *const T, mquat: *const T) {
    // WARNING: signature changed — verify body
    // Previous params: (state_name : * const i32, qpos : * const T, qvel : * const T, act : * const T, ctrl : * const T, mpos : * const T, mquat : * const T)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::RestoreState (user/user_model.cc:4185)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_restore_state(state_name: *const i32, pos0: *const f64, mpos0: *const f64, mquat0: *const f64, qpos: *mut T, qvel: *mut T, act: *mut T, ctrl: *mut T, mpos: *mut T, mquat: *mut T) {
    // WARNING: signature changed — verify body
    // Previous params: (state_name : * const i32, pos0 : * const f64, mpos0 : * const f64, mquat0 : * const f64, qpos : * mut T, qvel : * mut T, act : * mut T, ctrl : * mut T, mpos : * mut T, mquat : * mut T)
    // Previous return: ()
    todo ! ()
}

/// C: makelistid (user/user_model.cc:4311)
#[allow(unused_variables, non_snake_case)]
pub fn makelistid(dest: *mut i32, source: *mut i32) {
    // WARNING: signature changed — verify body
    // Previous params: (dest : * mut i32, source : * mut i32)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (childpos : * mut f64, childquat : * mut f64, bodypos : * const f64, bodyquat : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::ReassignChild (user/user_model.cc:4355)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_reassign_child(dest: *mut i32, list: *mut i32, parent: *mut mjCBody, body: *mut mjCBody) {
    // WARNING: signature changed — verify body
    // Previous params: (dest : * mut i32, list : * mut i32, parent : * mut mjCBody, body : * mut mjCBody)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::ResolveReferences (user/user_model.cc:4371)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_resolve_references(list: *mut i32, body: *mut mjCBody) {
    // WARNING: signature changed — verify body
    // Previous params: (list : * mut i32, body : * mut mjCBody)
    // Previous return: ()
    todo ! ()
}

/// C: comparePair (user/user_model.cc:4565)
/// Calls: mjCPair::GetSignature
#[allow(unused_variables, non_snake_case)]
pub fn compare_pair(el1: *mut mjCPair, el2: *mut mjCPair) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (el1 : * mut mjCPair, el2 : * mut mjCPair)
    // Previous return: i32
    todo ! ()
}

/// C: compareBodyPair (user/user_model.cc:4568)
/// Calls: mjCBodyPair::GetSignature
#[allow(unused_variables, non_snake_case)]
pub fn compare_body_pair(el1: *mut mjCBodyPair, el2: *mut mjCBodyPair) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (el1 : * mut mjCBodyPair, el2 : * mut mjCBodyPair)
    // Previous return: i32
    todo ! ()
}

/// C: reassignid (user/user_model.cc:4575)
#[allow(unused_variables, non_snake_case)]
pub fn reassignid(list: *mut i32) {
    // WARNING: signature changed — verify body
    // Previous params: (list : * mut i32)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::ProcessList_ (user/user_model.cc:4600)
/// Calls: mjCModel::CheckRepeat, mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_process_list(ids: *mut mjListKeyMap, list: *mut i32, r#type: mjtObj, checkrepeat: bool) {
    // WARNING: signature changed — verify body
    // Previous params: (ids : * mut mjListKeyMap, list : * mut i32, r#type : mjtObj, checkrepeat : bool)
    // Previous return: ()
    todo ! ()
}

/// C: compilerLogHandler (user/user_model.cc:4665)
#[allow(unused_variables, non_snake_case)]
pub fn compiler_log_handler(msg: *const mjLogMessage) {
    // WARNING: signature changed — verify body
    // Previous params: (msg : * const mjLogMessage)
    // Previous return: ()
    todo ! ()
}

/// C: CompileMesh (user/user_model.cc:4770)
/// Calls: _mjPRIVATE_setTlsLogHandler, mjCMesh::Compile
#[allow(unused_variables, non_snake_case)]
pub fn compile_mesh(mesh: *mut mjCMesh, vfs: *const mjVFS, exception: *mut std__exception_ptr, exception_mutex: *mut std__mutex, warningtext: *mut i32) {
    // WARNING: signature changed — verify body
    // Previous params: (mesh : * mut mjCMesh, vfs : * const mjVFS, exception : * mut std__exception_ptr, exception_mutex : * mut std__mutex, warningtext : * mut i32)
    // Previous return: ()
    todo ! ()
}

/// C: CompileTexture (user/user_model.cc:4790)
/// Calls: _mjPRIVATE_setTlsLogHandler, mjCTexture::Compile
#[allow(unused_variables, non_snake_case)]
pub fn compile_texture(texture: *mut mjCTexture, vfs: *const mjVFS, exception: *mut std__exception_ptr, exception_mutex: *mut std__mutex, warningtext: *mut i32) {
    // WARNING: signature changed — verify body
    // Previous params: (texture : * mut mjCTexture, vfs : * const mjVFS, exception : * mut std__exception_ptr, exception_mutex : * mut std__mutex, warningtext : * mut i32)
    // Previous return: ()
    todo ! ()
}

/// C: PrintIndent (user/user_model.cc:5457)
#[allow(unused_variables, non_snake_case)]
pub fn print_indent(ss: *mut i32, depth: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (ss : * mut i32, depth : i32)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::PrintTree (user/user_model.cc:5470)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_print_tree(tree: *mut i32, body: *const mjCBody, depth: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (tree : * mut i32, body : * const mjCBody, depth : i32)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::Signature (user/user_model.cc:5505)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_signature() -> i32 {
    todo ! ()
}

/// C: mjCModel::ResolvePlugin (user/user_model.cc:5915)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_resolve_plugin(obj: *mut mjCBase, plugin_name: *const i32, plugin_instance_name: *const i32, plugin_instance: *mut *mut mjCPlugin) {
    // WARNING: signature changed — verify body
    // Previous params: (obj : * mut mjCBase, plugin_name : * const i32, plugin_instance_name : * const i32, plugin_instance : * mut * mut mjCPlugin)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::CopyFromSpec (user/user_model.h:191)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_from_spec() {
    todo ! ()
}

/// C: mjCModel::PointToLocal (user/user_model.h:192)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_point_to_local() {
    todo ! ()
}

/// C: mjCModel::Compile (user/user_model.h:203)
/// Calls: _mjPRIVATE_setTlsLogHandler, mjCModel::Clear, mjCModel::ClearCompileWarnings, mjCModel::CopyFromSpec, mjCModel::TryCompile, mj_deleteData, mj_deleteModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_compile(vfs: *const mjVFS, m: *mut *mut mjModel) -> *mut mjModel {
    // WARNING: signature changed — verify body
    // Previous params: (vfs : * const mjVFS, m : * mut * mut mjModel)
    // Previous return: * mut mjModel
    todo ! ()
}

/// C: mjCModel::CopyBack (user/user_model.h:204)
/// Calls: mjCMesh::GetPosPtr, mjCMesh::GetQuatPtr
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_back(arg0: *const mjModel) -> bool {
    // WARNING: signature changed — verify body
    // Previous params: (arg0 : * const mjModel)
    // Previous return: bool
    todo ! ()
}

/// C: mjCModel::FuseStatic (user/user_model.h:205)
/// Calls: mjCBody::ComputeBVH, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_fuse_static() {
    todo ! ()
}

/// C: mjCModel::FuseReindex (user/user_model.h:206)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_fuse_reindex(body: *mut mjCBody) {
    // WARNING: signature changed — verify body
    // Previous params: (body : * mut mjCBody)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::AddFlex (user/user_model.h:209)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_flex() -> *mut mjCFlex {
    // WARNING: signature changed — verify body
    // Previous params: ()
    // Previous return: * mut mjCFlex
    todo ! ()
}

/// C: mjCModel::AddMesh (user/user_model.h:210)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_mesh(def: *mut mjCDef) -> *mut mjCMesh {
    // WARNING: signature changed — verify body
    // Previous params: (def : * mut mjCDef)
    // Previous return: * mut mjCMesh
    todo ! ()
}

/// C: mjCModel::AddSkin (user/user_model.h:211)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_skin() -> *mut mjCSkin {
    // WARNING: signature changed — verify body
    // Previous params: ()
    // Previous return: * mut mjCSkin
    todo ! ()
}

/// C: mjCModel::AddHField (user/user_model.h:212)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_h_field() -> *mut mjCHField {
    // WARNING: signature changed — verify body
    // Previous params: ()
    // Previous return: * mut mjCHField
    todo ! ()
}

/// C: mjCModel::AddTexture (user/user_model.h:213)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_texture() -> *mut mjCTexture {
    // WARNING: signature changed — verify body
    // Previous params: ()
    // Previous return: * mut mjCTexture
    todo ! ()
}

/// C: mjCModel::AddMaterial (user/user_model.h:214)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_material(def: *mut mjCDef) -> *mut mjCMaterial {
    // WARNING: signature changed — verify body
    // Previous params: (def : * mut mjCDef)
    // Previous return: * mut mjCMaterial
    todo ! ()
}

/// C: mjCModel::AddPair (user/user_model.h:215)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_pair(def: *mut mjCDef) -> *mut mjCPair {
    // WARNING: signature changed — verify body
    // Previous params: (def : * mut mjCDef)
    // Previous return: * mut mjCPair
    todo ! ()
}

/// C: mjCModel::AddExclude (user/user_model.h:216)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_exclude() -> *mut mjCBodyPair {
    // WARNING: signature changed — verify body
    // Previous params: ()
    // Previous return: * mut mjCBodyPair
    todo ! ()
}

/// C: mjCModel::AddEquality (user/user_model.h:217)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_equality(def: *mut mjCDef) -> *mut mjCEquality {
    // WARNING: signature changed — verify body
    // Previous params: (def : * mut mjCDef)
    // Previous return: * mut mjCEquality
    todo ! ()
}

/// C: mjCModel::AddTendon (user/user_model.h:218)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_tendon(def: *mut mjCDef) -> *mut mjCTendon {
    // WARNING: signature changed — verify body
    // Previous params: (def : * mut mjCDef)
    // Previous return: * mut mjCTendon
    todo ! ()
}

/// C: mjCModel::AddActuator (user/user_model.h:219)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_actuator(def: *mut mjCDef) -> *mut mjCActuator {
    // WARNING: signature changed — verify body
    // Previous params: (def : * mut mjCDef)
    // Previous return: * mut mjCActuator
    todo ! ()
}

/// C: mjCModel::AddSensor (user/user_model.h:220)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_sensor() -> *mut mjCSensor {
    // WARNING: signature changed — verify body
    // Previous params: ()
    // Previous return: * mut mjCSensor
    todo ! ()
}

/// C: mjCModel::AddNumeric (user/user_model.h:221)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_numeric() -> *mut mjCNumeric {
    // WARNING: signature changed — verify body
    // Previous params: ()
    // Previous return: * mut mjCNumeric
    todo ! ()
}

/// C: mjCModel::AddText (user/user_model.h:222)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_text() -> *mut mjCText {
    // WARNING: signature changed — verify body
    // Previous params: ()
    // Previous return: * mut mjCText
    todo ! ()
}

/// C: mjCModel::AddTuple (user/user_model.h:223)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_tuple() -> *mut mjCTuple {
    // WARNING: signature changed — verify body
    // Previous params: ()
    // Previous return: * mut mjCTuple
    todo ! ()
}

/// C: mjCModel::AddKey (user/user_model.h:224)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_key() -> *mut mjCKey {
    // WARNING: signature changed — verify body
    // Previous params: ()
    // Previous return: * mut mjCKey
    todo ! ()
}

/// C: mjCModel::AddPlugin (user/user_model.h:225)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_plugin() -> *mut mjCPlugin {
    // WARNING: signature changed — verify body
    // Previous params: ()
    // Previous return: * mut mjCPlugin
    todo ! ()
}

/// C: mjCModel::AppendSpec (user/user_model.h:228)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_append_spec(spec: *mut mjSpec, compiler: *const mjsCompiler) {
    // WARNING: signature changed — verify body
    // Previous params: (spec : * mut mjSpec, compiler : * const mjsCompiler)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::NumObjects (user/user_model.h:244)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_num_objects(r#type: mjtObj) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (r#type : mjtObj)
    // Previous return: i32
    todo ! ()
}

/// C: mjCModel::GetObject (user/user_model.h:245)
/// Calls: mjCModel::NumObjects
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_object(r#type: mjtObj, id: i32) -> *mut mjCBase {
    // WARNING: signature changed — verify body
    // Previous params: (r#type : mjtObj, id : i32)
    // Previous return: * mut mjCBase
    todo ! ()
}

/// C: mjCModel::NextObject (user/user_model.h:246)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_next_object(object: *const mjsElement, r#type: mjtObj) -> *mut mjsElement {
    // WARNING: signature changed — verify body
    // Previous params: (object : * const mjsElement, r#type : mjtObj)
    // Previous return: * mut mjsElement
    todo ! ()
}

/// C: mjCModel::IsCompiled (user/user_model.h:249)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_is_compiled() -> bool {
    todo ! ()
}

/// C: mjCModel::GetError (user/user_model.h:250)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_error() -> *const mjCError {
    // WARNING: signature changed — verify body
    // Previous params: ()
    // Previous return: * const mjCError
    todo ! ()
}

/// C: mjCModel::SetError (user/user_model.h:251)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_set_error(error: *const mjCError) {
    // WARNING: signature changed — verify body
    // Previous params: (error : * const mjCError)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::GetWarnings (user/user_model.h:256)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_warnings() -> *const i32 {
    // WARNING: signature changed — verify body
    // Previous params: ()
    // Previous return: * const i32
    todo ! ()
}

/// C: mjCModel::ClearWarnings (user/user_model.h:260)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_clear_warnings() {
    todo ! ()
}

/// C: mjCModel::ClearCompileWarnings (user/user_model.h:264)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_clear_compile_warnings() {
    todo ! ()
}

/// C: mjCModel::SetAttachWarningBoundary (user/user_model.h:267)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_set_attach_warning_boundary() {
    todo ! ()
}

/// C: mjCModel::GetWorld (user/user_model.h:271)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_world() -> *mut mjCBody {
    // WARNING: signature changed — verify body
    // Previous params: ()
    // Previous return: * mut mjCBody
    todo ! ()
}

/// C: mjCModel::FindSpec (user/user_model.h:277)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_find_spec(compiler_: *const mjsCompiler) -> *mut mjSpec {
    // WARNING: signature changed — verify body
    // Previous params: (compiler_ : * const mjsCompiler)
    // Previous return: * mut mjSpec
    todo ! ()
}

/// C: mjCModel::ActivatePlugin (user/user_model.h:278)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_activate_plugin(plugin: *const mjpPlugin, slot: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (plugin : * const mjpPlugin, slot : i32)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::get_meshdir (user/user_model.h:285)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_meshdir() -> i32 {
    todo ! ()
}

/// C: mjCModel::get_texturedir (user/user_model.h:286)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_texturedir() -> i32 {
    todo ! ()
}

/// C: mjCModel::Default (user/user_model.h:288)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_default() -> *mut mjCDef {
    // WARNING: signature changed — verify body
    // Previous params: ()
    // Previous return: * mut mjCDef
    todo ! ()
}

/// C: mjCModel::NumDefaults (user/user_model.h:289)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_num_defaults() -> i32 {
    todo ! ()
}

/// C: mjCModel::ActivePlugins (user/user_model.h:291)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_active_plugins() -> *const i32 {
    // WARNING: signature changed — verify body
    // Previous params: ()
    // Previous return: * const i32
    todo ! ()
}

/// C: mjCModel::Flexes (user/user_model.h:295)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_flexes() -> *const i32 {
    // WARNING: signature changed — verify body
    // Previous params: ()
    // Previous return: * const i32
    todo ! ()
}

/// C: mjCModel::Meshes (user/user_model.h:296)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_meshes() -> *const i32 {
    // WARNING: signature changed — verify body
    // Previous params: ()
    // Previous return: * const i32
    todo ! ()
}

/// C: mjCModel::Skins (user/user_model.h:297)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_skins() -> *const i32 {
    // WARNING: signature changed — verify body
    // Previous params: ()
    // Previous return: * const i32
    todo ! ()
}

/// C: mjCModel::HFields (user/user_model.h:298)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_h_fields() -> *const i32 {
    // WARNING: signature changed — verify body
    // Previous params: ()
    // Previous return: * const i32
    todo ! ()
}

/// C: mjCModel::Textures (user/user_model.h:299)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_textures() -> *const i32 {
    // WARNING: signature changed — verify body
    // Previous params: ()
    // Previous return: * const i32
    todo ! ()
}

/// C: mjCModel::Materials (user/user_model.h:300)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_materials() -> *const i32 {
    // WARNING: signature changed — verify body
    // Previous params: ()
    // Previous return: * const i32
    todo ! ()
}

/// C: mjCModel::Pairs (user/user_model.h:301)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_pairs() -> *const i32 {
    // WARNING: signature changed — verify body
    // Previous params: ()
    // Previous return: * const i32
    todo ! ()
}

/// C: mjCModel::Excludes (user/user_model.h:302)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_excludes() -> *const i32 {
    // WARNING: signature changed — verify body
    // Previous params: ()
    // Previous return: * const i32
    todo ! ()
}

/// C: mjCModel::Equalities (user/user_model.h:303)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_equalities() -> *const i32 {
    // WARNING: signature changed — verify body
    // Previous params: ()
    // Previous return: * const i32
    todo ! ()
}

/// C: mjCModel::Tendons (user/user_model.h:304)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_tendons() -> *const i32 {
    // WARNING: signature changed — verify body
    // Previous params: ()
    // Previous return: * const i32
    todo ! ()
}

/// C: mjCModel::Actuators (user/user_model.h:305)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_actuators() -> *const i32 {
    // WARNING: signature changed — verify body
    // Previous params: ()
    // Previous return: * const i32
    todo ! ()
}

/// C: mjCModel::Sensors (user/user_model.h:306)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_sensors() -> *const i32 {
    // WARNING: signature changed — verify body
    // Previous params: ()
    // Previous return: * const i32
    todo ! ()
}

/// C: mjCModel::Numerics (user/user_model.h:307)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_numerics() -> *const i32 {
    // WARNING: signature changed — verify body
    // Previous params: ()
    // Previous return: * const i32
    todo ! ()
}

/// C: mjCModel::Texts (user/user_model.h:308)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_texts() -> *const i32 {
    // WARNING: signature changed — verify body
    // Previous params: ()
    // Previous return: * const i32
    todo ! ()
}

/// C: mjCModel::Tuples (user/user_model.h:309)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_tuples() -> *const i32 {
    // WARNING: signature changed — verify body
    // Previous params: ()
    // Previous return: * const i32
    todo ! ()
}

/// C: mjCModel::Keys (user/user_model.h:310)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_keys() -> *const i32 {
    // WARNING: signature changed — verify body
    // Previous params: ()
    // Previous return: * const i32
    todo ! ()
}

/// C: mjCModel::Plugins (user/user_model.h:311)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_plugins() -> *const i32 {
    // WARNING: signature changed — verify body
    // Previous params: ()
    // Previous return: * const i32
    todo ! ()
}

/// C: mjCModel::Bodies (user/user_model.h:312)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_bodies() -> *const i32 {
    // WARNING: signature changed — verify body
    // Previous params: ()
    // Previous return: * const i32
    todo ! ()
}

/// C: mjCModel::Geoms (user/user_model.h:313)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_geoms() -> *const i32 {
    // WARNING: signature changed — verify body
    // Previous params: ()
    // Previous return: * const i32
    todo ! ()
}

/// C: mjCModel::Clear (user/user_model.h:321)
/// Calls: mjCModel::ClearCompileWarnings
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_clear() {
    todo ! ()
}

/// C: mjCModel::MakeData (user/user_model.h:338)
/// Calls: mj_initPlugin, mj_makeRawData, mj_resetData
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_make_data(m: *const mjModel, dest: *mut *mut mjData) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, dest : * mut * mut mjData)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::StoreKeyframes (user/user_model.h:341)
/// Calls: mjCModel::ComputeReference, mjCModel::SaveDofOffsets
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_store_keyframes(dest: *mut mjCModel) {
    // WARNING: signature changed — verify body
    // Previous params: (dest : * mut mjCModel)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::SetDeepCopy (user/user_model.h:347)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_set_deep_copy(deepcopy: bool) {
    // WARNING: signature changed — verify body
    // Previous params: (deepcopy : bool)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::SetAttached (user/user_model.h:350)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_set_attached(deepcopy: bool) {
    // WARNING: signature changed — verify body
    // Previous params: (deepcopy : bool)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::IsAttached (user/user_model.h:353)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_is_attached() -> bool {
    todo ! ()
}

/// C: mjCModel::CheckRepeat (user/user_model.h:356)
/// Calls: mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_check_repeat(r#type: mjtObj) {
    // WARNING: signature changed — verify body
    // Previous params: (r#type : mjtObj)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::AddRef (user/user_model.h:359)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_ref() {
    todo ! ()
}

/// C: mjCModel::GetRef (user/user_model.h:360)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_ref() -> i32 {
    todo ! ()
}

/// C: mjCModel::Release (user/user_model.h:361)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_release() {
    todo ! ()
}

/// C: mjCModel::MakeTreeLists (user/user_model.h:377)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_make_tree_lists(body: *mut mjCBody) {
    // WARNING: signature changed — verify body
    // Previous params: (body : * mut mjCBody)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::TryCompile (user/user_model.h:380)
/// Calls: mjCModel::AddKey, mjCModel::AutoSpringDamper, mjCModel::CheckEmptyNames, mjCModel::ClearCompileWarnings, mjCModel::CompileMeshesAndTextures, mjCModel::ComputeSparseSizes, mjCModel::CopyNames, mjCModel::CopyObjects, mjCModel::CopyPaths, mjCModel::CopyPlugins, mjCModel::CopyTree, mjCModel::ExpandAllKeyframes, mjCModel::FinalizeSimple, mjCModel::FuseStatic, mjCModel::IndexAssets, mjCModel::LengthRange, mjCModel::ProcessLists, mjCModel::ResolveKeyframes, mjCModel::SaveDofOffsets, mjCModel::SetNuser, mjCModel::SetSizes, mj_deleteData, mj_makeData, mj_makeRawData, mj_normalizeQuat, mj_resetData, mj_setConst, mj_step, mj_validateReferences, mjuu_defined
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_try_compile(m: *mut *mut mjModel, d: *mut *mut mjData, vfs: *const mjVFS) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * mut * mut mjModel, d : * mut * mut mjData, vfs : * const mjVFS)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::CompileMeshesAndTextures (user/user_model.h:381)
/// Calls: NumCompilerThreads, ThreadPool::WaitCount
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_compile_meshes_and_textures(vfs: *const mjVFS) {
    // WARNING: signature changed — verify body
    // Previous params: (vfs : * const mjVFS)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::SetNuser (user/user_model.h:383)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_set_nuser() {
    todo ! ()
}

/// C: mjCModel::IndexAssets (user/user_model.h:384)
/// Calls: mjCGeom::IsVisual, mjCMesh::IsVisual, mjCMesh::SetNotVisual
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_index_assets(discard: bool) {
    // WARNING: signature changed — verify body
    // Previous params: (discard : bool)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::CheckEmptyNames (user/user_model.h:385)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_check_empty_names() {
    todo ! ()
}

/// C: mjCModel::SetSizes (user/user_model.h:386)
/// Calls: mjCBody::GetParent
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_set_sizes() {
    todo ! ()
}

/// C: mjCModel::ComputeSparseSizes (user/user_model.h:387)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_compute_sparse_sizes() {
    todo ! ()
}

/// C: mjCModel::AutoSpringDamper (user/user_model.h:388)
/// Calls: mjCJoint::nv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_auto_spring_damper(arg0: *mut mjModel) {
    // WARNING: signature changed — verify body
    // Previous params: (arg0 : * mut mjModel)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::LengthRange (user/user_model.h:389)
/// Calls: NumCompilerThreads, ThreadPool::WaitCount, mj_makeData
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_length_range(arg0: *mut mjModel, arg1: *mut mjData) {
    // WARNING: signature changed — verify body
    // Previous params: (arg0 : * mut mjModel, arg1 : * mut mjData)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::CopyNames (user/user_model.h:390)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_names(arg0: *mut mjModel) {
    // WARNING: signature changed — verify body
    // Previous params: (arg0 : * mut mjModel)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::CopyPaths (user/user_model.h:391)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_paths(arg0: *mut mjModel) {
    // WARNING: signature changed — verify body
    // Previous params: (arg0 : * mut mjModel)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::CopyObjects (user/user_model.h:392)
/// Calls: mjCActuator::is_actlimited, mjCActuator::is_ctrllimited, mjCActuator::is_forcelimited, mjCBoundingVolumeHierarchy::Nbvh, mjCBoundingVolumeHierarchy::Nodeid, mjCMesh::CopyFace, mjCMesh::CopyFaceNormal, mjCMesh::CopyFaceTexcoord, mjCMesh::CopyGraph, mjCMesh::CopyNormal, mjCMesh::CopyPolygonMap, mjCMesh::CopyPolygonNormals, mjCMesh::CopyPolygons, mjCMesh::CopyTexcoord, mjCMesh::CopyVert, mjCMesh::GetPosPtr, mjCMesh::GetQuatPtr, mjCMesh::HasTexcoord, mjCMesh::Scale, mjCMesh::nface, mjCMesh::nnormal, mjCMesh::npolygon, mjCMesh::npolygonmap, mjCMesh::npolygonvert, mjCMesh::ntexcoord, mjCMesh::nvert, mjCMesh::octree, mjCMesh::szgraph, mjCMesh::tree, mjCOctree::CopyAabb, mjCOctree::CopyChild, mjCOctree::CopyCoeff, mjCOctree::CopyLevel, mjCOctree::NumNodes, mjCTendon::is_actfrclimited, mjCTendon::is_limited
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_objects(arg0: *mut mjModel) {
    // WARNING: signature changed — verify body
    // Previous params: (arg0 : * mut mjModel)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::CopyTree (user/user_model.h:393)
/// Calls: mjCGeom::GetRBound, mjCJoint::is_actfrclimited, mjCJoint::is_limited, mjCJoint::nq, mjCJoint::nv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_tree(arg0: *mut mjModel) {
    // WARNING: signature changed — verify body
    // Previous params: (arg0 : * mut mjModel)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::FinalizeSimple (user/user_model.h:394)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_finalize_simple(m: *mut mjModel) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * mut mjModel)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::CopyPlugins (user/user_model.h:395)
/// Calls: mjp_getPluginAtSlot, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_plugins(arg0: *mut mjModel) {
    // WARNING: signature changed — verify body
    // Previous params: (arg0 : * mut mjModel)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::CountTendonDofs (user/user_model.h:396)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_count_tendon_dofs(m: *const mjModel, id: i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, id : i32)
    // Previous return: i32
    todo ! ()
}

/// C: mjCModel::CountNJmom (user/user_model.h:398)
/// Calls: mjCModel::CountTendonDofs
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_count_n_jmom(m: *const mjModel) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel)
    // Previous return: i32
    todo ! ()
}

/// C: mjCModel::CountNJten (user/user_model.h:399)
/// Calls: mjCModel::CountTendonDofs
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_count_n_jten(m: *const mjModel) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel)
    // Previous return: i32
    todo ! ()
}

/// C: mjCModel::RemovePlugins (user/user_model.h:402)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_remove_plugins() {
    todo ! ()
}

/// C: mjCModel::CopyExplicitPlugin (user/user_model.h:448)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_explicit_plugin(obj: *mut T) {
    // WARNING: signature changed — verify body
    // Previous params: (obj : * mut T)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::CreateObjectLists (user/user_model.h:458)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_create_object_lists() {
    todo ! ()
}

/// C: mjCModel::ProcessLists (user/user_model.h:461)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_process_lists(checkrepeat: bool) {
    // WARNING: signature changed — verify body
    // Previous params: (checkrepeat : bool)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::ResetTreeLists (user/user_model.h:468)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_reset_tree_lists() {
    todo ! ()
}

/// C: mjCModel::SaveDofOffsets (user/user_model.h:471)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_save_dof_offsets(computesize: bool) {
    // WARNING: signature changed — verify body
    // Previous params: (computesize : bool)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::ResolveKeyframes (user/user_model.h:474)
/// Calls: mjCModel::SaveDofOffsets
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_resolve_keyframes(m: *const mjModel) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::ExpandKeyframe (user/user_model.h:477)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_expand_keyframe(key: *mut mjCKey, qpos0_: *const f64, bpos: *const f64, bquat: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (key : * mut mjCKey, qpos0_ : * const f64, bpos : * const f64, bquat : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::ComputeReference (user/user_model.h:480)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_compute_reference() {
    todo ! ()
}

/// C: mjCModel::CheckBodyMassInertia (user/user_model.h:483)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_check_body_mass_inertia(body: *mut mjCBody) -> bool {
    // WARNING: signature changed — verify body
    // Previous params: (body : * mut mjCBody)
    // Previous return: bool
    todo ! ()
}

/// C: mjCModel::DeleteSubtreePlugin (user/user_model.h:505)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_delete_subtree_plugin(subtree: *mut mjCBody) {
    // WARNING: signature changed — verify body
    // Previous params: (subtree : * mut mjCBody)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::ExpandAllKeyframes (user/user_model.h:508)
/// Calls: mjCModel::ComputeReference, mjCModel::SaveDofOffsets
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_expand_all_keyframes() {
    todo ! ()
}

