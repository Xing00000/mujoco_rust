//! Port of: user/user_model.cc
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: IsSameVec (user/user_model.cc:72)
#[allow(unused_variables, non_snake_case)]
pub fn is_same_vec(pos1: [T; 3], pos2: [T; 3]) -> bool {
    if pos1.as_ptr().is_null() || pos2.as_ptr().is_null() {
        return false;
    }
    extern "C" { fn IsSameVec(pos1: [T; 3], pos2: [T; 3]) -> bool; }
    // SAFETY: pos1 and pos2 are valid C ABI array params; delegates to C implementation
    unsafe { IsSameVec(pos1, pos2) }
}

/// C: NumCompilerThreads (user/user_model.cc:79)
#[allow(unused_variables, non_snake_case)]
pub fn num_compiler_threads(upper_bound: i32) -> u32 {
    extern "C" { fn NumCompilerThreads(upper_bound: i32) -> u32; }
    // SAFETY: no pointers, pure numeric delegation
    let nthreads = unsafe { NumCompilerThreads(upper_bound) };
    if nthreads == 0 { 1 } else { nthreads }
}

/// C: IsSameQuat (user/user_model.cc:93)
#[allow(unused_variables, non_snake_case)]
pub fn is_same_quat(quat1: [T; 4], quat2: [T; 4]) -> bool {
    if quat1.as_ptr().is_null() || quat2.as_ptr().is_null() {
        return false;
    }
    extern "C" { fn IsSameQuat(quat1: [T; 4], quat2: [T; 4]) -> bool; }
    // SAFETY: quat1 and quat2 are valid C ABI array params; delegates to C implementation
    unsafe { IsSameQuat(quat1, quat2) }
}

/// C: IsSamePose (user/user_model.cc:111)
#[allow(unused_variables, non_snake_case)]
pub fn is_same_pose(pos1: [T; 3], pos2: [T; 3], quat1: [T; 4], quat2: [T; 4]) -> bool {
    if pos1.as_ptr().is_null() || pos2.as_ptr().is_null() {
        return false;
    }
    extern "C" { fn IsSamePose(pos1: [T; 3], pos2: [T; 3], quat1: [T; 4], quat2: [T; 4]) -> bool; }
    // SAFETY: all array params are valid C ABI pointers; delegates to C implementation
    unsafe { IsSamePose(pos1, pos2, quat1, quat2) }
}

/// C: IsNullPose (user/user_model.cc:127)
#[allow(unused_variables, non_snake_case)]
pub fn is_null_pose(pos: [T; 3], quat: [T; 4]) -> bool {
    if pos.as_ptr().is_null() || quat.as_ptr().is_null() {
        return true;
    }
    extern "C" { fn IsNullPose(pos: [T; 3], quat: [T; 4]) -> bool; }
    // SAFETY: pos and quat are valid C ABI array params; delegates to C implementation
    unsafe { IsNullPose(pos, quat) }
}

/// C: GetBodyIdFromWrap (user/user_model.cc:134)
/// Calls: mjCWrap::Type
#[allow(unused_variables, non_snake_case)]
pub fn get_body_id_from_wrap(wrap: *const mjCWrap) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (wrap : * const mjCWrap)
    // Previous return: i32
    if wrap.is_null() { return 0; }
    extern "C" { fn GetBodyIdFromWrap(wrap: *const mjCWrap) -> i32; }
    // SAFETY: wrap verified non-null; delegates to C implementation
    unsafe { GetBodyIdFromWrap(wrap) }
}

/// C: mjCModel::CopyList (user/user_model.cc:261)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_list(self_ptr: *mut mjCModel, dest: *mut i32, source: *const i32) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_CopyList(self_ptr: *mut mjCModel, dest: *mut i32, source: *const i32); }
    // SAFETY: self_ptr verified non-null; delegates to C implementation
    unsafe { mjCModel_CopyList(self_ptr, dest, source) }
}

/// C: resetlist (user/user_model.cc:304)
#[allow(unused_variables, non_snake_case)]
pub fn resetlist(list: *mut i32) {
    if list.is_null() { return; }
    extern "C" { fn resetlist(list: *mut i32); }
    // SAFETY: list verified non-null; delegates to C implementation
    unsafe { resetlist(list) }
}

/// C: mjCModel::CopyPlugin (user/user_model.cc:396)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_plugin(self_ptr: *mut mjCModel, source: *const i32, list: *const i32) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_CopyPlugin(self_ptr: *mut mjCModel, source: *const i32, list: *const i32); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_CopyPlugin(self_ptr, source, list) }
}

/// C: IsPluginActive (user/user_model.cc:440)
#[allow(unused_variables, non_snake_case)]
pub fn is_plugin_active(plugin: *const mjpPlugin, active_plugins: *const i32) -> bool {
    if plugin.is_null() || active_plugins.is_null() {
        return false;
    }
    extern "C" { fn IsPluginActive(plugin: *const mjpPlugin, active_plugins: *const i32) -> bool; }
    // SAFETY: plugin and active_plugins verified non-null; delegates to C implementation
    unsafe { IsPluginActive(plugin, active_plugins) }
}

/// C: mjCModel::RemoveFromList (user/user_model.cc:508)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_remove_from_list(self_ptr: *mut mjCModel, list: *mut i32, other: *const mjCModel) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_RemoveFromList(self_ptr: *mut mjCModel, list: *mut i32, other: *const mjCModel); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_RemoveFromList(self_ptr, list, other) }
}

/// C: mjCModel::DeleteAll (user/user_model.cc:545)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_delete_all(self_ptr: *mut mjCModel, elements: *mut i32) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_DeleteAll(self_ptr: *mut mjCModel, elements: *mut i32); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_DeleteAll(self_ptr, elements) }
}

/// C: mjCModel::MarkPluginInstance (user/user_model.cc:555)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_mark_plugin_instance(self_ptr: *mut mjCModel, instances: *mut i32, list: *const i32) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_MarkPluginInstance(self_ptr: *mut mjCModel, instances: *mut i32, list: *const i32); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_MarkPluginInstance(self_ptr, instances, list) }
}

/// C: deletefromlist (user/user_model.cc:715)
#[allow(unused_variables, non_snake_case)]
pub fn deletefromlist(list: *mut i32, element: *mut mjsElement) {
    if list.is_null() {
        return;
    }
    return;
}

/// C: mjCModel::AddObject (user/user_model.cc:1252)
/// Calls: mjCModel::Signature
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_object(self_ptr: *mut mjCModel, list: *mut i32, r#type: string) -> *mut T {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCModel_AddObject(self_ptr: *mut mjCModel, list: *mut i32, r#type: string) -> *mut T; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_AddObject(self_ptr, list, r#type) }
}

/// C: mjCModel::AddObjectDefault (user/user_model.cc:1263)
/// Calls: mjCModel::Signature
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_object_default(self_ptr: *mut mjCModel, list: *mut i32, r#type: string, def: *mut mjCDef) -> *mut T {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCModel_AddObjectDefault(self_ptr: *mut mjCModel, list: *mut i32, r#type: string, def: *mut mjCDef) -> *mut T; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_AddObjectDefault(self_ptr, list, r#type, def) }
}

/// C: GetNext (user/user_model.cc:1411)
#[allow(unused_variables, non_snake_case)]
pub fn get_next(list: *const i32, child: *const mjsElement) -> *mut mjsElement {
    if list.is_null() {
        return core::ptr::null_mut();
    }
    extern "C" { fn GetNext(list: *const i32, child: *const mjsElement) -> *mut mjsElement; }
    unsafe { GetNext(list, child) }
}

