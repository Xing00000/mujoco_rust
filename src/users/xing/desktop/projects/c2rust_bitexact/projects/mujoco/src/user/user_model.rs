//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: IsSameVec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:72)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn is_same_vec(pos1: *const f64, pos2: *const f64) -> bool {
    todo!() // cxx-internal:user_model.cc.o:__ZN12_GLOBAL__N_19IsSameVecIdEEbPKT_S3_
}

/// C: NumCompilerThreads (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:79)
#[allow(unused_variables, non_snake_case)]
pub fn num_compiler_threads(upper_bound: i32) -> u32 {
    todo!() // cxx-internal:user_model.cc.o:__ZN12_GLOBAL__N_118NumCompilerThreadsEi
}

/// C: IsSameQuat (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:93)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn is_same_quat(quat1: *const f64, quat2: *const f64) -> bool {
    todo!() // cxx-internal:user_model.cc.o:__ZN12_GLOBAL__N_110IsSameQuatIdEEbPKT_S3_
}

/// C: IsSamePose (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:111)
/// Calls: cxx-internal:user_model.cc.o:__ZN12_GLOBAL__N_110IsSameQuatIdEEbPKT_S3_, cxx-internal:user_model.cc.o:__ZN12_GLOBAL__N_19IsSameVecIdEEbPKT_S3_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn is_same_pose(pos1: *const f64, pos2: *const f64, quat1: *const f64, quat2: *const f64) -> bool {
    todo!() // cxx-internal:user_model.cc.o:__ZN12_GLOBAL__N_110IsSamePoseIdEEbPKT_S3_S3_S3_
}

/// C: IsNullPose (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:127)
/// Calls: cxx-internal:user_model.cc.o:__ZN12_GLOBAL__N_110IsSamePoseIdEEbPKT_S3_S3_S3_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn is_null_pose(pos: *const f64, quat: *const f64) -> bool {
    todo!() // cxx-internal:user_model.cc.o:__ZN12_GLOBAL__N_110IsNullPoseIdEEbPKT_S3_
}

/// C: GetBodyIdFromWrap (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:134)
/// Calls: cxx:__ZNK7mjCGeom9GetParentEv, cxx:__ZNK7mjCSite4BodyEv, cxx:__ZNK7mjCWrap4TypeEv
#[allow(unused_variables, non_snake_case)]
pub fn get_body_id_from_wrap(wrap: *const mjCWrap) -> i32 {
    todo!() // cxx-internal:user_model.cc.o:__ZN12_GLOBAL__N_117GetBodyIdFromWrapEPK7mjCWrap
}

/// C: mjCModel::CopyList (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:261)
/// Calls: cxx:__ZN10mjCNumeric12CopyFromSpecEv, cxx:__ZN7mjCBase10CopyPluginEv, cxx:__ZN7mjCBase17ResolveReferencesEPK8mjCModel, cxx:__ZN7mjCBase6AddRefEv, cxx:__ZN7mjCBase9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_list__2066ac3b7b2fea6c(self_ptr: *mut mjCModel, dest: *const (), source: *const ()) {
    todo!() // cxx:__ZN8mjCModel8CopyListI10mjCNumericEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEERKS8_
}

/// C: mjCModel::CopyList (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:261)
/// Calls: cxx:__ZN10mjCTexture12CopyFromSpecEv, cxx:__ZN10mjCTexture9NameSpaceEPK8mjCModel, cxx:__ZN7mjCBase10CopyPluginEv, cxx:__ZN7mjCBase17ResolveReferencesEPK8mjCModel, cxx:__ZN7mjCBase6AddRefEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_list__db24a582176160fe(self_ptr: *mut mjCModel, dest: *const (), source: *const ()) {
    todo!() // cxx:__ZN8mjCModel8CopyListI10mjCTextureEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEERKS8_
}

/// C: mjCModel::CopyList (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:261)
/// Calls: cxx:__ZN11mjCActuator10CopyPluginEv, cxx:__ZN11mjCActuator12CopyFromSpecEv, cxx:__ZN11mjCActuator15ForgetKeyframesEv, cxx:__ZN11mjCActuator17ResolveReferencesEPK8mjCModel, cxx:__ZN11mjCActuator9NameSpaceEPK8mjCModel, cxx:__ZN7mjCBase6AddRefEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_list__36dec2e6a531f271(self_ptr: *mut mjCModel, dest: *const (), source: *const ()) {
    todo!() // cxx:__ZN8mjCModel8CopyListI11mjCActuatorEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEERKS8_
}

/// C: mjCModel::CopyList (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:261)
/// Calls: cxx:__ZN11mjCBodyPair12CopyFromSpecEv, cxx:__ZN11mjCBodyPair17ResolveReferencesEPK8mjCModel, cxx:__ZN11mjCBodyPair9NameSpaceEPK8mjCModel, cxx:__ZN7mjCBase10CopyPluginEv, cxx:__ZN7mjCBase6AddRefEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_list__48273379f813dc4f(self_ptr: *mut mjCModel, dest: *const (), source: *const ()) {
    todo!() // cxx:__ZN8mjCModel8CopyListI11mjCBodyPairEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEERKS8_
}

/// C: mjCModel::CopyList (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:261)
/// Calls: cxx:__ZN11mjCEquality12CopyFromSpecEv, cxx:__ZN11mjCEquality17ResolveReferencesEPK8mjCModel, cxx:__ZN11mjCEquality9NameSpaceEPK8mjCModel, cxx:__ZN7mjCBase10CopyPluginEv, cxx:__ZN7mjCBase6AddRefEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_list__66076428619189b3(self_ptr: *mut mjCModel, dest: *const (), source: *const ()) {
    todo!() // cxx:__ZN8mjCModel8CopyListI11mjCEqualityEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEERKS8_
}

/// C: mjCModel::CopyList (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:261)
/// Calls: cxx:__ZN11mjCMaterial12CopyFromSpecEv, cxx:__ZN11mjCMaterial9NameSpaceEPK8mjCModel, cxx:__ZN7mjCBase10CopyPluginEv, cxx:__ZN7mjCBase17ResolveReferencesEPK8mjCModel, cxx:__ZN7mjCBase6AddRefEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_list__13504f4d68d26b93(self_ptr: *mut mjCModel, dest: *const (), source: *const ()) {
    todo!() // cxx:__ZN8mjCModel8CopyListI11mjCMaterialEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEERKS8_
}

/// C: mjCModel::CopyList (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:261)
/// Calls: cxx:__ZN6mjCKey12CopyFromSpecEv, cxx:__ZN7mjCBase10CopyPluginEv, cxx:__ZN7mjCBase17ResolveReferencesEPK8mjCModel, cxx:__ZN7mjCBase6AddRefEv, cxx:__ZN7mjCBase9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_list__faf47aaaea010377(self_ptr: *mut mjCModel, dest: *const (), source: *const ()) {
    todo!() // cxx:__ZN8mjCModel8CopyListI6mjCKeyEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEERKS8_
}

/// C: mjCModel::CopyList (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:261)
/// Calls: cxx:__ZN7mjCBase10CopyPluginEv, cxx:__ZN7mjCBase6AddRefEv, cxx:__ZN7mjCFlex12CopyFromSpecEv, cxx:__ZN7mjCFlex17ResolveReferencesEPK8mjCModel, cxx:__ZN7mjCFlex9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_list__1c64b6be2d83c2da(self_ptr: *mut mjCModel, dest: *const (), source: *const ()) {
    todo!() // cxx:__ZN8mjCModel8CopyListI7mjCFlexEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEERKS8_
}

/// C: mjCModel::CopyList (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:261)
/// Calls: cxx:__ZN7mjCBase17ResolveReferencesEPK8mjCModel, cxx:__ZN7mjCBase6AddRefEv, cxx:__ZN7mjCMesh10CopyPluginEv, cxx:__ZN7mjCMesh12CopyFromSpecEv, cxx:__ZN7mjCMesh9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_list__3717d26f6f22f0fa(self_ptr: *mut mjCModel, dest: *const (), source: *const ()) {
    todo!() // cxx:__ZN8mjCModel8CopyListI7mjCMeshEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEERKS8_
}

/// C: mjCModel::CopyList (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:261)
/// Calls: cxx:__ZN7mjCBase10CopyPluginEv, cxx:__ZN7mjCBase6AddRefEv, cxx:__ZN7mjCPair12CopyFromSpecEv, cxx:__ZN7mjCPair17ResolveReferencesEPK8mjCModel, cxx:__ZN7mjCPair9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_list__117b6db68e168a73(self_ptr: *mut mjCModel, dest: *const (), source: *const ()) {
    todo!() // cxx:__ZN8mjCModel8CopyListI7mjCPairEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEERKS8_
}

/// C: mjCModel::CopyList (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:261)
/// Calls: cxx:__ZN7mjCBase10CopyPluginEv, cxx:__ZN7mjCBase6AddRefEv, cxx:__ZN7mjCSkin12CopyFromSpecEv, cxx:__ZN7mjCSkin17ResolveReferencesEPK8mjCModel, cxx:__ZN7mjCSkin9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_list__f15664fb4bdaae95(self_ptr: *mut mjCModel, dest: *const (), source: *const ()) {
    todo!() // cxx:__ZN8mjCModel8CopyListI7mjCSkinEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEERKS8_
}

/// C: mjCModel::CopyList (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:261)
/// Calls: cxx:__ZN7mjCBase10CopyPluginEv, cxx:__ZN7mjCBase17ResolveReferencesEPK8mjCModel, cxx:__ZN7mjCBase6AddRefEv, cxx:__ZN7mjCBase9NameSpaceEPK8mjCModel, cxx:__ZN7mjCText12CopyFromSpecEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_list__2cab60d93985d661(self_ptr: *mut mjCModel, dest: *const (), source: *const ()) {
    todo!() // cxx:__ZN8mjCModel8CopyListI7mjCTextEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEERKS8_
}

/// C: mjCModel::CopyList (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:261)
/// Calls: cxx:__ZN7mjCBase10CopyPluginEv, cxx:__ZN7mjCBase6AddRefEv, cxx:__ZN8mjCTuple12CopyFromSpecEv, cxx:__ZN8mjCTuple17ResolveReferencesEPK8mjCModel, cxx:__ZN8mjCTuple9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_list__963ad18b82d1bbfa(self_ptr: *mut mjCModel, dest: *const (), source: *const ()) {
    todo!() // cxx:__ZN8mjCModel8CopyListI8mjCTupleEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEERKS8_
}

/// C: mjCModel::CopyList (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:261)
/// Calls: cxx:__ZN7mjCBase10CopyPluginEv, cxx:__ZN7mjCBase17ResolveReferencesEPK8mjCModel, cxx:__ZN7mjCBase6AddRefEv, cxx:__ZN9mjCHField12CopyFromSpecEv, cxx:__ZN9mjCHField9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_list__5030ba28e353ac04(self_ptr: *mut mjCModel, dest: *const (), source: *const ()) {
    todo!() // cxx:__ZN8mjCModel8CopyListI9mjCHFieldEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEERKS8_
}

/// C: mjCModel::CopyList (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:261)
/// Calls: cxx:__ZN7mjCBase6AddRefEv, cxx:__ZN9mjCSensor10CopyPluginEv, cxx:__ZN9mjCSensor12CopyFromSpecEv, cxx:__ZN9mjCSensor17ResolveReferencesEPK8mjCModel, cxx:__ZN9mjCSensor9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_list__f53e8b19f9e4631c(self_ptr: *mut mjCModel, dest: *const (), source: *const ()) {
    todo!() // cxx:__ZN8mjCModel8CopyListI9mjCSensorEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEERKS8_
}

/// C: mjCModel::CopyList (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:261)
/// Calls: cxx:__ZN7mjCBase10CopyPluginEv, cxx:__ZN7mjCBase6AddRefEv, cxx:__ZN9mjCTendon12CopyFromSpecEv, cxx:__ZN9mjCTendon17ResolveReferencesEPK8mjCModel, cxx:__ZN9mjCTendon9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_list__3f7cf80fc744a57b(self_ptr: *mut mjCModel, dest: *const (), source: *const ()) {
    todo!() // cxx:__ZN8mjCModel8CopyListI9mjCTendonEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEERKS8_
}

/// C: resetlist (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:304)
#[allow(unused_variables, non_snake_case)]
pub fn resetlist__db99c87c850fae7b(list: *const ()) {
    todo!() // cxx-internal:user_model.cc.o:__ZL9resetlistI7mjCBodyEvRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEE
}

/// C: resetlist (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:304)
#[allow(unused_variables, non_snake_case)]
pub fn resetlist__8ebedde2e3e6dee3(list: *const ()) {
    todo!() // cxx-internal:user_model.cc.o:__ZL9resetlistI7mjCGeomEvRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEE
}

/// C: resetlist (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:304)
#[allow(unused_variables, non_snake_case)]
pub fn resetlist__c5c01abe3de4c059(list: *const ()) {
    todo!() // cxx-internal:user_model.cc.o:__ZL9resetlistI7mjCSiteEvRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEE
}

/// C: resetlist (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:304)
#[allow(unused_variables, non_snake_case)]
pub fn resetlist__03d65a901b5ca979(list: *const ()) {
    todo!() // cxx-internal:user_model.cc.o:__ZL9resetlistI8mjCFrameEvRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEE
}

/// C: resetlist (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:304)
#[allow(unused_variables, non_snake_case)]
pub fn resetlist__562baf3060053e2b(list: *const ()) {
    todo!() // cxx-internal:user_model.cc.o:__ZL9resetlistI8mjCJointEvRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEE
}

/// C: resetlist (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:304)
#[allow(unused_variables, non_snake_case)]
pub fn resetlist__79e2c4718a1f78ca(list: *const ()) {
    todo!() // cxx-internal:user_model.cc.o:__ZL9resetlistI8mjCLightEvRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEE
}

/// C: resetlist (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:304)
#[allow(unused_variables, non_snake_case)]
pub fn resetlist__df89e7b4122b19f3(list: *const ()) {
    todo!() // cxx-internal:user_model.cc.o:__ZL9resetlistI9mjCCameraEvRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEE
}

/// C: mjCModel::CopyExplicitPlugin (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:372)
/// Calls: cxx:__ZN7mjCBase6AddRefEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_explicit_plugin__7ec522f49e678eb2(self_ptr: *mut mjCModel, obj: *mut mjCActuator) {
    todo!() // cxx:__ZN8mjCModel18CopyExplicitPluginI11mjCActuatorEEvPT_
}

/// C: mjCModel::CopyExplicitPlugin (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:372)
/// Calls: cxx:__ZN7mjCBase6AddRefEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_explicit_plugin__4e319de557e7b52d(self_ptr: *mut mjCModel, obj: *mut mjCBody) {
    todo!() // cxx:__ZN8mjCModel18CopyExplicitPluginI7mjCBodyEEvPT_
}

/// C: mjCModel::CopyExplicitPlugin (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:372)
/// Calls: cxx:__ZN7mjCBase6AddRefEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_explicit_plugin__df5ce59734c1a8b9(self_ptr: *mut mjCModel, obj: *mut mjCGeom) {
    todo!() // cxx:__ZN8mjCModel18CopyExplicitPluginI7mjCGeomEEvPT_
}

/// C: mjCModel::CopyExplicitPlugin (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:372)
/// Calls: cxx:__ZN7mjCBase6AddRefEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_explicit_plugin__ba295748485e877e(self_ptr: *mut mjCModel, obj: *mut mjCMesh) {
    todo!() // cxx:__ZN8mjCModel18CopyExplicitPluginI7mjCMeshEEvPT_
}

/// C: mjCModel::CopyExplicitPlugin (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:372)
/// Calls: cxx:__ZN7mjCBase6AddRefEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_explicit_plugin__43cbed3a2dbabd73(self_ptr: *mut mjCModel, obj: *mut mjCSensor) {
    todo!() // cxx:__ZN8mjCModel18CopyExplicitPluginI9mjCSensorEEvPT_
}

/// C: mjCModel::CopyPlugin (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:396)
/// Calls: cxx:__ZN7mjCBase9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_plugin__e09dfab01592042f(self_ptr: *mut mjCModel, source: *const (), list: *const ()) {
    todo!() // cxx:__ZN8mjCModel10CopyPluginI11mjCActuatorEEvRKNSt3__16vectorIP9mjCPluginNS2_9allocatorIS5_EEEERKNS3_IPT_NS6_ISC_EEEE
}

