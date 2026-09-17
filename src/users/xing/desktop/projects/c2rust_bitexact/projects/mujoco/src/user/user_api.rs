//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.cc
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_parse (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.cc:78)
/// Calls: cxx:__ZNK6mujoco4user8FilePath3ExtEv, cxx:_mj_defaultVFS, cxx:_mj_deleteVFS, cxx:_mj_parseXML, cxx:_mju_closeResource, cxx:_mju_decodeResource, cxx:_mju_free, cxx:_mju_malloc, cxx:_mju_openResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_parse(filename: *const i8, content_type: *const i8, vfs: *const mjVFS, error: *mut i8, error_sz: i32) -> *mut mjSpec {
    todo!() // cxx:_mj_parse
}

/// C: mj_encode (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.cc:151)
/// Calls: cxx:__ZNK6mujoco4user8FilePath3ExtEv, cxx:_mj_saveXML, cxx:_mjp_findEncoder
#[allow(unused_variables, non_snake_case)]
pub fn mj_encode(s: *const mjSpec, m: *const mjModel, filename: *const i8, content_type: *const i8, vfs: *const mjVFS, error: *mut i8, error_sz: i32) -> i32 {
    todo!() // cxx:_mj_encode
}

/// C: LogCompileTime (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.cc:212)
/// Calls: cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn log_compile_time(t: *const f64) {
    todo!() // cxx-internal:user_api.cc.o:__ZL14LogCompileTimePKd
}

/// C: SetFrame (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.cc:293)
/// Calls: cxx:_mjs_firstChild, cxx:_mjs_getFrame, cxx:_mjs_nextChild, cxx:_mjs_setFrame
#[allow(unused_variables, non_snake_case)]
pub fn set_frame(body: *mut mjsBody, objtype: u32, frame: *mut mjsFrame) {
    todo!() // cxx-internal:user_api.cc.o:__ZL8SetFrameP8mjsBody_6mjtObjP9mjsFrame_
}

/// C: attachBody (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.cc:306)
/// Calls: cxx:__ZN8mjCModel8SetErrorERK8mjCError
#[allow(unused_variables, non_snake_case)]
pub fn attach_body(parent: *mut mjCFrame, child: *const mjCBody, prefix: *const i8, suffix: *const i8) -> *mut mjsElement {
    todo!() // cxx-internal:user_api.cc.o:__ZL10attachBodyP8mjCFramePK7mjCBodyPKcS5_
}

/// C: attachFrame (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.cc:325)
/// Calls: cxx:__ZN8mjCModel8SetErrorERK8mjCError
#[allow(unused_variables, non_snake_case)]
pub fn attach_frame(parent: *mut mjCBody, child: *const mjCFrame, prefix: *const i8, suffix: *const i8) -> *mut mjsElement {
    todo!() // cxx-internal:user_api.cc.o:__ZL11attachFrameP7mjCBodyPK8mjCFramePKcS5_
}

/// C: attachToSite (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.cc:344)
/// Calls: cxx-internal:user_api.cc.o:__ZL10attachBodyP8mjCFramePK7mjCBodyPKcS5_, cxx:__ZN7mjCBody8AddFrameEP8mjCFrame, cxx:__ZN8mjCFrame9SetParentEP7mjCBody, cxx:__ZNK7mjCSite4BodyEv, cxx:_mjs_getSpec, cxx:_mjs_resolveOrientation
#[allow(unused_variables, non_snake_case)]
pub fn attach_to_site(parent: *mut mjCSite, child: *const mjCBody, prefix: *const i8, suffix: *const i8) -> *mut mjsElement {
    todo!() // cxx-internal:user_api.cc.o:__ZL12attachToSiteP7mjCSitePK7mjCBodyPKcS5_
}

/// C: attachFrameToSite (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.cc:365)
/// Calls: cxx-internal:user_api.cc.o:__ZL11attachFrameP7mjCBodyPK8mjCFramePKcS5_, cxx:__ZN7mjCBody8AddFrameEP8mjCFrame, cxx:__ZN8mjCFrame9SetParentEP7mjCBody, cxx:__ZNK7mjCSite4BodyEv, cxx:_mjs_getSpec, cxx:_mjs_resolveOrientation, cxx:_mjs_setFrame
#[allow(unused_variables, non_snake_case)]
pub fn attach_frame_to_site(parent: *mut mjCSite, child: *const mjCFrame, prefix: *const i8, suffix: *const i8) -> *mut mjsElement {
    todo!() // cxx-internal:user_api.cc.o:__ZL17attachFrameToSiteP7mjCSitePK8mjCFramePKcS5_
}

/// C: mjs_getTimer (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.cc:515)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_timer(s: *mut mjSpec) -> *const f64 {
    todo!() // cxx:_mjs_getTimer
}

/// C: mjs_numWarnings (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.cc:535)
/// Calls: cxx:__ZNK8mjCModel11GetWarningsEv
#[allow(unused_variables, non_snake_case)]
pub fn mjs_num_warnings(spec: *const mjSpec) -> i32 {
    todo!() // cxx:_mjs_numWarnings
}

/// C: mjs_getWarning (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.cc:544)
/// Calls: cxx:__ZNK8mjCModel11GetWarningsEv
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_warning(spec: *const mjSpec, index: i32) -> *const i8 {
    todo!() // cxx:_mjs_getWarning
}

/// C: FlexcompTypeFromStr (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.cc:709)
#[allow(unused_variables, non_snake_case)]
pub fn flexcomp_type_from_str(r#type: *const i8) -> mjtFcompType {
    todo!() // cxx-internal:user_api.cc.o:__ZL19FlexcompTypeFromStrPKc
}

