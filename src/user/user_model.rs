//! Port of: user/user_model.cc
//! IR hash: 6ff71909dacce27f
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: IsSameVec (user/user_model.cc:72)
#[allow(unused_variables, non_snake_case)]
pub fn is_same_vec(pos1: *const type_parameter_0_0, pos2: *const type_parameter_0_0) -> bool {
    todo!() // IsSameVec
}

/// C: NumCompilerThreads (user/user_model.cc:79)
#[allow(unused_variables, non_snake_case)]
pub fn num_compiler_threads(upper_bound: i32) -> u32 {
    todo!() // NumCompilerThreads
}

/// C: IsSameQuat (user/user_model.cc:93)
#[allow(unused_variables, non_snake_case)]
pub fn is_same_quat(quat1: *const type_parameter_0_0, quat2: *const type_parameter_0_0) -> bool {
    todo!() // IsSameQuat
}

/// C: IsSamePose (user/user_model.cc:111)
#[allow(unused_variables, non_snake_case)]
pub fn is_same_pose(pos1: *const type_parameter_0_0, pos2: *const type_parameter_0_0, quat1: *const type_parameter_0_0, quat2: *const type_parameter_0_0) -> bool {
    todo!() // IsSamePose
}

/// C: IsNullPose (user/user_model.cc:127)
#[allow(unused_variables, non_snake_case)]
pub fn is_null_pose(pos: *const type_parameter_0_0, quat: *const type_parameter_0_0) -> bool {
    todo!() // IsNullPose
}

/// C: GetBodyIdFromWrap (user/user_model.cc:134)
/// Calls: mjCWrap::Type
#[allow(unused_variables, non_snake_case)]
pub fn get_body_id_from_wrap(wrap: *const mjCWrap) -> i32 {
    todo!() // GetBodyIdFromWrap
}

/// C: mjCModel::CopyList (user/user_model.cc:261)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_list(self_ptr: *mut mjCModel, dest: *mut i32, source: *const i32) {
    todo!() // mjCModel::CopyList
}

/// C: resetlist (user/user_model.cc:304)
#[allow(unused_variables, non_snake_case)]
pub fn resetlist(list: *mut i32) {
    todo!() // resetlist
}

/// C: mjCModel::CopyPlugin (user/user_model.cc:396)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_plugin(self_ptr: *mut mjCModel, source: *const i32, list: *const i32) {
    todo!() // mjCModel::CopyPlugin
}

/// C: IsPluginActive (user/user_model.cc:440)
#[allow(unused_variables, non_snake_case)]
pub fn is_plugin_active(plugin: *const mjpPlugin, active_plugins: *const i32) -> bool {
    todo!() // IsPluginActive
}

/// C: mjCModel::RemoveFromList (user/user_model.cc:508)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_remove_from_list(self_ptr: *mut mjCModel, list: *mut i32, other: *const mjCModel) {
    todo!() // mjCModel::RemoveFromList
}

/// C: mjCModel::DeleteAll (user/user_model.cc:545)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_delete_all(self_ptr: *mut mjCModel, elements: *mut i32) {
    todo!() // mjCModel::DeleteAll
}

/// C: mjCModel::MarkPluginInstance (user/user_model.cc:555)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_mark_plugin_instance(self_ptr: *mut mjCModel, instances: *mut i32, list: *const i32) {
    todo!() // mjCModel::MarkPluginInstance
}

/// C: deletefromlist (user/user_model.cc:715)
#[allow(unused_variables, non_snake_case)]
pub fn deletefromlist(list: *mut i32, element: *mut mjsElement) {
    todo!() // deletefromlist
}

/// C: mjCModel::AddObject (user/user_model.cc:1252)
/// Calls: mjCModel::Signature
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_object(self_ptr: *mut mjCModel, list: *mut i32, r#type: string) -> *mut T {
    todo!() // mjCModel::AddObject
}

/// C: mjCModel::AddObjectDefault (user/user_model.cc:1263)
/// Calls: mjCModel::Signature
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_object_default(self_ptr: *mut mjCModel, list: *mut i32, r#type: string, def: *mut mjCDef) -> *mut T {
    todo!() // mjCModel::AddObjectDefault
}

/// C: GetNext (user/user_model.cc:1411)
#[allow(unused_variables, non_snake_case)]
pub fn get_next(list: *const i32, child: *const mjsElement) -> *mut mjsElement {
    todo!() // GetNext
}

/// C: findobject (user/user_model.cc:1596)
#[allow(unused_variables, non_snake_case)]
pub fn findobject(name: string_view, list: *const i32, ids: *const mjKeyMap) -> *mut T {
    todo!() // findobject
}

/// C: mjCModel::FindAsset (user/user_model.cc:1621)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_find_asset(self_ptr: *mut mjCModel, name: string_view, list: *const i32) -> *mut mjCBase {
    todo!() // mjCModel::FindAsset
}