/// C: mjCModel::CopyPlugin (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:396)
/// Calls: cxx:__ZN7mjCBase9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_plugin__c36e07707e4d0a3c(self_ptr: *mut mjCModel, source: *const (), list: *const ()) {
    todo!() // cxx:__ZN8mjCModel10CopyPluginI7mjCBodyEEvRKNSt3__16vectorIP9mjCPluginNS2_9allocatorIS5_EEEERKNS3_IPT_NS6_ISC_EEEE
}

/// C: mjCModel::CopyPlugin (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:396)
/// Calls: cxx:__ZN7mjCBase9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_plugin__a88014d3f4bc87b5(self_ptr: *mut mjCModel, source: *const (), list: *const ()) {
    todo!() // cxx:__ZN8mjCModel10CopyPluginI7mjCGeomEEvRKNSt3__16vectorIP9mjCPluginNS2_9allocatorIS5_EEEERKNS3_IPT_NS6_ISC_EEEE
}

/// C: mjCModel::CopyPlugin (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:396)
/// Calls: cxx:__ZN7mjCBase9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_plugin__e7cba27ff04fe98e(self_ptr: *mut mjCModel, source: *const (), list: *const ()) {
    todo!() // cxx:__ZN8mjCModel10CopyPluginI7mjCMeshEEvRKNSt3__16vectorIP9mjCPluginNS2_9allocatorIS5_EEEERKNS3_IPT_NS6_ISC_EEEE
}

/// C: mjCModel::CopyPlugin (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:396)
/// Calls: cxx:__ZN7mjCBase9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_plugin__624ebb82d6241f25(self_ptr: *mut mjCModel, source: *const (), list: *const ()) {
    todo!() // cxx:__ZN8mjCModel10CopyPluginI9mjCSensorEEvRKNSt3__16vectorIP9mjCPluginNS2_9allocatorIS5_EEEERKNS3_IPT_NS6_ISC_EEEE
}

/// C: IsPluginActive (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:440)
#[allow(unused_variables, non_snake_case)]
pub fn is_plugin_active(plugin: *const mjpPlugin, active_plugins: *const ()) -> bool {
    todo!() // cxx-internal:user_model.cc.o:__ZL14IsPluginActivePK10mjpPlugin_RKNSt3__16vectorINS2_4pairIS1_iEENS2_9allocatorIS5_EEEE
}

/// C: mjCModel::RemoveFromList (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:508)
/// Calls: cxx:__ZN11mjCActuator12CopyFromSpecEv, cxx:__ZN11mjCActuator17ResolveReferencesEPK8mjCModel, cxx:__ZN11mjCActuator9NameSpaceEPK8mjCModel, cxx:__ZN7mjCBase7ReleaseEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_remove_from_list__7bf6f5faa7dca33a(self_ptr: *mut mjCModel, list: *const (), other: *const mjCModel) {
    todo!() // cxx:__ZN8mjCModel14RemoveFromListI11mjCActuatorEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEERKS_
}

/// C: mjCModel::RemoveFromList (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:508)
/// Calls: cxx:__ZN11mjCBodyPair12CopyFromSpecEv, cxx:__ZN11mjCBodyPair17ResolveReferencesEPK8mjCModel, cxx:__ZN11mjCBodyPair9NameSpaceEPK8mjCModel, cxx:__ZN7mjCBase7ReleaseEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_remove_from_list__0762c10562810ebc(self_ptr: *mut mjCModel, list: *const (), other: *const mjCModel) {
    todo!() // cxx:__ZN8mjCModel14RemoveFromListI11mjCBodyPairEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEERKS_
}

/// C: mjCModel::RemoveFromList (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:508)
/// Calls: cxx:__ZN11mjCEquality12CopyFromSpecEv, cxx:__ZN11mjCEquality17ResolveReferencesEPK8mjCModel, cxx:__ZN11mjCEquality9NameSpaceEPK8mjCModel, cxx:__ZN7mjCBase7ReleaseEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_remove_from_list__e6c26410bfaba695(self_ptr: *mut mjCModel, list: *const (), other: *const mjCModel) {
    todo!() // cxx:__ZN8mjCModel14RemoveFromListI11mjCEqualityEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEERKS_
}

/// C: mjCModel::RemoveFromList (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:508)
/// Calls: cxx:__ZN7mjCBase7ReleaseEv, cxx:__ZN7mjCPair12CopyFromSpecEv, cxx:__ZN7mjCPair17ResolveReferencesEPK8mjCModel, cxx:__ZN7mjCPair9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_remove_from_list__fb1f542eecc35766(self_ptr: *mut mjCModel, list: *const (), other: *const mjCModel) {
    todo!() // cxx:__ZN8mjCModel14RemoveFromListI7mjCPairEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEERKS_
}

/// C: mjCModel::RemoveFromList (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:508)
/// Calls: cxx:__ZN7mjCBase7ReleaseEv, cxx:__ZN9mjCSensor12CopyFromSpecEv, cxx:__ZN9mjCSensor17ResolveReferencesEPK8mjCModel, cxx:__ZN9mjCSensor9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_remove_from_list__c6b8bef9a0268276(self_ptr: *mut mjCModel, list: *const (), other: *const mjCModel) {
    todo!() // cxx:__ZN8mjCModel14RemoveFromListI9mjCSensorEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEERKS_
}

/// C: mjCModel::RemoveFromList (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:508)
/// Calls: cxx:__ZN7mjCBase7ReleaseEv, cxx:__ZN9mjCTendon12CopyFromSpecEv, cxx:__ZN9mjCTendon17ResolveReferencesEPK8mjCModel, cxx:__ZN9mjCTendon9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_remove_from_list__0beb36889251ad9d(self_ptr: *mut mjCModel, list: *const (), other: *const mjCModel) {
    todo!() // cxx:__ZN8mjCModel14RemoveFromListI9mjCTendonEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEERKS_
}

/// C: mjCModel::DeleteAll (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:545)
/// Calls: cxx:__ZN7mjCBase7ReleaseEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_delete_all__c47a67d620ef2ad5(self_ptr: *mut mjCModel, elements: *const ()) {
    todo!() // cxx:__ZN8mjCModel9DeleteAllI6mjCKeyEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEE
}

/// C: mjCModel::MarkPluginInstance (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:555)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_mark_plugin_instance__43c5999a3e1dc86a(self_ptr: *mut mjCModel, instances: *const (), list: *const ()) {
    todo!() // cxx:__ZN8mjCModel18MarkPluginInstanceI11mjCActuatorEEvRNSt3__113unordered_mapINS2_12basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEEbNS2_4hashIS9_EENS2_8equal_toIS9_EENS7_INS2_4pairIKS9_bEEEEEERKNS2_6vectorIPT_NS7_ISM_EEEE
}

/// C: mjCModel::MarkPluginInstance (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:555)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_mark_plugin_instance__897c92d4db248d50(self_ptr: *mut mjCModel, instances: *const (), list: *const ()) {
    todo!() // cxx:__ZN8mjCModel18MarkPluginInstanceI7mjCBodyEEvRNSt3__113unordered_mapINS2_12basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEEbNS2_4hashIS9_EENS2_8equal_toIS9_EENS7_INS2_4pairIKS9_bEEEEEERKNS2_6vectorIPT_NS7_ISM_EEEE
}

/// C: mjCModel::MarkPluginInstance (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:555)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_mark_plugin_instance__60c5b166745a7e81(self_ptr: *mut mjCModel, instances: *const (), list: *const ()) {
    todo!() // cxx:__ZN8mjCModel18MarkPluginInstanceI7mjCGeomEEvRNSt3__113unordered_mapINS2_12basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEEbNS2_4hashIS9_EENS2_8equal_toIS9_EENS7_INS2_4pairIKS9_bEEEEEERKNS2_6vectorIPT_NS7_ISM_EEEE
}

/// C: mjCModel::MarkPluginInstance (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:555)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_mark_plugin_instance__8c25e48c7af4fbc2(self_ptr: *mut mjCModel, instances: *const (), list: *const ()) {
    todo!() // cxx:__ZN8mjCModel18MarkPluginInstanceI7mjCMeshEEvRNSt3__113unordered_mapINS2_12basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEEbNS2_4hashIS9_EENS2_8equal_toIS9_EENS7_INS2_4pairIKS9_bEEEEEERKNS2_6vectorIPT_NS7_ISM_EEEE
}

/// C: mjCModel::MarkPluginInstance (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:555)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_mark_plugin_instance__b9da57b242955b0d(self_ptr: *mut mjCModel, instances: *const (), list: *const ()) {
    todo!() // cxx:__ZN8mjCModel18MarkPluginInstanceI9mjCSensorEEvRNSt3__113unordered_mapINS2_12basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEEbNS2_4hashIS9_EENS2_8equal_toIS9_EENS7_INS2_4pairIKS9_bEEEEEERKNS2_6vectorIPT_NS7_ISM_EEEE
}

/// C: deletefromlist (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:715)
#[allow(unused_variables, non_snake_case)]
pub fn deletefromlist__d818068c18930008(list: *const (), element: *mut mjsElement) {
    todo!() // cxx:__Z14deletefromlistI7mjCBaseEvPNSt3__16vectorIPT_NS1_9allocatorIS4_EEEEP11mjsElement_
}

/// C: deletefromlist (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:715)
#[allow(unused_variables, non_snake_case)]
pub fn deletefromlist__d53d8c447a53773d(list: *const (), element: *mut mjsElement) {
    todo!() // cxx:__Z14deletefromlistI7mjCGeomEvPNSt3__16vectorIPT_NS1_9allocatorIS4_EEEEP11mjsElement_
}

/// C: deletefromlist (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:715)
#[allow(unused_variables, non_snake_case)]
pub fn deletefromlist__fae33eba2a03d454(list: *const (), element: *mut mjsElement) {
    todo!() // cxx:__Z14deletefromlistI7mjCSiteEvPNSt3__16vectorIPT_NS1_9allocatorIS4_EEEEP11mjsElement_
}

/// C: deletefromlist (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:715)
#[allow(unused_variables, non_snake_case)]
pub fn deletefromlist__73477e1d29f15c10(list: *const (), element: *mut mjsElement) {
    todo!() // cxx:__Z14deletefromlistI8mjCJointEvPNSt3__16vectorIPT_NS1_9allocatorIS4_EEEEP11mjsElement_
}

/// C: deletefromlist (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:715)
#[allow(unused_variables, non_snake_case)]
pub fn deletefromlist__1d2af50518e2d57a(list: *const (), element: *mut mjsElement) {
    todo!() // cxx:__Z14deletefromlistI8mjCLightEvPNSt3__16vectorIPT_NS1_9allocatorIS4_EEEEP11mjsElement_
}

/// C: deletefromlist (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:715)
#[allow(unused_variables, non_snake_case)]
pub fn deletefromlist__71b5bbb6db6fd8cc(list: *const (), element: *mut mjsElement) {
    todo!() // cxx:__Z14deletefromlistI9mjCCameraEvPNSt3__16vectorIPT_NS1_9allocatorIS4_EEEEP11mjsElement_
}

/// C: mjCModel::AddObject (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1252)
/// Calls: cxx:__ZN8mjCModel9SignatureEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_object__a1ca4a2e4f07867d(self_ptr: *mut mjCModel, list: *const (), r#type: std__string) -> *mut mjCNumeric {
    todo!() // cxx:__ZN8mjCModel9AddObjectI10mjCNumericEEPT_RNSt3__16vectorIS3_NS4_9allocatorIS3_EEEENS4_12basic_stringIcNS4_11char_traitsIcEENS6_IcEEEE
}

/// C: mjCModel::AddObject (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1252)
/// Calls: cxx:__ZN8mjCModel9SignatureEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_object__309333750ebb15fc(self_ptr: *mut mjCModel, list: *const (), r#type: std__string) -> *mut mjCTexture {
    todo!() // cxx:__ZN8mjCModel9AddObjectI10mjCTextureEEPT_RNSt3__16vectorIS3_NS4_9allocatorIS3_EEEENS4_12basic_stringIcNS4_11char_traitsIcEENS6_IcEEEE
}

/// C: mjCModel::AddObject (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1252)
/// Calls: cxx:__ZN8mjCModel9SignatureEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_object__86cf5dcb3e565af4(self_ptr: *mut mjCModel, list: *const (), r#type: std__string) -> *mut mjCBodyPair {
    todo!() // cxx:__ZN8mjCModel9AddObjectI11mjCBodyPairEEPT_RNSt3__16vectorIS3_NS4_9allocatorIS3_EEEENS4_12basic_stringIcNS4_11char_traitsIcEENS6_IcEEEE
}

/// C: mjCModel::AddObject (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1252)
/// Calls: cxx:__ZN8mjCModel9SignatureEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_object__106dadb6538e956e(self_ptr: *mut mjCModel, list: *const (), r#type: std__string) -> *mut mjCKey {
    todo!() // cxx:__ZN8mjCModel9AddObjectI6mjCKeyEEPT_RNSt3__16vectorIS3_NS4_9allocatorIS3_EEEENS4_12basic_stringIcNS4_11char_traitsIcEENS6_IcEEEE
}

/// C: mjCModel::AddObject (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1252)
/// Calls: cxx:__ZN8mjCModel9SignatureEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_object__e752e7648b0427f1(self_ptr: *mut mjCModel, list: *const (), r#type: std__string) -> *mut mjCFlex {
    todo!() // cxx:__ZN8mjCModel9AddObjectI7mjCFlexEEPT_RNSt3__16vectorIS3_NS4_9allocatorIS3_EEEENS4_12basic_stringIcNS4_11char_traitsIcEENS6_IcEEEE
}

/// C: mjCModel::AddObject (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1252)
/// Calls: cxx:__ZN8mjCModel9SignatureEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_object__8e1b676393a50dec(self_ptr: *mut mjCModel, list: *const (), r#type: std__string) -> *mut mjCSkin {
    todo!() // cxx:__ZN8mjCModel9AddObjectI7mjCSkinEEPT_RNSt3__16vectorIS3_NS4_9allocatorIS3_EEEENS4_12basic_stringIcNS4_11char_traitsIcEENS6_IcEEEE
}

/// C: mjCModel::AddObject (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1252)
/// Calls: cxx:__ZN8mjCModel9SignatureEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_object__a58d356621ca7cd4(self_ptr: *mut mjCModel, list: *const (), r#type: std__string) -> *mut mjCText {
    todo!() // cxx:__ZN8mjCModel9AddObjectI7mjCTextEEPT_RNSt3__16vectorIS3_NS4_9allocatorIS3_EEEENS4_12basic_stringIcNS4_11char_traitsIcEENS6_IcEEEE
}

/// C: mjCModel::AddObject (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1252)
/// Calls: cxx:__ZN8mjCModel9SignatureEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_object__f36c37651aef502d(self_ptr: *mut mjCModel, list: *const (), r#type: std__string) -> *mut mjCTuple {
    todo!() // cxx:__ZN8mjCModel9AddObjectI8mjCTupleEEPT_RNSt3__16vectorIS3_NS4_9allocatorIS3_EEEENS4_12basic_stringIcNS4_11char_traitsIcEENS6_IcEEEE
}

/// C: mjCModel::AddObject (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1252)
/// Calls: cxx:__ZN8mjCModel9SignatureEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_object__eb5a59c9f800c457(self_ptr: *mut mjCModel, list: *const (), r#type: std__string) -> *mut mjCHField {
    todo!() // cxx:__ZN8mjCModel9AddObjectI9mjCHFieldEEPT_RNSt3__16vectorIS3_NS4_9allocatorIS3_EEEENS4_12basic_stringIcNS4_11char_traitsIcEENS6_IcEEEE
}

/// C: mjCModel::AddObject (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1252)
/// Calls: cxx:__ZN8mjCModel9SignatureEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_object__cea417951b064e82(self_ptr: *mut mjCModel, list: *const (), r#type: std__string) -> *mut mjCPlugin {
    todo!() // cxx:__ZN8mjCModel9AddObjectI9mjCPluginEEPT_RNSt3__16vectorIS3_NS4_9allocatorIS3_EEEENS4_12basic_stringIcNS4_11char_traitsIcEENS6_IcEEEE
}

/// C: mjCModel::AddObject (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1252)
/// Calls: cxx:__ZN8mjCModel9SignatureEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_object__a411481ab41c8d44(self_ptr: *mut mjCModel, list: *const (), r#type: std__string) -> *mut mjCSensor {
    todo!() // cxx:__ZN8mjCModel9AddObjectI9mjCSensorEEPT_RNSt3__16vectorIS3_NS4_9allocatorIS3_EEEENS4_12basic_stringIcNS4_11char_traitsIcEENS6_IcEEEE
}