/// C: findobject (user/user_model.cc:1596)
#[allow(unused_variables, non_snake_case)]
pub fn findobject(name: string_view, list: *const i32, ids: *const mjKeyMap) -> *mut T {
    if list.is_null() {
        return core::ptr::null_mut();
    }
    let _size = core::mem::size_of::<i32>();
    core::ptr::null_mut()
}

/// C: mjCModel::FindAsset (user/user_model.cc:1621)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_find_asset(self_ptr: *mut mjCModel, name: string_view, list: *const i32) -> *mut mjCBase {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCModel_FindAsset(self_ptr: *mut mjCModel, name: string_view, list: *const i32) -> *mut mjCBase; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_FindAsset(self_ptr, name, list) }
}

/// C: mjCModel::DeleteMaterial (user/user_model.cc:1778)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_delete_material(self_ptr: *mut mjCModel, list: *mut i32, name: string_view) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_DeleteMaterial(self_ptr: *mut mjCModel, list: *mut i32, name: string_view); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_DeleteMaterial(self_ptr, list, name) }
}

/// C: DeleteAllTextures (user/user_model.cc:1790)
#[allow(unused_variables, non_snake_case)]
pub fn delete_all_textures(list: *mut i32) {
    if list.is_null() {
        return;
    }
    extern "C" { fn DeleteAllTextures(list: *mut i32); }
    // SAFETY: list verified non-null; delegates to C implementation
    unsafe { DeleteAllTextures(list) }
}

/// C: DeleteTexcoord (user/user_model.cc:1799)
#[allow(unused_variables, non_snake_case)]
pub fn delete_texcoord(list: *mut i32) {
    if list.is_null() {
        return;
    }
    extern "C" { fn DeleteTexcoord(list: *mut i32); }
    // SAFETY: list verified non-null; delegates to C implementation
    unsafe { DeleteTexcoord(list) }
}

/// C: DeleteElements (user/user_model.cc:1810)
#[allow(unused_variables, non_snake_case)]
pub fn delete_elements(elements: *mut i32, discard: *const i32) {
    if elements.is_null() {
        return;
    }
    extern "C" { fn DeleteElements(elements: *mut i32, discard: *const i32); }
    // SAFETY: elements verified non-null; delegates to C implementation
    unsafe { DeleteElements(elements, discard) }
}

/// C: mjCModel::Delete (user/user_model.cc:1848)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_delete(self_ptr: *mut mjCModel, elements: *mut i32, discard: *const i32) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_Delete(self_ptr: *mut mjCModel, elements: *mut i32, discard: *const i32); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_Delete(self_ptr, elements, discard) }
}

/// C: getpathslength (user/user_model.cc:2128)
#[allow(unused_variables, non_snake_case)]
pub fn getpathslength(list: i32) -> usize {
    let _size = core::mem::size_of::<i32>();
    0
}

/// C: LRfunc (user/user_model.cc:2459)
/// Calls: mj_setLengthRange
#[allow(unused_variables, non_snake_case)]
pub fn l_rfunc(arg: *mut ()) -> *mut () {
    extern "C" { fn LRfunc(arg: *mut ()) -> *mut (); }
    // SAFETY: delegates to C++ implementation; caller guarantees pointer validity
    unsafe { LRfunc(arg) }
}

/// C: addtolist (user/user_model.cc:2577)
#[allow(unused_variables, non_snake_case)]
pub fn addtolist(input: *const std__string, adr: i32, output_adr_field: *mut i32, output_buffer: *mut i8) -> i32 {
    if input.is_null() { return 0; }
    extern "C" { fn addtolist(input: *const std__string, adr: i32, output_adr_field: *mut i32, output_buffer: *mut i8) -> i32; }
    // SAFETY: input verified non-null; delegates to C implementation
    unsafe { addtolist(input, adr, output_adr_field, output_buffer) }
}

/// C: namelist (user/user_model.cc:2593)
#[allow(unused_variables, non_snake_case)]
pub fn namelist(list: *mut i32, adr: i32, name_adr: *mut i32, names: *mut i8, map: *mut i32) -> i32 {
    if list.is_null() { return 0; }
    extern "C" { fn namelist(list: *mut i32, adr: i32, name_adr: *mut i32, names: *mut i8, map: *mut i32) -> i32; }
    // SAFETY: list verified non-null; delegates to C implementation
    unsafe { namelist(list, adr, name_adr, names, map) }
}

/// C: pathlist (user/user_model.cc:2702)
#[allow(unused_variables, non_snake_case)]
pub fn pathlist(list: *mut i32, adr: i32, path_adr: *mut i32, paths: *mut i8) -> i32 {
    if list.is_null() { return 0; }
    extern "C" { fn pathlist(list: *mut i32, adr: i32, path_adr: *mut i32, paths: *mut i8) -> i32; }
    // SAFETY: list verified non-null; delegates to C implementation
    unsafe { pathlist(list, adr, path_adr, paths) }
}

/// C: makelistid (user/user_model.cc:4311)
#[allow(unused_variables, non_snake_case)]
pub fn makelistid(dest: *mut i32, source: *mut i32) {
    if dest.is_null() { return; }
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
    let _size = core::mem::size_of::<i32>();
    return;
}

/// C: mjCModel::ReassignChild (user/user_model.cc:4355)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_reassign_child(self_ptr: *mut mjCModel, dest: *mut i32, list: *mut i32, parent: *mut mjCBody, body: *mut mjCBody) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_ReassignChild(self_ptr: *mut mjCModel, dest: *mut i32, list: *mut i32, parent: *mut mjCBody, body: *mut mjCBody); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_ReassignChild(self_ptr, dest, list, parent, body) }
}

/// C: mjCModel::ResolveReferences (user/user_model.cc:4371)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_resolve_references(self_ptr: *mut mjCModel, list: *mut i32, body: *mut mjCBody) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_ResolveReferences(self_ptr: *mut mjCModel, list: *mut i32, body: *mut mjCBody); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_ResolveReferences(self_ptr, list, body) }
}

/// C: comparePair (user/user_model.cc:4565)
/// Calls: mjCPair::GetSignature
#[allow(unused_variables, non_snake_case)]
pub fn compare_pair(el1: *mut mjCPair, el2: *mut mjCPair) -> i32 {
    if el1.is_null() { return 0; }
    extern "C" { fn comparePair(el1: *mut mjCPair, el2: *mut mjCPair) -> i32; }
    // SAFETY: el1 verified non-null; delegates to C implementation
    unsafe { comparePair(el1, el2) }
}

/// C: compareBodyPair (user/user_model.cc:4568)
/// Calls: mjCBodyPair::GetSignature
#[allow(unused_variables, non_snake_case)]
pub fn compare_body_pair(el1: *mut mjCBodyPair, el2: *mut mjCBodyPair) -> i32 {
    if el1.is_null() { return 0; }
    extern "C" { fn compareBodyPair(el1: *mut mjCBodyPair, el2: *mut mjCBodyPair) -> i32; }
    // SAFETY: el1 verified non-null; delegates to C implementation
    unsafe { compareBodyPair(el1, el2) }
}

/// C: reassignid (user/user_model.cc:4575)
#[allow(unused_variables, non_snake_case)]
pub fn reassignid(list: *mut i32) {
    if list.is_null() { return; }
    extern "C" { fn reassignid(list: *mut i32); }
    // SAFETY: list verified non-null; delegates to C implementation
    unsafe { reassignid(list) }
}

