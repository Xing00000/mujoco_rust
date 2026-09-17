//! Port of: user/user_api.cc
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_makeSpec (user/user_api.cc:58)
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_spec() -> *mut mjSpec {
    todo!() // cxx:_mj_makeSpec
}

/// C: mj_copySpec (user/user_api.cc:65)
/// Calls: cxx:__ZN8mjCModel8SetErrorERK8mjCError
#[allow(unused_variables, non_snake_case)]
pub fn mj_copy_spec(s: *const mjSpec) -> *mut mjSpec {
    todo!() // cxx:_mj_copySpec
}

/// C: mj_parse (user/user_api.cc:78)
/// Calls: cxx:__ZNK6mujoco4user8FilePath3ExtEv, cxx:_mj_defaultVFS, cxx:_mj_deleteVFS, cxx:_mj_parseXML, cxx:_mju_closeResource, cxx:_mju_decodeResource, cxx:_mju_free, cxx:_mju_malloc, cxx:_mju_openResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_parse(filename: *const i8, content_type: *const i8, vfs: *const mjVFS, error: *mut i8, error_sz: i32) -> *mut mjSpec {
    todo!() // cxx:_mj_parse
}

/// C: encode_xml (user/user_api.cc:162)
/// Calls: cxx:_mj_saveLastXML, cxx:_mj_saveXML
#[allow(unused_variables, non_snake_case)]
pub fn encode_xml(s: *const mjSpec, m: *const mjModel, filename: *const i8, error: *mut i8, error_sz: i32) -> i64 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.cc:__ZL10encode_xmlPK7mjSpec_PK8mjModel_PKcPci
}

/// C: encode_mjb (user/user_api.cc:187)
/// Calls: cxx:_mj_saveModel
#[allow(unused_variables, non_snake_case)]
pub fn encode_mjb(m: *const mjModel, filename: *const i8, error: *mut i8, error_sz: i32) -> i64 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.cc:__ZL10encode_mjbPK8mjModel_PKcPci
}

/// C: encode_txt (user/user_api.cc:202)
/// Calls: cxx:_mj_printModel
#[allow(unused_variables, non_snake_case)]
pub fn encode_txt(m: *const mjModel, filename: *const i8, error: *mut i8, error_sz: i32) -> i64 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.cc:__ZL10encode_txtPK8mjModel_PKcPci
}

/// C: mj_encode (user/user_api.cc:221)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.cc:__ZL10encode_mjbPK8mjModel_PKcPci, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.cc:__ZL10encode_txtPK8mjModel_PKcPci, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.cc:__ZL10encode_xmlPK7mjSpec_PK8mjModel_PKcPci, cxx:__ZNK6mujoco4user8FilePath3ExtEv, cxx:_mjp_findEncoder, cxx:_mju_writeResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_encode(s: *const mjSpec, m: *const mjModel, filename: *const i8, content_type: *const i8, vfs: *const mjVFS, error: *mut i8, error_sz: i32) -> i64 {
    todo!() // cxx:_mj_encode
}

/// C: LogCompileTime (user/user_api.cc:277)
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

/// C: mj_compile (user/user_api.cc:313)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.cc:__ZL14LogCompileTimePKd, cxx:__ZN8mjCModel7CompileEPK6mjVFS_PP8mjModel_
#[allow(unused_variables, non_snake_case)]
pub fn mj_compile(s: *mut mjSpec, vfs: *const mjVFS) -> *mut mjModel {
    todo!() // cxx:_mj_compile
}

/// C: mj_recompile (user/user_api.cc:325)
/// Calls: cxx:__ZN8mjCModel12RestoreStateIdEEvRKNSt3__112basic_stringIcNS1_11char_traitsIcEENS1_9allocatorIcEEEEPKdSB_SB_PT_SD_SD_SD_SD_SD_, cxx:__ZN8mjCModel7CompileEPK6mjVFS_PP8mjModel_, cxx:__ZN8mjCModel8MakeDataEPK8mjModel_PP7mjData_, cxx:__ZN8mjCModel8SetErrorERK8mjCError, cxx:__ZN8mjCModel9SaveStateIdEEvRKNSt3__112basic_stringIcNS1_11char_traitsIcEENS1_9allocatorIcEEEEPKT_SC_SC_SC_SC_SC_, cxx:_mj_deleteData
#[allow(unused_variables, non_snake_case)]
pub fn mj_recompile(s: *mut mjSpec, vfs: *const mjVFS, m: *mut mjModel, d: *mut mjData) -> i32 {
    todo!() // cxx:_mj_recompile
}

/// C: SetFrame (user/user_api.cc:361)
/// Calls: cxx:_mjs_firstChild, cxx:_mjs_getFrame, cxx:_mjs_nextChild, cxx:_mjs_setFrame
#[allow(unused_variables, non_snake_case)]
pub fn set_frame(body: *mut mjsBody, objtype: u32, frame: *mut mjsFrame) {
    todo!() // cxx-internal:user_api.cc.o:__ZL8SetFrameP8mjsBody_6mjtObjP9mjsFrame_
}

/// C: attachBody (user/user_api.cc:371)
/// Calls: cxx:__ZN8mjCModel8SetErrorERK8mjCError
#[allow(unused_variables, non_snake_case)]
pub fn attach_body(parent: *mut mjCFrame, child: *const mjCBody, prefix: *const i8, suffix: *const i8) -> *mut mjsElement {
    todo!() // cxx-internal:user_api.cc.o:__ZL10attachBodyP8mjCFramePK7mjCBodyPKcS5_
}

