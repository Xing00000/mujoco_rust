//! Port of: user/user_api.cc
//! IR hash: bd605ac8158c32d6
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_parse (user/user_api.cc:78)
/// Calls: FilePath::Ext, mj_defaultVFS, mj_deleteVFS, mj_parseXML, mju_closeResource, mju_decodeResource, mju_free, mju_malloc, mju_openResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_parse(filename: *const i8, content_type: *const i8, vfs: *const mjVFS, error: *mut i8, error_sz: i32) -> *mut mjSpec {
    todo!() // mj_parse
}

/// C: mj_encode (user/user_api.cc:151)
/// Calls: FilePath::Ext, mj_saveXML, mjp_findEncoder
#[allow(unused_variables, non_snake_case)]
pub fn mj_encode(s: *const mjSpec, m: *const mjModel, filename: *const i8, content_type: *const i8, vfs: *const mjVFS, error: *mut i8, error_sz: i32) -> i32 {
    todo!() // mj_encode
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
    todo!() // LogCompileTime
}

/// C: SetFrame (user/user_api.cc:293)
/// Calls: mjs_firstChild, mjs_getFrame, mjs_nextChild, mjs_setFrame
#[allow(unused_variables, non_snake_case)]
pub fn set_frame(body: *mut mjsBody, objtype: u32, frame: *mut mjsFrame) {
    todo!() // SetFrame
}

/// C: attachBody (user/user_api.cc:306)
/// Calls: mjCModel::SetError
#[allow(unused_variables, non_snake_case)]
pub fn attach_body(parent: *mut mjCFrame, child: *const mjCBody, prefix: *const i8, suffix: *const i8) -> *mut mjsElement {
    todo!() // attachBody
}

/// C: attachFrame (user/user_api.cc:325)
/// Calls: mjCModel::SetError
#[allow(unused_variables, non_snake_case)]
pub fn attach_frame(parent: *mut mjCBody, child: *const mjCFrame, prefix: *const i8, suffix: *const i8) -> *mut mjsElement {
    todo!() // attachFrame
}

/// C: attachToSite (user/user_api.cc:344)
/// Calls: attachBody, mjCBody::AddFrame, mjCFrame::SetParent, mjCSite::Body, mjs_getSpec, mjs_resolveOrientation
#[allow(unused_variables, non_snake_case)]
pub fn attach_to_site(parent: *mut mjCSite, child: *const mjCBody, prefix: *const i8, suffix: *const i8) -> *mut mjsElement {
    todo!() // attachToSite
}

/// C: attachFrameToSite (user/user_api.cc:365)
/// Calls: attachFrame, mjCBody::AddFrame, mjCFrame::SetParent, mjCSite::Body, mjs_getSpec, mjs_resolveOrientation, mjs_setFrame
#[allow(unused_variables, non_snake_case)]
pub fn attach_frame_to_site(parent: *mut mjCSite, child: *const mjCFrame, prefix: *const i8, suffix: *const i8) -> *mut mjsElement {
    todo!() // attachFrameToSite
}

/// C: mjs_getTimer (user/user_api.cc:515)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_timer(s: *mut mjSpec) -> *const f64 {
    if s.is_null() {
        return std::ptr::null();
    }
    // SAFETY: s is a valid mjSpec pointer (checked above).
    // element points to the containing mjCModel (C++ static_cast pattern).
    unsafe {
        let model_c: *mut mjCModel = (*s).element as *mut mjCModel;
        (*model_c).timer.as_ptr()
    }
}

/// C: mjs_numWarnings (user/user_api.cc:535)
/// Calls: mjCModel::GetWarnings
#[allow(unused_variables, non_snake_case)]
pub fn mjs_num_warnings(spec: *const mjSpec) -> i32 {
    todo!() // mjs_numWarnings
}

/// C: mjs_getWarning (user/user_api.cc:544)
/// Calls: mjCModel::GetWarnings
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_warning(spec: *const mjSpec, index: i32) -> *const i8 {
    todo!() // mjs_getWarning
}

/// C: FlexcompTypeFromStr (user/user_api.cc:709)
#[allow(unused_variables, non_snake_case)]
pub fn flexcomp_type_from_str(r#type: *const i8) -> mjtFcompType {
    todo!() // FlexcompTypeFromStr
}

/// C: FlexcompDofFromStr (user/user_api.cc:724)
#[allow(unused_variables, non_snake_case)]
pub fn flexcomp_dof_from_str(dof: *const i8) -> mjtDof {
    todo!() // FlexcompDofFromStr
}

/// C: mjs_getCompiler (user/user_api.cc:1553)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_compiler(element: *const mjsElement) -> *mut mjsCompiler {
    todo!() // mjs_getCompiler
}

/// C: mj_makeSpec (user/user_api.h:40)
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_spec() -> *mut mjSpec {
    todo!() // mj_makeSpec
}

/// C: mj_compile (user/user_api.h:43)
/// Calls: LogCompileTime, mjCModel::Compile
#[allow(unused_variables, non_snake_case)]
pub fn mj_compile(s: *mut mjSpec, vfs: *const mjVFS) -> *mut mjModel {
    todo!() // mj_compile
}

/// C: mj_recompile (user/user_api.h:46)
/// Calls: mjCModel::Compile, mjCModel::MakeData, mjCModel::RestoreState, mjCModel::SaveState, mjCModel::SetError, mj_deleteData
#[allow(unused_variables, non_snake_case)]
pub fn mj_recompile(s: *mut mjSpec, vfs: *const mjVFS, m: *mut mjModel, d: *mut mjData) -> i32 {
    todo!() // mj_recompile
}

/// C: mj_copySpec (user/user_api.h:49)
/// Calls: mjCModel::SetError
#[allow(unused_variables, non_snake_case)]
pub fn mj_copy_spec(s: *const mjSpec) -> *mut mjSpec {
    todo!() // mj_copySpec
}

/// C: mjs_getError (user/user_api.h:52)
/// Calls: mjCModel::GetError, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_error(s: *mut mjSpec) -> *const i8 {
    todo!() // mjs_getError
}

/// C: mjs_isWarning (user/user_api.h:55)
/// Calls: mjCModel::GetError, mjCModel::GetWarnings
#[allow(unused_variables, non_snake_case)]
pub fn mjs_is_warning(s: *mut mjSpec) -> i32 {
    todo!() // mjs_isWarning
}

/// C: mj_deleteSpec (user/user_api.h:58)
/// Calls: mjCModel::Release
#[allow(unused_variables, non_snake_case)]
pub fn mj_delete_spec(s: *mut mjSpec) {
    if s.is_null() {
        return;
    }
    // SAFETY: s is a valid non-null pointer; element points to mjCModel
    unsafe {
        let model = (*s).element as *mut mjCModel;
        crate::user::user_model::mj_c_model_release(model);
    }
}

/// C: mjs_addSpec (user/user_api.h:61)
/// Calls: mjCModel::AppendSpec
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_spec(s: *mut mjSpec, child: *mut mjSpec) {
    todo!() // mjs_addSpec
}

/// C: mjs_activatePlugin (user/user_api.h:64)
/// Calls: mjCModel::ActivatePlugin, mjp_getPlugin
#[allow(unused_variables, non_snake_case)]
pub fn mjs_activate_plugin(s: *mut mjSpec, name: *const i8) -> i32 {
    todo!() // mjs_activatePlugin
}

/// C: mjs_setDeepCopy (user/user_api.h:67)
/// Calls: mjCModel::SetDeepCopy
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_deep_copy(s: *mut mjSpec, deepcopy: i32) -> i32 {
    todo!() // mjs_setDeepCopy
}

/// C: mj_copyBack (user/user_api.h:70)
/// Calls: mjCModel::CopyBack
#[allow(unused_variables, non_snake_case)]
pub fn mj_copy_back(s: *mut mjSpec, m: *const mjModel) -> i32 {
    todo!() // mj_copyBack
}

