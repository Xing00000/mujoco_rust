//! Port of: user/user_model.cc
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: IsSameVec (user/user_model.cc:72)
#[allow(unused_variables, non_snake_case)]
pub fn is_same_vec(pos1: [T; 3], pos2: [T; 3]) -> bool {
    // WARNING: signature changed — verify body
    // Previous params: (pos1 : [T ; 3], pos2 : [T ; 3])
    // Previous return: bool
    extern "C" { fn IsSameVec_impl (pos1 : [T ; 3] , pos2 : [T ; 3]) -> bool ; } unsafe { IsSameVec_impl (pos1 , pos2) }
}

/// C: NumCompilerThreads (user/user_model.cc:79)
#[allow(unused_variables, non_snake_case)]
pub fn num_compiler_threads(upper_bound: i32) -> u32 {
    extern "C" { fn NumCompilerThreads_impl(upper_bound: i32) -> u32; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { NumCompilerThreads_impl(upper_bound) }
}

/// C: IsSameQuat (user/user_model.cc:93)
#[allow(unused_variables, non_snake_case)]
pub fn is_same_quat(quat1: [T; 4], quat2: [T; 4]) -> bool {
    // WARNING: signature changed — verify body
    // Previous params: (quat1 : [T ; 4], quat2 : [T ; 4])
    // Previous return: bool
    extern "C" { fn IsSameQuat_impl (quat1 : [T ; 4] , quat2 : [T ; 4]) -> bool ; } unsafe { IsSameQuat_impl (quat1 , quat2) }
}

/// C: IsSamePose (user/user_model.cc:111)
#[allow(unused_variables, non_snake_case)]
pub fn is_same_pose(pos1: [T; 3], pos2: [T; 3], quat1: [T; 4], quat2: [T; 4]) -> bool {
    // WARNING: signature changed — verify body
    // Previous params: (pos1 : [T ; 3], pos2 : [T ; 3], quat1 : [T ; 4], quat2 : [T ; 4])
    // Previous return: bool
    extern "C" { fn IsSamePose_impl (pos1 : [T ; 3] , pos2 : [T ; 3] , quat1 : [T ; 4] , quat2 : [T ; 4]) -> bool ; } unsafe { IsSamePose_impl (pos1 , pos2 , quat1 , quat2) }
}

/// C: IsNullPose (user/user_model.cc:127)
#[allow(unused_variables, non_snake_case)]
pub fn is_null_pose(pos: [T; 3], quat: [T; 4]) -> bool {
    // WARNING: signature changed — verify body
    // Previous params: (pos : [T ; 3], quat : [T ; 4])
    // Previous return: bool
    extern "C" { fn IsNullPose_impl (pos : [T ; 3] , quat : [T ; 4]) -> bool ; } unsafe { IsNullPose_impl (pos , quat) }
}

/// C: GetBodyIdFromWrap (user/user_model.cc:134)
/// Calls: mjCWrap::Type
#[allow(unused_variables, non_snake_case)]
pub fn get_body_id_from_wrap(wrap: *const mjCWrap) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (wrap : * const mjCWrap)
    // Previous return: i32
    extern "C" { fn GetBodyIdFromWrap_impl (wrap : * const mjCWrap) -> i32 ; } unsafe { GetBodyIdFromWrap_impl (wrap) }
}

/// C: mjCModel::CopyList (user/user_model.cc:261)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_list(self_ptr: *mut mjCModel, dest: *mut i32, source: *const i32) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, dest : * mut i32, source : * const i32)
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
pub fn mj_c_model_copy_plugin(self_ptr: *mut mjCModel, source: *const i32, list: *const i32) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, source : * const i32, list : * const i32)
    // Previous return: ()
    todo ! ()
}

/// C: IsPluginActive (user/user_model.cc:440)
#[allow(unused_variables, non_snake_case)]
pub fn is_plugin_active(plugin: *const mjpPlugin, active_plugins: *const i32) -> bool {
    // WARNING: signature changed — verify body
    // Previous params: (plugin : * const mjpPlugin, active_plugins : * const i32)
    // Previous return: bool
    extern "C" { fn IsPluginActive_impl (plugin : * const mjpPlugin , active_plugins : * const i32) -> bool ; } unsafe { IsPluginActive_impl (plugin , active_plugins) }
}

/// C: mjCModel::RemoveFromList (user/user_model.cc:508)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_remove_from_list(self_ptr: *mut mjCModel, list: *mut i32, other: *const mjCModel) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, list : * mut i32, other : * const mjCModel)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::DeleteAll (user/user_model.cc:545)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_delete_all(self_ptr: *mut mjCModel, elements: *mut i32) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, elements : * mut i32)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::MarkPluginInstance (user/user_model.cc:555)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_mark_plugin_instance(self_ptr: *mut mjCModel, instances: *mut i32, list: *const i32) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, instances : * mut i32, list : * const i32)
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
/// Calls: mjCModel::Signature
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_object(self_ptr: *mut mjCModel, list: *mut i32, r#type: string) -> *mut T {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, list : * mut i32, r#type : string)
    // Previous return: * mut T
    todo ! ()
}