/// C: attachFrame (user/user_api.cc:391)
/// Calls: cxx:__ZN8mjCModel8SetErrorERK8mjCError
#[allow(unused_variables, non_snake_case)]
pub fn attach_frame(parent: *mut mjCBody, child: *const mjCFrame, prefix: *const i8, suffix: *const i8) -> *mut mjsElement {
    todo!() // cxx-internal:user_api.cc.o:__ZL11attachFrameP7mjCBodyPK8mjCFramePKcS5_
}

/// C: attachToSite (user/user_api.cc:411)
/// Calls: cxx-internal:user_api.cc.o:__ZL10attachBodyP8mjCFramePK7mjCBodyPKcS5_, cxx:__ZN7mjCBody8AddFrameEP8mjCFrame, cxx:__ZN8mjCFrame9SetParentEP7mjCBody, cxx:__ZNK7mjCSite4BodyEv, cxx:_mjs_getSpec, cxx:_mjs_resolveOrientation
#[allow(unused_variables, non_snake_case)]
pub fn attach_to_site(parent: *mut mjCSite, child: *const mjCBody, prefix: *const i8, suffix: *const i8) -> *mut mjsElement {
    todo!() // cxx-internal:user_api.cc.o:__ZL12attachToSiteP7mjCSitePK7mjCBodyPKcS5_
}

/// C: attachFrameToSite (user/user_api.cc:435)
/// Calls: cxx-internal:user_api.cc.o:__ZL11attachFrameP7mjCBodyPK8mjCFramePKcS5_, cxx:__ZN7mjCBody8AddFrameEP8mjCFrame, cxx:__ZN8mjCFrame9SetParentEP7mjCBody, cxx:__ZNK7mjCSite4BodyEv, cxx:_mjs_getSpec, cxx:_mjs_resolveOrientation, cxx:_mjs_setFrame
#[allow(unused_variables, non_snake_case)]
pub fn attach_frame_to_site(parent: *mut mjCSite, child: *const mjCFrame, prefix: *const i8, suffix: *const i8) -> *mut mjsElement {
    todo!() // cxx-internal:user_api.cc.o:__ZL17attachFrameToSiteP7mjCSitePK8mjCFramePKcS5_
}

/// C: mjs_attach (user/user_api.cc:463)
/// Calls: cxx-internal:user_api.cc.o:__ZL10attachBodyP8mjCFramePK7mjCBodyPKcS5_, cxx-internal:user_api.cc.o:__ZL11attachFrameP7mjCBodyPK8mjCFramePKcS5_, cxx-internal:user_api.cc.o:__ZL12attachToSiteP7mjCSitePK7mjCBodyPKcS5_, cxx-internal:user_api.cc.o:__ZL17attachFrameToSiteP7mjCSitePK8mjCFramePKcS5_, cxx-internal:user_api.cc.o:__ZL8SetFrameP8mjsBody_6mjtObjP9mjsFrame_, cxx:__ZN6mujoco16ResolveConflictsEP7mjSpec_PKS0_11mjtConflictPNSt3__112basic_stringIcNS5_11char_traitsIcEENS5_9allocatorIcEEEESC_SC_, cxx:__ZN8mjCModel17AddGroupedWarningERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEES8_, cxx:__ZN8mjCModel24SetAttachWarningBoundaryEv, cxx:__ZN8mjCModel8SetErrorERK8mjCError, cxx:_mjs_addFrame, cxx:_mjs_findBody, cxx:_mjs_getParent, cxx:_mjs_getSpec, cxx:_mjs_setFrame, cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjs_attach(parent: *mut mjsElement, child: *const mjsElement, prefix: *const i8, suffix: *const i8) -> *mut mjsElement {
    todo!() // cxx:_mjs_attach
}

/// C: mjs_getError (user/user_api.cc:580)
/// Calls: cxx:__ZNK8mjCModel8GetErrorEv, cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_error(s: *mut mjSpec) -> *const i8 {
    todo!() // cxx:_mjs_getError
}

/// C: mjs_getTimer (user/user_api.cc:591)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_timer(s: *mut mjSpec) -> *const f64 {
    todo!() // cxx:_mjs_getTimer
}

/// C: mjs_isWarning (user/user_api.cc:599)
/// Calls: cxx:__ZNK8mjCModel11GetWarningsEv, cxx:__ZNK8mjCModel8GetErrorEv
#[allow(unused_variables, non_snake_case)]
pub fn mjs_is_warning(s: *mut mjSpec) -> i32 {
    todo!() // cxx:_mjs_isWarning
}

/// C: mjs_numWarnings (user/user_api.cc:606)
/// Calls: cxx:__ZNK8mjCModel11GetWarningsEv
#[allow(unused_variables, non_snake_case)]
pub fn mjs_num_warnings(spec: *const mjSpec) -> i32 {
    todo!() // cxx:_mjs_numWarnings
}

/// C: mjs_getWarning (user/user_api.cc:613)
/// Calls: cxx:__ZNK8mjCModel11GetWarningsEv
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_warning(spec: *const mjSpec, index: i32) -> *const i8 {
    todo!() // cxx:_mjs_getWarning
}

/// C: mj_deleteSpec (user/user_api.cc:621)
/// Calls: cxx:__ZN8mjCModel7ReleaseEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_delete_spec(s: *mut mjSpec) {
    todo!() // cxx:_mj_deleteSpec
}

/// C: mjs_addSpec (user/user_api.cc:630)
/// Calls: cxx:__ZN8mjCModel10AppendSpecEP7mjSpec_PK12mjsCompiler_
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_spec(s: *mut mjSpec, child: *mut mjSpec) {
    todo!() // cxx:_mjs_addSpec
}

/// C: mjs_activatePlugin (user/user_api.cc:637)
/// Calls: cxx:__ZN8mjCModel14ActivatePluginEPK10mjpPlugin_i, cxx:_mjp_getPlugin
#[allow(unused_variables, non_snake_case)]
pub fn mjs_activate_plugin(s: *mut mjSpec, name: *const i8) -> i32 {
    todo!() // cxx:_mjs_activatePlugin
}