/// C: mjs_attach (user/user_api.h:76)
/// Calls: ResolveConflicts, SetFrame, attachBody, attachFrame, attachFrameToSite, attachToSite, mjCModel::AddGroupedWarning, mjCModel::SetAttachWarningBoundary, mjCModel::SetError, mjs_addFrame, mjs_findBody, mjs_getParent, mjs_getSpec, mjs_setFrame, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjs_attach(parent: *mut mjsElement, child: *const mjsElement, prefix: *const i8, suffix: *const i8) -> *mut mjsElement {
    todo!() // mjs_attach
}

/// C: mjs_addBody (user/user_api.h:83)
/// Calls: mjCBody::AddBody
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_body(body: *mut mjsBody, def: *const mjsDefault) -> *mut mjsBody {
    todo!() // mjs_addBody
}

/// C: mjs_addSite (user/user_api.h:86)
/// Calls: mjCBody::AddSite
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_site(body: *mut mjsBody, def: *const mjsDefault) -> *mut mjsSite {
    todo!() // mjs_addSite
}

/// C: mjs_addJoint (user/user_api.h:89)
/// Calls: mjCBody::AddJoint
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_joint(body: *mut mjsBody, def: *const mjsDefault) -> *mut mjsJoint {
    todo!() // mjs_addJoint
}

/// C: mjs_addFreeJoint (user/user_api.h:92)
/// Calls: mjCBody::AddFreeJoint
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_free_joint(body: *mut mjsBody) -> *mut mjsJoint {
    todo!() // mjs_addFreeJoint
}

/// C: mjs_addGeom (user/user_api.h:95)
/// Calls: mjCBody::AddGeom
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_geom(body: *mut mjsBody, def: *const mjsDefault) -> *mut mjsGeom {
    todo!() // mjs_addGeom
}

/// C: mjs_addCamera (user/user_api.h:98)
/// Calls: mjCBody::AddCamera
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_camera(body: *mut mjsBody, def: *const mjsDefault) -> *mut mjsCamera {
    todo!() // mjs_addCamera
}

/// C: mjs_addLight (user/user_api.h:101)
/// Calls: mjCBody::AddLight
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_light(body: *mut mjsBody, def: *const mjsDefault) -> *mut mjsLight {
    todo!() // mjs_addLight
}

/// C: mjs_addFrame (user/user_api.h:104)
/// Calls: mjCBody::AddFrame, mjCFrame::SetParent
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_frame(body: *mut mjsBody, parentframe: *mut mjsFrame) -> *mut mjsFrame {
    todo!() // mjs_addFrame
}

/// C: mjs_delete (user/user_api.h:107)
/// Calls: mjCModel::IsAttached, mjCModel::SetError
#[allow(unused_variables, non_snake_case)]
pub fn mjs_delete(s: *mut mjSpec, element: *mut mjsElement) -> i32 {
    todo!() // mjs_delete
}

/// C: mjs_addActuator (user/user_api.h:113)
/// Calls: mjCModel::AddActuator
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_actuator(s: *mut mjSpec, def: *const mjsDefault) -> *mut mjsActuator {
    todo!() // mjs_addActuator
}

/// C: mjs_addSensor (user/user_api.h:116)
/// Calls: mjCModel::AddSensor
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_sensor(s: *mut mjSpec) -> *mut mjsSensor {
    todo!() // mjs_addSensor
}

/// C: mjs_addFlex (user/user_api.h:119)
/// Calls: mjCModel::AddFlex
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_flex(s: *mut mjSpec) -> *mut mjsFlex {
    todo!() // mjs_addFlex
}

/// C: mjs_makeFlex (user/user_api.h:122)
/// Calls: FlexcompDofFromStr, FlexcompTypeFromStr, mjCFlexcomp::Make, mjCModel::Flexes, mjCModel::SetError, mju_error
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_make_flex(body: *mut mjsBody, name: *const i8, r#type: *const i8, dim: i32, dof: *const i8, count: *const i32, cellcount: *const i32, spacing: *const f64, scale: *const f64, radius: f64, mass: f64, inertiabox: f64, equality: i32, rigid: i32, flatskin: i32, elastic2d: i32, pos: *const f64, quat: *const f64, origin: *const f64, file: *const i8, vfs: *const mjVFS) -> *mut mjsFlex {
    todo!() // mjs_makeFlex
}

/// C: mjs_addPair (user/user_api.h:130)
/// Calls: mjCModel::AddPair
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_pair(s: *mut mjSpec, def: *const mjsDefault) -> *mut mjsPair {
    todo!() // mjs_addPair
}

/// C: mjs_addExclude (user/user_api.h:133)
/// Calls: mjCModel::AddExclude
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_exclude(s: *mut mjSpec) -> *mut mjsExclude {
    todo!() // mjs_addExclude
}

/// C: mjs_addEquality (user/user_api.h:136)
/// Calls: mjCModel::AddEquality
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_equality(s: *mut mjSpec, def: *const mjsDefault) -> *mut mjsEquality {
    todo!() // mjs_addEquality
}

/// C: mjs_addTendon (user/user_api.h:139)
/// Calls: mjCModel::AddTendon
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_tendon(s: *mut mjSpec, def: *const mjsDefault) -> *mut mjsTendon {
    todo!() // mjs_addTendon
}

/// C: mjs_wrapSite (user/user_api.h:142)
/// Calls: mjCTendon::WrapSite
#[allow(unused_variables, non_snake_case)]
pub fn mjs_wrap_site(tendon: *mut mjsTendon, name: *const i8) -> *mut mjsWrap {
    todo!() // mjs_wrapSite
}

/// C: mjs_wrapGeom (user/user_api.h:145)
/// Calls: mjCTendon::WrapGeom
#[allow(unused_variables, non_snake_case)]
pub fn mjs_wrap_geom(tendon: *mut mjsTendon, name: *const i8, sidesite: *const i8) -> *mut mjsWrap {
    todo!() // mjs_wrapGeom
}

/// C: mjs_wrapJoint (user/user_api.h:148)
/// Calls: mjCTendon::WrapJoint
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_wrap_joint(tendon: *mut mjsTendon, name: *const i8, coef: f64) -> *mut mjsWrap {
    todo!() // mjs_wrapJoint
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
    todo!() // mjs_wrapPulley
}

/// C: mjs_addNumeric (user/user_api.h:154)
/// Calls: mjCModel::AddNumeric
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_numeric(s: *mut mjSpec) -> *mut mjsNumeric {
    todo!() // mjs_addNumeric
}

/// C: mjs_addText (user/user_api.h:157)
/// Calls: mjCModel::AddText
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_text(s: *mut mjSpec) -> *mut mjsText {
    todo!() // mjs_addText
}

/// C: mjs_addTuple (user/user_api.h:160)
/// Calls: mjCModel::AddTuple
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_tuple(s: *mut mjSpec) -> *mut mjsTuple {
    todo!() // mjs_addTuple
}

/// C: mjs_addKey (user/user_api.h:163)
/// Calls: mjCModel::AddKey
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_key(s: *mut mjSpec) -> *mut mjsKey {
    todo!() // mjs_addKey
}

/// C: mjs_addPlugin (user/user_api.h:166)
/// Calls: mjCModel::AddPlugin
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_plugin(s: *mut mjSpec) -> *mut mjsPlugin {
    todo!() // mjs_addPlugin
}

/// C: mjs_addDefault (user/user_api.h:169)
/// Calls: mjCModel::AddDefault, mjCModel::Default
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_default(s: *mut mjSpec, classname: *const i8, parent: *const mjsDefault) -> *mut mjsDefault {
    todo!() // mjs_addDefault
}