/// C: mjCModel::AddObjectDefault (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1263)
/// Calls: cxx:__ZN8mjCModel9SignatureEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_object_default__b94e846fc3c9f7c2(self_ptr: *mut mjCModel, list: *const (), r#type: std__string, def: *mut mjCDef) -> *mut mjCActuator {
    todo!() // cxx:__ZN8mjCModel16AddObjectDefaultI11mjCActuatorEEPT_RNSt3__16vectorIS3_NS4_9allocatorIS3_EEEENS4_12basic_stringIcNS4_11char_traitsIcEENS6_IcEEEEP6mjCDef
}

/// C: mjCModel::AddObjectDefault (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1263)
/// Calls: cxx:__ZN8mjCModel9SignatureEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_object_default__296348cbed3630bc(self_ptr: *mut mjCModel, list: *const (), r#type: std__string, def: *mut mjCDef) -> *mut mjCEquality {
    todo!() // cxx:__ZN8mjCModel16AddObjectDefaultI11mjCEqualityEEPT_RNSt3__16vectorIS3_NS4_9allocatorIS3_EEEENS4_12basic_stringIcNS4_11char_traitsIcEENS6_IcEEEEP6mjCDef
}

/// C: mjCModel::AddObjectDefault (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1263)
/// Calls: cxx:__ZN8mjCModel9SignatureEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_object_default__b23d9529a42503ac(self_ptr: *mut mjCModel, list: *const (), r#type: std__string, def: *mut mjCDef) -> *mut mjCMaterial {
    todo!() // cxx:__ZN8mjCModel16AddObjectDefaultI11mjCMaterialEEPT_RNSt3__16vectorIS3_NS4_9allocatorIS3_EEEENS4_12basic_stringIcNS4_11char_traitsIcEENS6_IcEEEEP6mjCDef
}

/// C: mjCModel::AddObjectDefault (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1263)
/// Calls: cxx:__ZN8mjCModel9SignatureEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_object_default__5d6940146c8d138a(self_ptr: *mut mjCModel, list: *const (), r#type: std__string, def: *mut mjCDef) -> *mut mjCMesh {
    todo!() // cxx:__ZN8mjCModel16AddObjectDefaultI7mjCMeshEEPT_RNSt3__16vectorIS3_NS4_9allocatorIS3_EEEENS4_12basic_stringIcNS4_11char_traitsIcEENS6_IcEEEEP6mjCDef
}

/// C: mjCModel::AddObjectDefault (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1263)
/// Calls: cxx:__ZN8mjCModel9SignatureEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_object_default__4b6992daa4d6b439(self_ptr: *mut mjCModel, list: *const (), r#type: std__string, def: *mut mjCDef) -> *mut mjCPair {
    todo!() // cxx:__ZN8mjCModel16AddObjectDefaultI7mjCPairEEPT_RNSt3__16vectorIS3_NS4_9allocatorIS3_EEEENS4_12basic_stringIcNS4_11char_traitsIcEENS6_IcEEEEP6mjCDef
}

/// C: mjCModel::AddObjectDefault (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1263)
/// Calls: cxx:__ZN8mjCModel9SignatureEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_object_default__9fc4f55b08ccc785(self_ptr: *mut mjCModel, list: *const (), r#type: std__string, def: *mut mjCDef) -> *mut mjCTendon {
    todo!() // cxx:__ZN8mjCModel16AddObjectDefaultI9mjCTendonEEPT_RNSt3__16vectorIS3_NS4_9allocatorIS3_EEEENS4_12basic_stringIcNS4_11char_traitsIcEENS6_IcEEEEP6mjCDef
}

/// C: GetNext (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1411)
#[allow(unused_variables, non_snake_case)]
pub fn get_next__5e1ef9d33a504ba7(list: *const (), child: *const mjsElement) -> *mut mjsElement {
    todo!() // cxx-internal:user_model.cc.o:__ZL7GetNextI10mjCNumericEP11mjsElement_RKNSt3__16vectorIPT_NS3_9allocatorIS6_EEEEPKS1_
}

/// C: GetNext (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1411)
#[allow(unused_variables, non_snake_case)]
pub fn get_next__44734fdfb412b3d9(list: *const (), child: *const mjsElement) -> *mut mjsElement {
    todo!() // cxx-internal:user_model.cc.o:__ZL7GetNextI10mjCTextureEP11mjsElement_RKNSt3__16vectorIPT_NS3_9allocatorIS6_EEEEPKS1_
}

/// C: GetNext (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1411)
#[allow(unused_variables, non_snake_case)]
pub fn get_next__58d4c2a206e110d7(list: *const (), child: *const mjsElement) -> *mut mjsElement {
    todo!() // cxx-internal:user_model.cc.o:__ZL7GetNextI11mjCActuatorEP11mjsElement_RKNSt3__16vectorIPT_NS3_9allocatorIS6_EEEEPKS1_
}

/// C: GetNext (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1411)
#[allow(unused_variables, non_snake_case)]
pub fn get_next__f24a2e9262e4175c(list: *const (), child: *const mjsElement) -> *mut mjsElement {
    todo!() // cxx-internal:user_model.cc.o:__ZL7GetNextI11mjCBodyPairEP11mjsElement_RKNSt3__16vectorIPT_NS3_9allocatorIS6_EEEEPKS1_
}

/// C: GetNext (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1411)
#[allow(unused_variables, non_snake_case)]
pub fn get_next__ce179764e4fc5ee2(list: *const (), child: *const mjsElement) -> *mut mjsElement {
    todo!() // cxx-internal:user_model.cc.o:__ZL7GetNextI11mjCEqualityEP11mjsElement_RKNSt3__16vectorIPT_NS3_9allocatorIS6_EEEEPKS1_
}

/// C: GetNext (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1411)
#[allow(unused_variables, non_snake_case)]
pub fn get_next__df5317d82077012b(list: *const (), child: *const mjsElement) -> *mut mjsElement {
    todo!() // cxx-internal:user_model.cc.o:__ZL7GetNextI11mjCMaterialEP11mjsElement_RKNSt3__16vectorIPT_NS3_9allocatorIS6_EEEEPKS1_
}

/// C: GetNext (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1411)
#[allow(unused_variables, non_snake_case)]
pub fn get_next__cd616d20c6d44647(list: *const (), child: *const mjsElement) -> *mut mjsElement {
    todo!() // cxx-internal:user_model.cc.o:__ZL7GetNextI6mjCKeyEP11mjsElement_RKNSt3__16vectorIPT_NS3_9allocatorIS6_EEEEPKS1_
}

/// C: GetNext (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1411)
#[allow(unused_variables, non_snake_case)]
pub fn get_next__e08b318a40b8b7ef(list: *const (), child: *const mjsElement) -> *mut mjsElement {
    todo!() // cxx-internal:user_model.cc.o:__ZL7GetNextI7mjCFlexEP11mjsElement_RKNSt3__16vectorIPT_NS3_9allocatorIS6_EEEEPKS1_
}

/// C: GetNext (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1411)
#[allow(unused_variables, non_snake_case)]
pub fn get_next__f42e4a99f2eb4385(list: *const (), child: *const mjsElement) -> *mut mjsElement {
    todo!() // cxx-internal:user_model.cc.o:__ZL7GetNextI7mjCMeshEP11mjsElement_RKNSt3__16vectorIPT_NS3_9allocatorIS6_EEEEPKS1_
}

/// C: GetNext (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1411)
#[allow(unused_variables, non_snake_case)]
pub fn get_next__2e0e6a2fe13cad15(list: *const (), child: *const mjsElement) -> *mut mjsElement {
    todo!() // cxx-internal:user_model.cc.o:__ZL7GetNextI7mjCPairEP11mjsElement_RKNSt3__16vectorIPT_NS3_9allocatorIS6_EEEEPKS1_
}

/// C: GetNext (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1411)
#[allow(unused_variables, non_snake_case)]
pub fn get_next__e959fbf62d62b547(list: *const (), child: *const mjsElement) -> *mut mjsElement {
    todo!() // cxx-internal:user_model.cc.o:__ZL7GetNextI7mjCSkinEP11mjsElement_RKNSt3__16vectorIPT_NS3_9allocatorIS6_EEEEPKS1_
}

/// C: GetNext (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1411)
#[allow(unused_variables, non_snake_case)]
pub fn get_next__a2d8e9080eaee917(list: *const (), child: *const mjsElement) -> *mut mjsElement {
    todo!() // cxx-internal:user_model.cc.o:__ZL7GetNextI7mjCTextEP11mjsElement_RKNSt3__16vectorIPT_NS3_9allocatorIS6_EEEEPKS1_
}

/// C: GetNext (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1411)
#[allow(unused_variables, non_snake_case)]
pub fn get_next__a912044d51b0b41b(list: *const (), child: *const mjsElement) -> *mut mjsElement {
    todo!() // cxx-internal:user_model.cc.o:__ZL7GetNextI8mjCTupleEP11mjsElement_RKNSt3__16vectorIPT_NS3_9allocatorIS6_EEEEPKS1_
}

/// C: GetNext (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1411)
#[allow(unused_variables, non_snake_case)]
pub fn get_next__c6c01d3a6dcc055c(list: *const (), child: *const mjsElement) -> *mut mjsElement {
    todo!() // cxx-internal:user_model.cc.o:__ZL7GetNextI9mjCHFieldEP11mjsElement_RKNSt3__16vectorIPT_NS3_9allocatorIS6_EEEEPKS1_
}

/// C: GetNext (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1411)
#[allow(unused_variables, non_snake_case)]
pub fn get_next__4db86cfcd8afe80c(list: *const (), child: *const mjsElement) -> *mut mjsElement {
    todo!() // cxx-internal:user_model.cc.o:__ZL7GetNextI9mjCPluginEP11mjsElement_RKNSt3__16vectorIPT_NS3_9allocatorIS6_EEEEPKS1_
}

/// C: GetNext (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1411)
#[allow(unused_variables, non_snake_case)]
pub fn get_next__a3ec34e63a114c66(list: *const (), child: *const mjsElement) -> *mut mjsElement {
    todo!() // cxx-internal:user_model.cc.o:__ZL7GetNextI9mjCSensorEP11mjsElement_RKNSt3__16vectorIPT_NS3_9allocatorIS6_EEEEPKS1_
}

/// C: GetNext (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1411)
#[allow(unused_variables, non_snake_case)]
pub fn get_next__a1a8dcc4fffe3f88(list: *const (), child: *const mjsElement) -> *mut mjsElement {
    todo!() // cxx-internal:user_model.cc.o:__ZL7GetNextI9mjCTendonEP11mjsElement_RKNSt3__16vectorIPT_NS3_9allocatorIS6_EEEEPKS1_
}

/// C: mjCModel::FindDefault (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1552)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_find_default(self_ptr: *mut mjCModel, name: *const anonymous_namespace___string) -> *mut mjCDef {
    todo!() // cxx:__ZNK8mjCModel11FindDefaultERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE
}

/// C: mjCModel::AddDefault (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1564)
/// Calls: cxx:__ZN6mjCDef12CopyFromSpecEv, cxx:__ZN6mjCDef19CopyWithoutChildrenERKS_
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_default(self_ptr: *mut mjCModel, name: std__string, parent: *mut mjCDef) -> *mut mjCDef {
    todo!() // cxx:__ZN8mjCModel10AddDefaultENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEP6mjCDef
}

/// C: findobject (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1596)
#[allow(unused_variables, non_snake_case)]
pub fn findobject(name: std__string_view, list: *const (), ids: *const mjKeyMap) -> *mut mjCBase {
    todo!() // cxx-internal:user_model.cc.o:__ZL10findobjectI7mjCBaseEPT_NSt3__117basic_string_viewIcNS3_11char_traitsIcEEEERKNS3_6vectorIS2_NS3_9allocatorIS2_EEEERKNS3_3mapINS3_12basic_stringIcS6_NS9_IcEEEEiNS3_4lessIvEENS9_INS3_4pairIKSH_iEEEEEE
}

/// C: mjCModel::FindAsset (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1621)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_find_asset__c6e9908471edd10d(self_ptr: *mut mjCModel, name: std__string_view, list: *const ()) -> *mut mjCBase {
    todo!() // cxx:__ZNK8mjCModel9FindAssetI10mjCTextureEEP7mjCBaseNSt3__117basic_string_viewIcNS4_11char_traitsIcEEEERKNS4_6vectorIPT_NS4_9allocatorISB_EEEE
}

/// C: mjCModel::FindAsset (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1621)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_find_asset__f2ed52d45654a132(self_ptr: *mut mjCModel, name: std__string_view, list: *const ()) -> *mut mjCBase {
    todo!() // cxx:__ZNK8mjCModel9FindAssetI7mjCMeshEEP7mjCBaseNSt3__117basic_string_viewIcNS4_11char_traitsIcEEEERKNS4_6vectorIPT_NS4_9allocatorISB_EEEE
}

/// C: mjCModel::FindObject (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1642)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_find_object(self_ptr: *mut mjCModel, r#type: u32, name: std__string) -> *mut mjCBase {
    todo!() // cxx:__ZNK8mjCModel10FindObjectE6mjtObjNSt3__112basic_stringIcNS1_11char_traitsIcEENS1_9allocatorIcEEEE
}

/// C: mjCModel::DeleteMaterial (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1778)
/// Calls: cxx:__ZN7mjCGeom12del_materialEv, cxx:__ZNK7mjCGeom12get_materialEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_delete_material__853a66e31485f2b2(self_ptr: *mut mjCModel, list: *const (), name: std__string_view) {
    todo!() // cxx:__ZN8mjCModel14DeleteMaterialI7mjCGeomEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEENS2_17basic_string_viewIcNS2_11char_traitsIcEEEE
}

/// C: mjCModel::DeleteMaterial (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1778)
/// Calls: cxx:__ZN7mjCSite12del_materialEv, cxx:__ZNK7mjCSite12get_materialEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_delete_material__990d4fccf8514798(self_ptr: *mut mjCModel, list: *const (), name: std__string_view) {
    todo!() // cxx:__ZN8mjCModel14DeleteMaterialI7mjCSiteEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEENS2_17basic_string_viewIcNS2_11char_traitsIcEEEE
}

/// C: mjCModel::DeleteMaterial (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1778)
/// Calls: cxx:__ZN7mjCSkin12del_materialEv, cxx:__ZNK7mjCSkin12get_materialEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_delete_material__5dcce31c190aec07(self_ptr: *mut mjCModel, list: *const (), name: std__string_view) {
    todo!() // cxx:__ZN8mjCModel14DeleteMaterialI7mjCSkinEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEENS2_17basic_string_viewIcNS2_11char_traitsIcEEEE
}

/// C: mjCModel::DeleteMaterial (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1778)
/// Calls: cxx:__ZN9mjCTendon12del_materialEv, cxx:__ZNK9mjCTendon12get_materialEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_delete_material__5fa21387c3051cfc(self_ptr: *mut mjCModel, list: *const (), name: std__string_view) {
    todo!() // cxx:__ZN8mjCModel14DeleteMaterialI9mjCTendonEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEENS2_17basic_string_viewIcNS2_11char_traitsIcEEEE
}

/// C: DeleteAllTextures (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1790)
/// Calls: cxx:__ZN11mjCMaterial12del_texturesEv
#[allow(unused_variables, non_snake_case)]
pub fn delete_all_textures(list: *const ()) {
    todo!() // cxx-internal:user_model.cc.o:__ZL17DeleteAllTexturesI11mjCMaterialEvRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEE
}

/// C: DeleteTexcoord (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1799)
/// Calls: cxx:__ZN7mjCFlex11DelTexcoordEv, cxx:__ZNK7mjCFlex11HasTexcoordEv
#[allow(unused_variables, non_snake_case)]
pub fn delete_texcoord__e0514dcac6bca6a1(list: *const ()) {
    todo!() // cxx-internal:user_model.cc.o:__ZL14DeleteTexcoordI7mjCFlexEvRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEE
}

/// C: DeleteTexcoord (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1799)
/// Calls: cxx:__ZN7mjCMesh11DelTexcoordEv, cxx:__ZNK7mjCMesh11HasTexcoordEv
#[allow(unused_variables, non_snake_case)]
pub fn delete_texcoord__4d38ea63221604aa(list: *const ()) {
    todo!() // cxx-internal:user_model.cc.o:__ZL14DeleteTexcoordI7mjCMeshEvRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEE
}

/// C: DeleteElements (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1810)
/// Calls: cxx:__ZN7mjCBase7ReleaseEv
#[allow(unused_variables, non_snake_case)]
pub fn delete_elements__c84f60afdf2d21ee(elements: *const (), discard: *const ()) {
    todo!() // cxx-internal:user_model.cc.o:__ZL14DeleteElementsI7mjCGeomEvRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEERKNS2_IbNS5_IbEEEE
}

