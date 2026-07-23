//! Port of: user/user_model.cc
//! IR hash: bd605ac8158c32d6
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
/// Calls: IsSameQuat, IsSameVec, mjCBoundingVolume::Quat
#[allow(unused_variables, non_snake_case)]
pub fn is_same_pose(pos1: *const type_parameter_0_0, pos2: *const type_parameter_0_0, quat1: *const type_parameter_0_0, quat2: *const type_parameter_0_0) -> bool {
    todo!() // IsSamePose
}

/// C: IsNullPose (user/user_model.cc:127)
/// Calls: IsSamePose
#[allow(unused_variables, non_snake_case)]
pub fn is_null_pose(pos: *const type_parameter_0_0, quat: *const type_parameter_0_0) -> bool {
    todo!() // IsNullPose
}

/// C: GetBodyIdFromWrap (user/user_model.cc:134)
/// Calls: mjCGeom::GetParent, mjCSite::Body, mjCWrap::Type
#[allow(unused_variables, non_snake_case)]
pub fn get_body_id_from_wrap(wrap: *const mjCWrap) -> i32 {
    todo!() // GetBodyIdFromWrap
}

/// C: resetlist (user/user_model.cc:304)
#[allow(unused_variables, non_snake_case)]
pub fn resetlist(list: *const ()) {
    todo!() // resetlist
}

/// C: IsPluginActive (user/user_model.cc:440)
#[allow(unused_variables, non_snake_case)]
pub fn is_plugin_active(plugin: *const mjpPlugin, active_plugins: *const ()) -> bool {
    todo!() // IsPluginActive
}

/// C: mjCModel::DeleteAll (user/user_model.cc:545)
/// Calls: mjCBase::Release
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_delete_all(self_ptr: *mut mjCModel, elements: *const ()) {
    todo!() // mjCModel::DeleteAll
}

/// C: deletefromlist (user/user_model.cc:715)
/// Calls: FilePath::size
#[allow(unused_variables, non_snake_case)]
pub fn deletefromlist(list: *const (), element: *mut mjsElement) {
    todo!() // deletefromlist
}

/// C: GetNext (user/user_model.cc:1411)
/// Calls: FilePath::empty, FilePath::size
#[allow(unused_variables, non_snake_case)]
pub fn get_next(list: *const (), child: *const mjsElement) -> *mut mjsElement {
    todo!() // GetNext
}

/// C: findobject (user/user_model.cc:1596)
#[allow(unused_variables, non_snake_case)]
pub fn findobject(name: std__string_view, list: *const (), ids: *const mjKeyMap) -> *mut T {
    todo!() // findobject
}

/// C: DeleteAllTextures (user/user_model.cc:1790)
/// Calls: mjCMaterial::del_textures
#[allow(unused_variables, non_snake_case)]
pub fn delete_all_textures(list: *const ()) {
    todo!() // DeleteAllTextures
}

/// C: DeleteTexcoord (user/user_model.cc:1799)
/// Calls: mjCMesh::Texcoord
#[allow(unused_variables, non_snake_case)]
pub fn delete_texcoord(list: *const ()) {
    todo!() // DeleteTexcoord
}

/// C: DeleteElements (user/user_model.cc:1810)
#[allow(unused_variables, non_snake_case)]
pub fn delete_elements(elements: *const (), discard: *const ()) {
    todo!() // DeleteElements
}

/// C: mjCModel::Delete (user/user_model.cc:1848)
/// Calls: DeleteElements
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_delete(self_ptr: *mut mjCModel, elements: *const (), discard: *const ()) {
    todo!() // mjCModel::Delete
}