/// C: mjs_setDeepCopy (user/user_api.cc:648)
/// Calls: cxx:__ZN8mjCModel11SetDeepCopyEb
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_deep_copy(s: *mut mjSpec, deepcopy: i32) -> i32 {
    todo!() // cxx:_mjs_setDeepCopy
}

/// C: mj_copyBack (user/user_api.cc:656)
/// Calls: cxx:__ZN8mjCModel8CopyBackEPK8mjModel_
#[allow(unused_variables, non_snake_case)]
pub fn mj_copy_back(s: *mut mjSpec, m: *const mjModel) -> i32 {
    todo!() // cxx:_mj_copyBack
}

/// C: mjs_delete (user/user_api.cc:663)
/// Calls: cxx:__ZN8mjCModel8SetErrorERK8mjCError, cxx:__ZNK8mjCModel10IsAttachedEv
#[allow(unused_variables, non_snake_case)]
pub fn mjs_delete(s: *mut mjSpec, element: *mut mjsElement) -> i32 {
    todo!() // cxx:_mjs_delete
}

/// C: mjs_addBody (user/user_api.cc:689)
/// Calls: cxx:__ZN7mjCBody7AddBodyEP6mjCDef
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_body(body: *mut mjsBody, def: *const mjsDefault) -> *mut mjsBody {
    todo!() // cxx:_mjs_addBody
}

/// C: mjs_addSite (user/user_api.cc:697)
/// Calls: cxx:__ZN7mjCBody7AddSiteEP6mjCDef
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_site(body: *mut mjsBody, def: *const mjsDefault) -> *mut mjsSite {
    todo!() // cxx:_mjs_addSite
}

/// C: mjs_addJoint (user/user_api.cc:706)
/// Calls: cxx:__ZN7mjCBody8AddJointEP6mjCDef
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_joint(body: *mut mjsBody, def: *const mjsDefault) -> *mut mjsJoint {
    todo!() // cxx:_mjs_addJoint
}

/// C: mjs_addFreeJoint (user/user_api.cc:715)
/// Calls: cxx:__ZN7mjCBody12AddFreeJointEv
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_free_joint(body: *mut mjsBody) -> *mut mjsJoint {
    todo!() // cxx:_mjs_addFreeJoint
}

/// C: mjs_addGeom (user/user_api.cc:723)
/// Calls: cxx:__ZN7mjCBody7AddGeomEP6mjCDef
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_geom(body: *mut mjsBody, def: *const mjsDefault) -> *mut mjsGeom {
    todo!() // cxx:_mjs_addGeom
}

/// C: mjs_addCamera (user/user_api.cc:732)
/// Calls: cxx:__ZN7mjCBody9AddCameraEP6mjCDef
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_camera(body: *mut mjsBody, def: *const mjsDefault) -> *mut mjsCamera {
    todo!() // cxx:_mjs_addCamera
}

/// C: mjs_addLight (user/user_api.cc:741)
/// Calls: cxx:__ZN7mjCBody8AddLightEP6mjCDef
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_light(body: *mut mjsBody, def: *const mjsDefault) -> *mut mjsLight {
    todo!() // cxx:_mjs_addLight
}

/// C: mjs_addFlex (user/user_api.cc:750)
/// Calls: cxx:__ZN8mjCModel7AddFlexEv
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_flex(s: *mut mjSpec) -> *mut mjsFlex {
    todo!() // cxx:_mjs_addFlex
}

/// C: FlexcompTypeFromStr (user/user_api.cc:758)
#[allow(unused_variables, non_snake_case)]
pub fn flexcomp_type_from_str(r#type: *const i8) -> mjtFcompType {
    todo!() // cxx-internal:user_api.cc.o:__ZL19FlexcompTypeFromStrPKc
}

/// C: FlexcompDofFromStr (user/user_api.cc:775)
#[allow(unused_variables, non_snake_case)]
pub fn flexcomp_dof_from_str(dof: *const i8) -> mjtDof {
    todo!() // cxx-internal:user_api.cc.o:__ZL18FlexcompDofFromStrPKc
}

/// C: mjs_makeFlex (user/user_api.cc:808)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.cc:__ZL18FlexcompDofFromStrPKc, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_api.cc:__ZL19FlexcompTypeFromStrPKc, cxx:__ZN11mjCFlexcomp4MakeEP8mjsBody_PciPK6mjVFS_, cxx:__ZN8mjCModel8SetErrorERK8mjCError, cxx:__ZNK8mjCModel6FlexesEv, cxx:_mju_error
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_make_flex(body: *mut mjsBody, name: *const i8, r#type: *const i8, dim: i32, dof: *const i8, count: *const i32, cellcount: *const i32, spacing: *const f64, scale: *const f64, radius: f64, mass: f64, inertiabox: f64, equality: i32, rigid: i32, flatskin: i32, elastic2d: i32, pos: *const f64, quat: *const f64, origin: *const f64, file: *const i8, vfs: *const mjVFS) -> *mut mjsFlex {
    todo!() // cxx:_mjs_makeFlex
}

/// C: mjs_addFrame (user/user_api.cc:890)
/// Calls: cxx:__ZN7mjCBody8AddFrameEP8mjCFrame, cxx:__ZN8mjCFrame9SetParentEP7mjCBody
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_frame(body: *mut mjsBody, parentframe: *mut mjsFrame) -> *mut mjsFrame {
    todo!() // cxx:_mjs_addFrame
}

/// C: mjs_addMesh (user/user_api.cc:901)
/// Calls: cxx:__ZN8mjCModel7AddMeshEP6mjCDef
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_mesh(s: *mut mjSpec, def: *const mjsDefault) -> *mut mjsMesh {
    todo!() // cxx:_mjs_addMesh
}