/// C: mjs_setToMotor (user/user_api.h:175)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_to_motor(actuator: *mut mjsActuator) -> *const i8 {
    // SAFETY: caller guarantees actuator is a valid pointer to mjsActuator.
    unsafe {
        // unit gain
        (*actuator).gainprm[0] = 1.0;

        // implied parameters
        std::ptr::write((*actuator).dyntype.as_mut_ptr() as *mut i32, 0);   // mjDYN_NONE
        std::ptr::write((*actuator).gaintype.as_mut_ptr() as *mut i32, 0);  // mjGAIN_FIXED
        std::ptr::write((*actuator).biastype.as_mut_ptr() as *mut i32, 0);  // mjBIAS_NONE
    }
    b"\0".as_ptr() as *const i8
}

/// C: mjs_setToPosition (user/user_api.h:178)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_to_position(actuator: *mut mjsActuator, kp: f64, kv: *mut f64, dampratio: *mut f64, timeconst: *mut f64, inheritrange: f64) -> *const i8 {
    // SAFETY: caller guarantees actuator is valid. kv/dampratio/timeconst may be null.
    unsafe {
        (*actuator).gainprm[0] = kp;
        (*actuator).biasprm[1] = -kp;

        // set biasprm[2]; negative: regular damping, positive: dampratio
        if !dampratio.is_null() && !kv.is_null() {
            return b"kv and dampratio cannot both be defined\0".as_ptr() as *const i8;
        }

        if !kv.is_null() {
            if *kv < 0.0 {
                return b"kv cannot be negative\0".as_ptr() as *const i8;
            }
            (*actuator).biasprm[2] = -(*kv);
        }
        if !dampratio.is_null() {
            if *dampratio < 0.0 {
                return b"dampratio cannot be negative\0".as_ptr() as *const i8;
            }
            (*actuator).biasprm[2] = *dampratio;
        }
        if !timeconst.is_null() {
            if *timeconst < 0.0 {
                return b"timeconst cannot be negative\0".as_ptr() as *const i8;
            }
            (*actuator).dynprm[0] = *timeconst;
            let dynval: i32 = if *timeconst == 0.0 { 0 } else { 3 }; // mjDYN_NONE : mjDYN_FILTEREXACT
            std::ptr::write((*actuator).dyntype.as_mut_ptr() as *mut i32, dynval);
        }
        (*actuator).inheritrange = inheritrange;

        if inheritrange > 0.0 {
            if (*actuator).ctrlrange[0] != 0.0 || (*actuator).ctrlrange[1] != 0.0 {
                return b"ctrlrange and inheritrange cannot both be defined\0".as_ptr() as *const i8;
            }
        }

        std::ptr::write((*actuator).gaintype.as_mut_ptr() as *mut i32, 0);  // mjGAIN_FIXED
        std::ptr::write((*actuator).biastype.as_mut_ptr() as *mut i32, 1);  // mjBIAS_AFFINE
    }
    b"\0".as_ptr() as *const i8
}

/// C: mjs_setToIntVelocity (user/user_api.h:182)
/// Calls: mjs_setToPosition
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_to_int_velocity(actuator: *mut mjsActuator, kp: f64, kv: *mut f64, dampratio: *mut f64, timeconst: *mut f64, inheritrange: f64) -> *const i8 {
    // SAFETY: caller guarantees actuator is valid. kv/dampratio/timeconst may be null.
    unsafe {
        let err = mjs_set_to_position(actuator, kp, kv, dampratio, timeconst, inheritrange);
        // mjs_setToPosition returns "" on success, error string on failure
        // but we ignore its return and always continue (matches C behavior)
        let _ = err;

        std::ptr::write((*actuator).dyntype.as_mut_ptr() as *mut i32, 1);  // mjDYN_INTEGRATOR
        (*actuator).actlimited = 1;

        if inheritrange > 0.0 {
            if (*actuator).actrange[0] != 0.0 || (*actuator).actrange[1] != 0.0 {
                return b"actrange and inheritrange cannot both be defined\0".as_ptr() as *const i8;
            }
        }
    }
    b"\0".as_ptr() as *const i8
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
    // SAFETY: caller guarantees actuator is valid.
    unsafe {
        crate::user::user_util::mjuu_zerovec((*actuator).biasprm.as_mut_ptr(), 10);  // mjNBIAS
        (*actuator).gainprm[0] = kv;
        (*actuator).biasprm[2] = -kv;
        std::ptr::write((*actuator).dyntype.as_mut_ptr() as *mut i32, 0);   // mjDYN_NONE
        std::ptr::write((*actuator).gaintype.as_mut_ptr() as *mut i32, 0);  // mjGAIN_FIXED
        std::ptr::write((*actuator).biastype.as_mut_ptr() as *mut i32, 1);  // mjBIAS_AFFINE
    }
    b"\0".as_ptr() as *const i8
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
    // SAFETY: caller guarantees actuator is valid.
    unsafe {
        crate::user::user_util::mjuu_zerovec((*actuator).gainprm.as_mut_ptr(), 10);  // mjNGAIN
        (*actuator).gainprm[2] = -kv;
        (*actuator).ctrllimited = 1;
        std::ptr::write((*actuator).dyntype.as_mut_ptr() as *mut i32, 0);   // mjDYN_NONE
        std::ptr::write((*actuator).gaintype.as_mut_ptr() as *mut i32, 1);  // mjGAIN_AFFINE
        std::ptr::write((*actuator).biastype.as_mut_ptr() as *mut i32, 0);  // mjBIAS_NONE

        if kv < 0.0 {
            return b"damping coefficient cannot be negative\0".as_ptr() as *const i8;
        }
        if (*actuator).ctrlrange[0] < 0.0 || (*actuator).ctrlrange[1] < 0.0 {
            return b"damper control range cannot be negative\0".as_ptr() as *const i8;
        }
    }
    b"\0".as_ptr() as *const i8
}

/// C: mjs_setToCylinder (user/user_api.h:192)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_to_cylinder(actuator: *mut mjsActuator, timeconst: f64, bias: f64, area: f64, diameter: f64) -> *const i8 {
    // SAFETY: caller guarantees actuator is valid.
    unsafe {
        (*actuator).dynprm[0] = timeconst;
        (*actuator).biasprm[0] = bias;
        (*actuator).gainprm[0] = area;
        if diameter >= 0.0 {
            (*actuator).gainprm[0] = std::f64::consts::PI / 4.0 * diameter * diameter;
        }
        std::ptr::write((*actuator).dyntype.as_mut_ptr() as *mut i32, 2);   // mjDYN_FILTER
        std::ptr::write((*actuator).gaintype.as_mut_ptr() as *mut i32, 0);  // mjGAIN_FIXED
        std::ptr::write((*actuator).biastype.as_mut_ptr() as *mut i32, 1);  // mjBIAS_AFFINE
    }
    b"\0".as_ptr() as *const i8
}