/// C: DeleteElements (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1810)
/// Calls: cxx:__ZN7mjCBase7ReleaseEv
#[allow(unused_variables, non_snake_case)]
pub fn delete_elements__aa087ae03efad0d6(elements: *const (), discard: *const ()) {
    todo!() // cxx-internal:user_model.cc.o:__ZL14DeleteElementsI7mjCMeshEvRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEERKNS2_IbNS5_IbEEEE
}

/// C: mjCModel::Delete (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1848)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_delete__8108903c25a6cbc6(self_ptr: *mut mjCModel, elements: *const (), discard: *const ()) {
    todo!() // cxx:__ZN8mjCModel6DeleteI7mjCGeomEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEERKNS3_IbNS6_IbEEEE
}

/// C: mjCModel::Delete (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1866)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_delete__54bfbfb7e5658a34(self_ptr: *mut mjCModel, elements: *const (), discard: *const ()) {
    todo!() // cxx:__ZN8mjCModel6DeleteI7mjCMeshEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEERKNS3_IbNS6_IbEEEE
}

/// C: mjCModel::DeleteAll (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1873)
/// Calls: cxx:__ZN7mjCBase7ReleaseEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_delete_all__f78afda1016c6a5a(self_ptr: *mut mjCModel, elements: *const ()) {
    todo!() // cxx:__ZN8mjCModel9DeleteAllI11mjCMaterialEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEE
}

/// C: mjCModel::DeleteAll (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:1886)
/// Calls: cxx-internal:user_model.cc.o:__ZL17DeleteAllTexturesI11mjCMaterialEvRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEE, cxx:__ZN7mjCBase7ReleaseEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_delete_all__694b2cf98dd51743(self_ptr: *mut mjCModel, elements: *const ()) {
    todo!() // cxx:__ZN8mjCModel9DeleteAllI10mjCTextureEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEE
}

/// C: getpathslength (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:2128)
/// Calls: cxx:__ZNK10mjCTexture4FileEv
#[allow(unused_variables, non_snake_case)]
pub fn getpathslength__e66141b501182d48(list: *const ()) -> usize {
    todo!() // cxx-internal:user_model.cc.o:__ZL14getpathslengthIP10mjCTextureEmNSt3__16vectorIT_NS2_9allocatorIS4_EEEE
}

/// C: getpathslength (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:2128)
/// Calls: cxx:__ZNK7mjCMesh4FileEv
#[allow(unused_variables, non_snake_case)]
pub fn getpathslength__f7fb419238cb9a57(list: *const ()) -> usize {
    todo!() // cxx-internal:user_model.cc.o:__ZL14getpathslengthIP7mjCMeshEmNSt3__16vectorIT_NS2_9allocatorIS4_EEEE
}

/// C: getpathslength (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:2128)
/// Calls: cxx:__ZNK7mjCSkin4FileEv
#[allow(unused_variables, non_snake_case)]
pub fn getpathslength__6aedd18cbaf52ffe(list: *const ()) -> usize {
    todo!() // cxx-internal:user_model.cc.o:__ZL14getpathslengthIP7mjCSkinEmNSt3__16vectorIT_NS2_9allocatorIS4_EEEE
}

/// C: getpathslength (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:2128)
/// Calls: cxx:__ZNK9mjCHField4FileEv
#[allow(unused_variables, non_snake_case)]
pub fn getpathslength__35b2e4063a55fe01(list: *const ()) -> usize {
    todo!() // cxx-internal:user_model.cc.o:__ZL14getpathslengthIP9mjCHFieldEmNSt3__16vectorIT_NS2_9allocatorIS4_EEEE
}

/// C: LRfunc (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:2459)
/// Calls: cxx:_mj_setLengthRange
#[allow(unused_variables, non_snake_case)]
pub fn l_rfunc(arg: *mut ()) -> *mut () {
    todo!() // cxx:__Z6LRfuncPv
}

/// C: addtolist (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:2577)
#[allow(unused_variables, non_snake_case)]
pub fn addtolist(input: *const std__string, adr: i32, output_adr_field: *mut i32, output_buffer: *mut i8) -> i32 {
    todo!() // cxx-internal:user_model.cc.o:__ZL9addtolistRKNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEiPiPc
}

/// C: namelist (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:2593)
/// Calls: cxx-internal:user_model.cc.o:__ZL9addtolistRKNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEiPiPc, cxx:_mj_hashString
#[allow(unused_variables, non_snake_case)]
pub fn namelist__11af11b263c9a679(list: *const (), adr: i32, name_adr: *mut i32, names: *mut i8, map: *mut i32) -> i32 {
    todo!() // cxx-internal:user_model.cc.o:__ZL8namelistI10mjCNumericEiRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEEiPiPcS9_
}

/// C: namelist (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:2593)
/// Calls: cxx-internal:user_model.cc.o:__ZL9addtolistRKNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEiPiPc, cxx:_mj_hashString
#[allow(unused_variables, non_snake_case)]
pub fn namelist__3c96a2912a5d9cf7(list: *const (), adr: i32, name_adr: *mut i32, names: *mut i8, map: *mut i32) -> i32 {
    todo!() // cxx-internal:user_model.cc.o:__ZL8namelistI10mjCTextureEiRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEEiPiPcS9_
}

/// C: namelist (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:2593)
/// Calls: cxx-internal:user_model.cc.o:__ZL9addtolistRKNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEiPiPc, cxx:_mj_hashString
#[allow(unused_variables, non_snake_case)]
pub fn namelist__77baa1e46e8182a7(list: *const (), adr: i32, name_adr: *mut i32, names: *mut i8, map: *mut i32) -> i32 {
    todo!() // cxx-internal:user_model.cc.o:__ZL8namelistI11mjCActuatorEiRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEEiPiPcS9_
}

/// C: namelist (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:2593)
/// Calls: cxx-internal:user_model.cc.o:__ZL9addtolistRKNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEiPiPc, cxx:_mj_hashString
#[allow(unused_variables, non_snake_case)]
pub fn namelist__feaa85860705594b(list: *const (), adr: i32, name_adr: *mut i32, names: *mut i8, map: *mut i32) -> i32 {
    todo!() // cxx-internal:user_model.cc.o:__ZL8namelistI11mjCBodyPairEiRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEEiPiPcS9_
}

/// C: namelist (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:2593)
/// Calls: cxx-internal:user_model.cc.o:__ZL9addtolistRKNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEiPiPc, cxx:_mj_hashString
#[allow(unused_variables, non_snake_case)]
pub fn namelist__02f407bc640d2244(list: *const (), adr: i32, name_adr: *mut i32, names: *mut i8, map: *mut i32) -> i32 {
    todo!() // cxx-internal:user_model.cc.o:__ZL8namelistI11mjCEqualityEiRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEEiPiPcS9_
}

/// C: namelist (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:2593)
/// Calls: cxx-internal:user_model.cc.o:__ZL9addtolistRKNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEiPiPc, cxx:_mj_hashString
#[allow(unused_variables, non_snake_case)]
pub fn namelist__c16ed064cda28814(list: *const (), adr: i32, name_adr: *mut i32, names: *mut i8, map: *mut i32) -> i32 {
    todo!() // cxx-internal:user_model.cc.o:__ZL8namelistI11mjCMaterialEiRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEEiPiPcS9_
}

/// C: namelist (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:2593)
/// Calls: cxx-internal:user_model.cc.o:__ZL9addtolistRKNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEiPiPc, cxx:_mj_hashString
#[allow(unused_variables, non_snake_case)]
pub fn namelist__babf3938c0832750(list: *const (), adr: i32, name_adr: *mut i32, names: *mut i8, map: *mut i32) -> i32 {
    todo!() // cxx-internal:user_model.cc.o:__ZL8namelistI6mjCKeyEiRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEEiPiPcS9_
}

/// C: namelist (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:2593)
/// Calls: cxx-internal:user_model.cc.o:__ZL9addtolistRKNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEiPiPc, cxx:_mj_hashString
#[allow(unused_variables, non_snake_case)]
pub fn namelist__1c43ecbf40bee2dc(list: *const (), adr: i32, name_adr: *mut i32, names: *mut i8, map: *mut i32) -> i32 {
    todo!() // cxx-internal:user_model.cc.o:__ZL8namelistI7mjCBodyEiRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEEiPiPcS9_
}

/// C: namelist (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:2593)
/// Calls: cxx-internal:user_model.cc.o:__ZL9addtolistRKNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEiPiPc, cxx:_mj_hashString
#[allow(unused_variables, non_snake_case)]
pub fn namelist__7e52e0d1fb71c279(list: *const (), adr: i32, name_adr: *mut i32, names: *mut i8, map: *mut i32) -> i32 {
    todo!() // cxx-internal:user_model.cc.o:__ZL8namelistI7mjCFlexEiRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEEiPiPcS9_
}

/// C: namelist (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:2593)
/// Calls: cxx-internal:user_model.cc.o:__ZL9addtolistRKNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEiPiPc, cxx:_mj_hashString
#[allow(unused_variables, non_snake_case)]
pub fn namelist__b0ea7a17697db2a0(list: *const (), adr: i32, name_adr: *mut i32, names: *mut i8, map: *mut i32) -> i32 {
    todo!() // cxx-internal:user_model.cc.o:__ZL8namelistI7mjCGeomEiRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEEiPiPcS9_
}

/// C: namelist (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:2593)
/// Calls: cxx-internal:user_model.cc.o:__ZL9addtolistRKNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEiPiPc, cxx:_mj_hashString
#[allow(unused_variables, non_snake_case)]
pub fn namelist__29ebb0bc9ceda696(list: *const (), adr: i32, name_adr: *mut i32, names: *mut i8, map: *mut i32) -> i32 {
    todo!() // cxx-internal:user_model.cc.o:__ZL8namelistI7mjCMeshEiRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEEiPiPcS9_
}

/// C: namelist (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:2593)
/// Calls: cxx-internal:user_model.cc.o:__ZL9addtolistRKNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEiPiPc, cxx:_mj_hashString
#[allow(unused_variables, non_snake_case)]
pub fn namelist__675d12ebcf9407a6(list: *const (), adr: i32, name_adr: *mut i32, names: *mut i8, map: *mut i32) -> i32 {
    todo!() // cxx-internal:user_model.cc.o:__ZL8namelistI7mjCPairEiRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEEiPiPcS9_
}

/// C: namelist (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:2593)
/// Calls: cxx-internal:user_model.cc.o:__ZL9addtolistRKNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEiPiPc, cxx:_mj_hashString
#[allow(unused_variables, non_snake_case)]
pub fn namelist__894e9f3f2146abb2(list: *const (), adr: i32, name_adr: *mut i32, names: *mut i8, map: *mut i32) -> i32 {
    todo!() // cxx-internal:user_model.cc.o:__ZL8namelistI7mjCSiteEiRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEEiPiPcS9_
}

/// C: namelist (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:2593)
/// Calls: cxx-internal:user_model.cc.o:__ZL9addtolistRKNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEiPiPc, cxx:_mj_hashString
#[allow(unused_variables, non_snake_case)]
pub fn namelist__43611442b022bd88(list: *const (), adr: i32, name_adr: *mut i32, names: *mut i8, map: *mut i32) -> i32 {
    todo!() // cxx-internal:user_model.cc.o:__ZL8namelistI7mjCSkinEiRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEEiPiPcS9_
}

/// C: namelist (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:2593)
/// Calls: cxx-internal:user_model.cc.o:__ZL9addtolistRKNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEiPiPc, cxx:_mj_hashString
#[allow(unused_variables, non_snake_case)]
pub fn namelist__12f161e27ecc21f5(list: *const (), adr: i32, name_adr: *mut i32, names: *mut i8, map: *mut i32) -> i32 {
    todo!() // cxx-internal:user_model.cc.o:__ZL8namelistI7mjCTextEiRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEEiPiPcS9_
}

/// C: namelist (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:2593)
/// Calls: cxx-internal:user_model.cc.o:__ZL9addtolistRKNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEiPiPc, cxx:_mj_hashString
#[allow(unused_variables, non_snake_case)]
pub fn namelist__9fdfb709bc4b183c(list: *const (), adr: i32, name_adr: *mut i32, names: *mut i8, map: *mut i32) -> i32 {
    todo!() // cxx-internal:user_model.cc.o:__ZL8namelistI8mjCJointEiRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEEiPiPcS9_
}

/// C: namelist (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:2593)
/// Calls: cxx-internal:user_model.cc.o:__ZL9addtolistRKNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEiPiPc, cxx:_mj_hashString
#[allow(unused_variables, non_snake_case)]
pub fn namelist__bbc0d7e5fd33723b(list: *const (), adr: i32, name_adr: *mut i32, names: *mut i8, map: *mut i32) -> i32 {
    todo!() // cxx-internal:user_model.cc.o:__ZL8namelistI8mjCLightEiRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEEiPiPcS9_
}

/// C: namelist (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:2593)
/// Calls: cxx-internal:user_model.cc.o:__ZL9addtolistRKNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEiPiPc, cxx:_mj_hashString
#[allow(unused_variables, non_snake_case)]
pub fn namelist__b8a67cfdc693ee35(list: *const (), adr: i32, name_adr: *mut i32, names: *mut i8, map: *mut i32) -> i32 {
    todo!() // cxx-internal:user_model.cc.o:__ZL8namelistI8mjCTupleEiRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEEiPiPcS9_
}

/// C: namelist (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:2593)
/// Calls: cxx-internal:user_model.cc.o:__ZL9addtolistRKNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEiPiPc, cxx:_mj_hashString
#[allow(unused_variables, non_snake_case)]
pub fn namelist__9528e50d8dadca19(list: *const (), adr: i32, name_adr: *mut i32, names: *mut i8, map: *mut i32) -> i32 {
    todo!() // cxx-internal:user_model.cc.o:__ZL8namelistI9mjCCameraEiRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEEiPiPcS9_
}

/// C: namelist (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:2593)
/// Calls: cxx-internal:user_model.cc.o:__ZL9addtolistRKNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEiPiPc, cxx:_mj_hashString
#[allow(unused_variables, non_snake_case)]
pub fn namelist__bed72fc292932800(list: *const (), adr: i32, name_adr: *mut i32, names: *mut i8, map: *mut i32) -> i32 {
    todo!() // cxx-internal:user_model.cc.o:__ZL8namelistI9mjCHFieldEiRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEEiPiPcS9_
}

/// C: namelist (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:2593)
/// Calls: cxx-internal:user_model.cc.o:__ZL9addtolistRKNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEiPiPc, cxx:_mj_hashString
#[allow(unused_variables, non_snake_case)]
pub fn namelist__428e173c4d82257b(list: *const (), adr: i32, name_adr: *mut i32, names: *mut i8, map: *mut i32) -> i32 {
    todo!() // cxx-internal:user_model.cc.o:__ZL8namelistI9mjCPluginEiRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEEiPiPcS9_
}

/// C: namelist (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:2593)
/// Calls: cxx-internal:user_model.cc.o:__ZL9addtolistRKNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEiPiPc, cxx:_mj_hashString
#[allow(unused_variables, non_snake_case)]
pub fn namelist__d139b7e4e650b426(list: *const (), adr: i32, name_adr: *mut i32, names: *mut i8, map: *mut i32) -> i32 {
    todo!() // cxx-internal:user_model.cc.o:__ZL8namelistI9mjCSensorEiRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEEiPiPcS9_
}

/// C: namelist (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:2593)
/// Calls: cxx-internal:user_model.cc.o:__ZL9addtolistRKNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEiPiPc, cxx:_mj_hashString
#[allow(unused_variables, non_snake_case)]
pub fn namelist__8f463aac7efafcb6(list: *const (), adr: i32, name_adr: *mut i32, names: *mut i8, map: *mut i32) -> i32 {
    todo!() // cxx-internal:user_model.cc.o:__ZL8namelistI9mjCTendonEiRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEEiPiPcS9_
}

/// C: pathlist (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:2702)
/// Calls: cxx-internal:user_model.cc.o:__ZL9addtolistRKNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEiPiPc, cxx:__ZNK10mjCTexture4FileEv
#[allow(unused_variables, non_snake_case)]
pub fn pathlist__28b1902ff54da059(list: *const (), adr: i32, path_adr: *mut i32, paths: *mut i8) -> i32 {
    todo!() // cxx-internal:user_model.cc.o:__ZL8pathlistI10mjCTextureEiRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEEiPiPc
}