/// C: FlexcompDofFromStr (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.cc:724)
#[allow(unused_variables, non_snake_case)]
pub fn flexcomp_dof_from_str(dof: *const i8) -> mjtDof {
    todo!() // cxx-internal:user_api.cc.o:__ZL18FlexcompDofFromStrPKc
}

/// C: mjs_getCompiler (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.cc:1553)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_compiler(element: *const mjsElement) -> *mut mjsCompiler {
    todo!() // cxx:_mjs_getCompiler
}

/// C: mj_makeSpec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:40)
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_spec() -> *mut mjSpec {
    todo!() // cxx:_mj_makeSpec
}

/// C: mj_compile (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:43)
/// Calls: cxx-internal:user_api.cc.o:__ZL14LogCompileTimePKd, cxx:__ZN8mjCModel7CompileEPK6mjVFS_PP8mjModel_
#[allow(unused_variables, non_snake_case)]
pub fn mj_compile(s: *mut mjSpec, vfs: *const mjVFS) -> *mut mjModel {
    todo!() // cxx:_mj_compile
}

/// C: mj_recompile (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:46)
/// Calls: cxx:__ZN8mjCModel12RestoreStateIdEEvRKNSt3__112basic_stringIcNS1_11char_traitsIcEENS1_9allocatorIcEEEEPKdSB_SB_PT_SD_SD_SD_SD_SD_, cxx:__ZN8mjCModel7CompileEPK6mjVFS_PP8mjModel_, cxx:__ZN8mjCModel8MakeDataEPK8mjModel_PP7mjData_, cxx:__ZN8mjCModel8SetErrorERK8mjCError, cxx:__ZN8mjCModel9SaveStateIdEEvRKNSt3__112basic_stringIcNS1_11char_traitsIcEENS1_9allocatorIcEEEEPKT_SC_SC_SC_SC_SC_, cxx:_mj_deleteData
#[allow(unused_variables, non_snake_case)]
pub fn mj_recompile(s: *mut mjSpec, vfs: *const mjVFS, m: *mut mjModel, d: *mut mjData) -> i32 {
    todo!() // cxx:_mj_recompile
}

/// C: mj_copySpec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:49)
/// Calls: cxx:__ZN8mjCModel8SetErrorERK8mjCError
#[allow(unused_variables, non_snake_case)]
pub fn mj_copy_spec(s: *const mjSpec) -> *mut mjSpec {
    todo!() // cxx:_mj_copySpec
}

/// C: mjs_getError (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:52)
/// Calls: cxx:__ZNK8mjCModel8GetErrorEv, cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_error(s: *mut mjSpec) -> *const i8 {
    todo!() // cxx:_mjs_getError
}

/// C: mjs_isWarning (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:55)
/// Calls: cxx:__ZNK8mjCModel11GetWarningsEv, cxx:__ZNK8mjCModel8GetErrorEv
#[allow(unused_variables, non_snake_case)]
pub fn mjs_is_warning(s: *mut mjSpec) -> i32 {
    todo!() // cxx:_mjs_isWarning
}

/// C: mj_deleteSpec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:58)
/// Calls: cxx:__ZN8mjCModel7ReleaseEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_delete_spec(s: *mut mjSpec) {
    todo!() // cxx:_mj_deleteSpec
}

/// C: mjs_addSpec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:61)
/// Calls: cxx:__ZN8mjCModel10AppendSpecEP7mjSpec_PK12mjsCompiler_
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_spec(s: *mut mjSpec, child: *mut mjSpec) {
    todo!() // cxx:_mjs_addSpec
}

/// C: mjs_activatePlugin (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:64)
/// Calls: cxx:__ZN8mjCModel14ActivatePluginEPK10mjpPlugin_i, cxx:_mjp_getPlugin
#[allow(unused_variables, non_snake_case)]
pub fn mjs_activate_plugin(s: *mut mjSpec, name: *const i8) -> i32 {
    todo!() // cxx:_mjs_activatePlugin
}

/// C: mjs_setDeepCopy (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:67)
/// Calls: cxx:__ZN8mjCModel11SetDeepCopyEb
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_deep_copy(s: *mut mjSpec, deepcopy: i32) -> i32 {
    todo!() // cxx:_mjs_setDeepCopy
}

/// C: mj_copyBack (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:70)
/// Calls: cxx:__ZN8mjCModel8CopyBackEPK8mjModel_
#[allow(unused_variables, non_snake_case)]
pub fn mj_copy_back(s: *mut mjSpec, m: *const mjModel) -> i32 {
    todo!() // cxx:_mj_copyBack
}

/// C: mjs_attach (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:76)
/// Calls: cxx-internal:user_api.cc.o:__ZL10attachBodyP8mjCFramePK7mjCBodyPKcS5_, cxx-internal:user_api.cc.o:__ZL11attachFrameP7mjCBodyPK8mjCFramePKcS5_, cxx-internal:user_api.cc.o:__ZL12attachToSiteP7mjCSitePK7mjCBodyPKcS5_, cxx-internal:user_api.cc.o:__ZL17attachFrameToSiteP7mjCSitePK8mjCFramePKcS5_, cxx-internal:user_api.cc.o:__ZL8SetFrameP8mjsBody_6mjtObjP9mjsFrame_, cxx:__ZN6mujoco16ResolveConflictsEP7mjSpec_PKS0_11mjtConflictPNSt3__112basic_stringIcNS5_11char_traitsIcEENS5_9allocatorIcEEEESC_SC_, cxx:__ZN8mjCModel17AddGroupedWarningERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEES8_, cxx:__ZN8mjCModel24SetAttachWarningBoundaryEv, cxx:__ZN8mjCModel8SetErrorERK8mjCError, cxx:_mjs_addFrame, cxx:_mjs_findBody, cxx:_mjs_getParent, cxx:_mjs_getSpec, cxx:_mjs_setFrame, cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjs_attach(parent: *mut mjsElement, child: *const mjsElement, prefix: *const i8, suffix: *const i8) -> *mut mjsElement {
    todo!() // cxx:_mjs_attach
}