/// C: mjs_addHField (user/user_api.cc:910)
/// Calls: cxx:__ZN8mjCModel9AddHFieldEv
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_h_field(s: *mut mjSpec) -> *mut mjsHField {
    todo!() // cxx:_mjs_addHField
}

/// C: mjs_addSkin (user/user_api.cc:918)
/// Calls: cxx:__ZN8mjCModel7AddSkinEv
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_skin(s: *mut mjSpec) -> *mut mjsSkin {
    todo!() // cxx:_mjs_addSkin
}

/// C: mjs_addTexture (user/user_api.cc:926)
/// Calls: cxx:__ZN8mjCModel10AddTextureEv
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_texture(s: *mut mjSpec) -> *mut mjsTexture {
    todo!() // cxx:_mjs_addTexture
}

/// C: mjs_addMaterial (user/user_api.cc:934)
/// Calls: cxx:__ZN8mjCModel11AddMaterialEP6mjCDef
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_material(s: *mut mjSpec, def: *const mjsDefault) -> *mut mjsMaterial {
    todo!() // cxx:_mjs_addMaterial
}

/// C: mjs_makeMesh (user/user_api.cc:942)
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

/// C: mjs_addPair (user/user_api.cc:1095)
/// Calls: cxx:__ZN8mjCModel7AddPairEP6mjCDef
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_pair(s: *mut mjSpec, def: *const mjsDefault) -> *mut mjsPair {
    todo!() // cxx:_mjs_addPair
}

/// C: mjs_addExclude (user/user_api.cc:1104)
/// Calls: cxx:__ZN8mjCModel10AddExcludeEv
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_exclude(s: *mut mjSpec) -> *mut mjsExclude {
    todo!() // cxx:_mjs_addExclude
}

/// C: mjs_addEquality (user/user_api.cc:1112)
/// Calls: cxx:__ZN8mjCModel11AddEqualityEP6mjCDef
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_equality(s: *mut mjSpec, def: *const mjsDefault) -> *mut mjsEquality {
    todo!() // cxx:_mjs_addEquality
}

/// C: mjs_addTendon (user/user_api.cc:1121)
/// Calls: cxx:__ZN8mjCModel9AddTendonEP6mjCDef
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_tendon(s: *mut mjSpec, def: *const mjsDefault) -> *mut mjsTendon {
    todo!() // cxx:_mjs_addTendon
}

/// C: mjs_wrapSite (user/user_api.cc:1130)
/// Calls: cxx:__ZN9mjCTendon8WrapSiteENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEENS0_17basic_string_viewIcS3_EE
#[allow(unused_variables, non_snake_case)]
pub fn mjs_wrap_site(tendon: *mut mjsTendon, name: *const i8) -> *mut mjsWrap {
    todo!() // cxx:_mjs_wrapSite
}

/// C: mjs_wrapGeom (user/user_api.cc:1138)
/// Calls: cxx:__ZN9mjCTendon8WrapGeomENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEES6_NS0_17basic_string_viewIcS3_EE
#[allow(unused_variables, non_snake_case)]
pub fn mjs_wrap_geom(tendon: *mut mjsTendon, name: *const i8, sidesite: *const i8) -> *mut mjsWrap {
    todo!() // cxx:_mjs_wrapGeom
}

/// C: mjs_wrapJoint (user/user_api.cc:1146)
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

/// C: mjs_wrapPulley (user/user_api.cc:1154)
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

/// C: mjs_addActuator (user/user_api.cc:1162)
/// Calls: cxx:__ZN8mjCModel11AddActuatorEP6mjCDef
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_actuator(s: *mut mjSpec, def: *const mjsDefault) -> *mut mjsActuator {
    todo!() // cxx:_mjs_addActuator
}

/// C: mjs_addSensor (user/user_api.cc:1171)
/// Calls: cxx:__ZN8mjCModel9AddSensorEv
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_sensor(s: *mut mjSpec) -> *mut mjsSensor {
    todo!() // cxx:_mjs_addSensor
}

/// C: mjs_addNumeric (user/user_api.cc:1179)
/// Calls: cxx:__ZN8mjCModel10AddNumericEv
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_numeric(s: *mut mjSpec) -> *mut mjsNumeric {
    todo!() // cxx:_mjs_addNumeric
}

/// C: mjs_addText (user/user_api.cc:1187)
/// Calls: cxx:__ZN8mjCModel7AddTextEv
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_text(s: *mut mjSpec) -> *mut mjsText {
    todo!() // cxx:_mjs_addText
}

/// C: mjs_addTuple (user/user_api.cc:1195)
/// Calls: cxx:__ZN8mjCModel8AddTupleEv
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_tuple(s: *mut mjSpec) -> *mut mjsTuple {
    todo!() // cxx:_mjs_addTuple
}

/// C: mjs_addKey (user/user_api.cc:1203)
/// Calls: cxx:__ZN8mjCModel6AddKeyEv
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_key(s: *mut mjSpec) -> *mut mjsKey {
    todo!() // cxx:_mjs_addKey
}

/// C: mjs_addPlugin (user/user_api.cc:1211)
/// Calls: cxx:__ZN8mjCModel9AddPluginEv
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_plugin(s: *mut mjSpec) -> *mut mjsPlugin {
    todo!() // cxx:_mjs_addPlugin
}

/// C: mjs_addDefault (user/user_api.cc:1220)
/// Calls: cxx:__ZNK8mjCModel7DefaultEv
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_default(s: *mut mjSpec, classname: *const i8, parent: *const mjsDefault) -> *mut mjsDefault {
    todo!() // cxx:_mjs_addDefault
}