/// C: mjCModel::AddObjectDefault (user/user_model.cc:1263)
/// Calls: mjCModel::Signature
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_object_default(self_ptr: *mut mjCModel, list: *mut i32, r#type: string, def: *mut mjCDef) -> *mut T {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, list : * mut i32, r#type : string, def : * mut mjCDef)
    // Previous return: * mut T
    todo ! ()
}

/// C: GetNext (user/user_model.cc:1411)
#[allow(unused_variables, non_snake_case)]
pub fn get_next(list: *const i32, child: *const mjsElement) -> *mut mjsElement {
    // WARNING: signature changed — verify body
    // Previous params: (list : * const i32, child : * const mjsElement)
    // Previous return: * mut mjsElement
    extern "C" { fn GetNext_impl (list : * const i32 , child : * const mjsElement) -> * mut mjsElement ; } unsafe { GetNext_impl (list , child) }
}

/// C: findobject (user/user_model.cc:1596)
#[allow(unused_variables, non_snake_case)]
pub fn findobject(name: string_view, list: *const i32, ids: *const mjKeyMap) -> *mut T {
    // WARNING: signature changed — verify body
    // Previous params: (name : string_view, list : * const i32, ids : * const mjKeyMap)
    // Previous return: * mut T
    todo ! ()
}

/// C: mjCModel::FindAsset (user/user_model.cc:1621)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_find_asset(self_ptr: *mut mjCModel, name: string_view, list: *const i32) -> *mut mjCBase {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, name : string_view, list : * const i32)
    // Previous return: * mut mjCBase
    todo ! ()
}

/// C: mjCModel::DeleteMaterial (user/user_model.cc:1778)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_delete_material(self_ptr: *mut mjCModel, list: *mut i32, name: string_view) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, list : * mut i32, name : string_view)
    // Previous return: ()
    todo ! ()
}

/// C: DeleteAllTextures (user/user_model.cc:1790)
#[allow(unused_variables, non_snake_case)]
pub fn delete_all_textures(list: *mut i32) {
    // WARNING: signature changed — verify body
    // Previous params: (list : * mut i32)
    // Previous return: ()
    extern "C" { fn DeleteAllTextures_impl (list : * mut i32) ; } unsafe { DeleteAllTextures_impl (list) }
}

/// C: DeleteTexcoord (user/user_model.cc:1799)
#[allow(unused_variables, non_snake_case)]
pub fn delete_texcoord(list: *mut i32) {
    // WARNING: signature changed — verify body
    // Previous params: (list : * mut i32)
    // Previous return: ()
    extern "C" { fn DeleteTexcoord_impl (list : * mut i32) ; } unsafe { DeleteTexcoord_impl (list) }
}

/// C: DeleteElements (user/user_model.cc:1810)
#[allow(unused_variables, non_snake_case)]
pub fn delete_elements(elements: *mut i32, discard: *const i32) {
    // WARNING: signature changed — verify body
    // Previous params: (elements : * mut i32, discard : * const i32)
    // Previous return: ()
    extern "C" { fn DeleteElements_impl (elements : * mut i32 , discard : * const i32) ; } unsafe { DeleteElements_impl (elements , discard) }
}

/// C: mjCModel::Delete (user/user_model.cc:1848)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_delete(self_ptr: *mut mjCModel, elements: *mut i32, discard: *const i32) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, elements : * mut i32, discard : * const i32)
    // Previous return: ()
    todo ! ()
}

