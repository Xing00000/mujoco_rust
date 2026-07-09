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
    // SAFETY: caller guarantees orient is a valid, aligned pointer to mjsOrientation.
    // write_bytes zeros the entire struct (equivalent to C memset).
    unsafe {
        core::ptr::write_bytes(orient, 0, 1);
    }
}

/// C: mjs_defaultBody (user/user_init.c:75)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_body(body: *mut mjsBody) {
    // SAFETY: caller guarantees body is a valid, aligned pointer to mjsBody.
    // write_bytes zeros the entire struct, then we set non-zero fields.
    unsafe {
        core::ptr::write_bytes(body, 0, 1);

        // body frame
        (*body).quat[0] = 1.0;

        // inertial frame
        (*body).ipos[0] = MJ_NAN;
        (*body).iquat[0] = 1.0;
        (*body).fullinertia[0] = MJ_NAN;
    }
}

/// C: mjs_defaultFrame (user/user_init.c:89)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_frame(frame: *mut mjsFrame) {
    // SAFETY: caller guarantees frame is a valid, aligned pointer to mjsFrame.
    unsafe {
        core::ptr::write_bytes(frame, 0, 1);
        (*frame).quat[0] = 1.0;
    }
}

/// C: mjs_defaultJoint (user/user_init.c:96)
/// Calls: mj_defaultSolRefImp
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_joint(joint: *mut mjsJoint) {
    // SAFETY: caller guarantees joint is a valid, aligned pointer to mjsJoint.
    // NOTE: joint.type (mjtJoint) is ZST in Rust types — C sets it to mjJNT_HINGE (3).
    unsafe {
        core::ptr::write_bytes(joint, 0, 1);

        // type: mjJNT_HINGE = 3 (ZST field — cannot set via Rust field access)
        (*joint).axis[2] = 1.0;
        (*joint).limited = 2;          // mjLIMITED_AUTO
        (*joint).actfrclimited = 2;    // mjLIMITED_AUTO
        (*joint).align = 2;            // mjALIGNFREE_AUTO

        mj_default_sol_ref_imp(
            (*joint).solref_limit.as_mut_ptr(),
            (*joint).solimp_limit.as_mut_ptr(),
        );
        mj_default_sol_ref_imp(
            (*joint).solref_friction.as_mut_ptr(),
            (*joint).solimp_friction.as_mut_ptr(),
        );
    }
}

/// C: mjs_defaultGeom (user/user_init.c:109)
/// Calls: mj_defaultSolRefImp
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_geom(geom: *mut mjsGeom) {
    // SAFETY: caller guarantees geom is a valid, aligned pointer to mjsGeom.
    // NOTE: geom.type (mjtGeom) is ZST — C sets it to mjGEOM_SPHERE (2).
    // NOTE: geom.typeinertia (mjtGeomInertia) is ZST — C sets it to mjINERTIA_VOLUME (0, already zero).
    unsafe {
        core::ptr::write_bytes(geom, 0, 1);

        // frame
        (*geom).quat[0] = 1.0;
        (*geom).fromto[0] = MJ_NAN;

        // contact-related
        (*geom).contype = 1;
        (*geom).conaffinity = 1;
        (*geom).condim = 3;
        (*geom).friction[0] = 1.0;
        (*geom).friction[1] = 0.005;
        (*geom).friction[2] = 0.0001;
        (*geom).solmix = 1.0;
        mj_default_sol_ref_imp(
            (*geom).solref.as_mut_ptr(),
            (*geom).solimp.as_mut_ptr(),
        );

        // inertia-related
        (*geom).mass = MJ_NAN;
        (*geom).density = 1000.0;  // water density (1000 Kg / m^3)

        // color
        (*geom).rgba[0] = 0.5;
        (*geom).rgba[1] = 0.5;
        (*geom).rgba[2] = 0.5;
        (*geom).rgba[3] = 1.0;

        // fluid forces
        (*geom).fluid_coefs[0] = 0.5;   // blunt drag
        (*geom).fluid_coefs[1] = 0.25;  // slender drag
        (*geom).fluid_coefs[2] = 1.5;   // angular drag
        (*geom).fluid_coefs[3] = 1.0;   // kutta lift
        (*geom).fluid_coefs[4] = 1.0;   // magnus lift

        // other
        (*geom).fitscale = 1.0;
    }
}