/// C: getpathslength (user/user_model.cc:2128)
/// Calls: FilePath::empty
#[allow(unused_variables, non_snake_case)]
pub fn getpathslength(list: *const ()) -> usize {
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
/// Calls: FilePath::c_str, FilePath::empty, FilePath::size, addtolist, mj_hashString
#[allow(unused_variables, non_snake_case)]
pub fn namelist(list: *const (), adr: i32, name_adr: *mut i32, names: *mut i8, map: *mut i32) -> i32 {
    todo!() // namelist
}

/// C: pathlist (user/user_model.cc:2702)
/// Calls: FilePath::empty, FilePath::size, addtolist
#[allow(unused_variables, non_snake_case)]
pub fn pathlist(list: *const (), adr: i32, path_adr: *mut i32, paths: *mut i8) -> i32 {
    todo!() // pathlist
}

/// C: makelistid (user/user_model.cc:4311)
/// Calls: FilePath::size
#[allow(unused_variables, non_snake_case)]
pub fn makelistid(dest: *const (), source: *const ()) {
    todo!() // makelistid
}

/// C: changeframe (user/user_model.cc:4319)
/// Calls: mjuu_copyvec, mjuu_frameaccum
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn changeframe(childpos: *mut f64, childquat: *mut f64, bodypos: *const f64, bodyquat: *const f64) {
    let mut pos: [f64; 3] = [0.0; 3];
    let mut quat: [f64; 4] = [0.0; 4];
    crate::user::user_util::mjuu_copyvec(pos.as_mut_ptr() as *mut T1, bodypos as *const T2, 3);
    crate::user::user_util::mjuu_copyvec(quat.as_mut_ptr() as *mut T1, bodyquat as *const T2, 4);
    crate::user::user_util::mjuu_frameaccum(pos.as_mut_ptr(), quat.as_mut_ptr(), childpos, childquat);
    crate::user::user_util::mjuu_copyvec(childpos as *mut T1, pos.as_ptr() as *const T2, 3);
    crate::user::user_util::mjuu_copyvec(childquat as *mut T1, quat.as_ptr() as *const T2, 4);
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
/// Calls: FilePath::size
#[allow(unused_variables, non_snake_case)]
pub fn reassignid(list: *const ()) {
    todo!() // reassignid
}

/// C: compilerLogHandler (user/user_model.cc:4665)
/// Calls: strcat_arr, strcpy_arr
#[allow(unused_variables, non_snake_case)]
pub fn compiler_log_handler(msg: *const mjLogMessage) {
    todo!() // compilerLogHandler
}

/// C: CompileMesh (user/user_model.cc:4770)
/// Calls: _mjPRIVATE_setTlsLogHandler, mjCMesh::Compile
#[allow(unused_variables, non_snake_case)]
pub fn compile_mesh(mesh: *mut mjCMesh, vfs: *const mjVFS, exception: *mut std__exception_ptr, exception_mutex: *mut std__mutex, warningtext: *mut std__string) {
    todo!() // CompileMesh
}

/// C: CompileTexture (user/user_model.cc:4790)
/// Calls: _mjPRIVATE_setTlsLogHandler, mjCTexture::Compile
#[allow(unused_variables, non_snake_case)]
pub fn compile_texture(texture: *mut mjCTexture, vfs: *const mjVFS, exception: *mut std__exception_ptr, exception_mutex: *mut std__mutex, warningtext: *mut std__string) {
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
    // SAFETY: self_ptr is a valid pointer to mjCModel
    unsafe {
        (*self_ptr).spec.element = self_ptr as *mut mjsElement;
        (*self_ptr).spec.comment = &mut (*self_ptr).spec_comment_ as *mut std__string as *mut mjString;
        (*self_ptr).spec.modelfiledir = &mut (*self_ptr).spec_modelfiledir_ as *mut std__string as *mut mjString;
        (*self_ptr).spec.modelname = &mut (*self_ptr).spec_modelname_ as *mut std__string as *mut mjString;
        (*self_ptr).spec.compiler.meshdir = &mut (*self_ptr).meshdir_ as *mut std__string as *mut mjString;
        (*self_ptr).spec.compiler.texturedir = &mut (*self_ptr).texturedir_ as *mut std__string as *mut mjString;
        (*self_ptr).comment = std::ptr::null_mut();
        (*self_ptr).modelfiledir = std::ptr::null_mut();
        (*self_ptr).modelname = std::ptr::null_mut();
    }
}

/// C: mjCModel::Compile (user/user_model.h:203)
/// Calls: _mjPRIVATE_setTlsLogHandler, mjCModel::Clear, mjCModel::ClearCompileWarnings, mjCModel::CopyFromSpec, mjCModel::TryCompile, mj_deleteData, mj_deleteModel, mju_warning, strcat_arr
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_compile(self_ptr: *mut mjCModel, vfs: *const mjVFS, m: *mut *mut mjModel) -> *mut mjModel {
    todo!() // mjCModel::Compile
}

/// C: mjCModel::CopyBack (user/user_model.h:204)
/// Calls: mjCHField::get_userdata, mjCMesh::GetPosPtr, mjCMesh::GetQuatPtr, mjuu_copyvec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_back(self_ptr: *mut mjCModel, arg0: *const mjModel) -> bool {
    todo!() // mjCModel::CopyBack
}

/// C: mjCModel::FuseStatic (user/user_model.h:205)
/// Calls: changeframe, mjCBody::AccumulateInertia, mjCBody::ComputeBVH, mjCBoundingVolumeHierarchy::Nbvh, mjCModel::FuseReindex, mjCModel::ProcessList_, mjCModel::ReassignChild, mjCModel::ResolveReferences, mju_error, mjuu_rotVecQuat
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_fuse_static(self_ptr: *mut mjCModel) {
    todo!() // mjCModel::FuseStatic
}

/// C: mjCModel::FuseReindex (user/user_model.h:206)
/// Calls: makelistid
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_fuse_reindex(self_ptr: *mut mjCModel, body: *mut mjCBody) {
    todo!() // mjCModel::FuseReindex
}

/// C: mjCModel::AddFlex (user/user_model.h:209)
/// Calls: mjCModel::AddObject
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_flex(self_ptr: *mut mjCModel) -> *mut mjCFlex {
    todo!() // mjCModel::AddFlex
}

/// C: mjCModel::AddMesh (user/user_model.h:210)
/// Calls: mjCModel::AddObjectDefault
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_mesh(self_ptr: *mut mjCModel, def: *mut mjCDef) -> *mut mjCMesh {
    todo!() // mjCModel::AddMesh
}

/// C: mjCModel::AddSkin (user/user_model.h:211)
/// Calls: mjCModel::AddObject
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_skin(self_ptr: *mut mjCModel) -> *mut mjCSkin {
    todo!() // mjCModel::AddSkin
}

/// C: mjCModel::AddHField (user/user_model.h:212)
/// Calls: mjCModel::AddObject
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_h_field(self_ptr: *mut mjCModel) -> *mut mjCHField {
    todo!() // mjCModel::AddHField
}

/// C: mjCModel::AddTexture (user/user_model.h:213)
/// Calls: mjCModel::AddObject
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_texture(self_ptr: *mut mjCModel) -> *mut mjCTexture {
    todo!() // mjCModel::AddTexture
}

/// C: mjCModel::AddMaterial (user/user_model.h:214)
/// Calls: mjCModel::AddObjectDefault
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_material(self_ptr: *mut mjCModel, def: *mut mjCDef) -> *mut mjCMaterial {
    todo!() // mjCModel::AddMaterial
}

/// C: mjCModel::AddPair (user/user_model.h:215)
/// Calls: mjCModel::AddObjectDefault
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_pair(self_ptr: *mut mjCModel, def: *mut mjCDef) -> *mut mjCPair {
    todo!() // mjCModel::AddPair
}

/// C: mjCModel::AddExclude (user/user_model.h:216)
/// Calls: mjCModel::AddObject
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_exclude(self_ptr: *mut mjCModel) -> *mut mjCBodyPair {
    todo!() // mjCModel::AddExclude
}

/// C: mjCModel::AddEquality (user/user_model.h:217)
/// Calls: mjCModel::AddObjectDefault
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_equality(self_ptr: *mut mjCModel, def: *mut mjCDef) -> *mut mjCEquality {
    todo!() // mjCModel::AddEquality
}

/// C: mjCModel::AddTendon (user/user_model.h:218)
/// Calls: mjCModel::AddObjectDefault
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_tendon(self_ptr: *mut mjCModel, def: *mut mjCDef) -> *mut mjCTendon {
    todo!() // mjCModel::AddTendon
}

/// C: mjCModel::AddActuator (user/user_model.h:219)
/// Calls: mjCModel::AddObjectDefault
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_actuator(self_ptr: *mut mjCModel, def: *mut mjCDef) -> *mut mjCActuator {
    todo!() // mjCModel::AddActuator
}

/// C: mjCModel::AddSensor (user/user_model.h:220)
/// Calls: mjCModel::AddObject
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_sensor(self_ptr: *mut mjCModel) -> *mut mjCSensor {
    todo!() // mjCModel::AddSensor
}

/// C: mjCModel::AddNumeric (user/user_model.h:221)
/// Calls: mjCModel::AddObject
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_numeric(self_ptr: *mut mjCModel) -> *mut mjCNumeric {
    todo!() // mjCModel::AddNumeric
}

/// C: mjCModel::AddText (user/user_model.h:222)
/// Calls: mjCModel::AddObject
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_text(self_ptr: *mut mjCModel) -> *mut mjCText {
    todo!() // mjCModel::AddText
}

/// C: mjCModel::AddTuple (user/user_model.h:223)
/// Calls: mjCModel::AddObject
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_tuple(self_ptr: *mut mjCModel) -> *mut mjCTuple {
    todo!() // mjCModel::AddTuple
}

/// C: mjCModel::AddKey (user/user_model.h:224)
/// Calls: mjCModel::AddObject
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_key(self_ptr: *mut mjCModel) -> *mut mjCKey {
    todo!() // mjCModel::AddKey
}

/// C: mjCModel::AddPlugin (user/user_model.h:225)
/// Calls: mjCModel::AddObject
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
/// Calls: GetNext, mjCBody::NextChild
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_next_object(self_ptr: *mut mjCModel, object: *const mjsElement, r#type: u32) -> *mut mjsElement {
    todo!() // mjCModel::NextObject
}

/// C: mjCModel::IsCompiled (user/user_model.h:249)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_is_compiled(self_ptr: *mut mjCModel) -> bool {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { (*self_ptr).compiled }
}