/// C: mjs_addBody (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:83)
/// Calls: cxx:__ZN7mjCBody7AddBodyEP6mjCDef
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_body(body: *mut mjsBody, def: *const mjsDefault) -> *mut mjsBody {
    todo!() // cxx:_mjs_addBody
}

/// C: mjs_addSite (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:86)
/// Calls: cxx:__ZN7mjCBody7AddSiteEP6mjCDef
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_site(body: *mut mjsBody, def: *const mjsDefault) -> *mut mjsSite {
    todo!() // cxx:_mjs_addSite
}

/// C: mjs_addJoint (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:89)
/// Calls: cxx:__ZN7mjCBody8AddJointEP6mjCDef
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_joint(body: *mut mjsBody, def: *const mjsDefault) -> *mut mjsJoint {
    todo!() // cxx:_mjs_addJoint
}

/// C: mjs_addFreeJoint (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:92)
/// Calls: cxx:__ZN7mjCBody12AddFreeJointEv
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_free_joint(body: *mut mjsBody) -> *mut mjsJoint {
    todo!() // cxx:_mjs_addFreeJoint
}

/// C: mjs_addGeom (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:95)
/// Calls: cxx:__ZN7mjCBody7AddGeomEP6mjCDef
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_geom(body: *mut mjsBody, def: *const mjsDefault) -> *mut mjsGeom {
    todo!() // cxx:_mjs_addGeom
}

/// C: mjs_addCamera (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:98)
/// Calls: cxx:__ZN7mjCBody9AddCameraEP6mjCDef
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_camera(body: *mut mjsBody, def: *const mjsDefault) -> *mut mjsCamera {
    todo!() // cxx:_mjs_addCamera
}

/// C: mjs_addLight (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:101)
/// Calls: cxx:__ZN7mjCBody8AddLightEP6mjCDef
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_light(body: *mut mjsBody, def: *const mjsDefault) -> *mut mjsLight {
    todo!() // cxx:_mjs_addLight
}

/// C: mjs_addFrame (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:104)
/// Calls: cxx:__ZN7mjCBody8AddFrameEP8mjCFrame, cxx:__ZN8mjCFrame9SetParentEP7mjCBody
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_frame(body: *mut mjsBody, parentframe: *mut mjsFrame) -> *mut mjsFrame {
    todo!() // cxx:_mjs_addFrame
}

/// C: mjs_delete (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:107)
/// Calls: cxx:__ZN8mjCModel8SetErrorERK8mjCError, cxx:__ZNK8mjCModel10IsAttachedEv
#[allow(unused_variables, non_snake_case)]
pub fn mjs_delete(s: *mut mjSpec, element: *mut mjsElement) -> i32 {
    todo!() // cxx:_mjs_delete
}

/// C: mjs_addActuator (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:113)
/// Calls: cxx:__ZN8mjCModel11AddActuatorEP6mjCDef
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_actuator(s: *mut mjSpec, def: *const mjsDefault) -> *mut mjsActuator {
    todo!() // cxx:_mjs_addActuator
}

/// C: mjs_addSensor (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:116)
/// Calls: cxx:__ZN8mjCModel9AddSensorEv
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_sensor(s: *mut mjSpec) -> *mut mjsSensor {
    todo!() // cxx:_mjs_addSensor
}

/// C: mjs_addFlex (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:119)
/// Calls: cxx:__ZN8mjCModel7AddFlexEv
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_flex(s: *mut mjSpec) -> *mut mjsFlex {
    todo!() // cxx:_mjs_addFlex
}

/// C: mjs_makeFlex (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:122)
/// Calls: cxx-internal:user_api.cc.o:__ZL18FlexcompDofFromStrPKc, cxx-internal:user_api.cc.o:__ZL19FlexcompTypeFromStrPKc, cxx:__ZN11mjCFlexcomp4MakeEP8mjsBody_PciPK6mjVFS_, cxx:__ZN8mjCModel8SetErrorERK8mjCError, cxx:__ZNK8mjCModel6FlexesEv, cxx:_mju_error
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_make_flex(body: *mut mjsBody, name: *const i8, r#type: *const i8, dim: i32, dof: *const i8, count: *const i32, cellcount: *const i32, spacing: *const f64, scale: *const f64, radius: f64, mass: f64, inertiabox: f64, equality: i32, rigid: i32, flatskin: i32, elastic2d: i32, pos: *const f64, quat: *const f64, origin: *const f64, file: *const i8, vfs: *const mjVFS) -> *mut mjsFlex {
    todo!() // cxx:_mjs_makeFlex
}

/// C: mjs_addPair (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:130)
/// Calls: cxx:__ZN8mjCModel7AddPairEP6mjCDef
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_pair(s: *mut mjSpec, def: *const mjsDefault) -> *mut mjsPair {
    todo!() // cxx:_mjs_addPair
}