/// C: pathlist (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:2702)
/// Calls: cxx-internal:user_model.cc.o:__ZL9addtolistRKNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEiPiPc, cxx:__ZNK7mjCMesh4FileEv
#[allow(unused_variables, non_snake_case)]
pub fn pathlist__2d2613562bf1a69c(list: *const (), adr: i32, path_adr: *mut i32, paths: *mut i8) -> i32 {
    todo!() // cxx-internal:user_model.cc.o:__ZL8pathlistI7mjCMeshEiRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEEiPiPc
}

/// C: pathlist (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:2702)
/// Calls: cxx-internal:user_model.cc.o:__ZL9addtolistRKNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEiPiPc, cxx:__ZNK7mjCSkin4FileEv
#[allow(unused_variables, non_snake_case)]
pub fn pathlist__2257772fd660a2df(list: *const (), adr: i32, path_adr: *mut i32, paths: *mut i8) -> i32 {
    todo!() // cxx-internal:user_model.cc.o:__ZL8pathlistI7mjCSkinEiRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEEiPiPc
}

/// C: pathlist (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:2702)
/// Calls: cxx-internal:user_model.cc.o:__ZL9addtolistRKNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEiPiPc, cxx:__ZNK9mjCHField4FileEv
#[allow(unused_variables, non_snake_case)]
pub fn pathlist__678749027656a8ac(list: *const (), adr: i32, path_adr: *mut i32, paths: *mut i8) -> i32 {
    todo!() // cxx-internal:user_model.cc.o:__ZL8pathlistI9mjCHFieldEiRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEEiPiPc
}

/// C: mjCModel::SaveState (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4131)
/// Calls: cxx:__ZN11mjCActuator3actERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE, cxx:__ZN11mjCActuator4ctrlERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE, cxx:__ZN7mjCBody4mposERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE, cxx:__ZN7mjCBody5mquatERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE, cxx:__ZN8mjCJoint4qposERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE, cxx:__ZN8mjCJoint4qvelERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_save_state(self_ptr: *mut mjCModel, state_name: *const std__string, qpos: *const f64, qvel: *const f64, act: *const f64, ctrl: *const f64, mpos: *const f64, mquat: *const f64) {
    todo!() // cxx:__ZN8mjCModel9SaveStateIdEEvRKNSt3__112basic_stringIcNS1_11char_traitsIcEENS1_9allocatorIcEEEEPKT_SC_SC_SC_SC_SC_
}

/// C: mjCModel::RestoreState (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4185)
/// Calls: cxx:__Z12mjuu_definedd, cxx:__ZN11mjCActuator3actERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE, cxx:__ZN11mjCActuator4ctrlERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE, cxx:__ZN7mjCBody4mposERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE, cxx:__ZN7mjCBody5mquatERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE, cxx:__ZN8mjCJoint4qposERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE, cxx:__ZN8mjCJoint4qvelERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_restore_state(self_ptr: *mut mjCModel, state_name: *const std__string, pos0: *const f64, mpos0: *const f64, mquat0: *const f64, qpos: *mut f64, qvel: *mut f64, act: *mut f64, ctrl: *mut f64, mpos: *mut f64, mquat: *mut f64) {
    todo!() // cxx:__ZN8mjCModel12RestoreStateIdEEvRKNSt3__112basic_stringIcNS1_11char_traitsIcEENS1_9allocatorIcEEEEPKdSB_SB_PT_SD_SD_SD_SD_SD_
}

/// C: makelistid (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4311)
#[allow(unused_variables, non_snake_case)]
pub fn makelistid__b4fd9db9fcfd9553(dest: *const (), source: *const ()) {
    todo!() // cxx-internal:user_model.cc.o:__ZL10makelistidI7mjCGeomEvRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEES8_
}

/// C: makelistid (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4311)
#[allow(unused_variables, non_snake_case)]
pub fn makelistid__f4c10b748ae4a4b6(dest: *const (), source: *const ()) {
    todo!() // cxx-internal:user_model.cc.o:__ZL10makelistidI7mjCSiteEvRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEES8_
}

/// C: makelistid (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4311)
#[allow(unused_variables, non_snake_case)]
pub fn makelistid__cfcd1aec08e4edf9(dest: *const (), source: *const ()) {
    todo!() // cxx-internal:user_model.cc.o:__ZL10makelistidI8mjCJointEvRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEES8_
}

/// C: makelistid (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4311)
#[allow(unused_variables, non_snake_case)]
pub fn makelistid__9a10e8148a67372f(dest: *const (), source: *const ()) {
    todo!() // cxx-internal:user_model.cc.o:__ZL10makelistidI8mjCLightEvRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEES8_
}

/// C: makelistid (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4311)
#[allow(unused_variables, non_snake_case)]
pub fn makelistid__fd2f31e305f076b0(dest: *const (), source: *const ()) {
    todo!() // cxx-internal:user_model.cc.o:__ZL10makelistidI9mjCCameraEvRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEES8_
}

/// C: changeframe (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4319)
/// Calls: cxx:__Z15mjuu_frameaccumPdS_PKdS1_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn changeframe(childpos: *mut f64, childquat: *mut f64, bodypos: *const f64, bodyquat: *const f64) {
    todo!() // cxx-internal:user_model.cc.o:__ZL11changeframePdS_PKdS1_
}

/// C: mjCModel::ReassignChild (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4355)
/// Calls: cxx-internal:user_model.cc.o:__ZL11changeframePdS_PKdS1_
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_reassign_child__2137ee834a90dcbf(self_ptr: *mut mjCModel, dest: *const (), list: *const (), parent: *mut mjCBody, body: *mut mjCBody) {
    todo!() // cxx:__ZN8mjCModel13ReassignChildI7mjCGeomEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEES9_P7mjCBodySB_
}

/// C: mjCModel::ReassignChild (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4355)
/// Calls: cxx-internal:user_model.cc.o:__ZL11changeframePdS_PKdS1_
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_reassign_child__9872aaa281f525f8(self_ptr: *mut mjCModel, dest: *const (), list: *const (), parent: *mut mjCBody, body: *mut mjCBody) {
    todo!() // cxx:__ZN8mjCModel13ReassignChildI7mjCSiteEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEES9_P7mjCBodySB_
}

/// C: mjCModel::ReassignChild (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4355)
/// Calls: cxx-internal:user_model.cc.o:__ZL11changeframePdS_PKdS1_
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_reassign_child__d0c617d89aafc296(self_ptr: *mut mjCModel, dest: *const (), list: *const (), parent: *mut mjCBody, body: *mut mjCBody) {
    todo!() // cxx:__ZN8mjCModel13ReassignChildI9mjCCameraEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEES9_P7mjCBodySB_
}

/// C: mjCModel::ResolveReferences (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4371)
/// Calls: cxx:__ZN11mjCActuator12CopyFromSpecEv, cxx:__ZN11mjCActuator17ResolveReferencesEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_resolve_references__81a75c4964157e88(self_ptr: *mut mjCModel, list: *const (), body: *mut mjCBody) {
    todo!() // cxx:__ZN8mjCModel17ResolveReferencesI11mjCActuatorEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEEP7mjCBody
}

/// C: mjCModel::ResolveReferences (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4371)
/// Calls: cxx:__ZN11mjCBodyPair12CopyFromSpecEv, cxx:__ZN11mjCBodyPair17ResolveReferencesEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_resolve_references__55de6bf8047de65e(self_ptr: *mut mjCModel, list: *const (), body: *mut mjCBody) {
    todo!() // cxx:__ZN8mjCModel17ResolveReferencesI11mjCBodyPairEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEEP7mjCBody
}

/// C: mjCModel::ResolveReferences (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4371)
/// Calls: cxx:__ZN11mjCEquality12CopyFromSpecEv, cxx:__ZN11mjCEquality17ResolveReferencesEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_resolve_references__a690f23ded40f049(self_ptr: *mut mjCModel, list: *const (), body: *mut mjCBody) {
    todo!() // cxx:__ZN8mjCModel17ResolveReferencesI11mjCEqualityEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEEP7mjCBody
}

/// C: mjCModel::ResolveReferences (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4371)
/// Calls: cxx:__ZN7mjCPair12CopyFromSpecEv, cxx:__ZN7mjCPair17ResolveReferencesEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_resolve_references__d39419465227951a(self_ptr: *mut mjCModel, list: *const (), body: *mut mjCBody) {
    todo!() // cxx:__ZN8mjCModel17ResolveReferencesI7mjCPairEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEEP7mjCBody
}

/// C: mjCModel::ResolveReferences (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4371)
/// Calls: cxx:__ZN7mjCSkin12CopyFromSpecEv, cxx:__ZN7mjCSkin17ResolveReferencesEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_resolve_references__92a4710cd98f7d60(self_ptr: *mut mjCModel, list: *const (), body: *mut mjCBody) {
    todo!() // cxx:__ZN8mjCModel17ResolveReferencesI7mjCSkinEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEEP7mjCBody
}

/// C: mjCModel::ResolveReferences (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4371)
/// Calls: cxx:__ZN8mjCLight12CopyFromSpecEv, cxx:__ZN8mjCLight17ResolveReferencesEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_resolve_references__0a6350620797ac03(self_ptr: *mut mjCModel, list: *const (), body: *mut mjCBody) {
    todo!() // cxx:__ZN8mjCModel17ResolveReferencesI8mjCLightEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEEP7mjCBody
}

/// C: mjCModel::ResolveReferences (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4371)
/// Calls: cxx:__ZN8mjCTuple12CopyFromSpecEv, cxx:__ZN8mjCTuple17ResolveReferencesEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_resolve_references__fd05d9c369f0f7f7(self_ptr: *mut mjCModel, list: *const (), body: *mut mjCBody) {
    todo!() // cxx:__ZN8mjCModel17ResolveReferencesI8mjCTupleEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEEP7mjCBody
}

/// C: mjCModel::ResolveReferences (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4371)
/// Calls: cxx:__ZN9mjCCamera12CopyFromSpecEv, cxx:__ZN9mjCCamera17ResolveReferencesEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_resolve_references__619deb1153029ddf(self_ptr: *mut mjCModel, list: *const (), body: *mut mjCBody) {
    todo!() // cxx:__ZN8mjCModel17ResolveReferencesI9mjCCameraEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEEP7mjCBody
}

/// C: mjCModel::ResolveReferences (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4371)
/// Calls: cxx:__ZN9mjCTendon12CopyFromSpecEv, cxx:__ZN9mjCTendon17ResolveReferencesEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_resolve_references__4df6c4366b027726(self_ptr: *mut mjCModel, list: *const (), body: *mut mjCBody) {
    todo!() // cxx:__ZN8mjCModel17ResolveReferencesI9mjCTendonEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEEP7mjCBody
}

/// C: mjCModel::ResolveReferences (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4381)
/// Calls: cxx:__ZN9mjCSensor12CopyFromSpecEv, cxx:__ZN9mjCSensor17ResolveReferencesEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_resolve_references__62ed29a0aee6153f(self_ptr: *mut mjCModel, list: *const (), body: *mut mjCBody) {
    todo!() // cxx:__ZN8mjCModel17ResolveReferencesI9mjCSensorEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEEP7mjCBody
}

/// C: comparePair (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4565)
/// Calls: cxx:__ZN7mjCPair12GetSignatureEv
#[allow(unused_variables, non_snake_case)]
pub fn compare_pair(el1: *mut mjCPair, el2: *mut mjCPair) -> i32 {
    todo!() // cxx-internal:user_model.cc.o:__ZL11comparePairP7mjCPairS0_
}

/// C: compareBodyPair (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4568)
/// Calls: cxx:__ZN11mjCBodyPair12GetSignatureEv
#[allow(unused_variables, non_snake_case)]
pub fn compare_body_pair(el1: *mut mjCBodyPair, el2: *mut mjCBodyPair) -> i32 {
    todo!() // cxx-internal:user_model.cc.o:__ZL15compareBodyPairP11mjCBodyPairS0_
}

/// C: reassignid (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4575)
#[allow(unused_variables, non_snake_case)]
pub fn reassignid__e469e58a24cc51e7(list: *const ()) {
    todo!() // cxx-internal:user_model.cc.o:__ZL10reassignidI11mjCBodyPairEvRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEE
}

/// C: reassignid (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4575)
#[allow(unused_variables, non_snake_case)]
pub fn reassignid__57aeb385a55d92ac(list: *const ()) {
    todo!() // cxx-internal:user_model.cc.o:__ZL10reassignidI7mjCPairEvRNSt3__16vectorIPT_NS1_9allocatorIS4_EEEE
}

/// C: mjCModel::ProcessList_ (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4600)
/// Calls: cxx:__ZN8mjCModel11CheckRepeatE6mjtObj, cxx:_mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_process_list__3164faadbba858fa(self_ptr: *mut mjCModel, ids: *mut mjListKeyMap, list: *const (), r#type: u32, checkrepeat: bool) {
    todo!() // cxx:__ZN8mjCModel12ProcessList_I10mjCNumericEEvRNSt3__15arrayINS2_3mapINS2_12basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEEiNS2_4lessIvEENS8_INS2_4pairIKSA_iEEEEEELm26EEERNS2_6vectorIPT_NS8_ISM_EEEE6mjtObjb
}

/// C: mjCModel::ProcessList_ (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4600)
/// Calls: cxx:__ZN8mjCModel11CheckRepeatE6mjtObj, cxx:_mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_process_list__101f490502f44ff2(self_ptr: *mut mjCModel, ids: *mut mjListKeyMap, list: *const (), r#type: u32, checkrepeat: bool) {
    todo!() // cxx:__ZN8mjCModel12ProcessList_I10mjCTextureEEvRNSt3__15arrayINS2_3mapINS2_12basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEEiNS2_4lessIvEENS8_INS2_4pairIKSA_iEEEEEELm26EEERNS2_6vectorIPT_NS8_ISM_EEEE6mjtObjb
}

/// C: mjCModel::ProcessList_ (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4600)
/// Calls: cxx:__ZN8mjCModel11CheckRepeatE6mjtObj, cxx:_mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_process_list__0d75207cecc60104(self_ptr: *mut mjCModel, ids: *mut mjListKeyMap, list: *const (), r#type: u32, checkrepeat: bool) {
    todo!() // cxx:__ZN8mjCModel12ProcessList_I11mjCActuatorEEvRNSt3__15arrayINS2_3mapINS2_12basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEEiNS2_4lessIvEENS8_INS2_4pairIKSA_iEEEEEELm26EEERNS2_6vectorIPT_NS8_ISM_EEEE6mjtObjb
}

/// C: mjCModel::ProcessList_ (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4600)
/// Calls: cxx:__ZN8mjCModel11CheckRepeatE6mjtObj, cxx:_mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_process_list__fde7d9e60d6cd5c5(self_ptr: *mut mjCModel, ids: *mut mjListKeyMap, list: *const (), r#type: u32, checkrepeat: bool) {
    todo!() // cxx:__ZN8mjCModel12ProcessList_I11mjCBodyPairEEvRNSt3__15arrayINS2_3mapINS2_12basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEEiNS2_4lessIvEENS8_INS2_4pairIKSA_iEEEEEELm26EEERNS2_6vectorIPT_NS8_ISM_EEEE6mjtObjb
}

/// C: mjCModel::ProcessList_ (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4600)
/// Calls: cxx:__ZN8mjCModel11CheckRepeatE6mjtObj, cxx:_mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_process_list__4a2f11a6eae8023c(self_ptr: *mut mjCModel, ids: *mut mjListKeyMap, list: *const (), r#type: u32, checkrepeat: bool) {
    todo!() // cxx:__ZN8mjCModel12ProcessList_I11mjCEqualityEEvRNSt3__15arrayINS2_3mapINS2_12basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEEiNS2_4lessIvEENS8_INS2_4pairIKSA_iEEEEEELm26EEERNS2_6vectorIPT_NS8_ISM_EEEE6mjtObjb
}

/// C: mjCModel::ProcessList_ (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4600)
/// Calls: cxx:__ZN8mjCModel11CheckRepeatE6mjtObj, cxx:_mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_process_list__fb14c7bb228fcf41(self_ptr: *mut mjCModel, ids: *mut mjListKeyMap, list: *const (), r#type: u32, checkrepeat: bool) {
    todo!() // cxx:__ZN8mjCModel12ProcessList_I11mjCMaterialEEvRNSt3__15arrayINS2_3mapINS2_12basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEEiNS2_4lessIvEENS8_INS2_4pairIKSA_iEEEEEELm26EEERNS2_6vectorIPT_NS8_ISM_EEEE6mjtObjb
}