/// C: mjCModel::GetError (user/user_model.h:250)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_error(self_ptr: *mut mjCModel) -> *const mjCError {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { core::ptr::addr_of!((*self_ptr).errInfo).cast::<mjCError>() }
}

/// C: mjCModel::SetError (user/user_model.h:251)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_set_error(self_ptr: *mut mjCModel, error: *const mjCError) {
    // SAFETY: self_ptr and error are valid pointers provided by the caller.
    // errInfo is the raw byte storage for mjCError within mjCModel.
    unsafe {
        let dst = &mut (*self_ptr).errInfo as *mut [u8; 504] as *mut u8;
        let src = error as *const u8;
        std::ptr::copy_nonoverlapping(src, dst, std::mem::size_of::<mjCError>());
    }
}

/// C: mjCModel::AddWarning (user/user_model.h:252)
/// Calls: mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_warning(self_ptr: *mut mjCModel, msg: std__string, obj: *const mjCBase) {
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
pub fn mj_c_model_get_warnings(self_ptr: *mut mjCModel) -> *const () {
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
pub fn mj_c_model_add_default(self_ptr: *mut mjCModel, name: std__string, parent: *mut mjCDef) -> *mut mjCDef {
    todo!() // mjCModel::AddDefault
}

/// C: mjCModel::FindObject (user/user_model.h:274)
/// Calls: findobject
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_find_object(self_ptr: *mut mjCModel, r#type: u32, name: std__string) -> *mut mjCBase {
    todo!() // mjCModel::FindObject
}

/// C: mjCModel::FindTree (user/user_model.h:275)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_find_tree(self_ptr: *mut mjCModel, body: *mut mjCBody, r#type: u32, name: std__string) -> *mut mjCBase {
    todo!() // mjCModel::FindTree
}