/// C: getpathslength (user/user_model.cc:2128)
#[allow(unused_variables, non_snake_case)]
pub fn getpathslength(list: i32) -> usize {
    // WARNING: signature changed — verify body
    // Previous params: (list : i32)
    // Previous return: usize
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
pub fn addtolist(input: *const std__string, adr: i32, output_adr_field: *mut i32, output_buffer: *mut i8) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (input : * const std__string, adr : i32, output_adr_field : * mut i32, output_buffer : * mut i8)
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
pub fn changeframe(childpos: [f64; 3], childquat: [f64; 4], bodypos: [f64; 3], bodyquat: [f64; 4]) {
    // WARNING: signature changed — verify body
    // Previous params: (childpos : [f64 ; 3], childquat : [f64 ; 4], bodypos : [f64 ; 3], bodyquat : [f64 ; 4])
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::ReassignChild (user/user_model.cc:4355)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_reassign_child(self_ptr: *mut mjCModel, dest: *mut i32, list: *mut i32, parent: *mut mjCBody, body: *mut mjCBody) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, dest : * mut i32, list : * mut i32, parent : * mut mjCBody, body : * mut mjCBody)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::ResolveReferences (user/user_model.cc:4371)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_resolve_references(self_ptr: *mut mjCModel, list: *mut i32, body: *mut mjCBody) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, list : * mut i32, body : * mut mjCBody)
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
pub fn mj_c_model_process_list(self_ptr: *mut mjCModel, ids: *mut mjListKeyMap, list: *mut i32, r#type: mjtObj, checkrepeat: bool) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, ids : * mut mjListKeyMap, list : * mut i32, r#type : mjtObj, checkrepeat : bool)
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
pub fn compile_mesh(mesh: *mut mjCMesh, vfs: *const mjVFS, exception: *mut std__exception_ptr, exception_mutex: *mut std__mutex, warningtext: *mut string) {
    // WARNING: signature changed — verify body
    // Previous params: (mesh : * mut mjCMesh, vfs : * const mjVFS, exception : * mut std__exception_ptr, exception_mutex : * mut std__mutex, warningtext : * mut string)
    // Previous return: ()
    extern "C" { fn CompileMesh_impl (mesh : * mut mjCMesh , vfs : * const mjVFS , exception : * mut std__exception_ptr , exception_mutex : * mut std__mutex , warningtext : * mut string) ; } unsafe { CompileMesh_impl (mesh , vfs , exception , exception_mutex , warningtext) }
}

/// C: CompileTexture (user/user_model.cc:4790)
/// Calls: _mjPRIVATE_setTlsLogHandler, mjCTexture::Compile
#[allow(unused_variables, non_snake_case)]
pub fn compile_texture(texture: *mut mjCTexture, vfs: *const mjVFS, exception: *mut std__exception_ptr, exception_mutex: *mut std__mutex, warningtext: *mut string) {
    // WARNING: signature changed — verify body
    // Previous params: (texture : * mut mjCTexture, vfs : * const mjVFS, exception : * mut std__exception_ptr, exception_mutex : * mut std__mutex, warningtext : * mut string)
    // Previous return: ()
    extern "C" { fn CompileTexture_impl (texture : * mut mjCTexture , vfs : * const mjVFS , exception : * mut std__exception_ptr , exception_mutex : * mut std__mutex , warningtext : * mut string) ; } unsafe { CompileTexture_impl (texture , vfs , exception , exception_mutex , warningtext) }
}

/// C: PrintIndent (user/user_model.cc:5457)
#[allow(unused_variables, non_snake_case)]
pub fn print_indent(ss: *mut std__stringstream, depth: i32) {
    extern "C" { fn PrintIndent_impl(ss: *mut std__stringstream, depth: i32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { PrintIndent_impl(ss, depth) }
}

/// C: mjCModel::CopyFromSpec (user/user_model.h:191)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_from_spec(self_ptr: *mut mjCModel) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::PointToLocal (user/user_model.h:192)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_point_to_local(self_ptr: *mut mjCModel) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::Compile (user/user_model.h:203)
/// Calls: _mjPRIVATE_setTlsLogHandler, mjCModel::Clear, mjCModel::ClearCompileWarnings, mjCModel::CopyFromSpec, mjCModel::TryCompile, mj_deleteData, mj_deleteModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_compile(self_ptr: *mut mjCModel, vfs: *const mjVFS, m: *mut *mut mjModel) -> *mut mjModel {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, vfs : * const mjVFS, m : * mut * mut mjModel)
    // Previous return: * mut mjModel
    todo ! ()
}

/// C: mjCModel::CopyBack (user/user_model.h:204)
/// Calls: mjCMesh::GetPosPtr, mjCMesh::GetQuatPtr
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_back(self_ptr: *mut mjCModel, arg0: *const mjModel) -> bool {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, arg0 : * const mjModel)
    // Previous return: bool
    todo ! ()
}

/// C: mjCModel::FuseStatic (user/user_model.h:205)
/// Calls: mjCBody::ComputeBVH, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_fuse_static(self_ptr: *mut mjCModel) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::FuseReindex (user/user_model.h:206)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_fuse_reindex(self_ptr: *mut mjCModel, body: *mut mjCBody) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, body : * mut mjCBody)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::AddFlex (user/user_model.h:209)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_flex(self_ptr: *mut mjCModel) -> *mut mjCFlex {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * mut mjCFlex
    todo ! ()
}

/// C: mjCModel::AddMesh (user/user_model.h:210)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_mesh(self_ptr: *mut mjCModel, def: *mut mjCDef) -> *mut mjCMesh {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, def : * mut mjCDef)
    // Previous return: * mut mjCMesh
    todo ! ()
}

/// C: mjCModel::AddSkin (user/user_model.h:211)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_skin(self_ptr: *mut mjCModel) -> *mut mjCSkin {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * mut mjCSkin
    todo ! ()
}

/// C: mjCModel::AddHField (user/user_model.h:212)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_h_field(self_ptr: *mut mjCModel) -> *mut mjCHField {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * mut mjCHField
    todo ! ()
}

/// C: mjCModel::AddTexture (user/user_model.h:213)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_texture(self_ptr: *mut mjCModel) -> *mut mjCTexture {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * mut mjCTexture
    todo ! ()
}

/// C: mjCModel::AddMaterial (user/user_model.h:214)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_material(self_ptr: *mut mjCModel, def: *mut mjCDef) -> *mut mjCMaterial {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, def : * mut mjCDef)
    // Previous return: * mut mjCMaterial
    todo ! ()
}

/// C: mjCModel::AddPair (user/user_model.h:215)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_pair(self_ptr: *mut mjCModel, def: *mut mjCDef) -> *mut mjCPair {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, def : * mut mjCDef)
    // Previous return: * mut mjCPair
    todo ! ()
}