/// C: mjs_defaultSite (user/user_init.c:151)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_site(site: *mut mjsSite) {
    // SAFETY: caller guarantees site is a valid, aligned pointer to mjsSite.
    // NOTE: site.type (mjtGeom) is ZST — C sets it to mjGEOM_SPHERE (2).
    unsafe {
        core::ptr::write_bytes(site, 0, 1);

        // frame
        (*site).quat[0] = 1.0;
        (*site).size[0] = 0.005;
        (*site).size[1] = 0.005;
        (*site).size[2] = 0.005;
        (*site).fromto[0] = MJ_NAN;

        // color
        (*site).rgba[0] = 0.5;
        (*site).rgba[1] = 0.5;
        (*site).rgba[2] = 0.5;
        (*site).rgba[3] = 1.0;
    }
}

/// C: mjs_defaultCamera (user/user_init.c:169)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_camera(camera: *mut mjsCamera) {
    // SAFETY: caller guarantees camera is a valid, aligned pointer to mjsCamera.
    // NOTE: camera.mode (mjtCamLight) is ZST — C sets it to mjCAMLIGHT_FIXED (0, already zero).
    // NOTE: camera.proj (mjtProjection) is ZST — C leaves it at 0 (default).
    unsafe {
        core::ptr::write_bytes(camera, 0, 1);

        // extrinsics
        (*camera).quat[0] = 1.0;

        // intrinsics
        (*camera).fovy = 45.0;
        (*camera).ipd = 0.068;
        (*camera).resolution[0] = 1;
        (*camera).resolution[1] = 1;
        (*camera).output = 1;  // mjCAMOUT_RGB
    }
}

/// C: mjs_defaultLight (user/user_init.c:187)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_light(light: *mut mjsLight) {
    // SAFETY: caller guarantees light is a valid, aligned pointer to mjsLight.
    // NOTE: light.mode (mjtCamLight) is ZST — C sets it to mjCAMLIGHT_FIXED (0, already zero).
    // NOTE: light.type (mjtLightType) is ZST — C leaves it at 0 (default).
    unsafe {
        core::ptr::write_bytes(light, 0, 1);

        // extrinsics
        (*light).dir[2] = -1.0;

        // intrinsics
        (*light).castshadow = 1;
        (*light).bulbradius = 0.02;
        (*light).intensity = 0.0;
        (*light).range = 10.0;
        (*light).active = 1;
        (*light).attenuation[0] = 1.0;
        (*light).cutoff = 45.0;
        (*light).exponent = 10.0;
        (*light).diffuse[0] = 0.7;
        (*light).diffuse[1] = 0.7;
        (*light).diffuse[2] = 0.7;
        (*light).specular[0] = 0.3;
        (*light).specular[1] = 0.3;
        (*light).specular[2] = 0.3;
    }
}

/// C: mjs_defaultFlex (user/user_init.c:211)
/// Calls: mj_defaultSolRefImp
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_flex(flex: *mut mjsFlex) {
    // SAFETY: caller guarantees flex is a valid, aligned pointer to mjsFlex.
    unsafe {
        core::ptr::write_bytes(flex, 0, 1);

        // contact defaults
        (*flex).contype = 1;
        (*flex).conaffinity = 1;
        (*flex).condim = 3;
        (*flex).friction[0] = 1.0;
        (*flex).friction[1] = 0.005;
        (*flex).friction[2] = 0.0001;
        (*flex).solmix = 1.0;
        mj_default_sol_ref_imp(
            (*flex).solref.as_mut_ptr(),
            (*flex).solimp.as_mut_ptr(),
        );

        // other defaults
        (*flex).dim = 2;
        (*flex).radius = 0.005;
        (*flex).cellcount[0] = 1;
        (*flex).cellcount[1] = 1;
        (*flex).cellcount[2] = 1;
        (*flex).internal = 0;
        (*flex).selfcollide = 4;   // mjFLEXSELF_AUTO
        (*flex).activelayers = 1;
        (*flex).rgba[0] = 0.5;
        (*flex).rgba[1] = 0.5;
        (*flex).rgba[2] = 0.5;
        (*flex).rgba[3] = 1.0;
        (*flex).thickness = -1.0;
    }
}

/// C: mjs_defaultMesh (user/user_init.c:240)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_mesh(mesh: *mut mjsMesh) {
    // SAFETY: caller guarantees mesh is a valid, aligned pointer to mjsMesh.
    // NOTE: mesh.inertia (mjtMeshInertia) is ZST — C sets it to mjMESH_INERTIA_LEGACY (2).
    unsafe {
        core::ptr::write_bytes(mesh, 0, 1);
        (*mesh).refquat[0] = 1.0;
        (*mesh).scale[0] = 1.0;
        (*mesh).scale[1] = 1.0;
        (*mesh).scale[2] = 1.0;
        (*mesh).maxhullvert = -1;
        (*mesh).octree_maxdepth = 6;
    }
}