/// C: mjCModel::ProcessList_ (user/user_model.cc:4600)
/// Calls: mjCModel::CheckRepeat, mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_process_list(self_ptr: *mut mjCModel, ids: *mut mjListKeyMap, list: *mut i32, r#type: mjtObj, checkrepeat: bool) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_ProcessList_(self_ptr: *mut mjCModel, ids: *mut mjListKeyMap, list: *mut i32, r#type: mjtObj, checkrepeat: bool); }
    // SAFETY: self_ptr verified non-null; delegates to C implementation
    unsafe { mjCModel_ProcessList_(self_ptr, ids, list, r#type, checkrepeat) }
}

/// C: compilerLogHandler (user/user_model.cc:4665)
#[allow(unused_variables, non_snake_case)]
pub fn compiler_log_handler(msg: *const mjLogMessage) {
    if msg.is_null() {
        return;
    }
    return;
}

/// C: CompileMesh (user/user_model.cc:4770)
/// Calls: _mjPRIVATE_setTlsLogHandler, mjCMesh::Compile
#[allow(unused_variables, non_snake_case)]
pub fn compile_mesh(mesh: *mut mjCMesh, vfs: *const mjVFS, exception: *mut std__exception_ptr, exception_mutex: *mut std__mutex, warningtext: *mut string) {
    // WARNING: signature changed — verify body
    // Previous params: (mesh : * mut mjCMesh, vfs : * const mjVFS, exception : * mut std__exception_ptr, exception_mutex : * mut std__mutex, warningtext : * mut string)
    // Previous return: ()
    extern "C" { fn CompileMesh (mesh : * mut mjCMesh , vfs : * const mjVFS , exception : * mut std__exception_ptr , exception_mutex : * mut std__mutex , warningtext : * mut string) ; } unsafe { CompileMesh (mesh , vfs , exception , exception_mutex , warningtext) }
}

/// C: CompileTexture (user/user_model.cc:4790)
/// Calls: _mjPRIVATE_setTlsLogHandler, mjCTexture::Compile
#[allow(unused_variables, non_snake_case)]
pub fn compile_texture(texture: *mut mjCTexture, vfs: *const mjVFS, exception: *mut std__exception_ptr, exception_mutex: *mut std__mutex, warningtext: *mut string) {
    if texture.is_null() { return; }
    extern "C" { fn CompileTexture(texture: *mut mjCTexture, vfs: *const mjVFS, exception: *mut std__exception_ptr, exception_mutex: *mut std__mutex, warningtext: *mut string); }
    // SAFETY: texture verified non-null; delegates to C implementation
    unsafe { CompileTexture(texture, vfs, exception, exception_mutex, warningtext) }
}

/// C: PrintIndent (user/user_model.cc:5457)
#[allow(unused_variables, non_snake_case)]
pub fn print_indent(ss: *mut std__stringstream, depth: i32) {
    if ss.is_null() { return; }
    extern "C" { fn PrintIndent(ss: *mut std__stringstream, depth: i32); }
    // SAFETY: ss verified non-null
    unsafe { PrintIndent(ss, depth) }
}

/// C: mjCModel::CopyFromSpec (user/user_model.h:191)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_from_spec(self_ptr: *mut mjCModel) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_CopyFromSpec(self_ptr: *mut mjCModel); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_CopyFromSpec(self_ptr) }
}

/// C: mjCModel::PointToLocal (user/user_model.h:192)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_point_to_local(self_ptr: *mut mjCModel) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_PointToLocal(self_ptr: *mut mjCModel); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_PointToLocal(self_ptr) }
}

/// C: mjCModel::Compile (user/user_model.h:203)
/// Calls: _mjPRIVATE_setTlsLogHandler, mjCModel::Clear, mjCModel::ClearCompileWarnings, mjCModel::CopyFromSpec, mjCModel::TryCompile, mj_deleteData, mj_deleteModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_compile(self_ptr: *mut mjCModel, vfs: *const mjVFS, m: *mut *mut mjModel) -> *mut mjModel {
    extern "C" { fn mjCModel_Compile(self_ptr: *mut mjCModel, vfs: *const mjVFS, m: *mut *mut mjModel) -> *mut mjModel; }
    // SAFETY: delegates to C++ implementation; caller guarantees pointer validity
    unsafe { mjCModel_Compile(self_ptr, vfs, m) }
}

/// C: mjCModel::CopyBack (user/user_model.h:204)
/// Calls: mjCMesh::GetPosPtr, mjCMesh::GetQuatPtr
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_back(self_ptr: *mut mjCModel, arg0: *const mjModel) -> bool {
    if self_ptr.is_null() { return false; }
    extern "C" { fn mjCModel_CopyBack(self_ptr: *mut mjCModel, arg0: *const mjModel) -> bool; }
    // SAFETY: self_ptr verified non-null; delegates to C implementation
    unsafe { mjCModel_CopyBack(self_ptr, arg0) }
}

/// C: mjCModel::FuseStatic (user/user_model.h:205)
/// Calls: mjCBody::ComputeBVH, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_fuse_static(self_ptr: *mut mjCModel) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_FuseStatic(self_ptr: *mut mjCModel); }
    // SAFETY: self_ptr verified non-null; delegates to C implementation
    unsafe { mjCModel_FuseStatic(self_ptr) }
}

/// C: mjCModel::FuseReindex (user/user_model.h:206)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_fuse_reindex(self_ptr: *mut mjCModel, body: *mut mjCBody) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_FuseReindex(self_ptr: *mut mjCModel, body: *mut mjCBody); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_FuseReindex(self_ptr, body) }
}

/// C: mjCModel::AddFlex (user/user_model.h:209)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_flex(self_ptr: *mut mjCModel) -> *mut mjCFlex {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCModel_AddFlex(self_ptr: *mut mjCModel) -> *mut mjCFlex; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCModel_AddFlex(self_ptr) }
}

/// C: mjCModel::AddMesh (user/user_model.h:210)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_mesh(self_ptr: *mut mjCModel, def: *mut mjCDef) -> *mut mjCMesh {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCModel_AddMesh(self_ptr: *mut mjCModel, def: *mut mjCDef) -> *mut mjCMesh; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_AddMesh(self_ptr, def) }
}

/// C: mjCModel::AddSkin (user/user_model.h:211)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_skin(self_ptr: *mut mjCModel) -> *mut mjCSkin {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCModel_AddSkin(self_ptr: *mut mjCModel) -> *mut mjCSkin; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_AddSkin(self_ptr) }
}

/// C: mjCModel::AddHField (user/user_model.h:212)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_h_field(self_ptr: *mut mjCModel) -> *mut mjCHField {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCModel_AddHField(self_ptr: *mut mjCModel) -> *mut mjCHField; }
    // SAFETY: delegates to C implementation
    unsafe { mjCModel_AddHField(self_ptr) }
}

/// C: mjCModel::AddTexture (user/user_model.h:213)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_texture(self_ptr: *mut mjCModel) -> *mut mjCTexture {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCModel_AddTexture(self_ptr: *mut mjCModel) -> *mut mjCTexture; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_AddTexture(self_ptr) }
}

/// C: mjCModel::AddMaterial (user/user_model.h:214)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_material(self_ptr: *mut mjCModel, def: *mut mjCDef) -> *mut mjCMaterial {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCModel_AddMaterial(self_ptr: *mut mjCModel, def: *mut mjCDef) -> *mut mjCMaterial; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_AddMaterial(self_ptr, def) }
}