/// C: mjCModel::AddExclude (user/user_model.h:216)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_exclude(self_ptr: *mut mjCModel) -> *mut mjCBodyPair {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * mut mjCBodyPair
    todo ! ()
}

/// C: mjCModel::AddEquality (user/user_model.h:217)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_equality(self_ptr: *mut mjCModel, def: *mut mjCDef) -> *mut mjCEquality {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, def : * mut mjCDef)
    // Previous return: * mut mjCEquality
    todo ! ()
}

/// C: mjCModel::AddTendon (user/user_model.h:218)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_tendon(self_ptr: *mut mjCModel, def: *mut mjCDef) -> *mut mjCTendon {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, def : * mut mjCDef)
    // Previous return: * mut mjCTendon
    todo ! ()
}

/// C: mjCModel::AddActuator (user/user_model.h:219)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_actuator(self_ptr: *mut mjCModel, def: *mut mjCDef) -> *mut mjCActuator {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, def : * mut mjCDef)
    // Previous return: * mut mjCActuator
    todo ! ()
}

/// C: mjCModel::AddSensor (user/user_model.h:220)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_sensor(self_ptr: *mut mjCModel) -> *mut mjCSensor {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * mut mjCSensor
    todo ! ()
}

/// C: mjCModel::AddNumeric (user/user_model.h:221)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_numeric(self_ptr: *mut mjCModel) -> *mut mjCNumeric {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * mut mjCNumeric
    todo ! ()
}

/// C: mjCModel::AddText (user/user_model.h:222)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_text(self_ptr: *mut mjCModel) -> *mut mjCText {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * mut mjCText
    todo ! ()
}

/// C: mjCModel::AddTuple (user/user_model.h:223)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_tuple(self_ptr: *mut mjCModel) -> *mut mjCTuple {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * mut mjCTuple
    todo ! ()
}

/// C: mjCModel::AddKey (user/user_model.h:224)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_key(self_ptr: *mut mjCModel) -> *mut mjCKey {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * mut mjCKey
    todo ! ()
}

/// C: mjCModel::AddPlugin (user/user_model.h:225)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_plugin(self_ptr: *mut mjCModel) -> *mut mjCPlugin {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * mut mjCPlugin
    todo ! ()
}

/// C: mjCModel::AppendSpec (user/user_model.h:228)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_append_spec(self_ptr: *mut mjCModel, spec: *mut mjSpec, compiler: *const mjsCompiler) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, spec : * mut mjSpec, compiler : * const mjsCompiler)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::NumObjects (user/user_model.h:244)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_num_objects(self_ptr: *mut mjCModel, r#type: mjtObj) -> i32 {
    extern "C" { fn mjCModel_NumObjects_impl(self_ptr: *mut mjCModel, r#type: mjtObj) -> i32; }
    // SAFETY: delegates to C++ implementation, all pointers valid per caller contract
    unsafe { mjCModel_NumObjects_impl(self_ptr, r#type) }
}

/// C: mjCModel::GetObject (user/user_model.h:245)
/// Calls: mjCModel::NumObjects
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_object(self_ptr: *mut mjCModel, r#type: mjtObj, id: i32) -> *mut mjCBase {
    extern "C" { fn mjCModel_GetObject_impl(self_ptr: *mut mjCModel, r#type: mjtObj, id: i32) -> *mut mjCBase; }
    // SAFETY: delegates to C++ implementation; caller guarantees self_ptr is valid
    unsafe { mjCModel_GetObject_impl(self_ptr, r#type, id) }
}

/// C: mjCModel::NextObject (user/user_model.h:246)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_next_object(self_ptr: *mut mjCModel, object: *const mjsElement, r#type: mjtObj) -> *mut mjsElement {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, object : * const mjsElement, r#type : mjtObj)
    // Previous return: * mut mjsElement
    extern "C" { fn mjCModel_NextObject_impl (self_ptr : * mut mjCModel , object : * const mjsElement , r#type : mjtObj) -> * mut mjsElement ; } unsafe { mjCModel_NextObject_impl (self_ptr , object , r#type) }
}

/// C: mjCModel::IsCompiled (user/user_model.h:249)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_is_compiled(self_ptr: *mut mjCModel) -> bool {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: bool
    todo ! ()
}

/// C: mjCModel::GetError (user/user_model.h:250)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_error(self_ptr: *mut mjCModel) -> *const mjCError {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * const mjCError
    todo ! ()
}

/// C: mjCModel::SetError (user/user_model.h:251)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_set_error(self_ptr: *mut mjCModel, error: *const mjCError) {
    extern "C" { fn mjCModel_SetError_impl(self_ptr: *mut mjCModel, error: *const mjCError); }
    // SAFETY: delegates to C++ implementation, all pointers valid per caller contract
    unsafe { mjCModel_SetError_impl(self_ptr, error) }
}

/// C: mjCModel::AddWarning (user/user_model.h:252)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_warning(self_ptr: *mut mjCModel, msg: string, obj: *const mjCBase) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, msg : string, obj : * const mjCBase)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::AddGroupedWarning (user/user_model.h:254)
/// Calls: mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_grouped_warning(self_ptr: *mut mjCModel, subject: *const std__string, body: *const std__string) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, subject : * const std__string, body : * const std__string)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::GetWarnings (user/user_model.h:256)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_warnings(self_ptr: *mut mjCModel) -> *const i32 {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * const i32
    todo ! ()
}

/// C: mjCModel::ClearWarnings (user/user_model.h:260)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_clear_warnings(self_ptr: *mut mjCModel) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::ClearCompileWarnings (user/user_model.h:264)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_clear_compile_warnings(self_ptr: *mut mjCModel) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: ()
    extern "C" { fn mjCModel_ClearCompileWarnings_impl(self_ptr: *mut mjCModel); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjCModel_ClearCompileWarnings_impl(self_ptr) }
}

/// C: mjCModel::SetAttachWarningBoundary (user/user_model.h:267)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_set_attach_warning_boundary(self_ptr: *mut mjCModel) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::GetWorld (user/user_model.h:271)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_world(self_ptr: *mut mjCModel) -> *mut mjCBody {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * mut mjCBody
    extern "C" { fn mjCModel_GetWorld_impl(self_ptr: *mut mjCModel) -> *mut mjCBody; }
    // SAFETY: delegates to C++ implementation; caller guarantees self_ptr is valid
    unsafe { mjCModel_GetWorld_impl(self_ptr) }
}

/// C: mjCModel::FindDefault (user/user_model.h:272)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_find_default(self_ptr: *mut mjCModel, name: *const std__string) -> *mut mjCDef {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, name : * const std__string)
    // Previous return: * mut mjCDef
    todo ! ()
}

/// C: mjCModel::AddDefault (user/user_model.h:273)
/// Calls: mjCDef::CopyFromSpec, mjCDef::CopyWithoutChildren
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_default(self_ptr: *mut mjCModel, name: string, parent: *mut mjCDef) -> *mut mjCDef {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, name : string, parent : * mut mjCDef)
    // Previous return: * mut mjCDef
    todo ! ()
}