/// C: mjCModel::DeleteMaterial (user/user_model.cc:1778)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_delete_material(self_ptr: *mut mjCModel, list: *mut i32, name: string_view) {
    todo!() // mjCModel::DeleteMaterial
}

/// C: DeleteAllTextures (user/user_model.cc:1790)
#[allow(unused_variables, non_snake_case)]
pub fn delete_all_textures(list: *mut i32) {
    todo!() // DeleteAllTextures
}

/// C: DeleteTexcoord (user/user_model.cc:1799)
#[allow(unused_variables, non_snake_case)]
pub fn delete_texcoord(list: *mut i32) {
    todo!() // DeleteTexcoord
}

/// C: DeleteElements (user/user_model.cc:1810)
#[allow(unused_variables, non_snake_case)]
pub fn delete_elements(elements: *mut i32, discard: *const i32) {
    todo!() // DeleteElements
}

/// C: mjCModel::Delete (user/user_model.cc:1848)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_delete(self_ptr: *mut mjCModel, elements: *mut i32, discard: *const i32) {
    todo!() // mjCModel::Delete
}

/// C: getpathslength (user/user_model.cc:2128)
#[allow(unused_variables, non_snake_case)]
pub fn getpathslength(list: i32) -> usize {
    todo!() // getpathslength
}

/// C: LRfunc (user/user_model.cc:2459)
/// Calls: mj_setLengthRange
#[allow(unused_variables, non_snake_case)]
pub fn l_rfunc(arg: *mut ()) -> *mut () {
    todo!() // LRfunc
}

/// C: addtolist (user/user_model.cc:2577)
#[allow(unused_variables, non_snake_case)]
pub fn addtolist(input: *const std__string, adr: i32, output_adr_field: *mut i32, output_buffer: *mut i8) -> i32 {
    todo!() // addtolist
}

/// C: namelist (user/user_model.cc:2593)
#[allow(unused_variables, non_snake_case)]
pub fn namelist(list: *mut i32, adr: i32, name_adr: *mut i32, names: *mut i8, map: *mut i32) -> i32 {
    todo!() // namelist
}

/// C: pathlist (user/user_model.cc:2702)
#[allow(unused_variables, non_snake_case)]
pub fn pathlist(list: *mut i32, adr: i32, path_adr: *mut i32, paths: *mut i8) -> i32 {
    todo!() // pathlist
}

/// C: makelistid (user/user_model.cc:4311)
#[allow(unused_variables, non_snake_case)]
pub fn makelistid(dest: *mut i32, source: *mut i32) {
    todo!() // makelistid
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
    todo!() // changeframe
}

/// C: mjCModel::ReassignChild (user/user_model.cc:4355)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_reassign_child(self_ptr: *mut mjCModel, dest: *mut i32, list: *mut i32, parent: *mut mjCBody, body: *mut mjCBody) {
    todo!() // mjCModel::ReassignChild
}

/// C: mjCModel::ResolveReferences (user/user_model.cc:4371)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_resolve_references(self_ptr: *mut mjCModel, list: *mut i32, body: *mut mjCBody) {
    todo!() // mjCModel::ResolveReferences
}

/// C: comparePair (user/user_model.cc:4565)
/// Calls: mjCPair::GetSignature
#[allow(unused_variables, non_snake_case)]
pub fn compare_pair(el1: *mut mjCPair, el2: *mut mjCPair) -> i32 {
    todo!() // comparePair
}

/// C: compareBodyPair (user/user_model.cc:4568)
/// Calls: mjCBodyPair::GetSignature
#[allow(unused_variables, non_snake_case)]
pub fn compare_body_pair(el1: *mut mjCBodyPair, el2: *mut mjCBodyPair) -> i32 {
    todo!() // compareBodyPair
}

/// C: reassignid (user/user_model.cc:4575)
#[allow(unused_variables, non_snake_case)]
pub fn reassignid(list: *mut i32) {
    todo!() // reassignid
}

/// C: mjCModel::ProcessList_ (user/user_model.cc:4600)
/// Calls: mjCModel::CheckRepeat, mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_process_list(self_ptr: *mut mjCModel, ids: *mut mjListKeyMap, list: *mut i32, r#type: u32, checkrepeat: bool) {
    todo!() // mjCModel::ProcessList_
}

/// C: compilerLogHandler (user/user_model.cc:4665)
#[allow(unused_variables, non_snake_case)]
pub fn compiler_log_handler(msg: *const mjLogMessage) {
    todo!() // compilerLogHandler
}

/// C: CompileMesh (user/user_model.cc:4770)
/// Calls: _mjPRIVATE_setTlsLogHandler, mjCMesh::Compile
#[allow(unused_variables, non_snake_case)]
pub fn compile_mesh(mesh: *mut mjCMesh, vfs: *const mjVFS, exception: *mut std__exception_ptr, exception_mutex: *mut std__mutex, warningtext: *mut string) {
    todo!() // CompileMesh
}