/// C: mjs_addExclude (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:133)
/// Calls: cxx:__ZN8mjCModel10AddExcludeEv
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_exclude(s: *mut mjSpec) -> *mut mjsExclude {
    todo!() // cxx:_mjs_addExclude
}

/// C: mjs_addEquality (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:136)
/// Calls: cxx:__ZN8mjCModel11AddEqualityEP6mjCDef
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_equality(s: *mut mjSpec, def: *const mjsDefault) -> *mut mjsEquality {
    todo!() // cxx:_mjs_addEquality
}

/// C: mjs_addTendon (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:139)
/// Calls: cxx:__ZN8mjCModel9AddTendonEP6mjCDef
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_tendon(s: *mut mjSpec, def: *const mjsDefault) -> *mut mjsTendon {
    todo!() // cxx:_mjs_addTendon
}

/// C: mjs_wrapSite (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:142)
/// Calls: cxx:__ZN9mjCTendon8WrapSiteENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEENS0_17basic_string_viewIcS3_EE
#[allow(unused_variables, non_snake_case)]
pub fn mjs_wrap_site(tendon: *mut mjsTendon, name: *const i8) -> *mut mjsWrap {
    todo!() // cxx:_mjs_wrapSite
}

/// C: mjs_wrapGeom (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:145)
/// Calls: cxx:__ZN9mjCTendon8WrapGeomENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEES6_NS0_17basic_string_viewIcS3_EE
#[allow(unused_variables, non_snake_case)]
pub fn mjs_wrap_geom(tendon: *mut mjsTendon, name: *const i8, sidesite: *const i8) -> *mut mjsWrap {
    todo!() // cxx:_mjs_wrapGeom
}

/// C: mjs_wrapJoint (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:148)
/// Calls: cxx:__ZN9mjCTendon9WrapJointENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEdNS0_17basic_string_viewIcS3_EE
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_wrap_joint(tendon: *mut mjsTendon, name: *const i8, coef: f64) -> *mut mjsWrap {
    todo!() // cxx:_mjs_wrapJoint
}

/// C: mjs_wrapPulley (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:151)
/// Calls: cxx:__ZN9mjCTendon10WrapPulleyEdNSt3__117basic_string_viewIcNS0_11char_traitsIcEEEE
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_wrap_pulley(tendon: *mut mjsTendon, divisor: f64) -> *mut mjsWrap {
    todo!() // cxx:_mjs_wrapPulley
}

/// C: mjs_addNumeric (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:154)
/// Calls: cxx:__ZN8mjCModel10AddNumericEv
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_numeric(s: *mut mjSpec) -> *mut mjsNumeric {
    todo!() // cxx:_mjs_addNumeric
}

/// C: mjs_addText (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:157)
/// Calls: cxx:__ZN8mjCModel7AddTextEv
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_text(s: *mut mjSpec) -> *mut mjsText {
    todo!() // cxx:_mjs_addText
}

/// C: mjs_addTuple (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:160)
/// Calls: cxx:__ZN8mjCModel8AddTupleEv
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_tuple(s: *mut mjSpec) -> *mut mjsTuple {
    todo!() // cxx:_mjs_addTuple
}

/// C: mjs_addKey (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:163)
/// Calls: cxx:__ZN8mjCModel6AddKeyEv
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_key(s: *mut mjSpec) -> *mut mjsKey {
    todo!() // cxx:_mjs_addKey
}

/// C: mjs_addPlugin (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:166)
/// Calls: cxx:__ZN8mjCModel9AddPluginEv
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_plugin(s: *mut mjSpec) -> *mut mjsPlugin {
    todo!() // cxx:_mjs_addPlugin
}

/// C: mjs_addDefault (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:169)
/// Calls: cxx:__ZNK8mjCModel7DefaultEv
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_default(s: *mut mjSpec, classname: *const i8, parent: *const mjsDefault) -> *mut mjsDefault {
    todo!() // cxx:_mjs_addDefault
}

/// C: mjs_setToMotor (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:175)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_to_motor(actuator: *mut mjsActuator) -> *const i8 {
    todo!() // cxx:_mjs_setToMotor
}

/// C: mjs_setToPosition (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:178)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_to_position(actuator: *mut mjsActuator, kp: f64, kv: *mut f64, dampratio: *mut f64, timeconst: *mut f64, inheritrange: f64) -> *const i8 {
    todo!() // cxx:_mjs_setToPosition
}

/// C: mjs_setToIntVelocity (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:182)
/// Calls: cxx:_mjs_setToPosition
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_to_int_velocity(actuator: *mut mjsActuator, kp: f64, kv: *mut f64, dampratio: *mut f64, timeconst: *mut f64, inheritrange: f64) -> *const i8 {
    todo!() // cxx:_mjs_setToIntVelocity
}

/// C: mjs_setToVelocity (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:186)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_to_velocity(actuator: *mut mjsActuator, kv: f64) -> *const i8 {
    todo!() // cxx:_mjs_setToVelocity
}

/// C: mjs_setToDamper (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:189)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_to_damper(actuator: *mut mjsActuator, kv: f64) -> *const i8 {
    todo!() // cxx:_mjs_setToDamper
}

/// C: mjs_setToCylinder (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:192)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_to_cylinder(actuator: *mut mjsActuator, timeconst: f64, bias: f64, area: f64, diameter: f64) -> *const i8 {
    todo!() // cxx:_mjs_setToCylinder
}