/// C: mjCModel::FindObject (user/user_model.h:274)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_find_object(self_ptr: *mut mjCModel, r#type: mjtObj, name: string) -> *mut mjCBase {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, r#type : mjtObj, name : string)
    // Previous return: * mut mjCBase
    todo ! ()
}

/// C: mjCModel::FindTree (user/user_model.h:275)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_find_tree(self_ptr: *mut mjCModel, body: *mut mjCBody, r#type: mjtObj, name: string) -> *mut mjCBase {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, body : * mut mjCBody, r#type : mjtObj, name : string)
    // Previous return: * mut mjCBase
    todo ! ()
}

/// C: mjCModel::FindSpec (user/user_model.h:276)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_find_spec(self_ptr: *mut mjCModel, name: string) -> *mut mjSpec {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, name : string)
    // Previous return: * mut mjSpec
    extern "C" { fn mjCModel_FindSpec_impl (self_ptr : * mut mjCModel , name : string) -> * mut mjSpec ; } unsafe { mjCModel_FindSpec_impl (self_ptr , name) }
}

/// C: mjCModel::ActivatePlugin (user/user_model.h:278)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_activate_plugin(self_ptr: *mut mjCModel, plugin: *const mjpPlugin, slot: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, plugin : * const mjpPlugin, slot : i32)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::get_meshdir (user/user_model.h:285)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_meshdir(self_ptr: *mut mjCModel) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: i32
    todo ! ()
}

/// C: mjCModel::get_texturedir (user/user_model.h:286)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_texturedir(self_ptr: *mut mjCModel) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: i32
    todo ! ()
}

/// C: mjCModel::Default (user/user_model.h:288)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_default(self_ptr: *mut mjCModel) -> *mut mjCDef {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * mut mjCDef
    extern "C" { fn mjCModel_Default_impl(self_ptr: *mut mjCModel) -> *mut mjCDef; }
    // SAFETY: delegates to C++ implementation; caller guarantees self_ptr is valid
    unsafe { mjCModel_Default_impl(self_ptr) }
}

/// C: mjCModel::NumDefaults (user/user_model.h:289)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_num_defaults(self_ptr: *mut mjCModel) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: i32
    todo ! ()
}

/// C: mjCModel::ActivePlugins (user/user_model.h:291)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_active_plugins(self_ptr: *mut mjCModel) -> *const i32 {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * const i32
    todo ! ()
}

/// C: mjCModel::Flexes (user/user_model.h:295)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_flexes(self_ptr: *mut mjCModel) -> *const i32 {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * const i32
    todo ! ()
}

/// C: mjCModel::Meshes (user/user_model.h:296)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_meshes(self_ptr: *mut mjCModel) -> *const i32 {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * const i32
    todo ! ()
}

/// C: mjCModel::Skins (user/user_model.h:297)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_skins(self_ptr: *mut mjCModel) -> *const i32 {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * const i32
    todo ! ()
}

/// C: mjCModel::HFields (user/user_model.h:298)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_h_fields(self_ptr: *mut mjCModel) -> *const i32 {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * const i32
    todo ! ()
}

/// C: mjCModel::Textures (user/user_model.h:299)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_textures(self_ptr: *mut mjCModel) -> *const i32 {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * const i32
    todo ! ()
}