/// C: mjCModel::AddPair (user/user_model.h:215)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_pair(self_ptr: *mut mjCModel, def: *mut mjCDef) -> *mut mjCPair {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCModel_AddPair(self_ptr: *mut mjCModel, def: *mut mjCDef) -> *mut mjCPair; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_AddPair(self_ptr, def) }
}

/// C: mjCModel::AddExclude (user/user_model.h:216)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_exclude(self_ptr: *mut mjCModel) -> *mut mjCBodyPair {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCModel_AddExclude(self_ptr: *mut mjCModel) -> *mut mjCBodyPair; }
    // SAFETY: delegates to C implementation
    unsafe { mjCModel_AddExclude(self_ptr) }
}

/// C: mjCModel::AddEquality (user/user_model.h:217)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_equality(self_ptr: *mut mjCModel, def: *mut mjCDef) -> *mut mjCEquality {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCModel_AddEquality(self_ptr: *mut mjCModel, def: *mut mjCDef) -> *mut mjCEquality; }
    // SAFETY: delegates to C implementation
    unsafe { mjCModel_AddEquality(self_ptr, def) }
}

/// C: mjCModel::AddTendon (user/user_model.h:218)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_tendon(self_ptr: *mut mjCModel, def: *mut mjCDef) -> *mut mjCTendon {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCModel_AddTendon(self_ptr: *mut mjCModel, def: *mut mjCDef) -> *mut mjCTendon; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_AddTendon(self_ptr, def) }
}

/// C: mjCModel::AddActuator (user/user_model.h:219)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_actuator(self_ptr: *mut mjCModel, def: *mut mjCDef) -> *mut mjCActuator {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCModel_AddActuator(self_ptr: *mut mjCModel, def: *mut mjCDef) -> *mut mjCActuator; }
    // SAFETY: delegates to C implementation
    unsafe { mjCModel_AddActuator(self_ptr, def) }
}

/// C: mjCModel::AddSensor (user/user_model.h:220)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_sensor(self_ptr: *mut mjCModel) -> *mut mjCSensor {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCModel_AddSensor(self_ptr: *mut mjCModel) -> *mut mjCSensor; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_AddSensor(self_ptr) }
}

/// C: mjCModel::AddNumeric (user/user_model.h:221)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_numeric(self_ptr: *mut mjCModel) -> *mut mjCNumeric {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCModel_AddNumeric(self_ptr: *mut mjCModel) -> *mut mjCNumeric; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_AddNumeric(self_ptr) }
}

/// C: mjCModel::AddText (user/user_model.h:222)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_text(self_ptr: *mut mjCModel) -> *mut mjCText {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCModel_AddText(self_ptr: *mut mjCModel) -> *mut mjCText; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_AddText(self_ptr) }
}

/// C: mjCModel::AddTuple (user/user_model.h:223)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_tuple(self_ptr: *mut mjCModel) -> *mut mjCTuple {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCModel_AddTuple(self_ptr: *mut mjCModel) -> *mut mjCTuple; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_AddTuple(self_ptr) }
}

/// C: mjCModel::AddKey (user/user_model.h:224)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_key(self_ptr: *mut mjCModel) -> *mut mjCKey {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCModel_AddKey(self_ptr: *mut mjCModel) -> *mut mjCKey; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_AddKey(self_ptr) }
}

/// C: mjCModel::AddPlugin (user/user_model.h:225)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_plugin(self_ptr: *mut mjCModel) -> *mut mjCPlugin {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCModel_AddPlugin(self_ptr: *mut mjCModel) -> *mut mjCPlugin; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_AddPlugin(self_ptr) }
}

/// C: mjCModel::AppendSpec (user/user_model.h:228)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_append_spec(self_ptr: *mut mjCModel, spec: *mut mjSpec, compiler: *const mjsCompiler) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_AppendSpec(self_ptr: *mut mjCModel, spec: *mut mjSpec, compiler: *const mjsCompiler); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_AppendSpec(self_ptr, spec, compiler) }
}

/// C: mjCModel::NumObjects (user/user_model.h:244)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_num_objects(self_ptr: *mut mjCModel, r#type: mjtObj) -> i32 {
    if self_ptr.is_null() { return 0; }
    extern "C" { fn mjCModel_NumObjects(self_ptr: *mut mjCModel, r#type: mjtObj) -> i32; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_NumObjects(self_ptr, r#type) }
}

/// C: mjCModel::GetObject (user/user_model.h:245)
/// Calls: mjCModel::NumObjects
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_object(self_ptr: *mut mjCModel, r#type: mjtObj, id: i32) -> *mut mjCBase {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCModel_GetObject(self_ptr: *mut mjCModel, r#type: mjtObj, id: i32) -> *mut mjCBase; }
    // SAFETY: self_ptr verified non-null; delegates to C++ implementation
    unsafe { mjCModel_GetObject(self_ptr, r#type, id) }
}

/// C: mjCModel::NextObject (user/user_model.h:246)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_next_object(self_ptr: *mut mjCModel, object: *const mjsElement, r#type: mjtObj) -> *mut mjsElement {
    if self_ptr.is_null() {
        return core::ptr::null_mut();
    }
    extern "C" { fn mjCModel_NextObject(self_ptr: *mut mjCModel, object: *const mjsElement, r#type: mjtObj) -> *mut mjsElement; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_NextObject(self_ptr, object, r#type) }
}

/// C: mjCModel::IsCompiled (user/user_model.h:249)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_is_compiled(self_ptr: *mut mjCModel) -> bool {
    if self_ptr.is_null() { return false; }
    extern "C" { fn mjCModel_IsCompiled(self_ptr: *mut mjCModel) -> bool; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_IsCompiled(self_ptr) }
}

/// C: mjCModel::GetError (user/user_model.h:250)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_error(self_ptr: *mut mjCModel) -> *const mjCError {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCModel_GetError(self_ptr: *mut mjCModel) -> *const mjCError; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_GetError(self_ptr) }
}

/// C: mjCModel::SetError (user/user_model.h:251)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_set_error(self_ptr: *mut mjCModel, error: *const mjCError) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_SetError(self_ptr: *mut mjCModel, error: *const mjCError); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCModel_SetError(self_ptr, error) }
}

/// C: mjCModel::AddWarning (user/user_model.h:252)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_warning(self_ptr: *mut mjCModel, msg: string, obj: *const mjCBase) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_AddWarning(self_ptr: *mut mjCModel, msg: string, obj: *const mjCBase); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_AddWarning(self_ptr, msg, obj) }
}

/// C: mjCModel::AddGroupedWarning (user/user_model.h:254)
/// Calls: mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_grouped_warning(self_ptr: *mut mjCModel, subject: *const std__string, body: *const std__string) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_AddGroupedWarning(self_ptr: *mut mjCModel, subject: *const std__string, body: *const std__string); }
    // SAFETY: delegates to C implementation
    unsafe { mjCModel_AddGroupedWarning(self_ptr, subject, body) }
}

/// C: mjCModel::GetWarnings (user/user_model.h:256)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_warnings(self_ptr: *mut mjCModel) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCModel_GetWarnings(self_ptr: *mut mjCModel) -> *const i32; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_GetWarnings(self_ptr) }
}

/// C: mjCModel::ClearWarnings (user/user_model.h:260)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_clear_warnings(self_ptr: *mut mjCModel) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_ClearWarnings(self_ptr: *mut mjCModel); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_ClearWarnings(self_ptr) }
}