/// C: mjs_setToMuscle (user/user_api.h:196)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_to_muscle(actuator: *mut mjsActuator, timeconst: *mut f64, tausmooth: f64, range: *mut f64, force: f64, scale: f64, lmin: f64, lmax: f64, vmax: f64, fpmax: f64, fvmax: f64) -> *const i8 {
    // SAFETY: caller guarantees actuator is valid. timeconst and range are non-null arrays.
    unsafe {
        // set muscle defaults if same as global defaults
        if (*actuator).dynprm[0] == 1.0 { (*actuator).dynprm[0] = 0.01; }    // tau act
        if (*actuator).dynprm[1] == 0.0 { (*actuator).dynprm[1] = 0.04; }    // tau deact
        if (*actuator).gainprm[0] == 1.0 { (*actuator).gainprm[0] = 0.75; }  // range[0]
        if (*actuator).gainprm[1] == 0.0 { (*actuator).gainprm[1] = 1.05; }  // range[1]
        if (*actuator).gainprm[2] == 0.0 { (*actuator).gainprm[2] = -1.0; }  // force
        if (*actuator).gainprm[3] == 0.0 { (*actuator).gainprm[3] = 200.0; } // scale
        if (*actuator).gainprm[4] == 0.0 { (*actuator).gainprm[4] = 0.5; }   // lmin
        if (*actuator).gainprm[5] == 0.0 { (*actuator).gainprm[5] = 1.6; }   // lmax
        if (*actuator).gainprm[6] == 0.0 { (*actuator).gainprm[6] = 1.5; }   // vmax
        if (*actuator).gainprm[7] == 0.0 { (*actuator).gainprm[7] = 1.3; }   // fpmax
        if (*actuator).gainprm[8] == 0.0 { (*actuator).gainprm[8] = 1.2; }   // fvmax

        if tausmooth < 0.0 {
            return b"muscle tausmooth cannot be negative\0".as_ptr() as *const i8;
        }

        (*actuator).dynprm[2] = tausmooth;
        if *timeconst.add(0) >= 0.0 { (*actuator).dynprm[0] = *timeconst.add(0); }
        if *timeconst.add(1) >= 0.0 { (*actuator).dynprm[1] = *timeconst.add(1); }
        if *range.add(0) >= 0.0 { (*actuator).gainprm[0] = *range.add(0); }
        if *range.add(1) >= 0.0 { (*actuator).gainprm[1] = *range.add(1); }
        if force >= 0.0 { (*actuator).gainprm[2] = force; }
        if scale >= 0.0 { (*actuator).gainprm[3] = scale; }
        if lmin >= 0.0 { (*actuator).gainprm[4] = lmin; }
        if lmax >= 0.0 { (*actuator).gainprm[5] = lmax; }
        if vmax >= 0.0 { (*actuator).gainprm[6] = vmax; }
        if fpmax >= 0.0 { (*actuator).gainprm[7] = fpmax; }
        if fvmax >= 0.0 { (*actuator).gainprm[8] = fvmax; }

        // biasprm = gainprm
        let mut n: i32 = 0;
        while n < 9 {
            (*actuator).biasprm[n as usize] = (*actuator).gainprm[n as usize];
            n += 1;
        }

        std::ptr::write((*actuator).dyntype.as_mut_ptr() as *mut i32, 4);   // mjDYN_MUSCLE
        std::ptr::write((*actuator).gaintype.as_mut_ptr() as *mut i32, 2);  // mjGAIN_MUSCLE
        std::ptr::write((*actuator).biastype.as_mut_ptr() as *mut i32, 2);  // mjBIAS_MUSCLE
    }
    b"\0".as_ptr() as *const i8
}

/// C: mjs_setToAdhesion (user/user_api.h:201)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_to_adhesion(actuator: *mut mjsActuator, gain: f64) -> *const i8 {
    // SAFETY: caller guarantees actuator is valid.
    unsafe {
        (*actuator).gainprm[0] = gain;
        (*actuator).ctrllimited = 1;
        std::ptr::write((*actuator).gaintype.as_mut_ptr() as *mut i32, 0);  // mjGAIN_FIXED
        std::ptr::write((*actuator).biastype.as_mut_ptr() as *mut i32, 0);  // mjBIAS_NONE

        if gain < 0.0 {
            return b"adhesion gain cannot be negative\0".as_ptr() as *const i8;
        }
        if (*actuator).ctrlrange[0] < 0.0 || (*actuator).ctrlrange[1] < 0.0 {
            return b"adhesion control range cannot be negative\0".as_ptr() as *const i8;
        }
    }
    b"\0".as_ptr() as *const i8
}

/// C: mjs_setToDCMotor (user/user_api.h:204)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_to_dc_motor(actuator: *mut mjsActuator, motorconst: *mut f64, resistance: f64, nominal: *mut f64, saturation: *mut f64, inductance: *mut f64, cogging: *mut f64, controller: *mut f64, thermal: *mut f64, lugre: *mut f64, input_mode: i32) -> *const i8 {
    // SAFETY: all pointer dereferences follow the C original's null-check pattern.
    // Caller guarantees actuator is valid; optional params are null-checked before access.
    unsafe {
        let mut R = resistance;
        let Kt: f64 = if !motorconst.is_null() { *motorconst.add(0) } else { 0.0 };
        let mut Ke: f64 = if !motorconst.is_null() { *motorconst.add(1) } else { 0.0 };
        let vn: f64 = if !nominal.is_null() { *nominal.add(0) } else { 0.0 };
        let tau0: f64 = if !nominal.is_null() { *nominal.add(1) } else { 0.0 };
        let omega0: f64 = if !nominal.is_null() { *nominal.add(2) } else { 0.0 };

        // derive Ke from nominal: omega0 = vn*Ke / (Ke^2 + R*B)
        if vn > 0.0 && Ke <= 0.0 && omega0 > 0.0 {
            // viscous damping (linear)
            let B = (*actuator).damping[0];

            if B > 0.0 && R > 0.0 {
                // R known: solve quadratic Ke^2*omega0 - Ke*vn + R*B*omega0 = 0
                let disc = vn * vn - 4.0 * R * B * omega0 * omega0;
                Ke = if disc > 0.0 { (vn + f64::sqrt(disc)) / (2.0 * omega0) } else { vn / omega0 };
            } else if B > 0.0 && tau0 > 0.0 {
                // R from nominal (tau0 = Ke*vn/R, so R = Ke*vn/tau0)
                let Ke_exact = vn / omega0 - vn * B / tau0;
                Ke = if Ke_exact > 0.0 { Ke_exact } else { vn / omega0 };
            } else {
                // B = 0 or insufficient data for B-correction: omega0 = vn/Ke
                Ke = vn / omega0;
            }
        }

        // resolve effective motor constant K from [Kt, Ke]
        let K: f64 = if Kt > 0.0 && Ke > 0.0 {
            f64::sqrt(Kt * Ke)
        } else if Kt > 0.0 {
            Kt
        } else {
            Ke
        };

        // derive R from nominal: tau0 = K*vn/R
        if R == 0.0 && vn > 0.0 && tau0 > 0.0 && K > 0.0 {
            R = K * vn / tau0;
        }

        if K <= 0.0 {
            return b"DC motor: motor constant K must be positive\0".as_ptr() as *const i8;
        }
        if R <= 0.0 {
            return b"DC motor: resistance R must be positive\0".as_ptr() as *const i8;
        }

        // set types
        std::ptr::write((*actuator).dyntype.as_mut_ptr() as *mut i32, mjtDyn_mjDYN_DCMOTOR as i32);
        std::ptr::write((*actuator).gaintype.as_mut_ptr() as *mut i32, mjtGain_mjGAIN_DCMOTOR as i32);
        std::ptr::write((*actuator).biastype.as_mut_ptr() as *mut i32, mjtBias_mjBIAS_DCMOTOR as i32);

        // gainprm: [R, K, alpha, T0]
        (*actuator).gainprm[0] = R;
        (*actuator).gainprm[1] = K;

        // controller parameters: gainprm[4:6] for kp, ki, kd
        (*actuator).gainprm[4] = if !controller.is_null() { *controller.add(0) } else { 0.0 };
        (*actuator).gainprm[5] = if !controller.is_null() { *controller.add(1) } else { 0.0 };
        (*actuator).gainprm[6] = if !controller.is_null() { *controller.add(2) } else { 0.0 };

        // controller parameters: dynprm[7,8] for slewmax, Imax
        (*actuator).dynprm[7] = if !controller.is_null() { *controller.add(3) } else { 0.0 };
        (*actuator).dynprm[8] = if !controller.is_null() { *controller.add(4) } else { 0.0 };

        // controller parameters: gainprm[7] for v_max
        if !controller.is_null() && *controller.add(5) > 0.0 {
            (*actuator).gainprm[7] = *controller.add(5);
        }

        // saturation -> forcerange
        if !saturation.is_null() && (*saturation.add(0) > 0.0 || *saturation.add(1) > 0.0) {
            let mut tau_max = *saturation.add(0);
            if tau_max == 0.0 && *saturation.add(1) > 0.0 {
                tau_max = K * *saturation.add(1); // tau_max = K * i_max
            }
            (*actuator).forcerange[0] = -tau_max;
            (*actuator).forcerange[1] = tau_max;
            (*actuator).forcelimited = 1;
        }

        // saturation: [tau_max, i_max, (di/dt)_max]
        if !saturation.is_null() && *saturation.add(2) > 0.0 {
            (*actuator).dynprm[1] = *saturation.add(2);
        }

        // cogging: [amplitude, periodicity, phase] -> biasprm[0:3]
        (*actuator).biasprm[0] = if !cogging.is_null() { *cogging.add(0) } else { 0.0 };
        (*actuator).biasprm[1] = if !cogging.is_null() { *cogging.add(1) } else { 0.0 };
        (*actuator).biasprm[2] = if !cogging.is_null() { *cogging.add(2) } else { 0.0 };

        // count activation variables: slot order is slew, integral, temperature, bristle, current
        let mut actdim: i32 = 0;

        // inductance: [L, te]
        if !inductance.is_null() && *inductance.add(0) < 0.0 {
            return b"DC motor: inductance must be non-negative\0".as_ptr() as *const i8;
        }
        if !inductance.is_null() && *inductance.add(1) < 0.0 {
            return b"DC motor: electrical time constant must be non-negative\0".as_ptr() as *const i8;
        }
        let te: f64 = if !inductance.is_null() && *inductance.add(0) > 0.0 {
            *inductance.add(0) / R
        } else if !inductance.is_null() {
            *inductance.add(1)
        } else {
            0.0
        };
        (*actuator).dynprm[0] = te;
        if te > 0.0 {
            actdim += 1;
        }

        // controller states: slew rate limiting
        if !controller.is_null() && *controller.add(3) > 0.0 {
            actdim += 1;
        }

        // controller states: integral
        if !controller.is_null() && *controller.add(1) > 0.0 {
            actdim += 1;
        }

        // thermal -> temperature activation
        if !thermal.is_null() && (*thermal.add(0) > 0.0 || *thermal.add(1) > 0.0 || *thermal.add(2) > 0.0) {
            let mut RT = *thermal.add(0);    // thermal resistance
            let mut C = *thermal.add(1);     // thermal capacitance
            let mut tth = *thermal.add(2);   // thermal time constant
            let alpha = *thermal.add(3);     // temperature coefficient
            let T0 = *thermal.add(4);        // reference temperature
            let Ta = *thermal.add(5);        // ambient temperature

            if tth > 0.0 && RT > 0.0 && C == 0.0 {
                C = tth / RT;
            } else if tth > 0.0 && C > 0.0 && RT == 0.0 {
                RT = tth / C;
            } else if tth == 0.0 && RT > 0.0 && C > 0.0 {
                tth = RT * C;
            }

            if RT <= 0.0 {
                return b"DC motor: thermal resistance must be positive\0".as_ptr() as *const i8;
            }
            if C <= 0.0 {
                return b"DC motor: thermal capacitance must be positive\0".as_ptr() as *const i8;
            }

            (*actuator).dynprm[2] = RT;
            (*actuator).dynprm[3] = C;
            (*actuator).dynprm[4] = Ta;
            (*actuator).gainprm[2] = alpha;
            (*actuator).gainprm[3] = T0;
            actdim += 1;
        }

        // lugre: {stiffness, damping, coulomb, static, stribeck}
        if !lugre.is_null() && *lugre.add(0) > 0.0 {
            (*actuator).dynprm[5] = *lugre.add(0);    // stiffness -> sigma0
            (*actuator).dynprm[6] = *lugre.add(1);    // damping   -> sigma1
            (*actuator).biasprm[3] = *lugre.add(2);   // coulomb   -> tau_c
            (*actuator).biasprm[4] = *lugre.add(3);   // static    -> tau_s
            (*actuator).biasprm[5] = *lugre.add(4);   // stribeck  -> omega_s
            actdim += 1;
        }

        // set input mode and activation dimension
        (*actuator).gainprm[8] = input_mode as f64;
        (*actuator).actdim = actdim;

        // enforce actlimited = 0; homogeneous bounds are invalid across DC motor states
        (*actuator).actlimited = 0;

        // DC motor always uses actearly
        (*actuator).actearly = 1;

        b"\0".as_ptr() as *const i8
    }
}

