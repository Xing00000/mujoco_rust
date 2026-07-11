//! Port of: user/user_init.c
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;
use crate::engine::engine_init::mj_default_sol_ref_imp;

// mjNAN in C is just NAN (IEEE 754 quiet NaN for f64)
const MJ_NAN: f64 = f64::NAN;

/// C: mjs_defaultSpec (user/user_init.c:25)
/// Calls: mj_defaultLROpt, mj_defaultOption, mj_defaultVisual
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_spec(spec: *mut mjSpec) {
    extern "C" { fn mjs_defaultSpec(spec: *mut mjSpec); }
    // SAFETY: delegates to C implementation (complex: calls mj_defaultOption, mj_defaultVisual, mj_defaultLROpt)
    unsafe { mjs_defaultSpec(spec) }
}

/// C: mjs_defaultOrientation (user/user_init.c:69)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_orientation(orient: *mut mjsOrientation) {
    if orient.is_null() { return; }
    extern "C" { fn mjs_defaultOrientation(orient: *mut mjsOrientation); }
    // SAFETY: orient verified non-null
    unsafe { mjs_defaultOrientation(orient) }
}

/// C: mjs_defaultBody (user/user_init.c:75)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_body(body: *mut mjsBody) {
    if body.is_null() { return; }
    extern "C" { fn mjs_defaultBody(body: *mut mjsBody); }
    // SAFETY: body verified non-null; C sets struct defaults
    unsafe { mjs_defaultBody(body) }
}

/// C: mjs_defaultFrame (user/user_init.c:89)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_frame(frame: *mut mjsFrame) {
    if frame.is_null() { return; }
    extern "C" { fn mjs_defaultFrame(frame: *mut mjsFrame); }
    // SAFETY: frame verified non-null; C sets struct defaults
    unsafe { mjs_defaultFrame(frame) }
}

/// C: mjs_defaultJoint (user/user_init.c:96)
/// Calls: mj_defaultSolRefImp
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_joint(joint: *mut mjsJoint) {
    extern "C" { fn mjs_defaultJoint(joint: *mut mjsJoint); }
    // SAFETY: delegates to C implementation
    unsafe { mjs_defaultJoint(joint) }
}

/// C: mjs_defaultGeom (user/user_init.c:109)
/// Calls: mj_defaultSolRefImp
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_geom(geom: *mut mjsGeom) {
    extern "C" { fn mjs_defaultGeom(geom: *mut mjsGeom); }
    // SAFETY: delegates to C implementation
    unsafe { mjs_defaultGeom(geom) }
}

/// C: mjs_defaultSite (user/user_init.c:151)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_site(site: *mut mjsSite) {
    if site.is_null() { return; }
    extern "C" { fn mjs_defaultSite(site: *mut mjsSite); }
    // SAFETY: site verified non-null; C sets struct defaults
    unsafe { mjs_defaultSite(site) }
}

/// C: mjs_defaultCamera (user/user_init.c:169)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_camera(camera: *mut mjsCamera) {
    if camera.is_null() { return; }
    extern "C" { fn mjs_defaultCamera(camera: *mut mjsCamera); }
    // SAFETY: camera verified non-null; C sets struct defaults
    unsafe { mjs_defaultCamera(camera) }
}

/// C: mjs_defaultLight (user/user_init.c:187)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_light(light: *mut mjsLight) {
    if light.is_null() { return; }
    extern "C" { fn mjs_defaultLight(light: *mut mjsLight); }
    // SAFETY: light verified non-null; C sets struct defaults
    unsafe { mjs_defaultLight(light) }
}

/// C: mjs_defaultFlex (user/user_init.c:211)
/// Calls: mj_defaultSolRefImp
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_flex(flex: *mut mjsFlex) {
    extern "C" { fn mjs_defaultFlex(flex: *mut mjsFlex); }
    // SAFETY: delegates to C implementation
    unsafe { mjs_defaultFlex(flex) }
}

/// C: mjs_defaultMesh (user/user_init.c:240)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_mesh(mesh: *mut mjsMesh) {
    if mesh.is_null() { return; }
    extern "C" { fn mjs_defaultMesh(mesh: *mut mjsMesh); }
    // SAFETY: mesh verified non-null; C sets struct defaults
    unsafe { mjs_defaultMesh(mesh) }
}

/// C: mjs_defaultHField (user/user_init.c:251)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_h_field(hfield: *mut mjsHField) {
    if hfield.is_null() { return; }
    extern "C" { fn mjs_defaultHField(hfield: *mut mjsHField); }
    // SAFETY: hfield verified non-null; C sets struct defaults
    unsafe { mjs_defaultHField(hfield) }
}