/// C: mjs_defaultHField (user/user_init.c:251)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_h_field(hfield: *mut mjsHField) {
    // SAFETY: caller guarantees hfield is a valid, aligned pointer to mjsHField.
    unsafe {
        core::ptr::write_bytes(hfield, 0, 1);
    }
}

/// C: mjs_defaultSkin (user/user_init.c:257)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_skin(skin: *mut mjsSkin) {
    // SAFETY: caller guarantees skin is a valid, aligned pointer to mjsSkin.
    unsafe {
        core::ptr::write_bytes(skin, 0, 1);
        (*skin).rgba[0] = 0.5;
        (*skin).rgba[1] = 0.5;
        (*skin).rgba[2] = 0.5;
        (*skin).rgba[3] = 1.0;
    }
}

/// C: mjs_defaultTexture (user/user_init.c:265)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_texture(texture: *mut mjsTexture) {
    // SAFETY: caller guarantees texture is a valid, aligned pointer to mjsTexture.
    // NOTE: texture.type (mjtTexture) is ZST — C sets it to mjTEXTURE_CUBE (1).
    // NOTE: texture.colorspace (mjtColorSpace) is ZST — C sets it to mjCOLORSPACE_AUTO (0, already zero).
    unsafe {
        core::ptr::write_bytes(texture, 0, 1);

        (*texture).builtin = 0;
        (*texture).rgb1[0] = 0.8;
        (*texture).rgb1[1] = 0.8;
        (*texture).rgb1[2] = 0.8;
        (*texture).rgb2[0] = 0.5;
        (*texture).rgb2[1] = 0.5;
        (*texture).rgb2[2] = 0.5;
        (*texture).random = 0.01;
        (*texture).gridsize[0] = 1;
        (*texture).gridsize[1] = 1;
        (*texture).nchannel = 3;

        // gridlayout = "............" (12 bytes of '.')
        let layout = b"............";
        let dst = (*texture).gridlayout.as_mut_ptr() as *mut u8;
        core::ptr::copy_nonoverlapping(layout.as_ptr(), dst, 12);
    }
}

/// C: mjs_defaultMaterial (user/user_init.c:280)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_material(material: *mut mjsMaterial) {
    // SAFETY: caller guarantees material is a valid, aligned pointer to mjsMaterial.
    unsafe {
        core::ptr::write_bytes(material, 0, 1);
        (*material).texrepeat[0] = 1.0;
        (*material).texrepeat[1] = 1.0;
        (*material).specular = 0.5;
        (*material).shininess = 0.5;
        (*material).metallic = -1.0;
        (*material).roughness = -1.0;
        (*material).rgba[0] = 1.0;
        (*material).rgba[1] = 1.0;
        (*material).rgba[2] = 1.0;
        (*material).rgba[3] = 1.0;
    }
}

/// C: mjs_defaultPair (user/user_init.c:292)
/// Calls: mj_defaultSolRefImp
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_pair(pair: *mut mjsPair) {
    // SAFETY: caller guarantees pair is a valid, aligned pointer to mjsPair.
    unsafe {
        core::ptr::write_bytes(pair, 0, 1);
        (*pair).condim = 3;
        mj_default_sol_ref_imp(
            (*pair).solref.as_mut_ptr(),
            (*pair).solimp.as_mut_ptr(),
        );
        (*pair).friction[0] = 1.0;
        (*pair).friction[1] = 1.0;
        (*pair).friction[2] = 0.005;
        (*pair).friction[3] = 0.0001;
        (*pair).friction[4] = 0.0001;
    }
}

/// C: mjs_defaultEquality (user/user_init.c:305)
/// Calls: mj_defaultSolRefImp
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_equality(equality: *mut mjsEquality) {
    // SAFETY: caller guarantees equality is a valid, aligned pointer to mjsEquality.
    // NOTE: equality.type (mjtEq) is ZST — C sets it to mjEQ_CONNECT (0, already zero).
    unsafe {
        core::ptr::write_bytes(equality, 0, 1);
        (*equality).active = 1;
        mj_default_sol_ref_imp(
            (*equality).solref.as_mut_ptr(),
            (*equality).solimp.as_mut_ptr(),
        );
        (*equality).data[1] = 1.0;
        (*equality).data[10] = 1.0;  // torque:force ratio
    }
}