/// C: mjs_setToMuscle (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:196)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_to_muscle(actuator: *mut mjsActuator, timeconst: *mut f64, tausmooth: f64, range: *mut f64, force: f64, scale: f64, lmin: f64, lmax: f64, vmax: f64, fpmax: f64, fvmax: f64) -> *const i8 {
    todo!() // cxx:_mjs_setToMuscle
}

/// C: mjs_setToAdhesion (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:201)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_to_adhesion(actuator: *mut mjsActuator, gain: f64) -> *const i8 {
    todo!() // cxx:_mjs_setToAdhesion
}

/// C: mjs_setToDCMotor (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:204)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_to_dc_motor(actuator: *mut mjsActuator, motorconst: *mut f64, resistance: f64, nominal: *mut f64, saturation: *mut f64, inductance: *mut f64, cogging: *mut f64, controller: *mut f64, thermal: *mut f64, lugre: *mut f64, input_mode: i32) -> *const i8 {
    todo!() // cxx:_mjs_setToDCMotor
}

/// C: mjs_addMesh (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:213)
/// Calls: cxx:__ZN8mjCModel7AddMeshEP6mjCDef
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_mesh(s: *mut mjSpec, def: *const mjsDefault) -> *mut mjsMesh {
    todo!() // cxx:_mjs_addMesh
}

/// C: mjs_addHField (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:216)
/// Calls: cxx:__ZN8mjCModel9AddHFieldEv
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_h_field(s: *mut mjSpec) -> *mut mjsHField {
    todo!() // cxx:_mjs_addHField
}

/// C: mjs_addSkin (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:219)
/// Calls: cxx:__ZN8mjCModel7AddSkinEv
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_skin(s: *mut mjSpec) -> *mut mjsSkin {
    todo!() // cxx:_mjs_addSkin
}

/// C: mjs_addTexture (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:222)
/// Calls: cxx:__ZN8mjCModel10AddTextureEv
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_texture(s: *mut mjSpec) -> *mut mjsTexture {
    todo!() // cxx:_mjs_addTexture
}

/// C: mjs_addMaterial (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:225)
/// Calls: cxx:__ZN8mjCModel11AddMaterialEP6mjCDef
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_material(s: *mut mjSpec, def: *const mjsDefault) -> *mut mjsMaterial {
    todo!() // cxx:_mjs_addMaterial
}

/// C: mjs_makeMesh (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:228)
/// Calls: cxx:__ZN7mjCMesh10MakeSphereEib, cxx:__ZN7mjCMesh14MakeHemisphereEibb, cxx:__ZN7mjCMesh14MakeSupertorusEiddd, cxx:__ZN7mjCMesh15MakeSupersphereEidd, cxx:__ZN7mjCMesh8MakeConeEid, cxx:__ZN7mjCMesh8MakeRectEPi, cxx:__ZN7mjCMesh9MakeWedgeEPiPdd, cxx:__ZN8mjCModel8SetErrorERK8mjCError
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_make_mesh(mesh: *mut mjsMesh, builtin: u32, params: *mut f64, nparams: i32) -> i32 {
    todo!() // cxx:_mjs_makeMesh
}

/// C: mjs_getSpec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:233)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_spec(element: *const mjsElement) -> *mut mjSpec {
    todo!() // cxx:_mjs_getSpec
}

/// C: mjs_getOriginSpec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:237)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_origin_spec(element: *const mjsElement) -> *mut mjSpec {
    todo!() // cxx:_mjs_getOriginSpec
}

/// C: mjs_findSpec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:240)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_find_spec(spec: *const mjSpec, name: *const i8) -> *mut mjSpec {
    todo!() // cxx:_mjs_findSpec
}

/// C: mjs_findBody (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:243)
/// Calls: cxx:_mjs_findElement
#[allow(unused_variables, non_snake_case)]
pub fn mjs_find_body(s: *const mjSpec, name: *const i8) -> *mut mjsBody {
    todo!() // cxx:_mjs_findBody
}

/// C: mjs_findElement (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:246)
/// Calls: cxx:__ZN8mjCModel8FindTreeEP7mjCBody6mjtObjNSt3__112basic_stringIcNS3_11char_traitsIcEENS3_9allocatorIcEEEE, cxx:__ZN8mjCModel8GetWorldEv, cxx:__ZNK8mjCModel10IsCompiledEv, cxx:__ZNK8mjCModel6MeshesEv, cxx:__ZNK8mjCModel8TexturesEv
#[allow(unused_variables, non_snake_case)]
pub fn mjs_find_element(s: *const mjSpec, r#type: u32, name: *const i8) -> *mut mjsElement {
    todo!() // cxx:_mjs_findElement
}

/// C: mjs_findChild (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:249)
/// Calls: cxx:__ZNK7mjCBody10FindObjectE6mjtObjRKNSt3__112basic_stringIcNS1_11char_traitsIcEENS1_9allocatorIcEEEEb
#[allow(unused_variables, non_snake_case)]
pub fn mjs_find_child(body: *const mjsBody, name: *const i8) -> *mut mjsBody {
    todo!() // cxx:_mjs_findChild
}

/// C: mjs_getParent (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:252)
/// Calls: cxx:__ZNK7mjCBody9GetParentEv, cxx:__ZNK7mjCGeom9GetParentEv, cxx:__ZNK7mjCSite9GetParentEv, cxx:__ZNK8mjCFrame9GetParentEv, cxx:__ZNK8mjCJoint9GetParentEv, cxx:__ZNK8mjCLight9GetParentEv, cxx:__ZNK9mjCCamera9GetParentEv
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_parent(element: *const mjsElement) -> *mut mjsBody {
    todo!() // cxx:_mjs_getParent
}