/// C: mjs_setToMotor (user/user_api.cc:1234)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_to_motor(actuator: *mut mjsActuator) -> *const i8 {
    todo!() // cxx:_mjs_setToMotor
}

/// C: mjs_setToPosition (user/user_api.cc:1252)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_to_position(actuator: *mut mjsActuator, kp: f64, kv: *mut f64, dampratio: *mut f64, timeconst: *mut f64, inheritrange: f64) -> *const i8 {
    todo!() // cxx:_mjs_setToPosition
}

/// C: mjs_setToIntVelocity (user/user_api.cc:1292)
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

/// C: mjs_setToOrientation (user/user_api.cc:1307)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_to_orientation(actuator: *mut mjsActuator, kp: f64, kv: *mut f64, dampratio: *mut f64, ctrlspec: i32) -> *const i8 {
    todo!() // cxx:_mjs_setToOrientation
}

/// C: mjs_setToPID (user/user_api.cc:1336)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_to_pid(actuator: *mut mjsActuator, kp: f64, kv: *mut f64, dampratio: *mut f64, ki: *mut f64, imax: *mut f64, slewmax: *mut f64, inheritrange: f64, ctrlspec: i32) -> *const i8 {
    todo!() // cxx:_mjs_setToPID
}

/// C: mjs_setToVelocity (user/user_api.cc:1372)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_to_velocity(actuator: *mut mjsActuator, kv: f64) -> *const i8 {
    todo!() // cxx:_mjs_setToVelocity
}

/// C: mjs_setToDamper (user/user_api.cc:1384)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_to_damper(actuator: *mut mjsActuator, kv: f64) -> *const i8 {
    todo!() // cxx:_mjs_setToDamper
}

/// C: mjs_setToCylinder (user/user_api.cc:1402)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_to_cylinder(actuator: *mut mjsActuator, timeconst: f64, bias: f64, area: f64, diameter: f64) -> *const i8 {
    todo!() // cxx:_mjs_setToCylinder
}

/// C: mjs_setToMuscle (user/user_api.cc:1425)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_to_muscle(actuator: *mut mjsActuator, timeconst: *mut f64, tausmooth: f64, range: *mut f64, force: f64, scale: f64, lmin: f64, lmax: f64, vmax: f64, fpmax: f64, fvmax: f64) -> *const i8 {
    todo!() // cxx:_mjs_setToMuscle
}

/// C: mjs_setToAdhesion (user/user_api.cc:1465)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_to_adhesion(actuator: *mut mjsActuator, gain: f64) -> *const i8 {
    todo!() // cxx:_mjs_setToAdhesion
}

/// C: mjs_setToDCMotor (user/user_api.cc:1488)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_to_dc_motor(actuator: *mut mjsActuator, motorconst: *mut f64, resistance: f64, nominal: *mut f64, saturation: *mut f64, inductance: *mut f64, cogging: *mut f64, controller: *mut f64, thermal: *mut f64, lugre: *mut f64, ctrlspec: i32) -> *const i8 {
    todo!() // cxx:_mjs_setToDCMotor
}

/// C: mjs_getSpec (user/user_api.cc:1646)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_spec(element: *const mjsElement) -> *mut mjSpec {
    todo!() // cxx:_mjs_getSpec
}

/// C: mjs_getOriginSpec (user/user_api.cc:1653)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_origin_spec(element: *const mjsElement) -> *mut mjSpec {
    todo!() // cxx:_mjs_getOriginSpec
}

/// C: mjs_getCompiler (user/user_api.cc:1660)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_compiler(element: *const mjsElement) -> *mut mjsCompiler {
    todo!() // cxx:_mjs_getCompiler
}

/// C: mjs_findSpec (user/user_api.cc:1666)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_find_spec(spec: *const mjSpec, name: *const i8) -> *mut mjSpec {
    todo!() // cxx:_mjs_findSpec
}

/// C: mjs_getDefault (user/user_api.cc:1673)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_default(element: *const mjsElement) -> *mut mjsDefault {
    todo!() // cxx:_mjs_getDefault
}

/// C: mjs_findDefault (user/user_api.cc:1682)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_find_default(s: *const mjSpec, classname: *const i8) -> *mut mjsDefault {
    todo!() // cxx:_mjs_findDefault
}

/// C: mjs_getSpecDefault (user/user_api.cc:1690)
/// Calls: cxx:__ZNK8mjCModel7DefaultEv
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_spec_default(s: *const mjSpec) -> *mut mjsDefault {
    todo!() // cxx:_mjs_getSpecDefault
}

/// C: mjs_findBody (user/user_api.cc:1698)
/// Calls: cxx:_mjs_findElement
#[allow(unused_variables, non_snake_case)]
pub fn mjs_find_body(s: *const mjSpec, name: *const i8) -> *mut mjsBody {
    todo!() // cxx:_mjs_findBody
}

/// C: mjs_findElement (user/user_api.cc:1705)
/// Calls: cxx:__ZN8mjCModel8FindTreeEP7mjCBody6mjtObjNSt3__112basic_stringIcNS3_11char_traitsIcEENS3_9allocatorIcEEEE, cxx:__ZN8mjCModel8GetWorldEv, cxx:__ZNK8mjCModel10IsCompiledEv, cxx:__ZNK8mjCModel6MeshesEv, cxx:__ZNK8mjCModel8TexturesEv
#[allow(unused_variables, non_snake_case)]
pub fn mjs_find_element(s: *const mjSpec, r#type: u32, name: *const i8) -> *mut mjsElement {
    todo!() // cxx:_mjs_findElement
}