/// C: mjs_addMesh (user/user_api.h:213)
/// Calls: mjCModel::AddMesh
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_mesh(s: *mut mjSpec, def: *const mjsDefault) -> *mut mjsMesh {
    todo!() // mjs_addMesh
}

/// C: mjs_addHField (user/user_api.h:216)
/// Calls: mjCModel::AddHField
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_h_field(s: *mut mjSpec) -> *mut mjsHField {
    todo!() // mjs_addHField
}

/// C: mjs_addSkin (user/user_api.h:219)
/// Calls: mjCModel::AddSkin
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_skin(s: *mut mjSpec) -> *mut mjsSkin {
    todo!() // mjs_addSkin
}

/// C: mjs_addTexture (user/user_api.h:222)
/// Calls: mjCModel::AddTexture
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_texture(s: *mut mjSpec) -> *mut mjsTexture {
    todo!() // mjs_addTexture
}

/// C: mjs_addMaterial (user/user_api.h:225)
/// Calls: mjCModel::AddMaterial
#[allow(unused_variables, non_snake_case)]
pub fn mjs_add_material(s: *mut mjSpec, def: *const mjsDefault) -> *mut mjsMaterial {
    todo!() // mjs_addMaterial
}

/// C: mjs_makeMesh (user/user_api.h:228)
/// Calls: mjCMesh::MakeCone, mjCMesh::MakeHemisphere, mjCMesh::MakeRect, mjCMesh::MakeSphere, mjCMesh::MakeSupersphere, mjCMesh::MakeSupertorus, mjCMesh::MakeWedge, mjCModel::SetError
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_make_mesh(mesh: *mut mjsMesh, builtin: u32, params: *mut f64, nparams: i32) -> i32 {
    todo!() // mjs_makeMesh
}

/// C: mjs_getSpec (user/user_api.h:233)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_spec(element: *const mjsElement) -> *mut mjSpec {
    // SAFETY: element is a valid pointer to mjsElement within a mjCBase-derived object.
    // Accesses mjCBase.model->spec, matching C++ static_cast<const mjCBase*>(element)->model->spec.
    unsafe {
        let base = element as *const mjCBase;
        let model = (*base).model;
        &mut (*model).spec as *mut mjSpec
    }
}

/// C: mjs_getOriginSpec (user/user_api.h:237)
/// Calls: mjCModel::FindSpec
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_origin_spec(element: *const mjsElement) -> *mut mjSpec {
    todo!() // mjs_getOriginSpec
}

/// C: mjs_findSpec (user/user_api.h:240)
/// Calls: mjCModel::FindSpec
#[allow(unused_variables, non_snake_case)]
pub fn mjs_find_spec(spec: *const mjSpec, name: *const i8) -> *mut mjSpec {
    todo!() // mjs_findSpec
}

/// C: mjs_findBody (user/user_api.h:243)
/// Calls: mjs_findElement
#[allow(unused_variables, non_snake_case)]
pub fn mjs_find_body(s: *const mjSpec, name: *const i8) -> *mut mjsBody {
    todo!() // mjs_findBody
}

/// C: mjs_findElement (user/user_api.h:246)
/// Calls: mjCModel::FindAsset, mjCModel::FindObject, mjCModel::FindTree, mjCModel::GetWorld, mjCModel::IsCompiled, mjCModel::Meshes, mjCModel::Textures
#[allow(unused_variables, non_snake_case)]
pub fn mjs_find_element(s: *const mjSpec, r#type: u32, name: *const i8) -> *mut mjsElement {
    todo!() // mjs_findElement
}