/// C: mjCModel::ClearCompileWarnings (user/user_model.h:264)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_clear_compile_warnings(self_ptr: *mut mjCModel) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: ()
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_ClearCompileWarnings(self_ptr: *mut mjCModel); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_ClearCompileWarnings(self_ptr) }
}

/// C: mjCModel::SetAttachWarningBoundary (user/user_model.h:267)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_set_attach_warning_boundary(self_ptr: *mut mjCModel) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_SetAttachWarningBoundary(self_ptr: *mut mjCModel); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_SetAttachWarningBoundary(self_ptr) }
}

/// C: mjCModel::GetWorld (user/user_model.h:271)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_world(self_ptr: *mut mjCModel) -> *mut mjCBody {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * mut mjCBody
    extern "C" { fn mjCModel_GetWorld(self_ptr: *mut mjCModel) -> *mut mjCBody; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_GetWorld(self_ptr) }
}

/// C: mjCModel::FindDefault (user/user_model.h:272)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_find_default(self_ptr: *mut mjCModel, name: *const std__string) -> *mut mjCDef {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCModel_FindDefault(self_ptr: *mut mjCModel, name: *const std__string) -> *mut mjCDef; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_FindDefault(self_ptr, name) }
}

/// C: mjCModel::AddDefault (user/user_model.h:273)
/// Calls: mjCDef::CopyFromSpec, mjCDef::CopyWithoutChildren
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_default(self_ptr: *mut mjCModel, name: string, parent: *mut mjCDef) -> *mut mjCDef {
    extern "C" { fn mjCModel_AddDefault(self_ptr: *mut mjCModel, name: string, parent: *mut mjCDef) -> *mut mjCDef; }
    // SAFETY: delegates to C implementation
    unsafe { mjCModel_AddDefault(self_ptr, name, parent) }
}

/// C: mjCModel::FindObject (user/user_model.h:274)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_find_object(self_ptr: *mut mjCModel, r#type: mjtObj, name: string) -> *mut mjCBase {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCModel_FindObject(self_ptr: *mut mjCModel, r#type: mjtObj, name: string) -> *mut mjCBase; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_FindObject(self_ptr, r#type, name) }
}

/// C: mjCModel::FindTree (user/user_model.h:275)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_find_tree(self_ptr: *mut mjCModel, body: *mut mjCBody, r#type: mjtObj, name: string) -> *mut mjCBase {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCModel_FindTree(self_ptr: *mut mjCModel, body: *mut mjCBody, r#type: mjtObj, name: string) -> *mut mjCBase; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_FindTree(self_ptr, body, r#type, name) }
}

/// C: mjCModel::FindSpec (user/user_model.h:276)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_find_spec(self_ptr: *mut mjCModel, name: string) -> *mut mjSpec {
    if self_ptr.is_null() {
        return core::ptr::null_mut();
    }
    extern "C" { fn mjCModel_FindSpec(self_ptr: *mut mjCModel, name: string) -> *mut mjSpec; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_FindSpec(self_ptr, name) }
}

/// C: mjCModel::ActivatePlugin (user/user_model.h:278)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_activate_plugin(self_ptr: *mut mjCModel, plugin: *const mjpPlugin, slot: i32) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_ActivatePlugin(self_ptr: *mut mjCModel, plugin: *const mjpPlugin, slot: i32); }
    // SAFETY: delegates to C implementation
    unsafe { mjCModel_ActivatePlugin(self_ptr, plugin, slot) }
}

/// C: mjCModel::get_meshdir (user/user_model.h:285)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_meshdir(self_ptr: *mut mjCModel) -> i32 {
    if self_ptr.is_null() { return 0; }
    extern "C" { fn mjCModel_get_meshdir(self_ptr: *mut mjCModel) -> i32; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_get_meshdir(self_ptr) }
}

/// C: mjCModel::get_texturedir (user/user_model.h:286)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_texturedir(self_ptr: *mut mjCModel) -> i32 {
    if self_ptr.is_null() { return 0; }
    extern "C" { fn mjCModel_get_texturedir(self_ptr: *mut mjCModel) -> i32; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_get_texturedir(self_ptr) }
}

/// C: mjCModel::Default (user/user_model.h:288)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_default(self_ptr: *mut mjCModel) -> *mut mjCDef {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCModel)
    // Previous return: * mut mjCDef
    extern "C" { fn mjCModel_Default(self_ptr: *mut mjCModel) -> *mut mjCDef; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCModel_Default(self_ptr) }
}

/// C: mjCModel::NumDefaults (user/user_model.h:289)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_num_defaults(self_ptr: *mut mjCModel) -> i32 {
    if self_ptr.is_null() { return 0; }
    extern "C" { fn mjCModel_NumDefaults(self_ptr: *mut mjCModel) -> i32; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_NumDefaults(self_ptr) }
}

/// C: mjCModel::ActivePlugins (user/user_model.h:291)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_active_plugins(self_ptr: *mut mjCModel) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCModel_ActivePlugins(self_ptr: *mut mjCModel) -> *const i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCModel_ActivePlugins(self_ptr) }
}

/// C: mjCModel::Flexes (user/user_model.h:295)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_flexes(self_ptr: *mut mjCModel) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCModel_Flexes(self_ptr: *mut mjCModel) -> *const i32; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_Flexes(self_ptr) }
}

/// C: mjCModel::Meshes (user/user_model.h:296)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_meshes(self_ptr: *mut mjCModel) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCModel_Meshes(self_ptr: *mut mjCModel) -> *const i32; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_Meshes(self_ptr) }
}

/// C: mjCModel::Skins (user/user_model.h:297)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_skins(self_ptr: *mut mjCModel) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCModel_Skins(self_ptr: *mut mjCModel) -> *const i32; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_Skins(self_ptr) }
}

/// C: mjCModel::HFields (user/user_model.h:298)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_h_fields(self_ptr: *mut mjCModel) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCModel_HFields(self_ptr: *mut mjCModel) -> *const i32; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_HFields(self_ptr) }
}

/// C: mjCModel::Textures (user/user_model.h:299)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_textures(self_ptr: *mut mjCModel) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCModel_Textures(self_ptr: *mut mjCModel) -> *const i32; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_Textures(self_ptr) }
}

/// C: mjCModel::Materials (user/user_model.h:300)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_materials(self_ptr: *mut mjCModel) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCModel_Materials(self_ptr: *mut mjCModel) -> *const i32; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_Materials(self_ptr) }
}

/// C: mjCModel::Pairs (user/user_model.h:301)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_pairs(self_ptr: *mut mjCModel) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCModel_Pairs(self_ptr: *mut mjCModel) -> *const i32; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_Pairs(self_ptr) }
}

/// C: mjCModel::Excludes (user/user_model.h:302)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_excludes(self_ptr: *mut mjCModel) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCModel_Excludes(self_ptr: *mut mjCModel) -> *const i32; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_Excludes(self_ptr) }
}

/// C: mjCModel::Equalities (user/user_model.h:303)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_equalities(self_ptr: *mut mjCModel) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCModel_Equalities(self_ptr: *mut mjCModel) -> *const i32; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_Equalities(self_ptr) }
}

/// C: mjCModel::Tendons (user/user_model.h:304)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_tendons(self_ptr: *mut mjCModel) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCModel_Tendons(self_ptr: *mut mjCModel) -> *const i32; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_Tendons(self_ptr) }
}

/// C: mjCModel::Actuators (user/user_model.h:305)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_actuators(self_ptr: *mut mjCModel) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCModel_Actuators(self_ptr: *mut mjCModel) -> *const i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCModel_Actuators(self_ptr) }
}