/// C: mjCModel::Materials (user/user_model.h:300)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_materials(self_ptr: *mut mjCModel) -> *const i32 {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * const i32
    todo ! ()
}

/// C: mjCModel::Pairs (user/user_model.h:301)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_pairs(self_ptr: *mut mjCModel) -> *const i32 {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * const i32
    todo ! ()
}

/// C: mjCModel::Excludes (user/user_model.h:302)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_excludes(self_ptr: *mut mjCModel) -> *const i32 {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * const i32
    todo ! ()
}

/// C: mjCModel::Equalities (user/user_model.h:303)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_equalities(self_ptr: *mut mjCModel) -> *const i32 {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * const i32
    todo ! ()
}

/// C: mjCModel::Tendons (user/user_model.h:304)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_tendons(self_ptr: *mut mjCModel) -> *const i32 {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * const i32
    todo ! ()
}

/// C: mjCModel::Actuators (user/user_model.h:305)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_actuators(self_ptr: *mut mjCModel) -> *const i32 {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * const i32
    todo ! ()
}

/// C: mjCModel::Sensors (user/user_model.h:306)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_sensors(self_ptr: *mut mjCModel) -> *const i32 {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * const i32
    todo ! ()
}

/// C: mjCModel::Numerics (user/user_model.h:307)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_numerics(self_ptr: *mut mjCModel) -> *const i32 {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * const i32
    todo ! ()
}

/// C: mjCModel::Texts (user/user_model.h:308)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_texts(self_ptr: *mut mjCModel) -> *const i32 {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * const i32
    todo ! ()
}

/// C: mjCModel::Tuples (user/user_model.h:309)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_tuples(self_ptr: *mut mjCModel) -> *const i32 {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * const i32
    todo ! ()
}

/// C: mjCModel::Keys (user/user_model.h:310)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_keys(self_ptr: *mut mjCModel) -> *const i32 {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * const i32
    todo ! ()
}

/// C: mjCModel::Plugins (user/user_model.h:311)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_plugins(self_ptr: *mut mjCModel) -> *const i32 {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * const i32
    todo ! ()
}

/// C: mjCModel::Bodies (user/user_model.h:312)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_bodies(self_ptr: *mut mjCModel) -> *const i32 {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * const i32
    todo ! ()
}

/// C: mjCModel::Geoms (user/user_model.h:313)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_geoms(self_ptr: *mut mjCModel) -> *const i32 {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * const i32
    todo ! ()
}

/// C: mjCModel::ResolvePlugin (user/user_model.h:316)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_resolve_plugin(self_ptr: *mut mjCModel, obj: *mut mjCBase, plugin_name: *const std__string, plugin_instance_name: *const std__string, plugin_instance: *mut *mut mjCPlugin) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, obj : * mut mjCBase, plugin_name : * const std__string, plugin_instance_name : * const std__string, plugin_instance : * mut * mut mjCPlugin)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::Clear (user/user_model.h:321)
/// Calls: mjCModel::ClearCompileWarnings
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_clear(self_ptr: *mut mjCModel) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::SaveState (user/user_model.h:329)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_save_state(self_ptr: *mut mjCModel, state_name: *const std__string, qpos: *const T, qvel: *const T, act: *const T, ctrl: *const T, mpos: *const T, mquat: *const T) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, state_name : * const std__string, qpos : * const T, qvel : * const T, act : * const T, ctrl : * const T, mpos : * const T, mquat : * const T)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::RestoreState (user/user_model.h:334)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_restore_state(self_ptr: *mut mjCModel, state_name: *const std__string, pos0: *const f64, mpos0: *const f64, mquat0: *const f64, qpos: *mut T, qvel: *mut T, act: *mut T, ctrl: *mut T, mpos: *mut T, mquat: *mut T) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, state_name : * const std__string, pos0 : * const f64, mpos0 : * const f64, mquat0 : * const f64, qpos : * mut T, qvel : * mut T, act : * mut T, ctrl : * mut T, mpos : * mut T, mquat : * mut T)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::MakeData (user/user_model.h:338)
/// Calls: mj_initPlugin, mj_makeRawData, mj_resetData
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_make_data(self_ptr: *mut mjCModel, m: *const mjModel, dest: *mut *mut mjData) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, m : * const mjModel, dest : * mut * mut mjData)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::StoreKeyframes (user/user_model.h:341)
/// Calls: mjCModel::ComputeReference, mjCModel::SaveDofOffsets
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_store_keyframes(self_ptr: *mut mjCModel, dest: *mut mjCModel) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, dest : * mut mjCModel)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::SetDeepCopy (user/user_model.h:347)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_set_deep_copy(self_ptr: *mut mjCModel, deepcopy: bool) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, deepcopy : bool)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::SetAttached (user/user_model.h:350)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_set_attached(self_ptr: *mut mjCModel, deepcopy: bool) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, deepcopy : bool)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::IsAttached (user/user_model.h:353)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_is_attached(self_ptr: *mut mjCModel) -> bool {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: bool
    todo ! ()
}