/// C: mjs_getFrame (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:255)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_frame(element: *const mjsElement) -> *mut mjsFrame {
    todo!() // cxx:_mjs_getFrame
}

/// C: mjs_findFrame (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:258)
/// Calls: cxx:_mjs_findElement
#[allow(unused_variables, non_snake_case)]
pub fn mjs_find_frame(s: *const mjSpec, name: *const i8) -> *mut mjsFrame {
    todo!() // cxx:_mjs_findFrame
}

/// C: mjs_getDefault (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:261)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_default(element: *const mjsElement) -> *mut mjsDefault {
    todo!() // cxx:_mjs_getDefault
}

/// C: mjs_findDefault (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:264)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_find_default(s: *const mjSpec, classname: *const i8) -> *mut mjsDefault {
    todo!() // cxx:_mjs_findDefault
}

/// C: mjs_getSpecDefault (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:267)
/// Calls: cxx:__ZNK8mjCModel7DefaultEv
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_spec_default(s: *const mjSpec) -> *mut mjsDefault {
    todo!() // cxx:_mjs_getSpecDefault
}

/// C: mjs_getId (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:270)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_id(element: *const mjsElement) -> i32 {
    todo!() // cxx:_mjs_getId
}

/// C: mjs_firstChild (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:276)
/// Calls: cxx:__ZN8mjCModel8SetErrorERK8mjCError, cxx:__ZNK7mjCBody9NextChildEPK11mjsElement_6mjtObjb
#[allow(unused_variables, non_snake_case)]
pub fn mjs_first_child(body: *const mjsBody, r#type: u32, recurse: i32) -> *mut mjsElement {
    todo!() // cxx:_mjs_firstChild
}

/// C: mjs_nextChild (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:280)
/// Calls: cxx:__ZN8mjCModel8SetErrorERK8mjCError, cxx:__ZNK7mjCBody9NextChildEPK11mjsElement_6mjtObjb
#[allow(unused_variables, non_snake_case)]
pub fn mjs_next_child(body: *const mjsBody, child: *const mjsElement, recurse: i32) -> *mut mjsElement {
    todo!() // cxx:_mjs_nextChild
}

/// C: mjs_firstElement (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:283)
/// Calls: cxx:__ZNK8mjCModel10NextObjectEPK11mjsElement_6mjtObj
#[allow(unused_variables, non_snake_case)]
pub fn mjs_first_element(s: *const mjSpec, r#type: u32) -> *mut mjsElement {
    todo!() // cxx:_mjs_firstElement
}

/// C: mjs_nextElement (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:286)
/// Calls: cxx:__ZNK8mjCModel10NextObjectEPK11mjsElement_6mjtObj
#[allow(unused_variables, non_snake_case)]
pub fn mjs_next_element(s: *const mjSpec, element: *const mjsElement) -> *mut mjsElement {
    todo!() // cxx:_mjs_nextElement
}

/// C: mjs_getWrapTarget (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:289)
/// Calls: cxx:__ZNK7mjCWrap4TypeEv, cxx:_mjs_findElement, cxx:_mjs_getSpec
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_wrap_target(wrap: *const mjsWrap) -> *mut mjsElement {
    todo!() // cxx:_mjs_getWrapTarget
}

/// C: mjs_getWrapSideSite (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:292)
/// Calls: cxx:__ZNK7mjCWrap4TypeEv, cxx:_mjs_asSite, cxx:_mjs_findElement, cxx:_mjs_getSpec, cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_wrap_side_site(wrap: *const mjsWrap) -> *mut mjsSite {
    todo!() // cxx:_mjs_getWrapSideSite
}

/// C: mjs_getWrapDivisor (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:295)
/// Calls: cxx:__ZNK7mjCWrap4TypeEv, cxx:_mju_warning
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_wrap_divisor(wrap: *const mjsWrap) -> f64 {
    todo!() // cxx:_mjs_getWrapDivisor
}

/// C: mjs_getWrapCoef (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:298)
/// Calls: cxx:__ZNK7mjCWrap4TypeEv, cxx:_mju_warning
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_wrap_coef(wrap: *const mjsWrap) -> f64 {
    todo!() // cxx:_mjs_getWrapCoef
}

/// C: mjs_asBody (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:301)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_body(element: *mut mjsElement) -> *mut mjsBody {
    todo!() // cxx:_mjs_asBody
}

/// C: mjs_asGeom (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:304)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_geom(element: *mut mjsElement) -> *mut mjsGeom {
    todo!() // cxx:_mjs_asGeom
}

/// C: mjs_asJoint (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:307)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_joint(element: *mut mjsElement) -> *mut mjsJoint {
    todo!() // cxx:_mjs_asJoint
}

/// C: mjs_asSite (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:310)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_site(element: *mut mjsElement) -> *mut mjsSite {
    todo!() // cxx:_mjs_asSite
}

/// C: mjs_asCamera (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:313)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_camera(element: *mut mjsElement) -> *mut mjsCamera {
    todo!() // cxx:_mjs_asCamera
}

/// C: mjs_asLight (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:316)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_light(element: *mut mjsElement) -> *mut mjsLight {
    todo!() // cxx:_mjs_asLight
}

/// C: mjs_asFrame (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:319)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_frame(element: *mut mjsElement) -> *mut mjsFrame {
    todo!() // cxx:_mjs_asFrame
}