/// C: mjCModel::ProcessList_ (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4600)
/// Calls: cxx:__ZN8mjCModel11CheckRepeatE6mjtObj, cxx:_mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_process_list__2846ecee1725513a(self_ptr: *mut mjCModel, ids: *mut mjListKeyMap, list: *const (), r#type: u32, checkrepeat: bool) {
    todo!() // cxx:__ZN8mjCModel12ProcessList_I6mjCKeyEEvRNSt3__15arrayINS2_3mapINS2_12basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEEiNS2_4lessIvEENS8_INS2_4pairIKSA_iEEEEEELm26EEERNS2_6vectorIPT_NS8_ISM_EEEE6mjtObjb
}

/// C: mjCModel::ProcessList_ (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4600)
/// Calls: cxx:__ZN8mjCModel11CheckRepeatE6mjtObj, cxx:_mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_process_list__1fd239fbca036e1b(self_ptr: *mut mjCModel, ids: *mut mjListKeyMap, list: *const (), r#type: u32, checkrepeat: bool) {
    todo!() // cxx:__ZN8mjCModel12ProcessList_I7mjCBaseEEvRNSt3__15arrayINS2_3mapINS2_12basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEEiNS2_4lessIvEENS8_INS2_4pairIKSA_iEEEEEELm26EEERNS2_6vectorIPT_NS8_ISM_EEEE6mjtObjb
}

/// C: mjCModel::ProcessList_ (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4600)
/// Calls: cxx:__ZN8mjCModel11CheckRepeatE6mjtObj, cxx:_mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_process_list__f80fc75329bd2b21(self_ptr: *mut mjCModel, ids: *mut mjListKeyMap, list: *const (), r#type: u32, checkrepeat: bool) {
    todo!() // cxx:__ZN8mjCModel12ProcessList_I7mjCBodyEEvRNSt3__15arrayINS2_3mapINS2_12basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEEiNS2_4lessIvEENS8_INS2_4pairIKSA_iEEEEEELm26EEERNS2_6vectorIPT_NS8_ISM_EEEE6mjtObjb
}

/// C: mjCModel::ProcessList_ (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4600)
/// Calls: cxx:__ZN8mjCModel11CheckRepeatE6mjtObj, cxx:_mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_process_list__b5b0d7504cbe7c08(self_ptr: *mut mjCModel, ids: *mut mjListKeyMap, list: *const (), r#type: u32, checkrepeat: bool) {
    todo!() // cxx:__ZN8mjCModel12ProcessList_I7mjCFlexEEvRNSt3__15arrayINS2_3mapINS2_12basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEEiNS2_4lessIvEENS8_INS2_4pairIKSA_iEEEEEELm26EEERNS2_6vectorIPT_NS8_ISM_EEEE6mjtObjb
}

/// C: mjCModel::ProcessList_ (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4600)
/// Calls: cxx:__ZN8mjCModel11CheckRepeatE6mjtObj, cxx:_mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_process_list__e26d89d3a622286d(self_ptr: *mut mjCModel, ids: *mut mjListKeyMap, list: *const (), r#type: u32, checkrepeat: bool) {
    todo!() // cxx:__ZN8mjCModel12ProcessList_I7mjCMeshEEvRNSt3__15arrayINS2_3mapINS2_12basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEEiNS2_4lessIvEENS8_INS2_4pairIKSA_iEEEEEELm26EEERNS2_6vectorIPT_NS8_ISM_EEEE6mjtObjb
}

/// C: mjCModel::ProcessList_ (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4600)
/// Calls: cxx:__ZN8mjCModel11CheckRepeatE6mjtObj, cxx:_mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_process_list__0785554f26e650c7(self_ptr: *mut mjCModel, ids: *mut mjListKeyMap, list: *const (), r#type: u32, checkrepeat: bool) {
    todo!() // cxx:__ZN8mjCModel12ProcessList_I7mjCPairEEvRNSt3__15arrayINS2_3mapINS2_12basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEEiNS2_4lessIvEENS8_INS2_4pairIKSA_iEEEEEELm26EEERNS2_6vectorIPT_NS8_ISM_EEEE6mjtObjb
}

/// C: mjCModel::ProcessList_ (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4600)
/// Calls: cxx:__ZN8mjCModel11CheckRepeatE6mjtObj, cxx:_mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_process_list__a6bb17815696cdf3(self_ptr: *mut mjCModel, ids: *mut mjListKeyMap, list: *const (), r#type: u32, checkrepeat: bool) {
    todo!() // cxx:__ZN8mjCModel12ProcessList_I7mjCSkinEEvRNSt3__15arrayINS2_3mapINS2_12basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEEiNS2_4lessIvEENS8_INS2_4pairIKSA_iEEEEEELm26EEERNS2_6vectorIPT_NS8_ISM_EEEE6mjtObjb
}

/// C: mjCModel::ProcessList_ (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4600)
/// Calls: cxx:__ZN8mjCModel11CheckRepeatE6mjtObj, cxx:_mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_process_list__d91f0bee0f14e198(self_ptr: *mut mjCModel, ids: *mut mjListKeyMap, list: *const (), r#type: u32, checkrepeat: bool) {
    todo!() // cxx:__ZN8mjCModel12ProcessList_I7mjCTextEEvRNSt3__15arrayINS2_3mapINS2_12basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEEiNS2_4lessIvEENS8_INS2_4pairIKSA_iEEEEEELm26EEERNS2_6vectorIPT_NS8_ISM_EEEE6mjtObjb
}

/// C: mjCModel::ProcessList_ (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4600)
/// Calls: cxx:__ZN8mjCModel11CheckRepeatE6mjtObj, cxx:_mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_process_list__c8f18809bff4e850(self_ptr: *mut mjCModel, ids: *mut mjListKeyMap, list: *const (), r#type: u32, checkrepeat: bool) {
    todo!() // cxx:__ZN8mjCModel12ProcessList_I8mjCFrameEEvRNSt3__15arrayINS2_3mapINS2_12basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEEiNS2_4lessIvEENS8_INS2_4pairIKSA_iEEEEEELm26EEERNS2_6vectorIPT_NS8_ISM_EEEE6mjtObjb
}

/// C: mjCModel::ProcessList_ (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4600)
/// Calls: cxx:__ZN8mjCModel11CheckRepeatE6mjtObj, cxx:_mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_process_list__c669eef386b67e47(self_ptr: *mut mjCModel, ids: *mut mjListKeyMap, list: *const (), r#type: u32, checkrepeat: bool) {
    todo!() // cxx:__ZN8mjCModel12ProcessList_I8mjCTupleEEvRNSt3__15arrayINS2_3mapINS2_12basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEEiNS2_4lessIvEENS8_INS2_4pairIKSA_iEEEEEELm26EEERNS2_6vectorIPT_NS8_ISM_EEEE6mjtObjb
}

/// C: mjCModel::ProcessList_ (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4600)
/// Calls: cxx:__ZN8mjCModel11CheckRepeatE6mjtObj, cxx:_mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_process_list__13a58b57a7061bf7(self_ptr: *mut mjCModel, ids: *mut mjListKeyMap, list: *const (), r#type: u32, checkrepeat: bool) {
    todo!() // cxx:__ZN8mjCModel12ProcessList_I9mjCHFieldEEvRNSt3__15arrayINS2_3mapINS2_12basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEEiNS2_4lessIvEENS8_INS2_4pairIKSA_iEEEEEELm26EEERNS2_6vectorIPT_NS8_ISM_EEEE6mjtObjb
}

/// C: mjCModel::ProcessList_ (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4600)
/// Calls: cxx:__ZN8mjCModel11CheckRepeatE6mjtObj, cxx:_mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_process_list__9ab75ca0457c59aa(self_ptr: *mut mjCModel, ids: *mut mjListKeyMap, list: *const (), r#type: u32, checkrepeat: bool) {
    todo!() // cxx:__ZN8mjCModel12ProcessList_I9mjCPluginEEvRNSt3__15arrayINS2_3mapINS2_12basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEEiNS2_4lessIvEENS8_INS2_4pairIKSA_iEEEEEELm26EEERNS2_6vectorIPT_NS8_ISM_EEEE6mjtObjb
}

/// C: mjCModel::ProcessList_ (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4600)
/// Calls: cxx:__ZN8mjCModel11CheckRepeatE6mjtObj, cxx:_mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_process_list__4f93cc0967735918(self_ptr: *mut mjCModel, ids: *mut mjListKeyMap, list: *const (), r#type: u32, checkrepeat: bool) {
    todo!() // cxx:__ZN8mjCModel12ProcessList_I9mjCSensorEEvRNSt3__15arrayINS2_3mapINS2_12basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEEiNS2_4lessIvEENS8_INS2_4pairIKSA_iEEEEEELm26EEERNS2_6vectorIPT_NS8_ISM_EEEE6mjtObjb
}

/// C: mjCModel::ProcessList_ (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4600)
/// Calls: cxx:__ZN8mjCModel11CheckRepeatE6mjtObj, cxx:_mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_process_list__0b93173cf570729b(self_ptr: *mut mjCModel, ids: *mut mjListKeyMap, list: *const (), r#type: u32, checkrepeat: bool) {
    todo!() // cxx:__ZN8mjCModel12ProcessList_I9mjCTendonEEvRNSt3__15arrayINS2_3mapINS2_12basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEEiNS2_4lessIvEENS8_INS2_4pairIKSA_iEEEEEELm26EEERNS2_6vectorIPT_NS8_ISM_EEEE6mjtObjb
}

/// C: compilerLogHandler (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4665)
/// Calls: cxx-internal:user_model.cc.o:__ZN6mujoco4utilL10strcpy_arrILm500EEEPcRAT__cPKc
#[allow(unused_variables, non_snake_case)]
pub fn compiler_log_handler(msg: *const mjLogMessage) {
    todo!() // cxx-internal:user_model.cc.o:__ZL18compilerLogHandlerPK13mjLogMessage_
}

/// C: CompileMesh (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4770)
/// Calls: cxx:__ZN7mjCMesh7CompileEPK6mjVFS_, cxx:__mjPRIVATE_setTlsLogHandler
#[allow(unused_variables, non_snake_case)]
pub fn compile_mesh(mesh: *mut mjCMesh, vfs: *const mjVFS, exception: *mut std__exception_ptr, exception_mutex: *mut std__mutex, warningtext: *mut std__string) {
    todo!() // cxx-internal:user_model.cc.o:__ZL11CompileMeshP7mjCMeshPK6mjVFS_RSt13exception_ptrRNSt3__15mutexEPNS6_12basic_stringIcNS6_11char_traitsIcEENS6_9allocatorIcEEEE
}

/// C: CompileTexture (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:4790)
/// Calls: cxx:__ZN10mjCTexture7CompileEPK6mjVFS_, cxx:__mjPRIVATE_setTlsLogHandler
#[allow(unused_variables, non_snake_case)]
pub fn compile_texture(texture: *mut mjCTexture, vfs: *const mjVFS, exception: *mut std__exception_ptr, exception_mutex: *mut std__mutex, warningtext: *mut std__string) {
    todo!() // cxx-internal:user_model.cc.o:__ZL14CompileTextureP10mjCTexturePK6mjVFS_RSt13exception_ptrRNSt3__15mutexEPNS6_12basic_stringIcNS6_11char_traitsIcEENS6_9allocatorIcEEEE
}

/// C: PrintIndent (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.cc:5457)
#[allow(unused_variables, non_snake_case)]
pub fn print_indent(ss: *mut std__stringstream, depth: i32) {
    todo!() // cxx-internal:user_model.cc.o:__ZL11PrintIndentRNSt3__118basic_stringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEEEi
}

/// C: mjCModel::CopyFromSpec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:191)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_from_spec(self_ptr: *mut mjCModel) {
    todo!() // cxx:__ZN8mjCModel12CopyFromSpecEv
}

/// C: mjCModel::PointToLocal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:192)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_point_to_local(self_ptr: *mut mjCModel) {
    todo!() // cxx:__ZN8mjCModel12PointToLocalEv
}

/// C: mjCModel::Compile (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:203)
/// Calls: cxx:__ZN8mjCModel10TryCompileERP8mjModel_RP7mjData_PK6mjVFS_, cxx:__ZN8mjCModel12CopyFromSpecEv, cxx:__ZN8mjCModel20ClearCompileWarningsEv, cxx:__ZN8mjCModel5ClearEv, cxx:__mjPRIVATE_setTlsLogHandler, cxx:_mj_deleteData, cxx:_mj_deleteModel, cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_compile(self_ptr: *mut mjCModel, vfs: *const mjVFS, m: *mut *mut mjModel) -> *mut mjModel {
    todo!() // cxx:__ZN8mjCModel7CompileEPK6mjVFS_PP8mjModel_
}

/// C: mjCModel::CopyBack (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:204)
/// Calls: cxx:__ZN7mjCMesh10GetQuatPtrEv, cxx:__ZN7mjCMesh9GetPosPtrEv, cxx:__ZN9mjCHField12get_userdataEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_back(self_ptr: *mut mjCModel, arg0: *const mjModel) -> bool {
    todo!() // cxx:__ZN8mjCModel8CopyBackEPK8mjModel_
}

/// C: mjCModel::FuseStatic (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:205)
/// Calls: cxx-internal:user_model.cc.o:__ZL11changeframePdS_PKdS1_, cxx:__Z15mjuu_rotVecQuatPdPKdS1_, cxx:__ZN7mjCBody10ComputeBVHEv, cxx:__ZN7mjCBody17AccumulateInertiaEPK8mjsBody_PS0_, cxx:__ZN8mjCModel11FuseReindexEP7mjCBody, cxx:__ZNK26mjCBoundingVolumeHierarchy4NbvhEv, cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_fuse_static(self_ptr: *mut mjCModel) {
    todo!() // cxx:__ZN8mjCModel10FuseStaticEv
}

/// C: mjCModel::FuseReindex (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:206)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_fuse_reindex(self_ptr: *mut mjCModel, body: *mut mjCBody) {
    todo!() // cxx:__ZN8mjCModel11FuseReindexEP7mjCBody
}

/// C: mjCModel::AddFlex (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:209)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_flex(self_ptr: *mut mjCModel) -> *mut mjCFlex {
    todo!() // cxx:__ZN8mjCModel7AddFlexEv
}

/// C: mjCModel::AddMesh (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:210)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_mesh(self_ptr: *mut mjCModel, def: *mut mjCDef) -> *mut mjCMesh {
    todo!() // cxx:__ZN8mjCModel7AddMeshEP6mjCDef
}

/// C: mjCModel::AddSkin (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:211)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_skin(self_ptr: *mut mjCModel) -> *mut mjCSkin {
    todo!() // cxx:__ZN8mjCModel7AddSkinEv
}

/// C: mjCModel::AddHField (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:212)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_h_field(self_ptr: *mut mjCModel) -> *mut mjCHField {
    todo!() // cxx:__ZN8mjCModel9AddHFieldEv
}

/// C: mjCModel::AddTexture (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:213)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_texture(self_ptr: *mut mjCModel) -> *mut mjCTexture {
    todo!() // cxx:__ZN8mjCModel10AddTextureEv
}

/// C: mjCModel::AddMaterial (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:214)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_material(self_ptr: *mut mjCModel, def: *mut mjCDef) -> *mut mjCMaterial {
    todo!() // cxx:__ZN8mjCModel11AddMaterialEP6mjCDef
}

/// C: mjCModel::AddPair (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:215)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_pair(self_ptr: *mut mjCModel, def: *mut mjCDef) -> *mut mjCPair {
    todo!() // cxx:__ZN8mjCModel7AddPairEP6mjCDef
}

/// C: mjCModel::AddExclude (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:216)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_exclude(self_ptr: *mut mjCModel) -> *mut mjCBodyPair {
    todo!() // cxx:__ZN8mjCModel10AddExcludeEv
}

/// C: mjCModel::AddEquality (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:217)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_equality(self_ptr: *mut mjCModel, def: *mut mjCDef) -> *mut mjCEquality {
    todo!() // cxx:__ZN8mjCModel11AddEqualityEP6mjCDef
}

/// C: mjCModel::AddTendon (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:218)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_tendon(self_ptr: *mut mjCModel, def: *mut mjCDef) -> *mut mjCTendon {
    todo!() // cxx:__ZN8mjCModel9AddTendonEP6mjCDef
}

/// C: mjCModel::AddActuator (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:219)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_actuator(self_ptr: *mut mjCModel, def: *mut mjCDef) -> *mut mjCActuator {
    todo!() // cxx:__ZN8mjCModel11AddActuatorEP6mjCDef
}