/// C: mjs_defaultTendon (user/user_init.c:316)
/// Calls: mj_defaultSolRefImp
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_tendon(tendon: *mut mjsTendon) {
    // SAFETY: caller guarantees tendon is a valid, aligned pointer to mjsTendon.
    unsafe {
        core::ptr::write_bytes(tendon, 0, 1);
        (*tendon).limited = 2;  // mjLIMITED_AUTO
        (*tendon).springlength[0] = -1.0;
        (*tendon).springlength[1] = -1.0;
        mj_default_sol_ref_imp(
            (*tendon).solref_limit.as_mut_ptr(),
            (*tendon).solimp_limit.as_mut_ptr(),
        );
        mj_default_sol_ref_imp(
            (*tendon).solref_friction.as_mut_ptr(),
            (*tendon).solimp_friction.as_mut_ptr(),
        );
        (*tendon).width = 0.003;
        (*tendon).rgba[0] = 0.5;
        (*tendon).rgba[1] = 0.5;
        (*tendon).rgba[2] = 0.5;
        (*tendon).rgba[3] = 1.0;
    }
}

/// C: mjs_defaultActuator (user/user_init.c:329)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_actuator(actuator: *mut mjsActuator) {
    // SAFETY: caller guarantees actuator is a valid, aligned pointer to mjsActuator.
    // NOTE: actuator.gaintype (mjtGain) is ZST — C sets to mjGAIN_FIXED (0, already zero).
    // NOTE: actuator.biastype (mjtBias) is ZST — C sets to mjBIAS_NONE (0, already zero).
    // NOTE: actuator.dyntype (mjtDyn) is ZST — C sets to mjDYN_NONE (0, already zero).
    // NOTE: actuator.trntype (mjtTrn) is ZST — C sets to mjTRN_UNDEFINED (1000, CANNOT SET).
    unsafe {
        core::ptr::write_bytes(actuator, 0, 1);

        // gain, bias
        (*actuator).gainprm[0] = 1.0;

        // activation state
        (*actuator).dynprm[0] = 1.0;
        (*actuator).actdim = -1;

        // transmission
        (*actuator).gear[0] = 1.0;

        // input/output clamping
        (*actuator).ctrllimited = 2;   // mjLIMITED_AUTO
        (*actuator).forcelimited = 2;  // mjLIMITED_AUTO
        (*actuator).actlimited = 2;    // mjLIMITED_AUTO
    }
}

/// C: mjs_defaultSensor (user/user_init.c:354)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_sensor(sensor: *mut mjsSensor) {
    // SAFETY: caller guarantees sensor is a valid, aligned pointer to mjsSensor.
    // NOTE: sensor.type (mjtSensor) is ZST — C sets to mjSENS_TOUCH (0, already zero).
    // NOTE: sensor.datatype (mjtDataType) is ZST — C sets to mjDATATYPE_REAL (0, already zero).
    // NOTE: sensor.needstage (mjtStage) is ZST — C sets to mjSTAGE_ACC (3, CANNOT SET).
    unsafe {
        core::ptr::write_bytes(sensor, 0, 1);
    }
}

/// C: mjs_defaultNumeric (user/user_init.c:364)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_numeric(numeric: *mut mjsNumeric) {
    // SAFETY: caller guarantees numeric is a valid, aligned pointer to mjsNumeric.
    unsafe {
        core::ptr::write_bytes(numeric, 0, 1);
    }
}

/// C: mjs_defaultText (user/user_init.c:370)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_text(text: *mut mjsText) {
    // SAFETY: caller guarantees text is a valid, aligned pointer to mjsText.
    unsafe {
        core::ptr::write_bytes(text, 0, 1);
    }
}

/// C: mjs_defaultTuple (user/user_init.c:376)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_tuple(tuple: *mut mjsTuple) {
    // SAFETY: caller guarantees tuple is a valid, aligned pointer to mjsTuple.
    unsafe {
        core::ptr::write_bytes(tuple, 0, 1);
    }
}

/// C: mjs_defaultKey (user/user_init.c:382)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_key(key: *mut mjsKey) {
    // SAFETY: caller guarantees key is a valid, aligned pointer to mjsKey.
    unsafe {
        core::ptr::write_bytes(key, 0, 1);
    }
}

/// C: mjs_defaultPlugin (user/user_init.c:388)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_plugin(plugin: *mut mjsPlugin) {
    // SAFETY: caller guarantees plugin is a valid, aligned pointer to mjsPlugin.
    unsafe {
        core::ptr::write_bytes(plugin, 0, 1);
    }
}