/// C: mjCModel::FindSpec (user/user_model.h:276)
/// Calls: mjs_getString
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_find_spec(self_ptr: *mut mjCModel, name: std__string) -> *mut mjSpec {
    todo!() // mjCModel::FindSpec
}

/// C: mjCModel::ActivatePlugin (user/user_model.h:278)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_activate_plugin(self_ptr: *mut mjCModel, plugin: *const mjpPlugin, slot: i32) {
    todo!() // mjCModel::ActivatePlugin
}

/// C: mjCModel::FindAsset (user/user_model.h:282)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_find_asset(self_ptr: *mut mjCModel, name: std__string_view, list: *const ()) -> *mut mjCBase {
    todo!() // mjCModel::FindAsset
}

/// C: mjCModel::get_meshdir (user/user_model.h:285)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_meshdir(self_ptr: *mut mjCModel) -> std__string {
    todo!() // mjCModel::get_meshdir
}

/// C: mjCModel::get_texturedir (user/user_model.h:286)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_texturedir(self_ptr: *mut mjCModel) -> std__string {
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
pub fn mj_c_model_active_plugins(self_ptr: *mut mjCModel) -> *const () {
    todo!() // mjCModel::ActivePlugins
}

/// C: mjCModel::Flexes (user/user_model.h:295)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_flexes(self_ptr: *mut mjCModel) -> *const () {
    todo!() // mjCModel::Flexes
}

/// C: mjCModel::Meshes (user/user_model.h:296)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_meshes(self_ptr: *mut mjCModel) -> *const () {
    todo!() // mjCModel::Meshes
}

/// C: mjCModel::Skins (user/user_model.h:297)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_skins(self_ptr: *mut mjCModel) -> *const () {
    todo!() // mjCModel::Skins
}

/// C: mjCModel::HFields (user/user_model.h:298)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_h_fields(self_ptr: *mut mjCModel) -> *const () {
    todo!() // mjCModel::HFields
}

/// C: mjCModel::Textures (user/user_model.h:299)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_textures(self_ptr: *mut mjCModel) -> *const () {
    todo!() // mjCModel::Textures
}

/// C: mjCModel::Materials (user/user_model.h:300)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_materials(self_ptr: *mut mjCModel) -> *const () {
    todo!() // mjCModel::Materials
}

/// C: mjCModel::Pairs (user/user_model.h:301)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_pairs(self_ptr: *mut mjCModel) -> *const () {
    todo!() // mjCModel::Pairs
}

/// C: mjCModel::Excludes (user/user_model.h:302)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_excludes(self_ptr: *mut mjCModel) -> *const () {
    todo!() // mjCModel::Excludes
}

/// C: mjCModel::Equalities (user/user_model.h:303)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_equalities(self_ptr: *mut mjCModel) -> *const () {
    todo!() // mjCModel::Equalities
}