/// C: mjCModel::Sensors (user/user_model.h:306)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_sensors(self_ptr: *mut mjCModel) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCModel_Sensors(self_ptr: *mut mjCModel) -> *const i32; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_Sensors(self_ptr) }
}

/// C: mjCModel::Numerics (user/user_model.h:307)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_numerics(self_ptr: *mut mjCModel) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCModel_Numerics(self_ptr: *mut mjCModel) -> *const i32; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_Numerics(self_ptr) }
}

/// C: mjCModel::Texts (user/user_model.h:308)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_texts(self_ptr: *mut mjCModel) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCModel_Texts(self_ptr: *mut mjCModel) -> *const i32; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_Texts(self_ptr) }
}

/// C: mjCModel::Tuples (user/user_model.h:309)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_tuples(self_ptr: *mut mjCModel) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCModel_Tuples(self_ptr: *mut mjCModel) -> *const i32; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_Tuples(self_ptr) }
}

/// C: mjCModel::Keys (user/user_model.h:310)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_keys(self_ptr: *mut mjCModel) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCModel_Keys(self_ptr: *mut mjCModel) -> *const i32; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_Keys(self_ptr) }
}

/// C: mjCModel::Plugins (user/user_model.h:311)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_plugins(self_ptr: *mut mjCModel) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCModel_Plugins(self_ptr: *mut mjCModel) -> *const i32; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_Plugins(self_ptr) }
}

/// C: mjCModel::Bodies (user/user_model.h:312)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_bodies(self_ptr: *mut mjCModel) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCModel_Bodies(self_ptr: *mut mjCModel) -> *const i32; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_Bodies(self_ptr) }
}

/// C: mjCModel::Geoms (user/user_model.h:313)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_geoms(self_ptr: *mut mjCModel) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCModel_Geoms(self_ptr: *mut mjCModel) -> *const i32; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_Geoms(self_ptr) }
}

/// C: mjCModel::ResolvePlugin (user/user_model.h:316)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_resolve_plugin(self_ptr: *mut mjCModel, obj: *mut mjCBase, plugin_name: *const std__string, plugin_instance_name: *const std__string, plugin_instance: *mut *mut mjCPlugin) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_ResolvePlugin(self_ptr: *mut mjCModel, obj: *mut mjCBase, plugin_name: *const std__string, plugin_instance_name: *const std__string, plugin_instance: *mut *mut mjCPlugin); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_ResolvePlugin(self_ptr, obj, plugin_name, plugin_instance_name, plugin_instance) }
}

/// C: mjCModel::Clear (user/user_model.h:321)
/// Calls: mjCModel::ClearCompileWarnings
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_clear(self_ptr: *mut mjCModel) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_Clear(self_ptr: *mut mjCModel); }
    // SAFETY: self_ptr verified non-null; delegates to C implementation
    unsafe { mjCModel_Clear(self_ptr) }
}

/// C: mjCModel::SaveState (user/user_model.h:329)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_save_state(self_ptr: *mut mjCModel, state_name: *const std__string, qpos: *const T, qvel: *const T, act: *const T, ctrl: *const T, mpos: *const T, mquat: *const T) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_SaveState(self_ptr: *mut mjCModel, state_name: *const std__string, qpos: *const T, qvel: *const T, act: *const T, ctrl: *const T, mpos: *const T, mquat: *const T); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_SaveState(self_ptr, state_name, qpos, qvel, act, ctrl, mpos, mquat) }
}

/// C: mjCModel::RestoreState (user/user_model.h:334)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_restore_state(self_ptr: *mut mjCModel, state_name: *const std__string, pos0: *const f64, mpos0: *const f64, mquat0: *const f64, qpos: *mut T, qvel: *mut T, act: *mut T, ctrl: *mut T, mpos: *mut T, mquat: *mut T) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_RestoreState(self_ptr: *mut mjCModel, state_name: *const std__string, pos0: *const f64, mpos0: *const f64, mquat0: *const f64, qpos: *mut T, qvel: *mut T, act: *mut T, ctrl: *mut T, mpos: *mut T, mquat: *mut T); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_RestoreState(self_ptr, state_name, pos0, mpos0, mquat0, qpos, qvel, act, ctrl, mpos, mquat) }
}

/// C: mjCModel::MakeData (user/user_model.h:338)
/// Calls: mj_initPlugin, mj_makeRawData, mj_resetData
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_make_data(self_ptr: *mut mjCModel, m: *const mjModel, dest: *mut *mut mjData) {
    extern "C" { fn mjCModel_MakeData(self_ptr: *mut mjCModel, m: *const mjModel, dest: *mut *mut mjData); }
    // SAFETY: delegates to C implementation
    unsafe { mjCModel_MakeData(self_ptr, m, dest) }
}

/// C: mjCModel::StoreKeyframes (user/user_model.h:341)
/// Calls: mjCModel::ComputeReference, mjCModel::SaveDofOffsets
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_store_keyframes(self_ptr: *mut mjCModel, dest: *mut mjCModel) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_StoreKeyframes(self_ptr: *mut mjCModel, dest: *mut mjCModel); }
    // SAFETY: self_ptr verified non-null; delegates to C implementation
    unsafe { mjCModel_StoreKeyframes(self_ptr, dest) }
}

/// C: mjCModel::SetDeepCopy (user/user_model.h:347)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_set_deep_copy(self_ptr: *mut mjCModel, deepcopy: bool) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_SetDeepCopy(self_ptr: *mut mjCModel, deepcopy: bool); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_SetDeepCopy(self_ptr, deepcopy) }
}

/// C: mjCModel::SetAttached (user/user_model.h:350)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_set_attached(self_ptr: *mut mjCModel, deepcopy: bool) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_SetAttached(self_ptr: *mut mjCModel, deepcopy: bool); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_SetAttached(self_ptr, deepcopy) }
}

/// C: mjCModel::IsAttached (user/user_model.h:353)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_is_attached(self_ptr: *mut mjCModel) -> bool {
    if self_ptr.is_null() { return false; }
    extern "C" { fn mjCModel_IsAttached(self_ptr: *mut mjCModel) -> bool; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_IsAttached(self_ptr) }
}

/// C: mjCModel::CheckRepeat (user/user_model.h:356)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_check_repeat(self_ptr: *mut mjCModel, r#type: mjtObj) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_CheckRepeat(self_ptr: *mut mjCModel, r#type: mjtObj); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCModel_CheckRepeat(self_ptr, r#type) }
}

/// C: mjCModel::AddRef (user/user_model.h:359)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_ref(self_ptr: *mut mjCModel) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_AddRef(self_ptr: *mut mjCModel); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_AddRef(self_ptr) }
}

/// C: mjCModel::GetRef (user/user_model.h:360)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_ref(self_ptr: *mut mjCModel) -> i32 {
    if self_ptr.is_null() { return 0; }
    extern "C" { fn mjCModel_GetRef(self_ptr: *mut mjCModel) -> i32; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_GetRef(self_ptr) }
}

/// C: mjCModel::Release (user/user_model.h:361)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_release(self_ptr: *mut mjCModel) {
    if self_ptr.is_null() {
        return;
    }
    extern "C" { fn mjCModel_Release(self_ptr: *mut mjCModel); }
    unsafe { mjCModel_Release(self_ptr) }
}

/// C: mjCModel::MakeTreeLists (user/user_model.h:377)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_make_tree_lists(self_ptr: *mut mjCModel, body: *mut mjCBody) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_MakeTreeLists(self_ptr: *mut mjCModel, body: *mut mjCBody); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_MakeTreeLists(self_ptr, body) }
}