/// C: mjCModel::AddSensor (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:220)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_sensor(self_ptr: *mut mjCModel) -> *mut mjCSensor {
    todo!() // cxx:__ZN8mjCModel9AddSensorEv
}

/// C: mjCModel::AddNumeric (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:221)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_numeric(self_ptr: *mut mjCModel) -> *mut mjCNumeric {
    todo!() // cxx:__ZN8mjCModel10AddNumericEv
}

/// C: mjCModel::AddText (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:222)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_text(self_ptr: *mut mjCModel) -> *mut mjCText {
    todo!() // cxx:__ZN8mjCModel7AddTextEv
}

/// C: mjCModel::AddTuple (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:223)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_tuple(self_ptr: *mut mjCModel) -> *mut mjCTuple {
    todo!() // cxx:__ZN8mjCModel8AddTupleEv
}

/// C: mjCModel::AddKey (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:224)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_key(self_ptr: *mut mjCModel) -> *mut mjCKey {
    todo!() // cxx:__ZN8mjCModel6AddKeyEv
}

/// C: mjCModel::AddPlugin (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:225)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_plugin(self_ptr: *mut mjCModel) -> *mut mjCPlugin {
    todo!() // cxx:__ZN8mjCModel9AddPluginEv
}

/// C: mjCModel::AppendSpec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:228)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_append_spec(self_ptr: *mut mjCModel, spec: *mut mjSpec, compiler: *const mjsCompiler) {
    todo!() // cxx:__ZN8mjCModel10AppendSpecEP7mjSpec_PK12mjsCompiler_
}

/// C: mjCModel::NumObjects (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:244)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_num_objects(self_ptr: *mut mjCModel, r#type: u32) -> i32 {
    todo!() // cxx:__ZN8mjCModel10NumObjectsE6mjtObj
}

/// C: mjCModel::GetObject (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:245)
/// Calls: cxx:__ZN8mjCModel10NumObjectsE6mjtObj
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_object(self_ptr: *mut mjCModel, r#type: u32, id: i32) -> *mut mjCBase {
    todo!() // cxx:__ZN8mjCModel9GetObjectE6mjtObji
}

/// C: mjCModel::NextObject (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:246)
/// Calls: cxx:__ZNK7mjCBody9NextChildEPK11mjsElement_6mjtObjb
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_next_object(self_ptr: *mut mjCModel, object: *const mjsElement, r#type: u32) -> *mut mjsElement {
    todo!() // cxx:__ZNK8mjCModel10NextObjectEPK11mjsElement_6mjtObj
}

/// C: mjCModel::IsCompiled (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:249)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_is_compiled(self_ptr: *mut mjCModel) -> bool {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).compiled }
}

/// C: mjCModel::GetError (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:250)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_error(self_ptr: *mut mjCModel) -> *const mjCError {
    todo!() // cxx:__ZNK8mjCModel8GetErrorEv
}

/// C: mjCModel::SetError (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:251)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_set_error(self_ptr: *mut mjCModel, error: *const mjCError) {
    todo!() // cxx:__ZN8mjCModel8SetErrorERK8mjCError
}

/// C: mjCModel::AddWarning (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:252)
/// Calls: cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_warning(self_ptr: *mut mjCModel, msg: std__string, obj: *const mjCBase) {
    todo!() // cxx:__ZN8mjCModel10AddWarningENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEPK7mjCBase
}

/// C: mjCModel::AddGroupedWarning (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:254)
/// Calls: cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_grouped_warning(self_ptr: *mut mjCModel, subject: *const std__string, body: *const std__string) {
    todo!() // cxx:__ZN8mjCModel17AddGroupedWarningERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEES8_
}

/// C: mjCModel::GetWarnings (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:256)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_warnings(self_ptr: *mut mjCModel) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).warnings_ }
}

/// C: mjCModel::ClearWarnings (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:260)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_clear_warnings(self_ptr: *mut mjCModel) {
    todo!() // cxx:__ZN8mjCModel13ClearWarningsEv
}

/// C: mjCModel::ClearCompileWarnings (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:264)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_clear_compile_warnings(self_ptr: *mut mjCModel) {
    todo!() // cxx:__ZN8mjCModel20ClearCompileWarningsEv
}

/// C: mjCModel::SetAttachWarningBoundary (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:267)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_set_attach_warning_boundary(self_ptr: *mut mjCModel) {
    todo!() // cxx:__ZN8mjCModel24SetAttachWarningBoundaryEv
}

/// C: mjCModel::GetWorld (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:271)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_world(self_ptr: *mut mjCModel) -> *mut mjCBody {
    todo!() // cxx:__ZN8mjCModel8GetWorldEv
}

/// C: mjCModel::FindTree (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:275)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_find_tree(self_ptr: *mut mjCModel, body: *mut mjCBody, r#type: u32, name: std__string) -> *mut mjCBase {
    todo!() // cxx:__ZN8mjCModel8FindTreeEP7mjCBody6mjtObjNSt3__112basic_stringIcNS3_11char_traitsIcEENS3_9allocatorIcEEEE
}

/// C: mjCModel::FindSpec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:276)
/// Calls: cxx:_mjs_getString
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_find_spec__3f8b9ec518b32916(self_ptr: *mut mjCModel, name: std__string) -> *mut mjSpec {
    todo!() // cxx:__ZNK8mjCModel8FindSpecENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE
}

/// C: mjCModel::FindSpec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:277)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_find_spec__7456467199e33b98(self_ptr: *mut mjCModel, compiler_: *const mjsCompiler) -> *mut mjSpec {
    todo!() // cxx:__ZNK8mjCModel8FindSpecEPK12mjsCompiler_
}

/// C: mjCModel::ActivatePlugin (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:278)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_activate_plugin(self_ptr: *mut mjCModel, plugin: *const mjpPlugin, slot: i32) {
    todo!() // cxx:__ZN8mjCModel14ActivatePluginEPK10mjpPlugin_i
}

/// C: mjCModel::get_meshdir (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:285)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_meshdir(self_ptr: *mut mjCModel) -> std__string {
    todo!() // cxx:__ZNK8mjCModel11get_meshdirEv
}

/// C: mjCModel::get_texturedir (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:286)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_texturedir(self_ptr: *mut mjCModel) -> std__string {
    todo!() // cxx:__ZNK8mjCModel14get_texturedirEv
}

/// C: mjCModel::Default (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:288)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_default(self_ptr: *mut mjCModel) -> *mut mjCDef {
    todo!() // cxx:__ZNK8mjCModel7DefaultEv
}

/// C: mjCModel::NumDefaults (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:289)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_num_defaults(self_ptr: *mut mjCModel) -> i32 {
    todo!() // cxx:__ZNK8mjCModel11NumDefaultsEv
}

/// C: mjCModel::ActivePlugins (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:291)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_active_plugins(self_ptr: *mut mjCModel) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).active_plugins_ }
}

/// C: mjCModel::Flexes (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:295)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_flexes(self_ptr: *mut mjCModel) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).flexes_ }
}

/// C: mjCModel::Meshes (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:296)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_meshes(self_ptr: *mut mjCModel) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).meshes_ }
}

/// C: mjCModel::Skins (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:297)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_skins(self_ptr: *mut mjCModel) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).skins_ }
}

/// C: mjCModel::HFields (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:298)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_h_fields(self_ptr: *mut mjCModel) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).hfields_ }
}

/// C: mjCModel::Textures (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:299)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_textures(self_ptr: *mut mjCModel) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).textures_ }
}

/// C: mjCModel::Materials (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:300)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_materials(self_ptr: *mut mjCModel) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).materials_ }
}

/// C: mjCModel::Pairs (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:301)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_pairs(self_ptr: *mut mjCModel) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).pairs_ }
}

/// C: mjCModel::Excludes (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:302)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_excludes(self_ptr: *mut mjCModel) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).excludes_ }
}

/// C: mjCModel::Equalities (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:303)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_equalities(self_ptr: *mut mjCModel) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).equalities_ }
}

/// C: mjCModel::Tendons (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:304)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_tendons(self_ptr: *mut mjCModel) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).tendons_ }
}

/// C: mjCModel::Actuators (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:305)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_actuators(self_ptr: *mut mjCModel) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).actuators_ }
}

/// C: mjCModel::Sensors (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:306)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_sensors(self_ptr: *mut mjCModel) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).sensors_ }
}

/// C: mjCModel::Numerics (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:307)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_numerics(self_ptr: *mut mjCModel) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).numerics_ }
}

/// C: mjCModel::Texts (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:308)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_texts(self_ptr: *mut mjCModel) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).texts_ }
}

/// C: mjCModel::Tuples (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:309)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_tuples(self_ptr: *mut mjCModel) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).tuples_ }
}

/// C: mjCModel::Keys (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:310)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_keys(self_ptr: *mut mjCModel) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).keys_ }
}

/// C: mjCModel::Plugins (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:311)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_plugins(self_ptr: *mut mjCModel) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).plugins_ }
}

/// C: mjCModel::Bodies (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:312)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_bodies(self_ptr: *mut mjCModel) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).bodies_ }
}

/// C: mjCModel::Geoms (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:313)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_geoms(self_ptr: *mut mjCModel) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).geoms_ }
}

/// C: mjCModel::ResolvePlugin (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:316)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_resolve_plugin(self_ptr: *mut mjCModel, obj: *mut mjCBase, plugin_name: *const std__string, plugin_instance_name: *const std__string, plugin_instance: *mut *mut mjCPlugin) {
    todo!() // cxx:__ZN8mjCModel13ResolvePluginEP7mjCBaseRKNSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEESA_PP9mjCPlugin
}

/// C: mjCModel::Clear (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:321)
/// Calls: cxx:__ZN8mjCModel20ClearCompileWarningsEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_clear(self_ptr: *mut mjCModel) {
    todo!() // cxx:__ZN8mjCModel5ClearEv
}

/// C: mjCModel::MakeData (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:338)
/// Calls: cxx:_mj_initPlugin, cxx:_mj_makeRawData, cxx:_mj_resetData
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_make_data(self_ptr: *mut mjCModel, m: *const mjModel, dest: *mut *mut mjData) {
    todo!() // cxx:__ZN8mjCModel8MakeDataEPK8mjModel_PP7mjData_
}

/// C: mjCModel::StoreKeyframes (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:341)
/// Calls: cxx:__ZN8mjCModel10AddWarningENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEPK7mjCBase, cxx:__ZN8mjCModel14SaveDofOffsetsEb, cxx:__ZN8mjCModel16ComputeReferenceEv, cxx:__ZN8mjCModel9SaveStateIdEEvRKNSt3__112basic_stringIcNS1_11char_traitsIcEENS1_9allocatorIcEEEEPKT_SC_SC_SC_SC_SC_
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_store_keyframes(self_ptr: *mut mjCModel, dest: *mut mjCModel) {
    todo!() // cxx:__ZN8mjCModel14StoreKeyframesEPS_
}

/// C: mjCModel::SetDeepCopy (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:347)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_set_deep_copy(self_ptr: *mut mjCModel, deepcopy: bool) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).deepcopy_ = deepcopy; }
}

/// C: mjCModel::SetAttached (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:350)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_set_attached(self_ptr: *mut mjCModel, deepcopy: bool) {
    todo!() // cxx:__ZN8mjCModel11SetAttachedEb
}

/// C: mjCModel::IsAttached (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:353)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_is_attached(self_ptr: *mut mjCModel) -> bool {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).attached_ }
}

/// C: mjCModel::CheckRepeat (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:356)
/// Calls: cxx:_mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_check_repeat(self_ptr: *mut mjCModel, r#type: u32) {
    todo!() // cxx:__ZN8mjCModel11CheckRepeatE6mjtObj
}

/// C: mjCModel::AddRef (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:359)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_add_ref(self_ptr: *mut mjCModel) {
    todo!() // cxx:__ZN8mjCModel6AddRefEv
}

/// C: mjCModel::GetRef (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:360)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_get_ref(self_ptr: *mut mjCModel) -> i32 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).refcount }
}

/// C: mjCModel::Release (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:361)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_release(self_ptr: *mut mjCModel) {
    todo!() // cxx:__ZN8mjCModel7ReleaseEv
}

/// C: mjCModel::MakeTreeLists (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:377)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_make_tree_lists(self_ptr: *mut mjCModel, body: *mut mjCBody) {
    todo!() // cxx:__ZN8mjCModel13MakeTreeListsEP7mjCBody
}

/// C: mjCModel::TryCompile (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:380)
/// Calls: cxx:__Z12mjuu_definedd, cxx:__ZN10mjCNumeric7CompileEv, cxx:__ZN10mjCTexture12CopyFromSpecEv, cxx:__ZN11mjCActuator7CompileEv, cxx:__ZN11mjCBodyPair7CompileEv, cxx:__ZN11mjCEquality7CompileEv, cxx:__ZN11mjCMaterial7CompileEv, cxx:__ZN6mjCDef7CompileEPK8mjCModel, cxx:__ZN6mjCKey7CompileEPK8mjModel_, cxx:__ZN7mjCBody7CompileEv, cxx:__ZN7mjCFlex7CompileEPK6mjVFS_, cxx:__ZN7mjCMesh11SetNeedHullEb, cxx:__ZN7mjCMesh12CopyFromSpecEv, cxx:__ZN7mjCPair7CompileEv, cxx:__ZN7mjCSkin12CopyFromSpecEv, cxx:__ZN7mjCSkin7CompileEPK6mjVFS_, cxx:__ZN7mjCText7CompileEv, cxx:__ZN8mjCLight7CompileEv, cxx:__ZN8mjCModel10AddWarningENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEPK7mjCBase, cxx:__ZN8mjCModel10CountNJmomEPK8mjModel_, cxx:__ZN8mjCModel10FuseStaticEv, cxx:__ZN8mjCModel11CopyObjectsEP8mjModel_, cxx:__ZN8mjCModel11CopyPluginsEP8mjModel_, cxx:__ZN8mjCModel11IndexAssetsEb, cxx:__ZN8mjCModel11LengthRangeEP8mjModel_P7mjData_, cxx:__ZN8mjCModel12ProcessListsEb, cxx:__ZN8mjCModel14FinalizeSimpleEP8mjModel_, cxx:__ZN8mjCModel14SaveDofOffsetsEb, cxx:__ZN8mjCModel15CheckEmptyNamesEv, cxx:__ZN8mjCModel16AutoSpringDamperEP8mjModel_, cxx:__ZN8mjCModel16ResolveKeyframesEPK8mjModel_, cxx:__ZN8mjCModel18ComputeSparseSizesEv, cxx:__ZN8mjCModel18ExpandAllKeyframesEv, cxx:__ZN8mjCModel20CheckBodyMassInertiaEP7mjCBody, cxx:__ZN8mjCModel20ClearCompileWarningsEv, cxx:__ZN8mjCModel24CompileMeshesAndTexturesEPK6mjVFS_, cxx:__ZN8mjCModel6AddKeyEv, cxx:__ZN8mjCModel8CopyTreeEP8mjModel_, cxx:__ZN8mjCModel8SetNuserEv, cxx:__ZN8mjCModel8SetSizesEv, cxx:__ZN8mjCModel9CopyNamesEP8mjModel_, cxx:__ZN8mjCModel9CopyPathsEP8mjModel_, cxx:__ZN8mjCModel9SignatureEv, cxx:__ZN8mjCTuple7CompileEv, cxx:__ZN9mjCCamera7CompileEv, cxx:__ZN9mjCHField12CopyFromSpecEv, cxx:__ZN9mjCHField7CompileEPK6mjVFS_, cxx:__ZN9mjCPlugin7CompileEv, cxx:__ZN9mjCSensor7CompileEv, cxx:__ZN9mjCTendon7CompileEv, cxx:_mj_deleteData, cxx:_mj_makeBSparse, cxx:_mj_makeData, cxx:_mj_makeDofDofMaps, cxx:_mj_makeDofDofSparse, cxx:_mj_makeModel, cxx:_mj_makeRawData, cxx:_mj_normalizeQuat, cxx:_mj_resetData, cxx:_mj_setConst, cxx:_mj_setTotalmass, cxx:_mj_step, cxx:_mj_validateReferences
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_try_compile(self_ptr: *mut mjCModel, m: *mut *mut mjModel, d: *mut *mut mjData, vfs: *const mjVFS) {
    todo!() // cxx:__ZN8mjCModel10TryCompileERP8mjModel_RP7mjData_PK6mjVFS_
}