/// C: mjCModel::Tendons (user/user_model.h:304)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_tendons(self_ptr: *mut mjCModel) -> *const () {
    todo!() // mjCModel::Tendons
}

/// C: mjCModel::Actuators (user/user_model.h:305)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_actuators(self_ptr: *mut mjCModel) -> *const () {
    todo!() // mjCModel::Actuators
}

/// C: mjCModel::Sensors (user/user_model.h:306)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_sensors(self_ptr: *mut mjCModel) -> *const () {
    todo!() // mjCModel::Sensors
}

/// C: mjCModel::Numerics (user/user_model.h:307)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_numerics(self_ptr: *mut mjCModel) -> *const () {
    todo!() // mjCModel::Numerics
}

/// C: mjCModel::Texts (user/user_model.h:308)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_texts(self_ptr: *mut mjCModel) -> *const () {
    todo!() // mjCModel::Texts
}

/// C: mjCModel::Tuples (user/user_model.h:309)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_tuples(self_ptr: *mut mjCModel) -> *const () {
    todo!() // mjCModel::Tuples
}

/// C: mjCModel::Keys (user/user_model.h:310)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_keys(self_ptr: *mut mjCModel) -> *const () {
    todo!() // mjCModel::Keys
}

/// C: mjCModel::Plugins (user/user_model.h:311)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_plugins(self_ptr: *mut mjCModel) -> *const () {
    todo!() // mjCModel::Plugins
}

/// C: mjCModel::Bodies (user/user_model.h:312)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_bodies(self_ptr: *mut mjCModel) -> *const () {
    todo!() // mjCModel::Bodies
}

/// C: mjCModel::Geoms (user/user_model.h:313)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_geoms(self_ptr: *mut mjCModel) -> *const () {
    todo!() // mjCModel::Geoms
}

/// C: mjCModel::ResolvePlugin (user/user_model.h:316)
/// Calls: mjCModel::FindObject
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

/// C: mjCModel::DeleteMaterial (user/user_model.h:324)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_delete_material(self_ptr: *mut mjCModel, list: *const (), name: std__string_view) {
    todo!() // mjCModel::DeleteMaterial
}

/// C: mjCModel::SaveState (user/user_model.h:329)
/// Calls: mjCActuator::act, mjCActuator::ctrl, mjCBody::mpos, mjCBody::mquat, mjCJoint::nq, mjCJoint::nv, mjCJoint::qpos, mjCJoint::qvel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_save_state(self_ptr: *mut mjCModel, state_name: *const std__string, qpos: *const T, qvel: *const T, act: *const T, ctrl: *const T, mpos: *const T, mquat: *const T) {
    todo!() // mjCModel::SaveState
}

/// C: mjCModel::RestoreState (user/user_model.h:334)
/// Calls: mjCActuator::act, mjCActuator::ctrl, mjCBody::mpos, mjCBody::mquat, mjCJoint::nq, mjCJoint::nv, mjCJoint::qpos, mjCJoint::qvel, mjuu_defined
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
/// Calls: mjCModel::AddWarning, mjCModel::ComputeReference, mjCModel::SaveDofOffsets, mjCModel::SaveState
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_store_keyframes(self_ptr: *mut mjCModel, dest: *mut mjCModel) {
    todo!() // mjCModel::StoreKeyframes
}

/// C: mjCModel::SetDeepCopy (user/user_model.h:347)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_set_deep_copy(self_ptr: *mut mjCModel, deepcopy: bool) {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { (*self_ptr).deepcopy_ = deepcopy; }
}

/// C: mjCModel::SetAttached (user/user_model.h:350)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_set_attached(self_ptr: *mut mjCModel, deepcopy: bool) {
    // SAFETY: self_ptr is a valid pointer to mjCModel
    unsafe { (*self_ptr).attached_ |= !deepcopy; }
}

/// C: mjCModel::IsAttached (user/user_model.h:353)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_is_attached(self_ptr: *mut mjCModel) -> bool {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { (*self_ptr).attached_ }
}

/// C: mjCModel::CheckRepeat (user/user_model.h:356)
/// Calls: mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_check_repeat(self_ptr: *mut mjCModel, r#type: u32) {
    todo!() // mjCModel::CheckRepeat
}

/// C: mjCModel::AddRef (user/user_model.h:359)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_ref(self_ptr: *mut mjCModel) {
    // SAFETY: self_ptr is a valid pointer to mjCModel
    unsafe { (*self_ptr).refcount += 1; }
}

/// C: mjCModel::GetRef (user/user_model.h:360)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_ref(self_ptr: *mut mjCModel) -> i32 {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { (*self_ptr).refcount }
}