/// C: mjCModel::CheckRepeat (user/user_model.h:356)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_check_repeat(self_ptr: *mut mjCModel, r#type: mjtObj) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, r#type : mjtObj)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::AddRef (user/user_model.h:359)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_ref(self_ptr: *mut mjCModel) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::GetRef (user/user_model.h:360)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_ref(self_ptr: *mut mjCModel) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: i32
    todo ! ()
}

/// C: mjCModel::Release (user/user_model.h:361)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_release(self_ptr: *mut mjCModel) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: ()
    extern "C" { fn mjCModel_Release_impl (self_ptr : * mut mjCModel) ; } unsafe { mjCModel_Release_impl (self_ptr) }
}

/// C: mjCModel::MakeTreeLists (user/user_model.h:377)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_make_tree_lists(self_ptr: *mut mjCModel, body: *mut mjCBody) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, body : * mut mjCBody)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::TryCompile (user/user_model.h:380)
/// Calls: mjCModel::AddKey, mjCModel::AddWarning, mjCModel::AutoSpringDamper, mjCModel::CheckEmptyNames, mjCModel::ClearCompileWarnings, mjCModel::CompileMeshesAndTextures, mjCModel::ComputeSparseSizes, mjCModel::CopyNames, mjCModel::CopyObjects, mjCModel::CopyPaths, mjCModel::CopyPlugins, mjCModel::CopyTree, mjCModel::ExpandAllKeyframes, mjCModel::FinalizeSimple, mjCModel::FuseStatic, mjCModel::IndexAssets, mjCModel::LengthRange, mjCModel::ProcessLists, mjCModel::ResolveKeyframes, mjCModel::SaveDofOffsets, mjCModel::SetNuser, mjCModel::SetSizes, mjCModel::Signature, mj_deleteData, mj_makeData, mj_makeRawData, mj_normalizeQuat, mj_resetData, mj_setConst, mj_step, mj_validateReferences, mjuu_defined
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_try_compile(self_ptr: *mut mjCModel, m: *mut *mut mjModel, d: *mut *mut mjData, vfs: *const mjVFS) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, m : * mut * mut mjModel, d : * mut * mut mjData, vfs : * const mjVFS)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::CompileMeshesAndTextures (user/user_model.h:381)
/// Calls: NumCompilerThreads, ThreadPool::WaitCount
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_compile_meshes_and_textures(self_ptr: *mut mjCModel, vfs: *const mjVFS) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, vfs : * const mjVFS)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::SetNuser (user/user_model.h:383)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_set_nuser(self_ptr: *mut mjCModel) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::IndexAssets (user/user_model.h:384)
/// Calls: mjCGeom::IsVisual, mjCGeom::get_hfieldname, mjCGeom::get_material, mjCGeom::get_meshname, mjCMesh::IsVisual, mjCMesh::SetNotVisual, mjCModel::FindObject, mjCSite::get_material
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_index_assets(self_ptr: *mut mjCModel, discard: bool) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, discard : bool)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::CheckEmptyNames (user/user_model.h:385)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_check_empty_names(self_ptr: *mut mjCModel) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::SetSizes (user/user_model.h:386)
/// Calls: mjCBody::GetParent
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_set_sizes(self_ptr: *mut mjCModel) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::ComputeSparseSizes (user/user_model.h:387)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_compute_sparse_sizes(self_ptr: *mut mjCModel) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::AutoSpringDamper (user/user_model.h:388)
/// Calls: mjCJoint::nv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_auto_spring_damper(self_ptr: *mut mjCModel, arg0: *mut mjModel) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, arg0 : * mut mjModel)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::LengthRange (user/user_model.h:389)
/// Calls: NumCompilerThreads, ThreadPool::WaitCount, mj_makeData
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_length_range(self_ptr: *mut mjCModel, arg0: *mut mjModel, arg1: *mut mjData) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, arg0 : * mut mjModel, arg1 : * mut mjData)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::CopyNames (user/user_model.h:390)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_names(self_ptr: *mut mjCModel, arg0: *mut mjModel) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, arg0 : * mut mjModel)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::CopyPaths (user/user_model.h:391)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_paths(self_ptr: *mut mjCModel, arg0: *mut mjModel) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, arg0 : * mut mjModel)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::CopyObjects (user/user_model.h:392)
/// Calls: mjCActuator::is_actlimited, mjCActuator::is_ctrllimited, mjCActuator::is_forcelimited, mjCBoundingVolumeHierarchy::Nbvh, mjCBoundingVolumeHierarchy::Nodeid, mjCMesh::CopyFace, mjCMesh::CopyFaceNormal, mjCMesh::CopyFaceTexcoord, mjCMesh::CopyGraph, mjCMesh::CopyNormal, mjCMesh::CopyPolygonMap, mjCMesh::CopyPolygonNormals, mjCMesh::CopyPolygons, mjCMesh::CopyTexcoord, mjCMesh::CopyVert, mjCMesh::GetPosPtr, mjCMesh::GetQuatPtr, mjCMesh::HasTexcoord, mjCMesh::Scale, mjCMesh::nface, mjCMesh::nnormal, mjCMesh::npolygon, mjCMesh::npolygonmap, mjCMesh::npolygonvert, mjCMesh::ntexcoord, mjCMesh::nvert, mjCMesh::octree, mjCMesh::szgraph, mjCMesh::tree, mjCModel::AddWarning, mjCOctree::CopyAabb, mjCOctree::CopyChild, mjCOctree::CopyCoeff, mjCOctree::CopyLevel, mjCOctree::NumNodes, mjCTendon::is_actfrclimited, mjCTendon::is_limited
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_objects(self_ptr: *mut mjCModel, arg0: *mut mjModel) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, arg0 : * mut mjModel)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::CopyTree (user/user_model.h:393)
/// Calls: mjCGeom::GetRBound, mjCJoint::is_actfrclimited, mjCJoint::is_limited, mjCJoint::nq, mjCJoint::nv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_tree(self_ptr: *mut mjCModel, arg0: *mut mjModel) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, arg0 : * mut mjModel)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::FinalizeSimple (user/user_model.h:394)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_finalize_simple(self_ptr: *mut mjCModel, m: *mut mjModel) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, m : * mut mjModel)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::CopyPlugins (user/user_model.h:395)
/// Calls: mjp_getPluginAtSlot, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_plugins(self_ptr: *mut mjCModel, arg0: *mut mjModel) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, arg0 : * mut mjModel)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::CountTendonDofs (user/user_model.h:396)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_count_tendon_dofs(self_ptr: *mut mjCModel, m: *const mjModel, id: i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, m : * const mjModel, id : i32)
    // Previous return: i32
    todo ! ()
}