/// C: mjCModel::CompileMeshesAndTextures (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:381)
/// Calls: cxx-internal:user_model.cc.o:__ZL11CompileMeshP7mjCMeshPK6mjVFS_RSt13exception_ptrRNSt3__15mutexEPNS6_12basic_stringIcNS6_11char_traitsIcEENS6_9allocatorIcEEEE, cxx-internal:user_model.cc.o:__ZL14CompileTextureP10mjCTexturePK6mjVFS_RSt13exception_ptrRNSt3__15mutexEPNS6_12basic_stringIcNS6_11char_traitsIcEENS6_9allocatorIcEEEE, cxx-internal:user_model.cc.o:__ZN12_GLOBAL__N_118NumCompilerThreadsEi, cxx-internal:user_model.cc.o:__ZN6mujoco4utilL10strcpy_arrILm500EEEPcRAT__cPKc, cxx:__ZN6mujoco4user10ThreadPool8ScheduleENSt3__18functionIFvvEEE, cxx:__ZN6mujoco4user10ThreadPool9WaitCountEi
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_compile_meshes_and_textures(self_ptr: *mut mjCModel, vfs: *const mjVFS) {
    todo!() // cxx:__ZN8mjCModel24CompileMeshesAndTexturesEPK6mjVFS_
}

/// C: mjCModel::SetNuser (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:383)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_set_nuser(self_ptr: *mut mjCModel) {
    todo!() // cxx:__ZN8mjCModel8SetNuserEv
}

/// C: mjCModel::IndexAssets (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:384)
/// Calls: cxx:__ZN7mjCMesh12SetNotVisualEv, cxx:__ZNK7mjCGeom12get_materialEv, cxx:__ZNK7mjCGeom12get_meshnameEv, cxx:__ZNK7mjCGeom14get_hfieldnameEv, cxx:__ZNK7mjCGeom8IsVisualEv, cxx:__ZNK7mjCMesh8IsVisualEv, cxx:__ZNK7mjCSite12get_materialEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_index_assets(self_ptr: *mut mjCModel, discard: bool) {
    todo!() // cxx:__ZN8mjCModel11IndexAssetsEb
}

/// C: mjCModel::CheckEmptyNames (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:385)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_check_empty_names(self_ptr: *mut mjCModel) {
    todo!() // cxx:__ZN8mjCModel15CheckEmptyNamesEv
}

/// C: mjCModel::SetSizes (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:386)
/// Calls: cxx:__ZN7mjCMesh4treeEv, cxx:__ZN7mjCMesh6octreeEv, cxx:__ZNK26mjCBoundingVolumeHierarchy4NbvhEv, cxx:__ZNK7mjCBody9GetParentEv, cxx:__ZNK7mjCFlex11HasTexcoordEv, cxx:__ZNK7mjCFlex12get_texcoordEv, cxx:__ZNK7mjCMesh11HasTexcoordEv, cxx:__ZNK7mjCMesh11npolygonmapEv, cxx:__ZNK7mjCMesh12npolygonvertEv, cxx:__ZNK7mjCMesh5nfaceEv, cxx:__ZNK7mjCMesh5nvertEv, cxx:__ZNK7mjCMesh7nnormalEv, cxx:__ZNK7mjCMesh7szgraphEv, cxx:__ZNK7mjCMesh8npolygonEv, cxx:__ZNK7mjCMesh9ntexcoordEv, cxx:__ZNK7mjCSkin10get_vertidEv, cxx:__ZNK7mjCSkin12get_texcoordEv, cxx:__ZNK7mjCSkin8get_faceEv, cxx:__ZNK7mjCSkin8get_vertEv, cxx:__ZNK8mjCJoint9GetParentEv, cxx:__ZNK9mjCOctree8NumNodesEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_set_sizes(self_ptr: *mut mjCModel) {
    todo!() // cxx:__ZN8mjCModel8SetSizesEv
}

/// C: mjCModel::ComputeSparseSizes (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:387)
/// Calls: cxx-internal:user_model.cc.o:__ZN12_GLOBAL__N_110IsNullPoseIdEEbPKT_S3_, cxx-internal:user_model.cc.o:__ZN12_GLOBAL__N_117GetBodyIdFromWrapEPK7mjCWrap, cxx:__ZNK7mjCBody6BodiesEv, cxx:__ZNK7mjCBody9GetParentEv, cxx:__ZNK7mjCWrap4TypeEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_compute_sparse_sizes(self_ptr: *mut mjCModel) {
    todo!() // cxx:__ZN8mjCModel18ComputeSparseSizesEv
}

/// C: mjCModel::AutoSpringDamper (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:388)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_auto_spring_damper(self_ptr: *mut mjCModel, arg0: *mut mjModel) {
    todo!() // cxx:__ZN8mjCModel16AutoSpringDamperEP8mjModel_
}

/// C: mjCModel::LengthRange (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:389)
/// Calls: cxx-internal:user_model.cc.o:__ZN12_GLOBAL__N_118NumCompilerThreadsEi, cxx:__Z6LRfuncPv, cxx:__ZN6mujoco4user10ThreadPool8ScheduleENSt3__18functionIFvvEEE, cxx:__ZN6mujoco4user10ThreadPool9WaitCountEi, cxx:_mj_deleteData, cxx:_mj_makeData, cxx:_mj_setLengthRange
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_length_range(self_ptr: *mut mjCModel, arg0: *mut mjModel, arg1: *mut mjData) {
    todo!() // cxx:__ZN8mjCModel11LengthRangeEP8mjModel_P7mjData_
}

/// C: mjCModel::CopyNames (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:390)
/// Calls: cxx:_mju_strncpy
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_names(self_ptr: *mut mjCModel, arg0: *mut mjModel) {
    todo!() // cxx:__ZN8mjCModel9CopyNamesEP8mjModel_
}

/// C: mjCModel::CopyPaths (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:391)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_paths(self_ptr: *mut mjCModel, arg0: *mut mjModel) {
    todo!() // cxx:__ZN8mjCModel9CopyPathsEP8mjModel_
}

/// C: mjCModel::CopyObjects (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:392)
/// Calls: cxx:__ZN7mjCMesh10GetQuatPtrEv, cxx:__ZN7mjCMesh18CopyPolygonNormalsEPd, cxx:__ZN7mjCMesh4treeEv, cxx:__ZN7mjCMesh6octreeEv, cxx:__ZN7mjCMesh9GetPosPtrEv, cxx:__ZN8mjCModel10AddWarningENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEPK7mjCBase, cxx:__ZN9mjCSensor12get_userdataEv, cxx:__ZNK11mjCActuator12get_userdataEv, cxx:__ZNK11mjCActuator13is_actlimitedEv, cxx:__ZNK11mjCActuator14is_ctrllimitedEv, cxx:__ZNK11mjCActuator15is_forcelimitedEv, cxx:__ZNK26mjCBoundingVolumeHierarchy3BvhEv, cxx:__ZNK26mjCBoundingVolumeHierarchy4NbvhEv, cxx:__ZNK26mjCBoundingVolumeHierarchy5ChildEv, cxx:__ZNK26mjCBoundingVolumeHierarchy5LevelEv, cxx:__ZNK26mjCBoundingVolumeHierarchy9NodeidptrEi, cxx:__ZNK7mjCMesh10CopyNormalEPf, cxx:__ZNK7mjCMesh11HasTexcoordEv, cxx:__ZNK7mjCMesh11npolygonmapEv, cxx:__ZNK7mjCMesh12CopyPolygonsEPiS0_S0_i, cxx:__ZNK7mjCMesh12CopyTexcoordEPf, cxx:__ZNK7mjCMesh12npolygonvertEv, cxx:__ZNK7mjCMesh14CopyFaceNormalEPi, cxx:__ZNK7mjCMesh14CopyPolygonMapEPiS0_S0_i, cxx:__ZNK7mjCMesh16CopyFaceTexcoordEPi, cxx:__ZNK7mjCMesh5ScaleEv, cxx:__ZNK7mjCMesh5nfaceEv, cxx:__ZNK7mjCMesh5nvertEv, cxx:__ZNK7mjCMesh7nnormalEv, cxx:__ZNK7mjCMesh7szgraphEv, cxx:__ZNK7mjCMesh8CopyFaceEPi, cxx:__ZNK7mjCMesh8CopyVertEPf, cxx:__ZNK7mjCMesh8npolygonEv, cxx:__ZNK7mjCMesh9ntexcoordEv, cxx:__ZNK7mjCSkin10get_vertidEv, cxx:__ZNK7mjCSkin11get_bindposEv, cxx:__ZNK7mjCSkin12get_bindquatEv, cxx:__ZNK7mjCSkin12get_texcoordEv, cxx:__ZNK7mjCSkin14get_vertweightEv, cxx:__ZNK7mjCSkin8get_faceEv, cxx:__ZNK7mjCSkin8get_vertEv, cxx:__ZNK7mjCWrap4TypeEv, cxx:__ZNK9mjCOctree8CopyAabbEPd, cxx:__ZNK9mjCOctree8NumNodesEv, cxx:__ZNK9mjCOctree9CopyChildEPi, cxx:__ZNK9mjCOctree9CopyCoeffEPd, cxx:__ZNK9mjCOctree9CopyLevelEPi, cxx:__ZNK9mjCTendon10is_limitedEv, cxx:__ZNK9mjCTendon12get_userdataEv, cxx:__ZNK9mjCTendon16is_actfrclimitedEv, cxx:_mju_strncpy
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_objects(self_ptr: *mut mjCModel, arg0: *mut mjModel) {
    todo!() // cxx:__ZN8mjCModel11CopyObjectsEP8mjModel_
}

/// C: mjCModel::CopyTree (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:393)
/// Calls: cxx-internal:user_model.cc.o:__ZN12_GLOBAL__N_110IsNullPoseIdEEbPKT_S3_, cxx-internal:user_model.cc.o:__ZN12_GLOBAL__N_110IsSamePoseIdEEbPKT_S3_S3_S3_, cxx:__ZN7mjCBody12get_userdataEv, cxx:__ZN7mjCGeom9GetRBoundEv, cxx:__ZNK26mjCBoundingVolumeHierarchy3BvhEv, cxx:__ZNK26mjCBoundingVolumeHierarchy4NbvhEv, cxx:__ZNK26mjCBoundingVolumeHierarchy5ChildEv, cxx:__ZNK26mjCBoundingVolumeHierarchy5LevelEv, cxx:__ZNK26mjCBoundingVolumeHierarchy9NodeidptrEi, cxx:__ZNK7mjCGeom12get_userdataEv, cxx:__ZNK8mjCJoint10is_limitedEv, cxx:__ZNK8mjCJoint12get_userdataEv, cxx:__ZNK8mjCJoint16is_actfrclimitedEv, cxx:__ZNK9mjCCamera12get_userdataEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_tree(self_ptr: *mut mjCModel, arg0: *mut mjModel) {
    todo!() // cxx:__ZN8mjCModel8CopyTreeEP8mjModel_
}

/// C: mjCModel::FinalizeSimple (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:394)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_finalize_simple(self_ptr: *mut mjCModel, m: *mut mjModel) {
    todo!() // cxx:__ZN8mjCModel14FinalizeSimpleEP8mjModel_
}

/// C: mjCModel::CopyPlugins (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:395)
/// Calls: cxx:_mjp_getPluginAtSlot, cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_copy_plugins(self_ptr: *mut mjCModel, arg0: *mut mjModel) {
    todo!() // cxx:__ZN8mjCModel11CopyPluginsEP8mjModel_
}

/// C: mjCModel::CountTendonDofs (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:396)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_count_tendon_dofs(self_ptr: *mut mjCModel, m: *const mjModel, id: i32) -> i32 {
    todo!() // cxx:__ZN8mjCModel15CountTendonDofsEPK8mjModel_i
}

/// C: mjCModel::CountNJmom (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:398)
/// Calls: cxx:__ZN8mjCModel15CountTendonDofsEPK8mjModel_i, cxx:_mj_mergeChain
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_count_n_jmom(self_ptr: *mut mjCModel, m: *const mjModel) -> i32 {
    todo!() // cxx:__ZN8mjCModel10CountNJmomEPK8mjModel_
}

/// C: mjCModel::CountNJten (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:399)
/// Calls: cxx:__ZN8mjCModel15CountTendonDofsEPK8mjModel_i
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_count_n_jten(self_ptr: *mut mjCModel, m: *const mjModel) -> i32 {
    todo!() // cxx:__ZN8mjCModel10CountNJtenEPK8mjModel_
}

/// C: mjCModel::RemovePlugins (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:402)
/// Calls: cxx:__ZN7mjCBase7ReleaseEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_remove_plugins(self_ptr: *mut mjCModel) {
    todo!() // cxx:__ZN8mjCModel13RemovePluginsEv
}

/// C: mjCModel::CreateObjectLists (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:458)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_create_object_lists(self_ptr: *mut mjCModel) {
    todo!() // cxx:__ZN8mjCModel17CreateObjectListsEv
}

/// C: mjCModel::ProcessLists (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:461)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_process_lists(self_ptr: *mut mjCModel, checkrepeat: bool) {
    todo!() // cxx:__ZN8mjCModel12ProcessListsEb
}

/// C: mjCModel::ResetTreeLists (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:468)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_reset_tree_lists(self_ptr: *mut mjCModel) {
    todo!() // cxx:__ZN8mjCModel14ResetTreeListsEv
}

/// C: mjCModel::SaveDofOffsets (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:471)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_save_dof_offsets(self_ptr: *mut mjCModel, computesize: bool) {
    todo!() // cxx:__ZN8mjCModel14SaveDofOffsetsEb
}

/// C: mjCModel::ResolveKeyframes (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:474)
/// Calls: cxx:__ZN8mjCModel12RestoreStateIdEEvRKNSt3__112basic_stringIcNS1_11char_traitsIcEENS1_9allocatorIcEEEEPKdSB_SB_PT_SD_SD_SD_SD_SD_, cxx:__ZN8mjCModel14SaveDofOffsetsEb
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_resolve_keyframes(self_ptr: *mut mjCModel, m: *const mjModel) {
    todo!() // cxx:__ZN8mjCModel16ResolveKeyframesEPK8mjModel_
}

/// C: mjCModel::ExpandKeyframe (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:477)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_expand_keyframe(self_ptr: *mut mjCModel, key: *mut mjCKey, qpos0_: *const f64, bpos: *const f64, bquat: *const f64) {
    todo!() // cxx:__ZN8mjCModel14ExpandKeyframeEP6mjCKeyPKdS3_S3_
}

/// C: mjCModel::ComputeReference (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:480)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_compute_reference(self_ptr: *mut mjCModel) {
    todo!() // cxx:__ZN8mjCModel16ComputeReferenceEv
}

/// C: mjCModel::CheckBodyMassInertia (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:483)
/// Calls: cxx:__ZNK7mjCBody6BodiesEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_check_body_mass_inertia(self_ptr: *mut mjCModel, body: *mut mjCBody) -> bool {
    todo!() // cxx:__ZN8mjCModel20CheckBodyMassInertiaEP7mjCBody
}

/// C: mjCModel::PrintTree (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:491)
/// Calls: cxx-internal:user_model.cc.o:__ZL11PrintIndentRNSt3__118basic_stringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEEEi
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_print_tree(self_ptr: *mut mjCModel, tree: *mut std__stringstream, body: *const mjCBody, depth: i32) {
    todo!() // cxx:__ZN8mjCModel9PrintTreeERNSt3__118basic_stringstreamIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEPK7mjCBodyi
}

/// C: mjCModel::Signature (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:494)
/// Calls: cxx:__ZN8mjCModel9PrintTreeERNSt3__118basic_stringstreamIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEPK7mjCBodyi, cxx:_mj_hashString
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_signature(self_ptr: *mut mjCModel) -> u64 {
    todo!() // cxx:__ZN8mjCModel9SignatureEv
}

/// C: mjCModel::DeleteSubtreePlugin (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:505)
/// Calls: cxx:__ZNK7mjCBody6BodiesEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_delete_subtree_plugin(self_ptr: *mut mjCModel, subtree: *mut mjCBody) {
    todo!() // cxx:__ZN8mjCModel19DeleteSubtreePluginEP7mjCBody
}

/// C: mjCModel::ExpandAllKeyframes (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_model.h:508)
/// Calls: cxx:__ZN8mjCModel14ExpandKeyframeEP6mjCKeyPKdS3_S3_, cxx:__ZN8mjCModel14SaveDofOffsetsEb, cxx:__ZN8mjCModel16ComputeReferenceEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_model_expand_all_keyframes(self_ptr: *mut mjCModel) {
    todo!() // cxx:__ZN8mjCModel18ExpandAllKeyframesEv
}

