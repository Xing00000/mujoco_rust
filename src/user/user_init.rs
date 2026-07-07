//! Port of: user/user_init.c
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mjs_defaultSpec (user/user_init.c:25)
/// Calls: mj_defaultLROpt, mj_defaultOption, mj_defaultVisual
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_spec(spec: *mut mjSpec) {
    extern "C" { fn mjs_defaultSpec_impl(spec: *mut mjSpec); }
    // SAFETY: delegates to C implementation
    unsafe { mjs_defaultSpec_impl(spec) }
}

/// C: mjs_defaultOrientation (user/user_init.c:69)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_orientation(orient: *mut mjsOrientation) {
    extern "C" { fn mjs_defaultOrientation_impl(orient: *mut mjsOrientation); }
    // SAFETY: delegates to C implementation
    unsafe { mjs_defaultOrientation_impl(orient) }
}

/// C: mjs_defaultBody (user/user_init.c:75)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_body(body: *mut mjsBody) {
    extern "C" { fn mjs_defaultBody_impl(body: *mut mjsBody); }
    // SAFETY: delegates to C implementation
    unsafe { mjs_defaultBody_impl(body) }
}

/// C: mjs_defaultFrame (user/user_init.c:89)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_frame(frame: *mut mjsFrame) {
    extern "C" { fn mjs_defaultFrame_impl(frame: *mut mjsFrame); }
    // SAFETY: delegates to C implementation
    unsafe { mjs_defaultFrame_impl(frame) }
}

/// C: mjs_defaultJoint (user/user_init.c:96)
/// Calls: mj_defaultSolRefImp
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_joint(joint: *mut mjsJoint) {
    extern "C" { fn mjs_defaultJoint_impl(joint: *mut mjsJoint); }
    // SAFETY: delegates to C implementation
    unsafe { mjs_defaultJoint_impl(joint) }
}

/// C: mjs_defaultGeom (user/user_init.c:109)
/// Calls: mj_defaultSolRefImp
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_geom(geom: *mut mjsGeom) {
    extern "C" { fn mjs_defaultGeom_impl(geom: *mut mjsGeom); }
    // SAFETY: delegates to C implementation
    unsafe { mjs_defaultGeom_impl(geom) }
}

/// C: mjs_defaultSite (user/user_init.c:151)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_site(site: *mut mjsSite) {
    extern "C" { fn mjs_defaultSite_impl(site: *mut mjsSite); }
    // SAFETY: delegates to C implementation
    unsafe { mjs_defaultSite_impl(site) }
}

/// C: mjs_defaultCamera (user/user_init.c:169)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_camera(camera: *mut mjsCamera) {
    extern "C" { fn mjs_defaultCamera_impl(camera: *mut mjsCamera); }
    // SAFETY: delegates to C implementation
    unsafe { mjs_defaultCamera_impl(camera) }
}

/// C: mjs_defaultLight (user/user_init.c:187)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_light(light: *mut mjsLight) {
    extern "C" { fn mjs_defaultLight_impl(light: *mut mjsLight); }
    // SAFETY: delegates to C implementation
    unsafe { mjs_defaultLight_impl(light) }
}

/// C: mjs_defaultFlex (user/user_init.c:211)
/// Calls: mj_defaultSolRefImp
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_flex(flex: *mut mjsFlex) {
    extern "C" { fn mjs_defaultFlex_impl(flex: *mut mjsFlex); }
    // SAFETY: delegates to C implementation
    unsafe { mjs_defaultFlex_impl(flex) }
}

/// C: mjs_defaultMesh (user/user_init.c:240)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_mesh(mesh: *mut mjsMesh) {
    extern "C" { fn mjs_defaultMesh_impl(mesh: *mut mjsMesh); }
    // SAFETY: delegates to C implementation
    unsafe { mjs_defaultMesh_impl(mesh) }
}

/// C: mjs_defaultHField (user/user_init.c:251)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_h_field(hfield: *mut mjsHField) {
    extern "C" { fn mjs_defaultHField_impl(hfield: *mut mjsHField); }
    // SAFETY: delegates to C implementation
    unsafe { mjs_defaultHField_impl(hfield) }
}