/// C: mjCModel::CountNJmom (user/user_model.h:398)
/// Calls: mjCModel::CountTendonDofs
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_count_n_jmom(self_ptr: *mut mjCModel, m: *const mjModel) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, m : * const mjModel)
    // Previous return: i32
    todo ! ()
}

/// C: mjCModel::CountNJten (user/user_model.h:399)
/// Calls: mjCModel::CountTendonDofs
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_count_n_jten(self_ptr: *mut mjCModel, m: *const mjModel) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, m : * const mjModel)
    // Previous return: i32
    todo ! ()
}

/// C: mjCModel::RemovePlugins (user/user_model.h:402)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_remove_plugins(self_ptr: *mut mjCModel) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::CopyExplicitPlugin (user/user_model.h:448)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_explicit_plugin(self_ptr: *mut mjCModel, obj: *mut T) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, obj : * mut T)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::CreateObjectLists (user/user_model.h:458)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_create_object_lists(self_ptr: *mut mjCModel) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::ProcessLists (user/user_model.h:461)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_process_lists(self_ptr: *mut mjCModel, checkrepeat: bool) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, checkrepeat : bool)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::ResetTreeLists (user/user_model.h:468)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_reset_tree_lists(self_ptr: *mut mjCModel) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::SaveDofOffsets (user/user_model.h:471)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_save_dof_offsets(self_ptr: *mut mjCModel, computesize: bool) {
    extern "C" { fn mjCModel_SaveDofOffsets_impl(self_ptr: *mut mjCModel, computesize: bool); }
    // SAFETY: delegates to C++ implementation; caller guarantees self_ptr is valid
    unsafe { mjCModel_SaveDofOffsets_impl(self_ptr, computesize) }
}

/// C: mjCModel::ResolveKeyframes (user/user_model.h:474)
/// Calls: mjCModel::SaveDofOffsets
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_resolve_keyframes(self_ptr: *mut mjCModel, m: *const mjModel) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, m : * const mjModel)
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
pub fn mj_c_model_expand_keyframe(self_ptr: *mut mjCModel, key: *mut mjCKey, qpos0_: *const f64, bpos: *const f64, bquat: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, key : * mut mjCKey, qpos0_ : * const f64, bpos : * const f64, bquat : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::ComputeReference (user/user_model.h:480)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_compute_reference(self_ptr: *mut mjCModel) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::CheckBodyMassInertia (user/user_model.h:483)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_check_body_mass_inertia(self_ptr: *mut mjCModel, body: *mut mjCBody) -> bool {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, body : * mut mjCBody)
    // Previous return: bool
    todo ! ()
}

/// C: mjCModel::PrintTree (user/user_model.h:491)
/// Calls: PrintIndent
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_print_tree(self_ptr: *mut mjCModel, tree: *mut std__stringstream, body: *const mjCBody, depth: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, tree : * mut std__stringstream, body : * const mjCBody, depth : i32)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::Signature (user/user_model.h:494)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_signature(self_ptr: *mut mjCModel) -> u64 {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: u64
    todo ! ()
}

/// C: mjCModel::DeleteSubtreePlugin (user/user_model.h:505)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_delete_subtree_plugin(self_ptr: *mut mjCModel, subtree: *mut mjCBody) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel, subtree : * mut mjCBody)
    // Previous return: ()
    todo ! ()
}

/// C: mjCModel::ExpandAllKeyframes (user/user_model.h:508)
/// Calls: mjCModel::ComputeReference, mjCModel::SaveDofOffsets
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_expand_all_keyframes(self_ptr: *mut mjCModel) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: ()
    todo ! ()
}