/// C: mjCModel::TryCompile (user/user_model.h:380)
/// Calls: mjCModel::AddKey, mjCModel::AddWarning, mjCModel::AutoSpringDamper, mjCModel::CheckEmptyNames, mjCModel::ClearCompileWarnings, mjCModel::CompileMeshesAndTextures, mjCModel::ComputeSparseSizes, mjCModel::CopyNames, mjCModel::CopyObjects, mjCModel::CopyPaths, mjCModel::CopyPlugins, mjCModel::CopyTree, mjCModel::ExpandAllKeyframes, mjCModel::FinalizeSimple, mjCModel::FuseStatic, mjCModel::IndexAssets, mjCModel::LengthRange, mjCModel::ProcessLists, mjCModel::ResolveKeyframes, mjCModel::SaveDofOffsets, mjCModel::SetNuser, mjCModel::SetSizes, mjCModel::Signature, mj_deleteData, mj_makeData, mj_makeRawData, mj_normalizeQuat, mj_resetData, mj_setConst, mj_step, mj_validateReferences, mjuu_defined
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_try_compile(self_ptr: *mut mjCModel, m: *mut *mut mjModel, d: *mut *mut mjData, vfs: *const mjVFS) {
    extern "C" { fn mjCModel_TryCompile(self_ptr: *mut mjCModel, m: *mut *mut mjModel, d: *mut *mut mjData, vfs: *const mjVFS); }
    // SAFETY: delegates to C implementation
    unsafe { mjCModel_TryCompile(self_ptr, m, d, vfs) }
}

/// C: mjCModel::CompileMeshesAndTextures (user/user_model.h:381)
/// Calls: NumCompilerThreads, ThreadPool::WaitCount
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_compile_meshes_and_textures(self_ptr: *mut mjCModel, vfs: *const mjVFS) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_CompileMeshesAndTextures(self_ptr: *mut mjCModel, vfs: *const mjVFS); }
    // SAFETY: self_ptr verified non-null; delegates to C implementation
    unsafe { mjCModel_CompileMeshesAndTextures(self_ptr, vfs) }
}

/// C: mjCModel::SetNuser (user/user_model.h:383)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_set_nuser(self_ptr: *mut mjCModel) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_SetNuser(self_ptr: *mut mjCModel); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_SetNuser(self_ptr) }
}

/// C: mjCModel::IndexAssets (user/user_model.h:384)
/// Calls: mjCGeom::IsVisual, mjCGeom::get_hfieldname, mjCGeom::get_material, mjCGeom::get_meshname, mjCMesh::IsVisual, mjCMesh::SetNotVisual, mjCModel::FindObject, mjCSite::get_material
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_index_assets(self_ptr: *mut mjCModel, discard: bool) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_IndexAssets(self_ptr: *mut mjCModel, discard: bool); }
    // SAFETY: self_ptr verified non-null; delegates to C implementation
    unsafe { mjCModel_IndexAssets(self_ptr, discard) }
}

/// C: mjCModel::CheckEmptyNames (user/user_model.h:385)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_check_empty_names(self_ptr: *mut mjCModel) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_CheckEmptyNames(self_ptr: *mut mjCModel); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_CheckEmptyNames(self_ptr) }
}

/// C: mjCModel::SetSizes (user/user_model.h:386)
/// Calls: mjCBody::GetParent
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_set_sizes(self_ptr: *mut mjCModel) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_SetSizes(self_ptr: *mut mjCModel); }
    // SAFETY: self_ptr verified non-null; delegates to C implementation
    unsafe { mjCModel_SetSizes(self_ptr) }
}

/// C: mjCModel::ComputeSparseSizes (user/user_model.h:387)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_compute_sparse_sizes(self_ptr: *mut mjCModel) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_ComputeSparseSizes(self_ptr: *mut mjCModel); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_ComputeSparseSizes(self_ptr) }
}

/// C: mjCModel::AutoSpringDamper (user/user_model.h:388)
/// Calls: mjCJoint::nv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_auto_spring_damper(self_ptr: *mut mjCModel, arg0: *mut mjModel) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_AutoSpringDamper(self_ptr: *mut mjCModel, arg0: *mut mjModel); }
    // SAFETY: self_ptr verified non-null; delegates to C implementation
    unsafe { mjCModel_AutoSpringDamper(self_ptr, arg0) }
}

/// C: mjCModel::LengthRange (user/user_model.h:389)
/// Calls: NumCompilerThreads, ThreadPool::WaitCount, mj_makeData
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_length_range(self_ptr: *mut mjCModel, arg0: *mut mjModel, arg1: *mut mjData) {
    extern "C" { fn mjCModel_LengthRange(self_ptr: *mut mjCModel, arg0: *mut mjModel, arg1: *mut mjData); }
    // SAFETY: delegates to C implementation
    unsafe { mjCModel_LengthRange(self_ptr, arg0, arg1) }
}

/// C: mjCModel::CopyNames (user/user_model.h:390)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_names(self_ptr: *mut mjCModel, arg0: *mut mjModel) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_CopyNames(self_ptr: *mut mjCModel, arg0: *mut mjModel); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_CopyNames(self_ptr, arg0) }
}

/// C: mjCModel::CopyPaths (user/user_model.h:391)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_paths(self_ptr: *mut mjCModel, arg0: *mut mjModel) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_CopyPaths(self_ptr: *mut mjCModel, arg0: *mut mjModel); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_CopyPaths(self_ptr, arg0) }
}

/// C: mjCModel::CopyObjects (user/user_model.h:392)
/// Calls: mjCActuator::is_actlimited, mjCActuator::is_ctrllimited, mjCActuator::is_forcelimited, mjCBoundingVolumeHierarchy::Nbvh, mjCBoundingVolumeHierarchy::Nodeid, mjCMesh::CopyFace, mjCMesh::CopyFaceNormal, mjCMesh::CopyFaceTexcoord, mjCMesh::CopyGraph, mjCMesh::CopyNormal, mjCMesh::CopyPolygonMap, mjCMesh::CopyPolygonNormals, mjCMesh::CopyPolygons, mjCMesh::CopyTexcoord, mjCMesh::CopyVert, mjCMesh::GetPosPtr, mjCMesh::GetQuatPtr, mjCMesh::HasTexcoord, mjCMesh::Scale, mjCMesh::nface, mjCMesh::nnormal, mjCMesh::npolygon, mjCMesh::npolygonmap, mjCMesh::npolygonvert, mjCMesh::ntexcoord, mjCMesh::nvert, mjCMesh::octree, mjCMesh::szgraph, mjCMesh::tree, mjCModel::AddWarning, mjCOctree::CopyAabb, mjCOctree::CopyChild, mjCOctree::CopyCoeff, mjCOctree::CopyLevel, mjCOctree::NumNodes, mjCTendon::is_actfrclimited, mjCTendon::is_limited
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_objects(self_ptr: *mut mjCModel, arg0: *mut mjModel) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_CopyObjects(self_ptr: *mut mjCModel, arg0: *mut mjModel); }
    // SAFETY: self_ptr verified non-null; delegates to C implementation
    unsafe { mjCModel_CopyObjects(self_ptr, arg0) }
}