/// C: mjs_findChild (user/user_api.cc:1730)
/// Calls: cxx:__ZNK7mjCBody10FindObjectE6mjtObjRKNSt3__112basic_stringIcNS1_11char_traitsIcEENS1_9allocatorIcEEEEb
#[allow(unused_variables, non_snake_case)]
pub fn mjs_find_child(body: *const mjsBody, name: *const i8) -> *mut mjsBody {
    todo!() // cxx:_mjs_findChild
}

/// C: mjs_getParent (user/user_api.cc:1738)
/// Calls: cxx:__ZNK7mjCBody9GetParentEv, cxx:__ZNK7mjCGeom9GetParentEv, cxx:__ZNK7mjCSite9GetParentEv, cxx:__ZNK8mjCFrame9GetParentEv, cxx:__ZNK8mjCJoint9GetParentEv, cxx:__ZNK8mjCLight9GetParentEv, cxx:__ZNK9mjCCamera9GetParentEv
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_parent(element: *const mjsElement) -> *mut mjsBody {
    todo!() // cxx:_mjs_getParent
}

/// C: mjs_getFrame (user/user_api.cc:1761)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_frame(element: *const mjsElement) -> *mut mjsFrame {
    todo!() // cxx:_mjs_getFrame
}

/// C: mjs_findFrame (user/user_api.cc:1779)
/// Calls: cxx:_mjs_findElement
#[allow(unused_variables, non_snake_case)]
pub fn mjs_find_frame(s: *const mjSpec, name: *const i8) -> *mut mjsFrame {
    todo!() // cxx:_mjs_findFrame
}

/// C: mjs_setFrame (user/user_api.cc:1786)
/// Calls: cxx:__ZN7mjCBase8SetFrameEP8mjCFrame, cxx:__ZN8mjCModel8SetErrorERK8mjCError
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_frame(dest: *mut mjsElement, frame: *mut mjsFrame) -> i32 {
    todo!() // cxx:_mjs_setFrame
}

/// C: mjs_resolveOrientation (user/user_api.cc:1804)
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

/// C: mjs_bodyToFrame (user/user_api.cc:1810)
/// Calls: cxx:__ZN7mjCBody7ToFrameEv
#[allow(unused_variables, non_snake_case)]
pub fn mjs_body_to_frame(body: *mut *mut mjsBody) -> *mut mjsFrame {
    todo!() // cxx:_mjs_bodyToFrame
}

/// C: mjs_setUserValue (user/user_api.cc:1818)
/// Calls: cxx:_mjs_setUserValueWithCleanup
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_user_value(element: *mut mjsElement, key: *const i8, data: *const ()) {
    todo!() // cxx:_mjs_setUserValue
}

/// C: mjs_setUserValueWithCleanup (user/user_api.cc:1826)
/// Calls: cxx:__ZN7mjCBase12SetUserValueENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEEPKvPFvS6_E
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_user_value_with_cleanup(element: *mut mjsElement, key: *const i8, data: *const (), cleanup: Option<unsafe extern "C" fn(*const ())>) {
    todo!() // cxx:_mjs_setUserValueWithCleanup
}

/// C: mjs_getUserValue (user/user_api.cc:1832)
/// Calls: cxx:__ZN7mjCBase12GetUserValueENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEE
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_user_value(element: *mut mjsElement, key: *const i8) -> *const () {
    todo!() // cxx:_mjs_getUserValue
}

/// C: mjs_deleteUserValue (user/user_api.cc:1839)
/// Calls: cxx:__ZN7mjCBase15DeleteUserValueENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEE
#[allow(unused_variables, non_snake_case)]
pub fn mjs_delete_user_value(element: *mut mjsElement, key: *const i8) {
    todo!() // cxx:_mjs_deleteUserValue
}

/// C: mjs_sensorDim (user/user_api.cc:1846)
/// Calls: cxx:__ZN9mjCSensor7get_objEv, cxx:__ZNK7mjCMesh5nvertEv, cxx:_mju_condataSize, cxx:_mju_raydataSize
#[allow(unused_variables, non_snake_case)]
pub fn mjs_sensor_dim(sensor: *const mjsSensor) -> i32 {
    todo!() // cxx:_mjs_sensorDim
}

/// C: mjs_getId (user/user_api.cc:1931)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_id(element: *const mjsElement) -> i32 {
    todo!() // cxx:_mjs_getId
}

/// C: mjs_setDefault (user/user_api.cc:1938)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_default(element: *mut mjsElement, def: *const mjsDefault) {
    todo!() // cxx:_mjs_setDefault
}

/// C: mjs_firstChild (user/user_api.cc:1945)
/// Calls: cxx:__ZN8mjCModel8SetErrorERK8mjCError, cxx:__ZNK7mjCBody9NextChildEPK11mjsElement_6mjtObjb
#[allow(unused_variables, non_snake_case)]
pub fn mjs_first_child(body: *const mjsBody, r#type: u32, recurse: i32) -> *mut mjsElement {
    todo!() // cxx:_mjs_firstChild
}

/// C: mjs_nextChild (user/user_api.cc:1957)
/// Calls: cxx:__ZN8mjCModel8SetErrorERK8mjCError, cxx:__ZNK7mjCBody9NextChildEPK11mjsElement_6mjtObjb
#[allow(unused_variables, non_snake_case)]
pub fn mjs_next_child(body: *const mjsBody, child: *const mjsElement, recurse: i32) -> *mut mjsElement {
    todo!() // cxx:_mjs_nextChild
}

/// C: mjs_firstElement (user/user_api.cc:1969)
/// Calls: cxx:__ZNK8mjCModel10NextObjectEPK11mjsElement_6mjtObj
#[allow(unused_variables, non_snake_case)]
pub fn mjs_first_element(s: *const mjSpec, r#type: u32) -> *mut mjsElement {
    todo!() // cxx:_mjs_firstElement
}