/// C: CompileTexture (user/user_model.cc:4790)
/// Calls: _mjPRIVATE_setTlsLogHandler, mjCTexture::Compile
#[allow(unused_variables, non_snake_case)]
pub fn compile_texture(texture: *mut mjCTexture, vfs: *const mjVFS, exception: *mut std__exception_ptr, exception_mutex: *mut std__mutex, warningtext: *mut string) {
    todo!() // CompileTexture
}

/// C: PrintIndent (user/user_model.cc:5457)
#[allow(unused_variables, non_snake_case)]
pub fn print_indent(ss: *mut std__stringstream, depth: i32) {
    todo!() // PrintIndent
}

/// C: mjCModel::CopyFromSpec (user/user_model.h:191)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_from_spec(self_ptr: *mut mjCModel) {
    todo!() // mjCModel::CopyFromSpec
}

/// C: mjCModel::PointToLocal (user/user_model.h:192)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_point_to_local(self_ptr: *mut mjCModel) {
    todo!() // mjCModel::PointToLocal
}

/// C: mjCModel::Compile (user/user_model.h:203)
/// Calls: _mjPRIVATE_setTlsLogHandler, mjCModel::Clear, mjCModel::ClearCompileWarnings, mjCModel::CopyFromSpec, mjCModel::TryCompile, mj_deleteData, mj_deleteModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_compile(self_ptr: *mut mjCModel, vfs: *const mjVFS, m: *mut *mut mjModel) -> *mut mjModel {
    todo!() // mjCModel::Compile
}

/// C: mjCModel::CopyBack (user/user_model.h:204)
/// Calls: mjCMesh::GetPosPtr, mjCMesh::GetQuatPtr
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_back(self_ptr: *mut mjCModel, arg0: *const mjModel) -> bool {
    todo!() // mjCModel::CopyBack
}

/// C: mjCModel::FuseStatic (user/user_model.h:205)
/// Calls: mjCBody::ComputeBVH, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_fuse_static(self_ptr: *mut mjCModel) {
    todo!() // mjCModel::FuseStatic
}

/// C: mjCModel::FuseReindex (user/user_model.h:206)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_fuse_reindex(self_ptr: *mut mjCModel, body: *mut mjCBody) {
    todo!() // mjCModel::FuseReindex
}

/// C: mjCModel::AddFlex (user/user_model.h:209)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_flex(self_ptr: *mut mjCModel) -> *mut mjCFlex {
    todo!() // mjCModel::AddFlex
}

/// C: mjCModel::AddMesh (user/user_model.h:210)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_mesh(self_ptr: *mut mjCModel, def: *mut mjCDef) -> *mut mjCMesh {
    todo!() // mjCModel::AddMesh
}

/// C: mjCModel::AddSkin (user/user_model.h:211)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_skin(self_ptr: *mut mjCModel) -> *mut mjCSkin {
    todo!() // mjCModel::AddSkin
}

/// C: mjCModel::AddHField (user/user_model.h:212)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_h_field(self_ptr: *mut mjCModel) -> *mut mjCHField {
    todo!() // mjCModel::AddHField
}

/// C: mjCModel::AddTexture (user/user_model.h:213)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_texture(self_ptr: *mut mjCModel) -> *mut mjCTexture {
    todo!() // mjCModel::AddTexture
}

/// C: mjCModel::AddMaterial (user/user_model.h:214)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_material(self_ptr: *mut mjCModel, def: *mut mjCDef) -> *mut mjCMaterial {
    todo!() // mjCModel::AddMaterial
}

/// C: mjCModel::AddPair (user/user_model.h:215)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_pair(self_ptr: *mut mjCModel, def: *mut mjCDef) -> *mut mjCPair {
    todo!() // mjCModel::AddPair
}

/// C: mjCModel::AddExclude (user/user_model.h:216)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_exclude(self_ptr: *mut mjCModel) -> *mut mjCBodyPair {
    todo!() // mjCModel::AddExclude
}

/// C: mjCModel::AddEquality (user/user_model.h:217)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_equality(self_ptr: *mut mjCModel, def: *mut mjCDef) -> *mut mjCEquality {
    todo!() // mjCModel::AddEquality
}

/// C: mjCModel::AddTendon (user/user_model.h:218)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_tendon(self_ptr: *mut mjCModel, def: *mut mjCDef) -> *mut mjCTendon {
    todo!() // mjCModel::AddTendon
}

/// C: mjCModel::AddActuator (user/user_model.h:219)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_actuator(self_ptr: *mut mjCModel, def: *mut mjCDef) -> *mut mjCActuator {
    todo!() // mjCModel::AddActuator
}