/// C: mjs_defaultSkin (user/user_init.c:257)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_skin(skin: *mut mjsSkin) {
    extern "C" { fn mjs_defaultSkin_impl(skin: *mut mjsSkin); }
    // SAFETY: delegates to C implementation
    unsafe { mjs_defaultSkin_impl(skin) }
}

/// C: mjs_defaultTexture (user/user_init.c:265)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_texture(texture: *mut mjsTexture) {
    extern "C" { fn mjs_defaultTexture_impl(texture: *mut mjsTexture); }
    // SAFETY: delegates to C implementation
    unsafe { mjs_defaultTexture_impl(texture) }
}

/// C: mjs_defaultMaterial (user/user_init.c:280)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_material(material: *mut mjsMaterial) {
    extern "C" { fn mjs_defaultMaterial_impl(material: *mut mjsMaterial); }
    // SAFETY: delegates to C implementation
    unsafe { mjs_defaultMaterial_impl(material) }
}

/// C: mjs_defaultPair (user/user_init.c:292)
/// Calls: mj_defaultSolRefImp
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_pair(pair: *mut mjsPair) {
    extern "C" { fn mjs_defaultPair_impl(pair: *mut mjsPair); }
    // SAFETY: delegates to C implementation
    unsafe { mjs_defaultPair_impl(pair) }
}

/// C: mjs_defaultEquality (user/user_init.c:305)
/// Calls: mj_defaultSolRefImp
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_equality(equality: *mut mjsEquality) {
    extern "C" { fn mjs_defaultEquality_impl(equality: *mut mjsEquality); }
    // SAFETY: delegates to C implementation
    unsafe { mjs_defaultEquality_impl(equality) }
}

/// C: mjs_defaultTendon (user/user_init.c:316)
/// Calls: mj_defaultSolRefImp
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_tendon(tendon: *mut mjsTendon) {
    extern "C" { fn mjs_defaultTendon_impl(tendon: *mut mjsTendon); }
    // SAFETY: delegates to C implementation
    unsafe { mjs_defaultTendon_impl(tendon) }
}

/// C: mjs_defaultActuator (user/user_init.c:329)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_actuator(actuator: *mut mjsActuator) {
    extern "C" { fn mjs_defaultActuator_impl(actuator: *mut mjsActuator); }
    // SAFETY: delegates to C implementation
    unsafe { mjs_defaultActuator_impl(actuator) }
}

/// C: mjs_defaultSensor (user/user_init.c:354)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_sensor(sensor: *mut mjsSensor) {
    extern "C" { fn mjs_defaultSensor_impl(sensor: *mut mjsSensor); }
    // SAFETY: delegates to C implementation
    unsafe { mjs_defaultSensor_impl(sensor) }
}

/// C: mjs_defaultNumeric (user/user_init.c:364)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_numeric(numeric: *mut mjsNumeric) {
    extern "C" { fn mjs_defaultNumeric_impl(numeric: *mut mjsNumeric); }
    // SAFETY: delegates to C implementation
    unsafe { mjs_defaultNumeric_impl(numeric) }
}

/// C: mjs_defaultText (user/user_init.c:370)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_text(text: *mut mjsText) {
    extern "C" { fn mjs_defaultText_impl(text: *mut mjsText); }
    // SAFETY: delegates to C implementation
    unsafe { mjs_defaultText_impl(text) }
}

/// C: mjs_defaultTuple (user/user_init.c:376)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_tuple(tuple: *mut mjsTuple) {
    extern "C" { fn mjs_defaultTuple_impl(tuple: *mut mjsTuple); }
    // SAFETY: delegates to C implementation
    unsafe { mjs_defaultTuple_impl(tuple) }
}

/// C: mjs_defaultKey (user/user_init.c:382)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_key(key: *mut mjsKey) {
    extern "C" { fn mjs_defaultKey_impl(key: *mut mjsKey); }
    // SAFETY: delegates to C implementation
    unsafe { mjs_defaultKey_impl(key) }
}

/// C: mjs_defaultPlugin (user/user_init.c:388)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_plugin(plugin: *mut mjsPlugin) {
    extern "C" { fn mjs_defaultPlugin_impl(plugin: *mut mjsPlugin); }
    // SAFETY: delegates to C implementation
    unsafe { mjs_defaultPlugin_impl(plugin) }
}