/// C: mjs_findChild (user/user_api.h:249)
/// Calls: mjCBody::FindObject
#[allow(unused_variables, non_snake_case)]
pub fn mjs_find_child(body: *const mjsBody, name: *const i8) -> *mut mjsBody {
    todo!() // mjs_findChild
}

/// C: mjs_getParent (user/user_api.h:252)
/// Calls: mjCBody::GetParent, mjCCamera::GetParent, mjCFrame::GetParent, mjCGeom::GetParent, mjCJoint::GetParent, mjCLight::GetParent, mjCSite::GetParent
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_parent(element: *const mjsElement) -> *mut mjsBody {
    todo!() // mjs_getParent
}

/// C: mjs_getFrame (user/user_api.h:255)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_frame(element: *const mjsElement) -> *mut mjsFrame {
    // SAFETY: element is a valid mjsElement pointer embedded in a mjCBase-derived object.
    // The cast mirrors C++ static_cast<const mjCBase*>(element).
    unsafe {
        let elemtype = *(element as *const i32);
        match elemtype {
            1 | 100 | 3 | 5 | 6 | 7 | 8 => {
                // mjOBJ_BODY=1, mjOBJ_FRAME=100, mjOBJ_JOINT=3, mjOBJ_GEOM=5,
                // mjOBJ_SITE=6, mjOBJ_CAMERA=7, mjOBJ_LIGHT=8
                let base = element as *const mjCBase;
                let frame = (*base).frame;
                if frame.is_null() {
                    std::ptr::null_mut()
                } else {
                    &mut (*frame).spec as *mut mjsFrame
                }
            }
            _ => std::ptr::null_mut(),
        }
    }
}

/// C: mjs_findFrame (user/user_api.h:258)
/// Calls: mjs_findElement
#[allow(unused_variables, non_snake_case)]
pub fn mjs_find_frame(s: *const mjSpec, name: *const i8) -> *mut mjsFrame {
    todo!() // mjs_findFrame
}

/// C: mjs_getDefault (user/user_api.h:261)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_default(element: *const mjsElement) -> *mut mjsDefault {
    todo!() // mjs_getDefault
}

/// C: mjs_findDefault (user/user_api.h:264)
/// Calls: mjCModel::FindDefault
#[allow(unused_variables, non_snake_case)]
pub fn mjs_find_default(s: *const mjSpec, classname: *const i8) -> *mut mjsDefault {
    todo!() // mjs_findDefault
}

/// C: mjs_getSpecDefault (user/user_api.h:267)
/// Calls: mjCModel::Default
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_spec_default(s: *const mjSpec) -> *mut mjsDefault {
    todo!() // mjs_getSpecDefault
}

/// C: mjs_getId (user/user_api.h:270)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_id(element: *const mjsElement) -> i32 {
    // SAFETY: element is a valid mjsElement pointer embedded in a mjCBase-derived object.
    unsafe {
        if element.is_null() {
            return -1;
        }
        (*(element as *const mjCBase)).id
    }
}

/// C: mjs_firstChild (user/user_api.h:276)
/// Calls: mjCBody::NextChild, mjCModel::SetError
#[allow(unused_variables, non_snake_case)]
pub fn mjs_first_child(body: *const mjsBody, r#type: u32, recurse: i32) -> *mut mjsElement {
    todo!() // mjs_firstChild
}

/// C: mjs_nextChild (user/user_api.h:280)
/// Calls: mjCBody::NextChild, mjCModel::SetError
#[allow(unused_variables, non_snake_case)]
pub fn mjs_next_child(body: *const mjsBody, child: *const mjsElement, recurse: i32) -> *mut mjsElement {
    todo!() // mjs_nextChild
}

/// C: mjs_firstElement (user/user_api.h:283)
/// Calls: mjCModel::NextObject
#[allow(unused_variables, non_snake_case)]
pub fn mjs_first_element(s: *const mjSpec, r#type: u32) -> *mut mjsElement {
    todo!() // mjs_firstElement
}

/// C: mjs_nextElement (user/user_api.h:286)
/// Calls: mjCModel::NextObject
#[allow(unused_variables, non_snake_case)]
pub fn mjs_next_element(s: *const mjSpec, element: *const mjsElement) -> *mut mjsElement {
    todo!() // mjs_nextElement
}

/// C: mjs_getWrapTarget (user/user_api.h:289)
/// Calls: mjCWrap::Type, mjs_findElement, mjs_getSpec
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_wrap_target(wrap: *const mjsWrap) -> *mut mjsElement {
    todo!() // mjs_getWrapTarget
}

/// C: mjs_getWrapSideSite (user/user_api.h:292)
/// Calls: mjCWrap::Type, mjs_asSite, mjs_findElement, mjs_getSpec, mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_wrap_side_site(wrap: *const mjsWrap) -> *mut mjsSite {
    todo!() // mjs_getWrapSideSite
}

/// C: mjs_getWrapDivisor (user/user_api.h:295)
/// Calls: mjCWrap::Type, mju_warning
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_wrap_divisor(wrap: *const mjsWrap) -> f64 {
    todo!() // mjs_getWrapDivisor
}

/// C: mjs_getWrapCoef (user/user_api.h:298)
/// Calls: mjCWrap::Type, mju_warning
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_wrap_coef(wrap: *const mjsWrap) -> f64 {
    todo!() // mjs_getWrapCoef
}