/// C: mjCModel::AddSensor (user/user_model.h:220)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_sensor(self_ptr: *mut mjCModel) -> *mut mjCSensor {
    todo!() // mjCModel::AddSensor
}

/// C: mjCModel::AddNumeric (user/user_model.h:221)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_numeric(self_ptr: *mut mjCModel) -> *mut mjCNumeric {
    todo!() // mjCModel::AddNumeric
}

/// C: mjCModel::AddText (user/user_model.h:222)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_text(self_ptr: *mut mjCModel) -> *mut mjCText {
    todo!() // mjCModel::AddText
}

/// C: mjCModel::AddTuple (user/user_model.h:223)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_tuple(self_ptr: *mut mjCModel) -> *mut mjCTuple {
    todo!() // mjCModel::AddTuple
}

/// C: mjCModel::AddKey (user/user_model.h:224)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_key(self_ptr: *mut mjCModel) -> *mut mjCKey {
    todo!() // mjCModel::AddKey
}

/// C: mjCModel::AddPlugin (user/user_model.h:225)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_plugin(self_ptr: *mut mjCModel) -> *mut mjCPlugin {
    todo!() // mjCModel::AddPlugin
}

/// C: mjCModel::AppendSpec (user/user_model.h:228)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_append_spec(self_ptr: *mut mjCModel, spec: *mut mjSpec, compiler: *const mjsCompiler) {
    todo!() // mjCModel::AppendSpec
}

/// C: mjCModel::NumObjects (user/user_model.h:244)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_num_objects(self_ptr: *mut mjCModel, r#type: u32) -> i32 {
    todo!() // mjCModel::NumObjects
}

/// C: mjCModel::GetObject (user/user_model.h:245)
/// Calls: mjCModel::NumObjects
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_object(self_ptr: *mut mjCModel, r#type: u32, id: i32) -> *mut mjCBase {
    todo!() // mjCModel::GetObject
}

/// C: mjCModel::NextObject (user/user_model.h:246)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_next_object(self_ptr: *mut mjCModel, object: *const mjsElement, r#type: u32) -> *mut mjsElement {
    todo!() // mjCModel::NextObject
}

/// C: mjCModel::IsCompiled (user/user_model.h:249)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_is_compiled(self_ptr: *mut mjCModel) -> bool {
    todo!() // mjCModel::IsCompiled
}

/// C: mjCModel::GetError (user/user_model.h:250)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_error(self_ptr: *mut mjCModel) -> *const mjCError {
    todo!() // mjCModel::GetError
}

/// C: mjCModel::SetError (user/user_model.h:251)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_set_error(self_ptr: *mut mjCModel, error: *const mjCError) {
    todo!() // mjCModel::SetError
}

/// C: mjCModel::AddWarning (user/user_model.h:252)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_warning(self_ptr: *mut mjCModel, msg: string, obj: *const mjCBase) {
    todo!() // mjCModel::AddWarning
}

/// C: mjCModel::AddGroupedWarning (user/user_model.h:254)
/// Calls: mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_grouped_warning(self_ptr: *mut mjCModel, subject: *const std__string, body: *const std__string) {
    todo!() // mjCModel::AddGroupedWarning
}

/// C: mjCModel::GetWarnings (user/user_model.h:256)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_warnings(self_ptr: *mut mjCModel) -> *const i32 {
    todo!() // mjCModel::GetWarnings
}

/// C: mjCModel::ClearWarnings (user/user_model.h:260)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_clear_warnings(self_ptr: *mut mjCModel) {
    todo!() // mjCModel::ClearWarnings
}

/// C: mjCModel::ClearCompileWarnings (user/user_model.h:264)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_clear_compile_warnings(self_ptr: *mut mjCModel) {
    todo!() // mjCModel::ClearCompileWarnings
}

/// C: mjCModel::SetAttachWarningBoundary (user/user_model.h:267)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_set_attach_warning_boundary(self_ptr: *mut mjCModel) {
    todo!() // mjCModel::SetAttachWarningBoundary
}

/// C: mjCModel::GetWorld (user/user_model.h:271)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_world(self_ptr: *mut mjCModel) -> *mut mjCBody {
    todo!() // mjCModel::GetWorld
}

/// C: mjCModel::FindDefault (user/user_model.h:272)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_find_default(self_ptr: *mut mjCModel, name: *const std__string) -> *mut mjCDef {
    todo!() // mjCModel::FindDefault
}

/// C: mjCModel::AddDefault (user/user_model.h:273)
/// Calls: mjCDef::CopyFromSpec, mjCDef::CopyWithoutChildren
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_default(self_ptr: *mut mjCModel, name: string, parent: *mut mjCDef) -> *mut mjCDef {
    todo!() // mjCModel::AddDefault
}