/// C: mjs_asActuator (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:322)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_actuator(element: *mut mjsElement) -> *mut mjsActuator {
    todo!() // cxx:_mjs_asActuator
}

/// C: mjs_asSensor (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:325)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_sensor(element: *mut mjsElement) -> *mut mjsSensor {
    todo!() // cxx:_mjs_asSensor
}

/// C: mjs_asFlex (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:328)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_flex(element: *mut mjsElement) -> *mut mjsFlex {
    todo!() // cxx:_mjs_asFlex
}

/// C: mjs_asPair (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:331)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_pair(element: *mut mjsElement) -> *mut mjsPair {
    todo!() // cxx:_mjs_asPair
}

/// C: mjs_asEquality (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:334)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_equality(element: *mut mjsElement) -> *mut mjsEquality {
    todo!() // cxx:_mjs_asEquality
}

/// C: mjs_asExclude (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:337)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_exclude(element: *mut mjsElement) -> *mut mjsExclude {
    todo!() // cxx:_mjs_asExclude
}

/// C: mjs_asTendon (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:340)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_tendon(element: *mut mjsElement) -> *mut mjsTendon {
    todo!() // cxx:_mjs_asTendon
}

/// C: mjs_asNumeric (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:343)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_numeric(element: *mut mjsElement) -> *mut mjsNumeric {
    todo!() // cxx:_mjs_asNumeric
}

/// C: mjs_asText (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:346)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_text(element: *mut mjsElement) -> *mut mjsText {
    todo!() // cxx:_mjs_asText
}

/// C: mjs_asTuple (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:349)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_tuple(element: *mut mjsElement) -> *mut mjsTuple {
    todo!() // cxx:_mjs_asTuple
}

/// C: mjs_asKey (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:352)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_key(element: *mut mjsElement) -> *mut mjsKey {
    todo!() // cxx:_mjs_asKey
}

/// C: mjs_asMesh (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:355)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_mesh(element: *mut mjsElement) -> *mut mjsMesh {
    todo!() // cxx:_mjs_asMesh
}

/// C: mjs_asHField (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:358)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_h_field(element: *mut mjsElement) -> *mut mjsHField {
    todo!() // cxx:_mjs_asHField
}

/// C: mjs_asSkin (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:361)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_skin(element: *mut mjsElement) -> *mut mjsSkin {
    todo!() // cxx:_mjs_asSkin
}

/// C: mjs_asTexture (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:364)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_texture(element: *mut mjsElement) -> *mut mjsTexture {
    todo!() // cxx:_mjs_asTexture
}

/// C: mjs_asMaterial (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:367)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_material(element: *mut mjsElement) -> *mut mjsMaterial {
    todo!() // cxx:_mjs_asMaterial
}

/// C: mjs_asPlugin (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:370)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_plugin(element: *mut mjsElement) -> *mut mjsPlugin {
    todo!() // cxx:_mjs_asPlugin
}

/// C: mjs_setName (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:376)
/// Calls: cxx:__ZN8mjCModel11CheckRepeatE6mjtObj, cxx:__ZN8mjCModel8SetErrorERK8mjCError
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_name(element: *mut mjsElement, name: *const i8) -> i32 {
    todo!() // cxx:_mjs_setName
}

/// C: mjs_setBuffer (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:379)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_buffer(dest: *mut mjByteVec, array: *const (), size: i32) {
    todo!() // cxx:_mjs_setBuffer
}

/// C: mjs_setString (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:382)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_string(dest: *mut mjString, text: *const i8) {
    todo!() // cxx:_mjs_setString
}

/// C: mjs_setStringVec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:385)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_string_vec(dest: *mut mjStringVec, text: *const i8) {
    todo!() // cxx:_mjs_setStringVec
}

/// C: mjs_setInStringVec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:388)
/// Calls: cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_in_string_vec(dest: *mut mjStringVec, i: i32, text: *const i8) -> bool {
    todo!() // cxx:_mjs_setInStringVec
}

/// C: mjs_appendString (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:391)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_append_string(dest: *mut mjStringVec, text: *const i8) {
    todo!() // cxx:_mjs_appendString
}

/// C: mjs_setInt (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:394)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_int(dest: *mut mjIntVec, array: *const i32, size: i32) {
    todo!() // cxx:_mjs_setInt
}

/// C: mjs_appendIntVec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:397)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_append_int_vec(dest: *mut mjIntVecVec, array: *const i32, size: i32) {
    todo!() // cxx:_mjs_appendIntVec
}

/// C: mjs_setFloat (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:400)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_float(dest: *mut mjFloatVec, array: *const f32, size: i32) {
    todo!() // cxx:_mjs_setFloat
}

/// C: mjs_appendFloatVec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:403)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_append_float_vec(dest: *mut mjFloatVecVec, array: *const f32, size: i32) {
    todo!() // cxx:_mjs_appendFloatVec
}

/// C: mjs_setDouble (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:406)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_double(dest: *mut mjDoubleVec, array: *const f64, size: i32) {
    todo!() // cxx:_mjs_setDouble
}

/// C: mjs_setPluginAttributes (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:409)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_plugin_attributes(plugin: *mut mjsPlugin, attributes: *mut ()) {
    todo!() // cxx:_mjs_setPluginAttributes
}

/// C: mjs_getName (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:415)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_name(element: *mut mjsElement) -> *mut mjString {
    todo!() // cxx:_mjs_getName
}

/// C: mjs_getString (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:418)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_string(source: *const mjString) -> *const i8 {
    todo!() // cxx:_mjs_getString
}