/// C: mjs_nextElement (user/user_api.cc:1976)
/// Calls: cxx:__ZNK8mjCModel10NextObjectEPK11mjsElement_6mjtObj
#[allow(unused_variables, non_snake_case)]
pub fn mjs_next_element(s: *const mjSpec, element: *const mjsElement) -> *mut mjsElement {
    todo!() // cxx:_mjs_nextElement
}

/// C: mjs_getWrapTarget (user/user_api.cc:1982)
/// Calls: cxx:__ZNK7mjCWrap4TypeEv, cxx:_mjs_findElement, cxx:_mjs_getSpec
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_wrap_target(wrap: *const mjsWrap) -> *mut mjsElement {
    todo!() // cxx:_mjs_getWrapTarget
}

/// C: mjs_getWrapSideSite (user/user_api.cc:2007)
/// Calls: cxx:__ZNK7mjCWrap4TypeEv, cxx:_mjs_asSite, cxx:_mjs_findElement, cxx:_mjs_getSpec, cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_wrap_side_site(wrap: *const mjsWrap) -> *mut mjsSite {
    todo!() // cxx:_mjs_getWrapSideSite
}

/// C: mjs_getWrapDivisor (user/user_api.cc:2027)
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

/// C: mjs_getWrapCoef (user/user_api.cc:2037)
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

/// C: mjs_asBody (user/user_api.cc:2048)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_body(element: *mut mjsElement) -> *mut mjsBody {
    todo!() // cxx:_mjs_asBody
}

/// C: mjs_asGeom (user/user_api.cc:2057)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_geom(element: *mut mjsElement) -> *mut mjsGeom {
    todo!() // cxx:_mjs_asGeom
}

/// C: mjs_asJoint (user/user_api.cc:2066)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_joint(element: *mut mjsElement) -> *mut mjsJoint {
    todo!() // cxx:_mjs_asJoint
}

/// C: mjs_asSite (user/user_api.cc:2075)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_site(element: *mut mjsElement) -> *mut mjsSite {
    todo!() // cxx:_mjs_asSite
}

/// C: mjs_asCamera (user/user_api.cc:2084)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_camera(element: *mut mjsElement) -> *mut mjsCamera {
    todo!() // cxx:_mjs_asCamera
}

/// C: mjs_asLight (user/user_api.cc:2093)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_light(element: *mut mjsElement) -> *mut mjsLight {
    todo!() // cxx:_mjs_asLight
}

/// C: mjs_asFrame (user/user_api.cc:2102)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_frame(element: *mut mjsElement) -> *mut mjsFrame {
    todo!() // cxx:_mjs_asFrame
}

/// C: mjs_asActuator (user/user_api.cc:2111)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_actuator(element: *mut mjsElement) -> *mut mjsActuator {
    todo!() // cxx:_mjs_asActuator
}

/// C: mjs_asSensor (user/user_api.cc:2120)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_sensor(element: *mut mjsElement) -> *mut mjsSensor {
    todo!() // cxx:_mjs_asSensor
}

/// C: mjs_asFlex (user/user_api.cc:2129)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_flex(element: *mut mjsElement) -> *mut mjsFlex {
    todo!() // cxx:_mjs_asFlex
}

/// C: mjs_asPair (user/user_api.cc:2138)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_pair(element: *mut mjsElement) -> *mut mjsPair {
    todo!() // cxx:_mjs_asPair
}

/// C: mjs_asEquality (user/user_api.cc:2147)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_equality(element: *mut mjsElement) -> *mut mjsEquality {
    todo!() // cxx:_mjs_asEquality
}

/// C: mjs_asExclude (user/user_api.cc:2156)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_exclude(element: *mut mjsElement) -> *mut mjsExclude {
    todo!() // cxx:_mjs_asExclude
}

/// C: mjs_asTendon (user/user_api.cc:2165)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_tendon(element: *mut mjsElement) -> *mut mjsTendon {
    todo!() // cxx:_mjs_asTendon
}

/// C: mjs_asNumeric (user/user_api.cc:2174)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_numeric(element: *mut mjsElement) -> *mut mjsNumeric {
    todo!() // cxx:_mjs_asNumeric
}

/// C: mjs_asText (user/user_api.cc:2183)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_text(element: *mut mjsElement) -> *mut mjsText {
    todo!() // cxx:_mjs_asText
}

/// C: mjs_asTuple (user/user_api.cc:2192)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_tuple(element: *mut mjsElement) -> *mut mjsTuple {
    todo!() // cxx:_mjs_asTuple
}

/// C: mjs_asKey (user/user_api.cc:2201)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_key(element: *mut mjsElement) -> *mut mjsKey {
    todo!() // cxx:_mjs_asKey
}

/// C: mjs_asMesh (user/user_api.cc:2208)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_mesh(element: *mut mjsElement) -> *mut mjsMesh {
    todo!() // cxx:_mjs_asMesh
}

/// C: mjs_asHField (user/user_api.cc:2217)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_h_field(element: *mut mjsElement) -> *mut mjsHField {
    todo!() // cxx:_mjs_asHField
}

/// C: mjs_asSkin (user/user_api.cc:2226)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_skin(element: *mut mjsElement) -> *mut mjsSkin {
    todo!() // cxx:_mjs_asSkin
}

/// C: mjs_asTexture (user/user_api.cc:2235)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_texture(element: *mut mjsElement) -> *mut mjsTexture {
    todo!() // cxx:_mjs_asTexture
}

/// C: mjs_asMaterial (user/user_api.cc:2244)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_material(element: *mut mjsElement) -> *mut mjsMaterial {
    todo!() // cxx:_mjs_asMaterial
}

/// C: mjs_asPlugin (user/user_api.cc:2253)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_plugin(element: *mut mjsElement) -> *mut mjsPlugin {
    todo!() // cxx:_mjs_asPlugin
}