/// C: mjCModel::Release (user/user_model.h:361)
/// Calls: GlobalTable::count
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
/// Calls: DeleteTexcoord, mjCActuator::Compile, mjCBody::Compile, mjCBodyPair::Compile, mjCCamera::Compile, mjCDef::Compile, mjCEquality::Compile, mjCFlex::Compile, mjCHField::Compile, mjCHField::CopyFromSpec, mjCKey::Compile, mjCLight::Compile, mjCMaterial::Compile, mjCMesh::CopyFromSpec, mjCMesh::SetNeedHull, mjCModel::AddKey, mjCModel::AddWarning, mjCModel::AutoSpringDamper, mjCModel::CheckBodyMassInertia, mjCModel::CheckEmptyNames, mjCModel::ClearCompileWarnings, mjCModel::CompileMeshesAndTextures, mjCModel::ComputeSparseSizes, mjCModel::CopyNames, mjCModel::CopyObjects, mjCModel::CopyPaths, mjCModel::CopyPlugins, mjCModel::CopyTree, mjCModel::CountNJmom, mjCModel::DeleteAll, mjCModel::ExpandAllKeyframes, mjCModel::FinalizeSimple, mjCModel::FuseStatic, mjCModel::IndexAssets, mjCModel::LengthRange, mjCModel::ProcessLists, mjCModel::ResolveKeyframes, mjCModel::SaveDofOffsets, mjCModel::SetNuser, mjCModel::SetSizes, mjCModel::Signature, mjCNumeric::Compile, mjCPair::Compile, mjCPlugin::Compile, mjCSensor::Compile, mjCSkin::Compile, mjCSkin::CopyFromSpec, mjCTendon::Compile, mjCText::Compile, mjCTexture::CopyFromSpec, mjCTuple::Compile, mj_deleteData, mj_makeBSparse, mj_makeData, mj_makeDofDofMaps, mj_makeDofDofSparse, mj_makeModel, mj_makeRawData, mj_normalizeQuat, mj_resetData, mj_setConst, mj_setTotalmass, mj_step, mj_validateReferences, mjuu_copyvec, mjuu_defined, reassignid
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_try_compile(self_ptr: *mut mjCModel, m: *mut *mut mjModel, d: *mut *mut mjData, vfs: *const mjVFS) {
    todo!() // mjCModel::TryCompile
}

/// C: mjCModel::CompileMeshesAndTextures (user/user_model.h:381)
/// Calls: CompileMesh, CompileTexture, NumCompilerThreads, ThreadPool::Schedule, ThreadPool::WaitCount, strcat_arr, strcpy_arr
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
/// Calls: mjCGeom::IsVisual, mjCGeom::get_hfieldname, mjCGeom::get_material, mjCGeom::get_meshname, mjCMesh::IsVisual, mjCMesh::SetNotVisual, mjCModel::Delete, mjCModel::FindObject, mjCSite::get_material, mjuu_copyvec
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
/// Calls: getpathslength, mjCBody::GetParent, mjCBoundingVolumeHierarchy::Nbvh, mjCFlex::HasTexcoord, mjCFlex::get_texcoord, mjCJoint::GetParent, mjCJoint::nq, mjCJoint::nv, mjCMesh::HasTexcoord, mjCMesh::nface, mjCMesh::nnormal, mjCMesh::npolygon, mjCMesh::npolygonmap, mjCMesh::npolygonvert, mjCMesh::ntexcoord, mjCMesh::nvert, mjCMesh::octree, mjCMesh::szgraph, mjCMesh::tree, mjCOctree::NumNodes, mjCSkin::get_face, mjCSkin::get_texcoord, mjCSkin::get_vert, mjCSkin::get_vertid
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_set_sizes(self_ptr: *mut mjCModel) {
    todo!() // mjCModel::SetSizes
}

/// C: mjCModel::ComputeSparseSizes (user/user_model.h:387)
/// Calls: GetBodyIdFromWrap, IsNullPose, mjCBody::Bodies, mjCBody::GetParent, mjCJoint::nv, mjCWrap::Type
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
/// Calls: LRfunc, NumCompilerThreads, ThreadPool::Schedule, ThreadPool::WaitCount, mj_deleteData, mj_makeData, mj_setLengthRange
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_length_range(self_ptr: *mut mjCModel, arg0: *mut mjModel, arg1: *mut mjData) {
    todo!() // mjCModel::LengthRange
}

/// C: mjCModel::CopyNames (user/user_model.h:390)
/// Calls: mju_strncpy, namelist
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_names(self_ptr: *mut mjCModel, arg0: *mut mjModel) {
    todo!() // mjCModel::CopyNames
}

/// C: mjCModel::CopyPaths (user/user_model.h:391)
/// Calls: pathlist
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_paths(self_ptr: *mut mjCModel, arg0: *mut mjModel) {
    todo!() // mjCModel::CopyPaths
}