/// C: mjs_getDouble (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:421)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_double(source: *const mjDoubleVec, size: *mut i32) -> *const f64 {
    todo!() // cxx:_mjs_getDouble
}

/// C: mjs_getWrapNum (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:424)
/// Calls: cxx:__ZNK9mjCTendon8NumWrapsEv
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_wrap_num(tendonspec: *const mjsTendon) -> i32 {
    todo!() // cxx:_mjs_getWrapNum
}

/// C: mjs_getWrap (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:426)
/// Calls: cxx:__ZNK9mjCTendon7GetWrapEi, cxx:__ZNK9mjCTendon8NumWrapsEv, cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_wrap(tendonspec: *const mjsTendon, i: i32) -> *mut mjsWrap {
    todo!() // cxx:_mjs_getWrap
}

/// C: mjs_getPluginAttributes (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:429)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_plugin_attributes(plugin: *const mjsPlugin) -> *const () {
    todo!() // cxx:_mjs_getPluginAttributes
}

/// C: mjs_isAuthored (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:435)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_is_authored(elem_ptr: *const (), field_ptr: *const ()) -> i32 {
    todo!() // cxx:_mjs_isAuthored
}

/// C: mjs_setAuthored (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:438)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_authored(elem_ptr: *const (), field_ptr: *const (), authored: i32) {
    todo!() // cxx:_mjs_setAuthored
}

/// C: mjs_setDefault (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:441)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_default(element: *mut mjsElement, def: *const mjsDefault) {
    todo!() // cxx:_mjs_setDefault
}

/// C: mjs_setFrame (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:444)
/// Calls: cxx:__ZN7mjCBase8SetFrameEP8mjCFrame, cxx:__ZN8mjCModel8SetErrorERK8mjCError
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_frame(dest: *mut mjsElement, frame: *mut mjsFrame) -> i32 {
    todo!() // cxx:_mjs_setFrame
}

/// C: mjs_resolveOrientation (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:447)
/// Calls: cxx:__Z18ResolveOrientationPdbPKcRK15mjsOrientation_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_resolve_orientation(quat: *mut f64, degree: u8, sequence: *const i8, orientation: *const mjsOrientation) -> *const i8 {
    todo!() // cxx:_mjs_resolveOrientation
}

/// C: mjs_bodyToFrame (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:451)
/// Calls: cxx:__ZN7mjCBody7ToFrameEv
#[allow(unused_variables, non_snake_case)]
pub fn mjs_body_to_frame(body: *mut *mut mjsBody) -> *mut mjsFrame {
    todo!() // cxx:_mjs_bodyToFrame
}

/// C: mjs_setUserValue (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:454)
/// Calls: cxx:_mjs_setUserValueWithCleanup
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_user_value(element: *mut mjsElement, key: *const i8, data: *const ()) {
    todo!() // cxx:_mjs_setUserValue
}

/// C: mjs_setUserValueWithCleanup (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:457)
/// Calls: cxx:__ZN7mjCBase12SetUserValueENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEEPKvPFvS6_E
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_user_value_with_cleanup(element: *mut mjsElement, key: *const i8, data: *const (), cleanup: Option<unsafe extern "C" fn(*const ())>) {
    todo!() // cxx:_mjs_setUserValueWithCleanup
}

/// C: mjs_getUserValue (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:462)
/// Calls: cxx:__ZN7mjCBase12GetUserValueENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEE
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_user_value(element: *mut mjsElement, key: *const i8) -> *const () {
    todo!() // cxx:_mjs_getUserValue
}

/// C: mjs_deleteUserValue (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:465)
/// Calls: cxx:__ZN7mjCBase15DeleteUserValueENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEE
#[allow(unused_variables, non_snake_case)]
pub fn mjs_delete_user_value(element: *mut mjsElement, key: *const i8) {
    todo!() // cxx:_mjs_deleteUserValue
}

/// C: mjs_sensorDim (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:468)
/// Calls: cxx:__ZN9mjCSensor7get_objEv, cxx:__ZNK7mjCMesh5nvertEv, cxx:_mju_condataSize, cxx:_mju_raydataSize
#[allow(unused_variables, non_snake_case)]
pub fn mjs_sensor_dim(sensor: *const mjsSensor) -> i32 {
    todo!() // cxx:_mjs_sensorDim
}

/// C: mj_getCacheCapacity (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:551)
/// Calls: cxx:__ZNK8mjCCache8CapacityEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_get_cache_capacity(cache: *const mjCache) -> usize {
    todo!() // cxx:_mj_getCacheCapacity
}

/// C: mj_setCacheCapacity (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:554)
/// Calls: cxx:__ZN8mjCCache11SetCapacityEm, cxx:__ZNK8mjCCache8CapacityEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_set_cache_capacity(cache: *mut mjCache, size: usize) -> usize {
    todo!() // cxx:_mj_setCacheCapacity
}

/// C: mj_getCacheSize (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:557)
/// Calls: cxx:__ZNK8mjCCache4SizeEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_get_cache_size(cache: *const mjCache) -> usize {
    todo!() // cxx:_mj_getCacheSize
}

/// C: mj_clearCache (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:560)
#[allow(unused_variables, non_snake_case)]
pub fn mj_clear_cache(cache: *mut mjCache) {
    todo!() // cxx:_mj_clearCache
}

/// C: mj_getCache (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.h:563)
/// Calls: cxx:__ZNK8mjCCache8CapacityEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_get_cache() -> *mut mjCache {
    todo!() // cxx:_mj_getCache
}