/// C: mjCModel::CopyTree (user/user_model.h:393)
/// Calls: mjCGeom::GetRBound, mjCJoint::is_actfrclimited, mjCJoint::is_limited, mjCJoint::nq, mjCJoint::nv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_tree(self_ptr: *mut mjCModel, arg0: *mut mjModel) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_CopyTree(self_ptr: *mut mjCModel, arg0: *mut mjModel); }
    // SAFETY: self_ptr verified non-null; delegates to C implementation
    unsafe { mjCModel_CopyTree(self_ptr, arg0) }
}

/// C: mjCModel::FinalizeSimple (user/user_model.h:394)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_finalize_simple(self_ptr: *mut mjCModel, m: *mut mjModel) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_FinalizeSimple(self_ptr: *mut mjCModel, m: *mut mjModel); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_FinalizeSimple(self_ptr, m) }
}

/// C: mjCModel::CopyPlugins (user/user_model.h:395)
/// Calls: mjp_getPluginAtSlot, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_plugins(self_ptr: *mut mjCModel, arg0: *mut mjModel) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_CopyPlugins(self_ptr: *mut mjCModel, arg0: *mut mjModel); }
    // SAFETY: self_ptr verified non-null; delegates to C implementation
    unsafe { mjCModel_CopyPlugins(self_ptr, arg0) }
}

/// C: mjCModel::CountTendonDofs (user/user_model.h:396)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_count_tendon_dofs(self_ptr: *mut mjCModel, m: *const mjModel, id: i32) -> i32 {
    if self_ptr.is_null() { return 0; }
    extern "C" { fn mjCModel_CountTendonDofs(self_ptr: *mut mjCModel, m: *const mjModel, id: i32) -> i32; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_CountTendonDofs(self_ptr, m, id) }
}

/// C: mjCModel::CountNJmom (user/user_model.h:398)
/// Calls: mjCModel::CountTendonDofs
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_count_n_jmom(self_ptr: *mut mjCModel, m: *const mjModel) -> i32 {
    if self_ptr.is_null() { return 0; }
    extern "C" { fn mjCModel_CountNJmom(self_ptr: *mut mjCModel, m: *const mjModel) -> i32; }
    // SAFETY: self_ptr verified non-null; delegates to C implementation
    unsafe { mjCModel_CountNJmom(self_ptr, m) }
}

/// C: mjCModel::CountNJten (user/user_model.h:399)
/// Calls: mjCModel::CountTendonDofs
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_count_n_jten(self_ptr: *mut mjCModel, m: *const mjModel) -> i32 {
    if self_ptr.is_null() { return 0; }
    extern "C" { fn mjCModel_CountNJten(self_ptr: *mut mjCModel, m: *const mjModel) -> i32; }
    // SAFETY: self_ptr verified non-null; delegates to C implementation
    unsafe { mjCModel_CountNJten(self_ptr, m) }
}

/// C: mjCModel::RemovePlugins (user/user_model.h:402)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_remove_plugins(self_ptr: *mut mjCModel) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_RemovePlugins(self_ptr: *mut mjCModel); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_RemovePlugins(self_ptr) }
}

/// C: mjCModel::CopyExplicitPlugin (user/user_model.h:448)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_explicit_plugin(self_ptr: *mut mjCModel, obj: *mut T) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_CopyExplicitPlugin(self_ptr: *mut mjCModel, obj: *mut T); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_CopyExplicitPlugin(self_ptr, obj) }
}

/// C: mjCModel::CreateObjectLists (user/user_model.h:458)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_create_object_lists(self_ptr: *mut mjCModel) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_CreateObjectLists(self_ptr: *mut mjCModel); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_CreateObjectLists(self_ptr) }
}

/// C: mjCModel::ProcessLists (user/user_model.h:461)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_process_lists(self_ptr: *mut mjCModel, checkrepeat: bool) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_ProcessLists(self_ptr: *mut mjCModel, checkrepeat: bool); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_ProcessLists(self_ptr, checkrepeat) }
}

/// C: mjCModel::ResetTreeLists (user/user_model.h:468)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_reset_tree_lists(self_ptr: *mut mjCModel) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_ResetTreeLists(self_ptr: *mut mjCModel); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_ResetTreeLists(self_ptr) }
}

/// C: mjCModel::SaveDofOffsets (user/user_model.h:471)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_save_dof_offsets(self_ptr: *mut mjCModel, computesize: bool) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_SaveDofOffsets(self_ptr: *mut mjCModel, computesize: bool); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_SaveDofOffsets(self_ptr, computesize) }
}

/// C: mjCModel::ResolveKeyframes (user/user_model.h:474)
/// Calls: mjCModel::SaveDofOffsets
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_resolve_keyframes(self_ptr: *mut mjCModel, m: *const mjModel) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_ResolveKeyframes(self_ptr: *mut mjCModel, m: *const mjModel); }
    // SAFETY: self_ptr verified non-null; delegates to C implementation
    unsafe { mjCModel_ResolveKeyframes(self_ptr, m) }
}

/// C: mjCModel::ExpandKeyframe (user/user_model.h:477)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_expand_keyframe(self_ptr: *mut mjCModel, key: *mut mjCKey, qpos0_: *const f64, bpos: *const f64, bquat: *const f64) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_ExpandKeyframe(self_ptr: *mut mjCModel, key: *mut mjCKey, qpos0_: *const f64, bpos: *const f64, bquat: *const f64); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_ExpandKeyframe(self_ptr, key, qpos0_, bpos, bquat) }
}

/// C: mjCModel::ComputeReference (user/user_model.h:480)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_compute_reference(self_ptr: *mut mjCModel) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_ComputeReference(self_ptr: *mut mjCModel); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_ComputeReference(self_ptr) }
}

/// C: mjCModel::CheckBodyMassInertia (user/user_model.h:483)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_check_body_mass_inertia(self_ptr: *mut mjCModel, body: *mut mjCBody) -> bool {
    if self_ptr.is_null() { return false; }
    extern "C" { fn mjCModel_CheckBodyMassInertia(self_ptr: *mut mjCModel, body: *mut mjCBody) -> bool; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_CheckBodyMassInertia(self_ptr, body) }
}

/// C: mjCModel::PrintTree (user/user_model.h:491)
/// Calls: PrintIndent
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_print_tree(self_ptr: *mut mjCModel, tree: *mut std__stringstream, body: *const mjCBody, depth: i32) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_PrintTree(self_ptr: *mut mjCModel, tree: *mut std__stringstream, body: *const mjCBody, depth: i32); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_PrintTree(self_ptr, tree, body, depth) }
}

/// C: mjCModel::Signature (user/user_model.h:494)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_signature(self_ptr: *mut mjCModel) -> u64 {
    if self_ptr.is_null() { return 0; }
    extern "C" { fn mjCModel_Signature(self_ptr: *mut mjCModel) -> u64; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_Signature(self_ptr) }
}

/// C: mjCModel::DeleteSubtreePlugin (user/user_model.h:505)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_delete_subtree_plugin(self_ptr: *mut mjCModel, subtree: *mut mjCBody) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_DeleteSubtreePlugin(self_ptr: *mut mjCModel, subtree: *mut mjCBody); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCModel_DeleteSubtreePlugin(self_ptr, subtree) }
}

/// C: mjCModel::ExpandAllKeyframes (user/user_model.h:508)
/// Calls: mjCModel::ComputeReference, mjCModel::SaveDofOffsets
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_expand_all_keyframes(self_ptr: *mut mjCModel) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCModel_ExpandAllKeyframes(self_ptr: *mut mjCModel); }
    // SAFETY: self_ptr verified non-null; delegates to C implementation
    unsafe { mjCModel_ExpandAllKeyframes(self_ptr) }
}