/// C: mjCModel::CopyObjects (user/user_model.h:392)
/// Calls: mjCActuator::get_userdata, mjCActuator::is_actlimited, mjCActuator::is_ctrllimited, mjCActuator::is_forcelimited, mjCBoundingVolumeHierarchy::Bvh, mjCBoundingVolumeHierarchy::Child, mjCBoundingVolumeHierarchy::Level, mjCBoundingVolumeHierarchy::Nbvh, mjCBoundingVolumeHierarchy::Nodeid, mjCBoundingVolumeHierarchy::Nodeidptr, mjCMesh::CopyFace, mjCMesh::CopyFaceNormal, mjCMesh::CopyFaceTexcoord, mjCMesh::CopyGraph, mjCMesh::CopyNormal, mjCMesh::CopyPolygonMap, mjCMesh::CopyPolygonNormals, mjCMesh::CopyPolygons, mjCMesh::CopyTexcoord, mjCMesh::CopyVert, mjCMesh::GetPosPtr, mjCMesh::GetQuatPtr, mjCMesh::HasTexcoord, mjCMesh::Scale, mjCMesh::nface, mjCMesh::nnormal, mjCMesh::npolygon, mjCMesh::npolygonmap, mjCMesh::npolygonvert, mjCMesh::ntexcoord, mjCMesh::nvert, mjCMesh::octree, mjCMesh::szgraph, mjCMesh::tree, mjCModel::AddWarning, mjCOctree::CopyAabb, mjCOctree::CopyChild, mjCOctree::CopyCoeff, mjCOctree::CopyLevel, mjCOctree::NumNodes, mjCSensor::get_userdata, mjCSkin::get_bindpos, mjCSkin::get_bindquat, mjCSkin::get_face, mjCSkin::get_texcoord, mjCSkin::get_vert, mjCSkin::get_vertid, mjCSkin::get_vertweight, mjCTendon::get_userdata, mjCTendon::is_actfrclimited, mjCTendon::is_limited, mjCWrap::Type, mju_strncpy, mjuu_copyvec, mjuu_normvec, mjuu_zerovec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_objects(self_ptr: *mut mjCModel, arg0: *mut mjModel) {
    todo!() // mjCModel::CopyObjects
}

/// C: mjCModel::CopyTree (user/user_model.h:393)
/// Calls: IsNullPose, IsSamePose, mjCBody::get_userdata, mjCBoundingVolumeHierarchy::Bvh, mjCBoundingVolumeHierarchy::Child, mjCBoundingVolumeHierarchy::Level, mjCBoundingVolumeHierarchy::Nbvh, mjCBoundingVolumeHierarchy::Nodeidptr, mjCCamera::get_userdata, mjCGeom::GetRBound, mjCGeom::get_userdata, mjCJoint::get_userdata, mjCJoint::is_actfrclimited, mjCJoint::is_limited, mjCJoint::nq, mjCJoint::nv, mjuu_copyvec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_tree(self_ptr: *mut mjCModel, arg0: *mut mjModel) {
    todo!() // mjCModel::CopyTree
}

/// C: mjCModel::FinalizeSimple (user/user_model.h:394)
/// Calls: GlobalTable::count
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
/// Calls: mjCModel::CountTendonDofs, mj_mergeChain
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
/// Calls: mjCBase::Release, mjCModel::MarkPluginInstance, mjCModel::ProcessList_
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_remove_plugins(self_ptr: *mut mjCModel) {
    todo!() // mjCModel::RemovePlugins
}

/// C: mjCModel::AddObject (user/user_model.h:437)
/// Calls: mjCModel::Signature
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_object(self_ptr: *mut mjCModel, list: *const (), r#type: std__string) -> *mut T {
    todo!() // mjCModel::AddObject
}

/// C: mjCModel::AddObjectDefault (user/user_model.h:440)
/// Calls: mjCModel::Signature
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_object_default(self_ptr: *mut mjCModel, list: *const (), r#type: std__string, def: *mut mjCDef) -> *mut T {
    todo!() // mjCModel::AddObjectDefault
}

/// C: mjCModel::CopyList (user/user_model.h:444)
/// Calls: FilePath::empty, FilePath::size, mjCAsset::References, mjCMesh::Plugin, mjCModel::FindSpec, mjCModel::ProcessList_
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_list(self_ptr: *mut mjCModel, dest: *const (), sources: *const ()) {
    todo!() // mjCModel::CopyList
}

/// C: mjCModel::CopyExplicitPlugin (user/user_model.h:448)
/// Calls: mjCBase::AddRef
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_explicit_plugin(self_ptr: *mut mjCModel, obj: *mut T) {
    todo!() // mjCModel::CopyExplicitPlugin
}