/// C: mjs_setName (user/user_api.cc:2262)
/// Calls: cxx:__ZN8mjCModel11CheckRepeatE6mjtObj, cxx:__ZN8mjCModel8SetErrorERK8mjCError
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_name(element: *mut mjsElement, name: *const i8) -> i32 {
    todo!() // cxx:_mjs_setName
}

/// C: mjs_setBuffer (user/user_api.cc:2281)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_buffer(dest: *mut mjByteVec, array: *const (), size: i32) {
    todo!() // cxx:_mjs_setBuffer
}

/// C: mjs_setString (user/user_api.cc:2290)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_string(dest: *mut mjString, text: *const i8) {
    todo!() // cxx:_mjs_setString
}

/// C: mjs_setInStringVec (user/user_api.cc:2297)
/// Calls: cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_in_string_vec(dest: *mut mjStringVec, i: i32, text: *const i8) -> bool {
    todo!() // cxx:_mjs_setInStringVec
}

/// C: mjs_setStringVec (user/user_api.cc:2308)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_string_vec(dest: *mut mjStringVec, text: *const i8) {
    todo!() // cxx:_mjs_setStringVec
}

/// C: mjs_appendString (user/user_api.cc:2315)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_append_string(dest: *mut mjStringVec, text: *const i8) {
    todo!() // cxx:_mjs_appendString
}

/// C: mjs_setInt (user/user_api.cc:2321)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_int(dest: *mut mjIntVec, array: *const i32, size: i32) {
    todo!() // cxx:_mjs_setInt
}

/// C: mjs_appendIntVec (user/user_api.cc:2328)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_append_int_vec(dest: *mut mjIntVecVec, array: *const i32, size: i32) {
    todo!() // cxx:_mjs_appendIntVec
}

/// C: mjs_setFloat (user/user_api.cc:2334)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_float(dest: *mut mjFloatVec, array: *const f32, size: i32) {
    todo!() // cxx:_mjs_setFloat
}

/// C: mjs_appendFloatVec (user/user_api.cc:2341)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_append_float_vec(dest: *mut mjFloatVecVec, array: *const f32, size: i32) {
    todo!() // cxx:_mjs_appendFloatVec
}

/// C: mjs_setDouble (user/user_api.cc:2347)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_double(dest: *mut mjDoubleVec, array: *const f64, size: i32) {
    todo!() // cxx:_mjs_setDouble
}

/// C: mjs_getName (user/user_api.cc:2354)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_name(element: *mut mjsElement) -> *mut mjString {
    todo!() // cxx:_mjs_getName
}

/// C: mjs_getString (user/user_api.cc:2361)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_string(source: *const mjString) -> *const i8 {
    todo!() // cxx:_mjs_getString
}

/// C: mjs_getDouble (user/user_api.cc:2367)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_double(source: *const mjDoubleVec, size: *mut i32) -> *const f64 {
    todo!() // cxx:_mjs_getDouble
}

/// C: mjs_getWrapNum (user/user_api.cc:2372)
/// Calls: cxx:__ZNK9mjCTendon8NumWrapsEv
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_wrap_num(tendonspec: *const mjsTendon) -> i32 {
    todo!() // cxx:_mjs_getWrapNum
}

/// C: mjs_getWrap (user/user_api.cc:2377)
/// Calls: cxx:__ZNK9mjCTendon7GetWrapEi, cxx:__ZNK9mjCTendon8NumWrapsEv, cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_wrap(tendonspec: *const mjsTendon, i: i32) -> *mut mjsWrap {
    todo!() // cxx:_mjs_getWrap
}

/// C: mjs_setPluginAttributes (user/user_api.cc:2386)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_plugin_attributes(plugin: *mut mjsPlugin, attributes: *mut ()) {
    todo!() // cxx:_mjs_setPluginAttributes
}

/// C: mjs_getPluginAttributes (user/user_api.cc:2395)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_plugin_attributes(plugin: *const mjsPlugin) -> *const () {
    todo!() // cxx:_mjs_getPluginAttributes
}

/// C: mj_getCacheCapacity (user/user_api.cc:2404)
/// Calls: cxx:__ZNK8mjCCache8CapacityEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_get_cache_capacity(cache: *const mjCache) -> usize {
    todo!() // cxx:_mj_getCacheCapacity
}

/// C: mj_setCacheCapacity (user/user_api.cc:2414)
/// Calls: cxx:__ZN8mjCCache11SetCapacityEm, cxx:__ZNK8mjCCache8CapacityEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_set_cache_capacity(cache: *mut mjCache, size: usize) -> usize {
    todo!() // cxx:_mj_setCacheCapacity
}

/// C: mj_getCacheSize (user/user_api.cc:2427)
/// Calls: cxx:__ZNK8mjCCache4SizeEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_get_cache_size(cache: *const mjCache) -> usize {
    todo!() // cxx:_mj_getCacheSize
}

/// C: mj_clearCache (user/user_api.cc:2437)
#[allow(unused_variables, non_snake_case)]
pub fn mj_clear_cache(cache: *mut mjCache) {
    todo!() // cxx:_mj_clearCache
}

/// C: mj_getCache (user/user_api.cc:2445)
/// Calls: cxx:__ZNK8mjCCache8CapacityEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_get_cache() -> *mut mjCache {
    todo!() // cxx:_mj_getCache
}

/// C: mjs_isAuthored (user/user_api.cc:2461)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_is_authored(elem_ptr: *const (), field_ptr: *const ()) -> i32 {
    todo!() // cxx:_mjs_isAuthored
}

/// C: mjs_setAuthored (user/user_api.cc:2540)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_authored(elem_ptr: *const (), field_ptr: *const (), authored: i32) {
    todo!() // cxx:_mjs_setAuthored
}