/// C: mjCModel::FindObject (user/user_model.h:274)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_find_object(self_ptr: *mut mjCModel, r#type: u32, name: string) -> *mut mjCBase {
    todo!() // mjCModel::FindObject
}

/// C: mjCModel::FindTree (user/user_model.h:275)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_find_tree(self_ptr: *mut mjCModel, body: *mut mjCBody, r#type: u32, name: string) -> *mut mjCBase {
    todo!() // mjCModel::FindTree
}

/// C: mjCModel::FindSpec (user/user_model.h:276)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_find_spec(self_ptr: *mut mjCModel, name: string) -> *mut mjSpec {
    todo!() // mjCModel::FindSpec
}

/// C: mjCModel::ActivatePlugin (user/user_model.h:278)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_activate_plugin(self_ptr: *mut mjCModel, plugin: *const mjpPlugin, slot: i32) {
    todo!() // mjCModel::ActivatePlugin
}

/// C: mjCModel::get_meshdir (user/user_model.h:285)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_meshdir(self_ptr: *mut mjCModel) -> i32 {
    todo!() // mjCModel::get_meshdir
}

/// C: mjCModel::get_texturedir (user/user_model.h:286)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_texturedir(self_ptr: *mut mjCModel) -> i32 {
    todo!() // mjCModel::get_texturedir
}

/// C: mjCModel::Default (user/user_model.h:288)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_default(self_ptr: *mut mjCModel) -> *mut mjCDef {
    todo!() // mjCModel::Default
}

/// C: mjCModel::NumDefaults (user/user_model.h:289)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_num_defaults(self_ptr: *mut mjCModel) -> i32 {
    todo!() // mjCModel::NumDefaults
}

/// C: mjCModel::ActivePlugins (user/user_model.h:291)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_active_plugins(self_ptr: *mut mjCModel) -> *const i32 {
    todo!() // mjCModel::ActivePlugins
}

/// C: mjCModel::Flexes (user/user_model.h:295)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_flexes(self_ptr: *mut mjCModel) -> *const i32 {
    todo!() // mjCModel::Flexes
}

/// C: mjCModel::Meshes (user/user_model.h:296)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_meshes(self_ptr: *mut mjCModel) -> *const i32 {
    todo!() // mjCModel::Meshes
}

/// C: mjCModel::Skins (user/user_model.h:297)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_skins(self_ptr: *mut mjCModel) -> *const i32 {
    todo!() // mjCModel::Skins
}

/// C: mjCModel::HFields (user/user_model.h:298)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_h_fields(self_ptr: *mut mjCModel) -> *const i32 {
    todo!() // mjCModel::HFields
}

/// C: mjCModel::Textures (user/user_model.h:299)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_textures(self_ptr: *mut mjCModel) -> *const i32 {
    todo!() // mjCModel::Textures
}

/// C: mjCModel::Materials (user/user_model.h:300)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_materials(self_ptr: *mut mjCModel) -> *const i32 {
    todo!() // mjCModel::Materials
}

/// C: mjCModel::Pairs (user/user_model.h:301)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_pairs(self_ptr: *mut mjCModel) -> *const i32 {
    todo!() // mjCModel::Pairs
}

/// C: mjCModel::Excludes (user/user_model.h:302)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_excludes(self_ptr: *mut mjCModel) -> *const i32 {
    todo!() // mjCModel::Excludes
}

/// C: mjCModel::Equalities (user/user_model.h:303)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_equalities(self_ptr: *mut mjCModel) -> *const i32 {
    todo!() // mjCModel::Equalities
}

/// C: mjCModel::Tendons (user/user_model.h:304)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_tendons(self_ptr: *mut mjCModel) -> *const i32 {
    todo!() // mjCModel::Tendons
}

/// C: mjCModel::Actuators (user/user_model.h:305)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_actuators(self_ptr: *mut mjCModel) -> *const i32 {
    todo!() // mjCModel::Actuators
}

/// C: mjCModel::Sensors (user/user_model.h:306)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_sensors(self_ptr: *mut mjCModel) -> *const i32 {
    todo!() // mjCModel::Sensors
}

/// C: mjCModel::Numerics (user/user_model.h:307)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_numerics(self_ptr: *mut mjCModel) -> *const i32 {
    todo!() // mjCModel::Numerics
}

/// C: mjCModel::Texts (user/user_model.h:308)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_texts(self_ptr: *mut mjCModel) -> *const i32 {
    todo!() // mjCModel::Texts
}

/// C: mjCModel::Tuples (user/user_model.h:309)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_tuples(self_ptr: *mut mjCModel) -> *const i32 {
    todo!() // mjCModel::Tuples
}

/// C: mjCModel::Keys (user/user_model.h:310)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_keys(self_ptr: *mut mjCModel) -> *const i32 {
    todo!() // mjCModel::Keys
}