/// C: mjs_asBody (user/user_api.h:301)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_body(element: *mut mjsElement) -> *mut mjsBody {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCBody*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 1 {
            &mut (*(element as *mut crate::types::mjCBody)).spec as *mut crate::types::mjsBody
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asGeom (user/user_api.h:304)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_geom(element: *mut mjsElement) -> *mut mjsGeom {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCGeom*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 5 {
            &mut (*(element as *mut crate::types::mjCGeom)).spec as *mut crate::types::mjsGeom
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asJoint (user/user_api.h:307)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_joint(element: *mut mjsElement) -> *mut mjsJoint {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCJoint*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 3 {
            &mut (*(element as *mut crate::types::mjCJoint)).spec as *mut crate::types::mjsJoint
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asSite (user/user_api.h:310)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_site(element: *mut mjsElement) -> *mut mjsSite {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCSite*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 6 {
            &mut (*(element as *mut crate::types::mjCSite)).spec as *mut crate::types::mjsSite
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asCamera (user/user_api.h:313)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_camera(element: *mut mjsElement) -> *mut mjsCamera {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCCamera*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 7 {
            &mut (*(element as *mut crate::types::mjCCamera)).spec as *mut crate::types::mjsCamera
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asLight (user/user_api.h:316)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_light(element: *mut mjsElement) -> *mut mjsLight {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCLight*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 8 {
            &mut (*(element as *mut crate::types::mjCLight)).spec as *mut crate::types::mjsLight
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asFrame (user/user_api.h:319)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_frame(element: *mut mjsElement) -> *mut mjsFrame {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCFrame*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 100 {
            &mut (*(element as *mut crate::types::mjCFrame)).spec as *mut crate::types::mjsFrame
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asActuator (user/user_api.h:322)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_actuator(element: *mut mjsElement) -> *mut mjsActuator {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCActuator*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 19 {
            &mut (*(element as *mut crate::types::mjCActuator)).spec as *mut crate::types::mjsActuator
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asSensor (user/user_api.h:325)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_sensor(element: *mut mjsElement) -> *mut mjsSensor {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCSensor*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 20 {
            &mut (*(element as *mut crate::types::mjCSensor)).spec as *mut crate::types::mjsSensor
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asFlex (user/user_api.h:328)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_flex(element: *mut mjsElement) -> *mut mjsFlex {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCFlex*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 9 { // mjOBJ_FLEX
            &mut (*(element as *mut crate::types::mjCFlex)).spec as *mut crate::types::mjsFlex
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asPair (user/user_api.h:331)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_pair(element: *mut mjsElement) -> *mut mjsPair {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCPair*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 15 {
            &mut (*(element as *mut crate::types::mjCPair)).spec as *mut crate::types::mjsPair
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asEquality (user/user_api.h:334)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_equality(element: *mut mjsElement) -> *mut mjsEquality {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCEquality*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 17 {
            &mut (*(element as *mut crate::types::mjCEquality)).spec as *mut crate::types::mjsEquality
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asExclude (user/user_api.h:337)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_exclude(element: *mut mjsElement) -> *mut mjsExclude {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCExclude*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 16 {
            &mut (*(element as *mut crate::types::mjCBodyPair)).spec as *mut crate::types::mjsExclude
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asTendon (user/user_api.h:340)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_tendon(element: *mut mjsElement) -> *mut mjsTendon {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCTendon*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 18 {
            &mut (*(element as *mut crate::types::mjCTendon)).spec as *mut crate::types::mjsTendon
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asNumeric (user/user_api.h:343)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_numeric(element: *mut mjsElement) -> *mut mjsNumeric {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCNumeric*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 21 {
            &mut (*(element as *mut crate::types::mjCNumeric)).spec as *mut crate::types::mjsNumeric
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asText (user/user_api.h:346)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_text(element: *mut mjsElement) -> *mut mjsText {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCText*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 22 {
            &mut (*(element as *mut crate::types::mjCText)).spec as *mut crate::types::mjsText
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asTuple (user/user_api.h:349)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_tuple(element: *mut mjsElement) -> *mut mjsTuple {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCTuple*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 23 {
            &mut (*(element as *mut crate::types::mjCTuple)).spec as *mut crate::types::mjsTuple
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asKey (user/user_api.h:352)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_key(element: *mut mjsElement) -> *mut mjsKey {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCKey*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 24 {
            &mut (*(element as *mut crate::types::mjCKey)).spec as *mut crate::types::mjsKey
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asMesh (user/user_api.h:355)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_mesh(element: *mut mjsElement) -> *mut mjsMesh {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCMesh*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 10 {
            &mut (*(element as *mut crate::types::mjCMesh)).spec as *mut crate::types::mjsMesh
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asHField (user/user_api.h:358)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_h_field(element: *mut mjsElement) -> *mut mjsHField {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCHField*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 12 {
            &mut (*(element as *mut crate::types::mjCHField)).spec as *mut crate::types::mjsHField
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asSkin (user/user_api.h:361)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_skin(element: *mut mjsElement) -> *mut mjsSkin {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCSkin*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 11 {
            &mut (*(element as *mut crate::types::mjCSkin)).spec as *mut crate::types::mjsSkin
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asTexture (user/user_api.h:364)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_texture(element: *mut mjsElement) -> *mut mjsTexture {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCTexture*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 13 {
            &mut (*(element as *mut crate::types::mjCTexture)).spec as *mut crate::types::mjsTexture
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asMaterial (user/user_api.h:367)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_material(element: *mut mjsElement) -> *mut mjsMaterial {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCMaterial*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 14 {
            &mut (*(element as *mut crate::types::mjCMaterial)).spec as *mut crate::types::mjsMaterial
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asPlugin (user/user_api.h:370)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_as_plugin(element: *mut mjsElement) -> *mut mjsPlugin {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCPlugin*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 25 {
            &mut (*(element as *mut crate::types::mjCPlugin)).spec as *mut crate::types::mjsPlugin
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_setName (user/user_api.h:376)
/// Calls: mjCModel::CheckRepeat, mjCModel::SetError
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_name(element: *mut mjsElement, name: *const i8) -> i32 {
    todo!() // mjs_setName
}

/// C: mjs_setBuffer (user/user_api.h:379)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_buffer(dest: *mut mjByteVec, array: *const (), size: i32) {
    todo!() // mjs_setBuffer
}

/// C: mjs_setString (user/user_api.h:382)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_string(dest: *mut mjString, text: *const i8) {
    todo!() // mjs_setString
}

/// C: mjs_setStringVec (user/user_api.h:385)
/// Calls: StringToVector
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_string_vec(dest: *mut mjStringVec, text: *const i8) {
    todo!() // mjs_setStringVec
}

/// C: mjs_setInStringVec (user/user_api.h:388)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_in_string_vec(dest: *mut mjStringVec, i: i32, text: *const i8) -> bool {
    todo!() // mjs_setInStringVec
}

/// C: mjs_appendString (user/user_api.h:391)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_append_string(dest: *mut mjStringVec, text: *const i8) {
    todo!() // mjs_appendString
}

/// C: mjs_setInt (user/user_api.h:394)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_int(dest: *mut mjIntVec, array: *const i32, size: i32) {
    todo!() // mjs_setInt
}

/// C: mjs_appendIntVec (user/user_api.h:397)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_append_int_vec(dest: *mut mjIntVecVec, array: *const i32, size: i32) {
    todo!() // mjs_appendIntVec
}

/// C: mjs_setFloat (user/user_api.h:400)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_float(dest: *mut mjFloatVec, array: *const f32, size: i32) {
    todo!() // mjs_setFloat
}

/// C: mjs_appendFloatVec (user/user_api.h:403)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_append_float_vec(dest: *mut mjFloatVecVec, array: *const f32, size: i32) {
    todo!() // mjs_appendFloatVec
}

/// C: mjs_setDouble (user/user_api.h:406)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_double(dest: *mut mjDoubleVec, array: *const f64, size: i32) {
    todo!() // mjs_setDouble
}

/// C: mjs_setPluginAttributes (user/user_api.h:409)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_plugin_attributes(plugin: *mut mjsPlugin, attributes: *mut ()) {
    todo!() // mjs_setPluginAttributes
}

/// C: mjs_getName (user/user_api.h:415)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_name(element: *mut mjsElement) -> *mut mjString {
    // SAFETY: element is a valid mjsElement pointer. Cast mirrors C++ static_cast.
    unsafe {
        let elemtype = *(element as *const i32);
        if elemtype == 101 {
            // mjOBJ_DEFAULT: cast to mjCDef, return &name
            &mut (*(element as *mut mjCDef)).name as *mut std__string as *mut mjString
        } else {
            // all others: cast to mjCBase, return &name
            &mut (*(element as *mut mjCBase)).name as *mut std__string as *mut mjString
        }
    }
}

/// C: mjs_getString (user/user_api.h:418)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_string(source: *const mjString) -> *const i8 {
    todo!() // mjs_getString
}

/// C: mjs_getDouble (user/user_api.h:421)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_double(source: *const mjDoubleVec, size: *mut i32) -> *const f64 {
    todo!() // mjs_getDouble
}

/// C: mjs_getWrapNum (user/user_api.h:424)
/// Calls: mjCTendon::NumWraps
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_wrap_num(tendonspec: *const mjsTendon) -> i32 {
    todo!() // mjs_getWrapNum
}

/// C: mjs_getWrap (user/user_api.h:426)
/// Calls: mjCTendon::GetWrap, mjCTendon::NumWraps, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_wrap(tendonspec: *const mjsTendon, i: i32) -> *mut mjsWrap {
    todo!() // mjs_getWrap
}

/// C: mjs_getPluginAttributes (user/user_api.h:429)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_plugin_attributes(plugin: *const mjsPlugin) -> *const () {
    // SAFETY: plugin is valid; plugin->element points to a mjCPlugin object.
    // Returns &pluginC->config_attribs (pointer to the config_attribs field).
    unsafe {
        let pluginC = (*plugin).element as *const mjCPlugin;
        &(*pluginC).config_attribs as *const _ as *const ()
    }
}

/// C: mjs_isAuthored (user/user_api.h:435)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_is_authored(elem_ptr: *const (), field_ptr: *const ()) -> i32 {
    todo!() // mjs_isAuthored
}

/// C: mjs_setAuthored (user/user_api.h:438)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_authored(elem_ptr: *const (), field_ptr: *const (), authored: i32) {
    todo!() // mjs_setAuthored
}

/// C: mjs_setDefault (user/user_api.h:441)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_default(element: *mut mjsElement, def: *const mjsDefault) {
    todo!() // mjs_setDefault
}

/// C: mjs_setFrame (user/user_api.h:444)
/// Calls: mjCBase::SetFrame, mjCModel::SetError
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_frame(dest: *mut mjsElement, frame: *mut mjsFrame) -> i32 {
    todo!() // mjs_setFrame
}

/// C: mjs_resolveOrientation (user/user_api.h:447)
/// Calls: ResolveOrientation
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_resolve_orientation(quat: *mut f64, degree: u8, sequence: *const i8, orientation: *const mjsOrientation) -> *const i8 {
    crate::user::user_objects::resolve_orientation(quat, degree != 0, sequence, orientation)
}

/// C: mjs_bodyToFrame (user/user_api.h:451)
/// Calls: mjCBody::ToFrame
#[allow(unused_variables, non_snake_case)]
pub fn mjs_body_to_frame(body: *mut *mut mjsBody) -> *mut mjsFrame {
    todo!() // mjs_bodyToFrame
}

/// C: mjs_setUserValue (user/user_api.h:454)
/// Calls: mjs_setUserValueWithCleanup
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_user_value(element: *mut mjsElement, key: *const i8, data: *const ()) {
    todo!() // mjs_setUserValue
}

/// C: mjs_setUserValueWithCleanup (user/user_api.h:457)
/// Calls: mjCBase::SetUserValue
#[allow(unused_variables, non_snake_case)]
pub fn mjs_set_user_value_with_cleanup(element: *mut mjsElement, key: *const i8, data: *const (), cleanup: Option<unsafe extern "C" fn()>) {
    todo!() // mjs_setUserValueWithCleanup
}

/// C: mjs_getUserValue (user/user_api.h:462)
/// Calls: mjCBase::GetUserValue
#[allow(unused_variables, non_snake_case)]
pub fn mjs_get_user_value(element: *mut mjsElement, key: *const i8) -> *const () {
    todo!() // mjs_getUserValue
}

/// C: mjs_deleteUserValue (user/user_api.h:465)
/// Calls: mjCBase::DeleteUserValue
#[allow(unused_variables, non_snake_case)]
pub fn mjs_delete_user_value(element: *mut mjsElement, key: *const i8) {
    todo!() // mjs_deleteUserValue
}

/// C: mjs_sensorDim (user/user_api.h:468)
/// Calls: mjCMesh::nvert, mjCSensor::get_obj, mju_condataSize, mju_raydataSize
#[allow(unused_variables, non_snake_case)]
pub fn mjs_sensor_dim(sensor: *const mjsSensor) -> i32 {
    // SAFETY: sensor is a valid pointer to mjsSensor (caller contract)
    unsafe {
        let sensor_type = u32::from_ne_bytes((*sensor).r#type);

        match sensor_type {
            mjtSensor_mjSENS_TOUCH |
            mjtSensor_mjSENS_JOINTPOS |
            mjtSensor_mjSENS_JOINTVEL |
            mjtSensor_mjSENS_TENDONPOS |
            mjtSensor_mjSENS_TENDONVEL |
            mjtSensor_mjSENS_ACTUATORPOS |
            mjtSensor_mjSENS_ACTUATORVEL |
            mjtSensor_mjSENS_ACTUATORFRC |
            mjtSensor_mjSENS_JOINTACTFRC |
            mjtSensor_mjSENS_TENDONACTFRC |
            mjtSensor_mjSENS_JOINTLIMITPOS |
            mjtSensor_mjSENS_JOINTLIMITVEL |
            mjtSensor_mjSENS_JOINTLIMITFRC |
            mjtSensor_mjSENS_TENDONLIMITPOS |
            mjtSensor_mjSENS_TENDONLIMITVEL |
            mjtSensor_mjSENS_TENDONLIMITFRC |
            mjtSensor_mjSENS_GEOMDIST |
            mjtSensor_mjSENS_INSIDESITE |
            mjtSensor_mjSENS_E_POTENTIAL |
            mjtSensor_mjSENS_E_KINETIC |
            mjtSensor_mjSENS_CLOCK => 1,

            mjtSensor_mjSENS_CAMPROJECTION => 2,

            mjtSensor_mjSENS_ACCELEROMETER |
            mjtSensor_mjSENS_VELOCIMETER |
            mjtSensor_mjSENS_GYRO |
            mjtSensor_mjSENS_FORCE |
            mjtSensor_mjSENS_TORQUE |
            mjtSensor_mjSENS_MAGNETOMETER |
            mjtSensor_mjSENS_BALLANGVEL |
            mjtSensor_mjSENS_FRAMEPOS |
            mjtSensor_mjSENS_FRAMEXAXIS |
            mjtSensor_mjSENS_FRAMEYAXIS |
            mjtSensor_mjSENS_FRAMEZAXIS |
            mjtSensor_mjSENS_FRAMELINVEL |
            mjtSensor_mjSENS_FRAMEANGVEL |
            mjtSensor_mjSENS_FRAMELINACC |
            mjtSensor_mjSENS_FRAMEANGACC |
            mjtSensor_mjSENS_SUBTREECOM |
            mjtSensor_mjSENS_SUBTREELINVEL |
            mjtSensor_mjSENS_SUBTREEANGMOM |
            mjtSensor_mjSENS_GEOMNORMAL => 3,

            mjtSensor_mjSENS_GEOMFROMTO => 6,

            mjtSensor_mjSENS_BALLQUAT |
            mjtSensor_mjSENS_FRAMEQUAT => 4,

            mjtSensor_mjSENS_CONTACT => {
                (*sensor).intprm[2] * crate::engine::engine_support::mju_condata_size((*sensor).intprm[0])
            }

            mjtSensor_mjSENS_TACTILE => {
                todo!("requires C++ virtual call: mjCSensor::get_obj() -> mjCMesh::nvert()")
            }

            mjtSensor_mjSENS_RANGEFINDER => {
                todo!("requires C++ virtual call: mjCSensor::get_obj() + mjCCamera resolution access")
            }

            mjtSensor_mjSENS_USER => (*sensor).dim,

            mjtSensor_mjSENS_PLUGIN => 0,  // to be filled in by plugin

            _ => -1,
        }
    }
}

/// C: mj_getCacheCapacity (user/user_api.h:551)
/// Calls: mjCCache::Capacity
#[allow(unused_variables, non_snake_case)]
pub fn mj_get_cache_capacity(cache: *const mjCache) -> usize {
    todo!() // mj_getCacheCapacity
}

/// C: mj_setCacheCapacity (user/user_api.h:554)
/// Calls: mjCCache::Capacity, mjCCache::SetCapacity
#[allow(unused_variables, non_snake_case)]
pub fn mj_set_cache_capacity(cache: *mut mjCache, size: usize) -> usize {
    todo!() // mj_setCacheCapacity
}

/// C: mj_getCacheSize (user/user_api.h:557)
/// Calls: mjCCache::Size
#[allow(unused_variables, non_snake_case)]
pub fn mj_get_cache_size(cache: *const mjCache) -> usize {
    todo!() // mj_getCacheSize
}

/// C: mj_clearCache (user/user_api.h:560)
/// Calls: mjCCache::Reset
#[allow(unused_variables, non_snake_case)]
pub fn mj_clear_cache(cache: *mut mjCache) {
    todo!() // mj_clearCache
}

/// C: mj_getCache (user/user_api.h:563)
/// Calls: mjCCache::Capacity
#[allow(unused_variables, non_snake_case)]
pub fn mj_get_cache() -> *mut mjCache {
    todo!() // mj_getCache
}