/// C: mjCModel::CopyPlugin (user/user_model.h:451)
/// Calls: mjCBase::NameSpace
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_plugin(self_ptr: *mut mjCModel, sources: *const (), list: *const ()) {
    todo!() // mjCModel::CopyPlugin
}

/// C: mjCModel::RemoveFromList (user/user_model.h:455)
/// Calls: FilePath::empty, FilePath::size, mjCAsset::References, mjCModel::ProcessList_
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_remove_from_list(self_ptr: *mut mjCModel, list: *const (), other: *const mjCModel) {
    todo!() // mjCModel::RemoveFromList
}

/// C: mjCModel::CreateObjectLists (user/user_model.h:458)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_create_object_lists(self_ptr: *mut mjCModel) {
    todo!() // mjCModel::CreateObjectLists
}

/// C: mjCModel::ProcessLists (user/user_model.h:461)
/// Calls: mjCModel::ProcessList_
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_process_lists(self_ptr: *mut mjCModel, checkrepeat: bool) {
    todo!() // mjCModel::ProcessLists
}

/// C: mjCModel::ProcessList_ (user/user_model.h:464)
/// Calls: mjCModel::CheckRepeat, mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_process_list(self_ptr: *mut mjCModel, ids: *mut mjListKeyMap, list: *const (), r#type: u32, checkrepeat: bool) {
    todo!() // mjCModel::ProcessList_
}

/// C: mjCModel::ResetTreeLists (user/user_model.h:468)
/// Calls: resetlist
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_reset_tree_lists(self_ptr: *mut mjCModel) {
    todo!() // mjCModel::ResetTreeLists
}

/// C: mjCModel::SaveDofOffsets (user/user_model.h:471)
/// Calls: mjCJoint::nq, mjCJoint::nv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_save_dof_offsets(self_ptr: *mut mjCModel, computesize: bool) {
    todo!() // mjCModel::SaveDofOffsets
}

/// C: mjCModel::ResolveKeyframes (user/user_model.h:474)
/// Calls: mjCModel::FindObject, mjCModel::RestoreState, mjCModel::SaveDofOffsets
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
/// Calls: mjuu_copyvec, mjuu_setvec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_compute_reference(self_ptr: *mut mjCModel) {
    todo!() // mjCModel::ComputeReference
}

/// C: mjCModel::CheckBodyMassInertia (user/user_model.h:483)
/// Calls: mjCBody::Bodies
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_check_body_mass_inertia(self_ptr: *mut mjCModel, body: *mut mjCBody) -> bool {
    todo!() // mjCModel::CheckBodyMassInertia
}

/// C: mjCModel::MarkPluginInstance (user/user_model.h:487)
/// Calls: FilePath::empty
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_mark_plugin_instance(self_ptr: *mut mjCModel, instances: *const (), list: *const ()) {
    todo!() // mjCModel::MarkPluginInstance
}

/// C: mjCModel::PrintTree (user/user_model.h:491)
/// Calls: PrintIndent, mjCJoint::nq
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_print_tree(self_ptr: *mut mjCModel, tree: *mut std__stringstream, body: *const mjCBody, depth: i32) {
    todo!() // mjCModel::PrintTree
}

/// C: mjCModel::Signature (user/user_model.h:494)
/// Calls: mjCModel::PrintTree, mj_hashString
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_signature(self_ptr: *mut mjCModel) -> u64 {
    todo!() // mjCModel::Signature
}

/// C: mjCModel::ReassignChild (user/user_model.h:498)
/// Calls: FilePath::size, changeframe
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_reassign_child(self_ptr: *mut mjCModel, dest: *const (), list: *const (), parent: *mut mjCBody, body: *mut mjCBody) {
    todo!() // mjCModel::ReassignChild
}

/// C: mjCModel::ResolveReferences (user/user_model.h:502)
/// Calls: mjCAsset::References
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_resolve_references(self_ptr: *mut mjCModel, list: *const (), body: *mut mjCBody) {
    todo!() // mjCModel::ResolveReferences
}

/// C: mjCModel::DeleteSubtreePlugin (user/user_model.h:505)
/// Calls: mjCBody::Bodies
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_delete_subtree_plugin(self_ptr: *mut mjCModel, subtree: *mut mjCBody) {
    todo!() // mjCModel::DeleteSubtreePlugin
}

/// C: mjCModel::ExpandAllKeyframes (user/user_model.h:508)
/// Calls: mjCModel::ComputeReference, mjCModel::ExpandKeyframe, mjCModel::SaveDofOffsets
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_expand_all_keyframes(self_ptr: *mut mjCModel) {
    todo!() // mjCModel::ExpandAllKeyframes
}