/// C: mjCModel::Plugins (user/user_model.h:311)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_plugins(self_ptr: *mut mjCModel) -> *const i32 {
    todo!() // mjCModel::Plugins
}

/// C: mjCModel::Bodies (user/user_model.h:312)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_bodies(self_ptr: *mut mjCModel) -> *const i32 {
    todo!() // mjCModel::Bodies
}

/// C: mjCModel::Geoms (user/user_model.h:313)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_geoms(self_ptr: *mut mjCModel) -> *const i32 {
    todo!() // mjCModel::Geoms
}

/// C: mjCModel::ResolvePlugin (user/user_model.h:316)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_resolve_plugin(self_ptr: *mut mjCModel, obj: *mut mjCBase, plugin_name: *const std__string, plugin_instance_name: *const std__string, plugin_instance: *mut *mut mjCPlugin) {
    todo!() // mjCModel::ResolvePlugin
}

/// C: mjCModel::Clear (user/user_model.h:321)
/// Calls: mjCModel::ClearCompileWarnings
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_clear(self_ptr: *mut mjCModel) {
    todo!() // mjCModel::Clear
}

/// C: mjCModel::SaveState (user/user_model.h:329)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_save_state(self_ptr: *mut mjCModel, state_name: *const std__string, qpos: *const T, qvel: *const T, act: *const T, ctrl: *const T, mpos: *const T, mquat: *const T) {
    todo!() // mjCModel::SaveState
}

/// C: mjCModel::RestoreState (user/user_model.h:334)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_restore_state(self_ptr: *mut mjCModel, state_name: *const std__string, pos0: *const f64, mpos0: *const f64, mquat0: *const f64, qpos: *mut T, qvel: *mut T, act: *mut T, ctrl: *mut T, mpos: *mut T, mquat: *mut T) {
    todo!() // mjCModel::RestoreState
}

/// C: mjCModel::MakeData (user/user_model.h:338)
/// Calls: mj_initPlugin, mj_makeRawData, mj_resetData
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_make_data(self_ptr: *mut mjCModel, m: *const mjModel, dest: *mut *mut mjData) {
    todo!() // mjCModel::MakeData
}

/// C: mjCModel::StoreKeyframes (user/user_model.h:341)
/// Calls: mjCModel::ComputeReference, mjCModel::SaveDofOffsets
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_store_keyframes(self_ptr: *mut mjCModel, dest: *mut mjCModel) {
    todo!() // mjCModel::StoreKeyframes
}

/// C: mjCModel::SetDeepCopy (user/user_model.h:347)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_set_deep_copy(self_ptr: *mut mjCModel, deepcopy: bool) {
    todo!() // mjCModel::SetDeepCopy
}

/// C: mjCModel::SetAttached (user/user_model.h:350)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_set_attached(self_ptr: *mut mjCModel, deepcopy: bool) {
    todo!() // mjCModel::SetAttached
}

/// C: mjCModel::IsAttached (user/user_model.h:353)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_is_attached(self_ptr: *mut mjCModel) -> bool {
    todo!() // mjCModel::IsAttached
}

/// C: mjCModel::CheckRepeat (user/user_model.h:356)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_check_repeat(self_ptr: *mut mjCModel, r#type: u32) {
    todo!() // mjCModel::CheckRepeat
}

/// C: mjCModel::AddRef (user/user_model.h:359)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_ref(self_ptr: *mut mjCModel) {
    todo!() // mjCModel::AddRef
}

/// C: mjCModel::GetRef (user/user_model.h:360)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_ref(self_ptr: *mut mjCModel) -> i32 {
    todo!() // mjCModel::GetRef
}

/// C: mjCModel::Release (user/user_model.h:361)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_release(self_ptr: *mut mjCModel) {
    todo!() // mjCModel::Release
}

/// C: mjCModel::MakeTreeLists (user/user_model.h:377)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_make_tree_lists(self_ptr: *mut mjCModel, body: *mut mjCBody) {
    todo!() // mjCModel::MakeTreeLists
}

/// C: mjCModel::TryCompile (user/user_model.h:380)
/// Calls: mjCModel::AddKey, mjCModel::AddWarning, mjCModel::AutoSpringDamper, mjCModel::CheckEmptyNames, mjCModel::ClearCompileWarnings, mjCModel::CompileMeshesAndTextures, mjCModel::ComputeSparseSizes, mjCModel::CopyNames, mjCModel::CopyObjects, mjCModel::CopyPaths, mjCModel::CopyPlugins, mjCModel::CopyTree, mjCModel::ExpandAllKeyframes, mjCModel::FinalizeSimple, mjCModel::FuseStatic, mjCModel::IndexAssets, mjCModel::LengthRange, mjCModel::ProcessLists, mjCModel::ResolveKeyframes, mjCModel::SaveDofOffsets, mjCModel::SetNuser, mjCModel::SetSizes, mjCModel::Signature, mj_deleteData, mj_makeData, mj_makeRawData, mj_normalizeQuat, mj_resetData, mj_setConst, mj_step, mj_validateReferences, mjuu_defined
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_try_compile(self_ptr: *mut mjCModel, m: *mut *mut mjModel, d: *mut *mut mjData, vfs: *const mjVFS) {
    todo!() // mjCModel::TryCompile
}