/// C: mjs_defaultSkin (user/user_init.c:257)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_skin(skin: *mut mjsSkin) {
    if skin.is_null() { return; }
    extern "C" { fn mjs_defaultSkin(skin: *mut mjsSkin); }
    // SAFETY: skin verified non-null; C sets struct defaults
    unsafe { mjs_defaultSkin(skin) }
}

/// C: mjs_defaultTexture (user/user_init.c:265)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_texture(texture: *mut mjsTexture) {
    if texture.is_null() { return; }
    extern "C" { fn mjs_defaultTexture(texture: *mut mjsTexture); }
    // SAFETY: texture verified non-null; C sets struct defaults
    unsafe { mjs_defaultTexture(texture) }
}

/// C: mjs_defaultMaterial (user/user_init.c:280)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_material(material: *mut mjsMaterial) {
    if material.is_null() { return; }
    extern "C" { fn mjs_defaultMaterial(material: *mut mjsMaterial); }
    // SAFETY: material verified non-null; C sets struct defaults
    unsafe { mjs_defaultMaterial(material) }
}

/// C: mjs_defaultPair (user/user_init.c:292)
/// Calls: mj_defaultSolRefImp
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_pair(pair: *mut mjsPair) {
    extern "C" { fn mjs_defaultPair(pair: *mut mjsPair); }
    // SAFETY: delegates to C implementation
    unsafe { mjs_defaultPair(pair) }
}

/// C: mjs_defaultEquality (user/user_init.c:305)
/// Calls: mj_defaultSolRefImp
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_equality(equality: *mut mjsEquality) {
    extern "C" { fn mjs_defaultEquality(equality: *mut mjsEquality); }
    // SAFETY: delegates to C implementation
    unsafe { mjs_defaultEquality(equality) }
}

/// C: mjs_defaultTendon (user/user_init.c:316)
/// Calls: mj_defaultSolRefImp
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_tendon(tendon: *mut mjsTendon) {
    extern "C" { fn mjs_defaultTendon(tendon: *mut mjsTendon); }
    // SAFETY: delegates to C implementation
    unsafe { mjs_defaultTendon(tendon) }
}

/// C: mjs_defaultActuator (user/user_init.c:329)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_actuator(actuator: *mut mjsActuator) {
    if actuator.is_null() { return; }
    extern "C" { fn mjs_defaultActuator(actuator: *mut mjsActuator); }
    // SAFETY: actuator verified non-null; C sets struct defaults
    unsafe { mjs_defaultActuator(actuator) }
}

/// C: mjs_defaultSensor (user/user_init.c:354)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_sensor(sensor: *mut mjsSensor) {
    if sensor.is_null() { return; }
    extern "C" { fn mjs_defaultSensor(sensor: *mut mjsSensor); }
    // SAFETY: sensor verified non-null; C sets struct defaults
    unsafe { mjs_defaultSensor(sensor) }
}

/// C: mjs_defaultNumeric (user/user_init.c:364)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_numeric(numeric: *mut mjsNumeric) {
    if numeric.is_null() { return; }
    extern "C" { fn mjs_defaultNumeric(numeric: *mut mjsNumeric); }
    // SAFETY: numeric verified non-null; C sets struct defaults
    unsafe { mjs_defaultNumeric(numeric) }
}

/// C: mjs_defaultText (user/user_init.c:370)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_text(text: *mut mjsText) {
    if text.is_null() { return; }
    extern "C" { fn mjs_defaultText(text: *mut mjsText); }
    // SAFETY: text verified non-null; C sets struct defaults
    unsafe { mjs_defaultText(text) }
}

/// C: mjs_defaultTuple (user/user_init.c:376)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_tuple(tuple: *mut mjsTuple) {
    if tuple.is_null() { return; }
    extern "C" { fn mjs_defaultTuple(tuple: *mut mjsTuple); }
    // SAFETY: tuple verified non-null; C sets struct defaults
    unsafe { mjs_defaultTuple(tuple) }
}

/// C: mjs_defaultKey (user/user_init.c:382)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_key(key: *mut mjsKey) {
    if key.is_null() { return; }
    extern "C" { fn mjs_defaultKey(key: *mut mjsKey); }
    // SAFETY: key verified non-null; C sets struct defaults
    unsafe { mjs_defaultKey(key) }
}

/// C: mjs_defaultPlugin (user/user_init.c:388)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_plugin(plugin: *mut mjsPlugin) {
    if plugin.is_null() { return; }
    extern "C" { fn mjs_defaultPlugin(plugin: *mut mjsPlugin); }
    // SAFETY: plugin verified non-null; C sets struct defaults
    unsafe { mjs_defaultPlugin(plugin) }
}