/// C: mjCModel::CompileMeshesAndTextures (user/user_model.h:381)
/// Calls: NumCompilerThreads, ThreadPool::WaitCount
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_compile_meshes_and_textures(self_ptr: *mut mjCModel, vfs: *const mjVFS) {
    todo!() // mjCModel::CompileMeshesAndTextures
}

/// C: mjCModel::SetNuser (user/user_model.h:383)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_set_nuser(self_ptr: *mut mjCModel) {
    todo!() // mjCModel::SetNuser
}

/// C: mjCModel::IndexAssets (user/user_model.h:384)
/// Calls: mjCGeom::IsVisual, mjCGeom::get_hfieldname, mjCGeom::get_material, mjCGeom::get_meshname, mjCMesh::IsVisual, mjCMesh::SetNotVisual, mjCModel::FindObject, mjCSite::get_material
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_index_assets(self_ptr: *mut mjCModel, discard: bool) {
    todo!() // mjCModel::IndexAssets
}

/// C: mjCModel::CheckEmptyNames (user/user_model.h:385)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_check_empty_names(self_ptr: *mut mjCModel) {
    todo!() // mjCModel::CheckEmptyNames
}

/// C: mjCModel::SetSizes (user/user_model.h:386)
/// Calls: mjCBody::GetParent
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_set_sizes(self_ptr: *mut mjCModel) {
    todo!() // mjCModel::SetSizes
}

/// C: mjCModel::ComputeSparseSizes (user/user_model.h:387)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_compute_sparse_sizes(self_ptr: *mut mjCModel) {
    todo!() // mjCModel::ComputeSparseSizes
}

/// C: mjCModel::AutoSpringDamper (user/user_model.h:388)
/// Calls: mjCJoint::nv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_auto_spring_damper(self_ptr: *mut mjCModel, arg0: *mut mjModel) {
    todo!() // mjCModel::AutoSpringDamper
}

/// C: mjCModel::LengthRange (user/user_model.h:389)
/// Calls: NumCompilerThreads, ThreadPool::WaitCount, mj_makeData
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_length_range(self_ptr: *mut mjCModel, arg0: *mut mjModel, arg1: *mut mjData) {
    todo!() // mjCModel::LengthRange
}

/// C: mjCModel::CopyNames (user/user_model.h:390)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_names(self_ptr: *mut mjCModel, arg0: *mut mjModel) {
    todo!() // mjCModel::CopyNames
}

/// C: mjCModel::CopyPaths (user/user_model.h:391)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_paths(self_ptr: *mut mjCModel, arg0: *mut mjModel) {
    todo!() // mjCModel::CopyPaths
}

/// C: mjCModel::CopyObjects (user/user_model.h:392)
/// Calls: mjCActuator::is_actlimited, mjCActuator::is_ctrllimited, mjCActuator::is_forcelimited, mjCBoundingVolumeHierarchy::Nbvh, mjCBoundingVolumeHierarchy::Nodeid, mjCMesh::CopyFace, mjCMesh::CopyFaceNormal, mjCMesh::CopyFaceTexcoord, mjCMesh::CopyGraph, mjCMesh::CopyNormal, mjCMesh::CopyPolygonMap, mjCMesh::CopyPolygonNormals, mjCMesh::CopyPolygons, mjCMesh::CopyTexcoord, mjCMesh::CopyVert, mjCMesh::GetPosPtr, mjCMesh::GetQuatPtr, mjCMesh::HasTexcoord, mjCMesh::Scale, mjCMesh::nface, mjCMesh::nnormal, mjCMesh::npolygon, mjCMesh::npolygonmap, mjCMesh::npolygonvert, mjCMesh::ntexcoord, mjCMesh::nvert, mjCMesh::octree, mjCMesh::szgraph, mjCMesh::tree, mjCModel::AddWarning, mjCOctree::CopyAabb, mjCOctree::CopyChild, mjCOctree::CopyCoeff, mjCOctree::CopyLevel, mjCOctree::NumNodes, mjCTendon::is_actfrclimited, mjCTendon::is_limited
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_objects(self_ptr: *mut mjCModel, arg0: *mut mjModel) {
    todo!() // mjCModel::CopyObjects
}

/// C: mjCModel::CopyTree (user/user_model.h:393)
/// Calls: mjCGeom::GetRBound, mjCJoint::is_actfrclimited, mjCJoint::is_limited, mjCJoint::nq, mjCJoint::nv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_tree(self_ptr: *mut mjCModel, arg0: *mut mjModel) {
    todo!() // mjCModel::CopyTree
}

/// C: mjCModel::FinalizeSimple (user/user_model.h:394)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_finalize_simple(self_ptr: *mut mjCModel, m: *mut mjModel) {
    todo!() // mjCModel::FinalizeSimple
}

/// C: mjCModel::CopyPlugins (user/user_model.h:395)
/// Calls: mjp_getPluginAtSlot, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_plugins(self_ptr: *mut mjCModel, arg0: *mut mjModel) {
    todo!() // mjCModel::CopyPlugins
}

/// C: mjCModel::CountTendonDofs (user/user_model.h:396)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_count_tendon_dofs(self_ptr: *mut mjCModel, m: *const mjModel, id: i32) -> i32 {
    todo!() // mjCModel::CountTendonDofs
}

/// C: mjCModel::CountNJmom (user/user_model.h:398)
/// Calls: mjCModel::CountTendonDofs
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_count_n_jmom(self_ptr: *mut mjCModel, m: *const mjModel) -> i32 {
    todo!() // mjCModel::CountNJmom
}

/// C: mjCModel::CountNJten (user/user_model.h:399)
/// Calls: mjCModel::CountTendonDofs
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_count_n_jten(self_ptr: *mut mjCModel, m: *const mjModel) -> i32 {
    todo!() // mjCModel::CountNJten
}

/// C: mjCModel::RemovePlugins (user/user_model.h:402)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_remove_plugins(self_ptr: *mut mjCModel) {
    todo!() // mjCModel::RemovePlugins
}

/// C: mjCModel::CopyExplicitPlugin (user/user_model.h:448)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_explicit_plugin(self_ptr: *mut mjCModel, obj: *mut T) {
    todo!() // mjCModel::CopyExplicitPlugin
}

/// C: mjCModel::CreateObjectLists (user/user_model.h:458)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_create_object_lists(self_ptr: *mut mjCModel) {
    todo!() // mjCModel::CreateObjectLists
}

/// C: mjCModel::ProcessLists (user/user_model.h:461)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_process_lists(self_ptr: *mut mjCModel, checkrepeat: bool) {
    todo!() // mjCModel::ProcessLists
}

/// C: mjCModel::ResetTreeLists (user/user_model.h:468)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_reset_tree_lists(self_ptr: *mut mjCModel) {
    todo!() // mjCModel::ResetTreeLists
}

/// C: mjCModel::SaveDofOffsets (user/user_model.h:471)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_save_dof_offsets(self_ptr: *mut mjCModel, computesize: bool) {
    todo!() // mjCModel::SaveDofOffsets
}

/// C: mjCModel::ResolveKeyframes (user/user_model.h:474)
/// Calls: mjCModel::SaveDofOffsets
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_resolve_keyframes(self_ptr: *mut mjCModel, m: *const mjModel) {
    todo!() // mjCModel::ResolveKeyframes
}

/// C: mjCModel::ExpandKeyframe (user/user_model.h:477)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_expand_keyframe(self_ptr: *mut mjCModel, key: *mut mjCKey, qpos0_: *const f64, bpos: *const f64, bquat: *const f64) {
    todo!() // mjCModel::ExpandKeyframe
}

/// C: mjCModel::ComputeReference (user/user_model.h:480)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_compute_reference(self_ptr: *mut mjCModel) {
    todo!() // mjCModel::ComputeReference
}

/// C: mjCModel::CheckBodyMassInertia (user/user_model.h:483)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_check_body_mass_inertia(self_ptr: *mut mjCModel, body: *mut mjCBody) -> bool {
    todo!() // mjCModel::CheckBodyMassInertia
}

/// C: mjCModel::PrintTree (user/user_model.h:491)
/// Calls: PrintIndent
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_print_tree(self_ptr: *mut mjCModel, tree: *mut std__stringstream, body: *const mjCBody, depth: i32) {
    todo!() // mjCModel::PrintTree
}

/// C: mjCModel::Signature (user/user_model.h:494)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_signature(self_ptr: *mut mjCModel) -> u64 {
    todo!() // mjCModel::Signature
}

/// C: mjCModel::DeleteSubtreePlugin (user/user_model.h:505)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_delete_subtree_plugin(self_ptr: *mut mjCModel, subtree: *mut mjCBody) {
    todo!() // mjCModel::DeleteSubtreePlugin
}

/// C: mjCModel::ExpandAllKeyframes (user/user_model.h:508)
/// Calls: mjCModel::ComputeReference, mjCModel::SaveDofOffsets
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_expand_all_keyframes(self_ptr: *mut mjCModel) {
    todo!() // mjCModel::ExpandAllKeyframes
}

