//! Port of: user/user_init.c
//! IR hash: 47ee20b2bff3660e
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mjs_defaultSpec (user/user_init.c:25)
/// Calls: mj_defaultLROpt, mj_defaultOption, mj_defaultVisual
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_spec(spec: *mut mjSpec) {
    todo!() // mjs_defaultSpec
}

/// C: mjs_defaultOrientation (user/user_init.c:69)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_orientation(orient: *mut mjsOrientation) {
    todo!() // mjs_defaultOrientation
}

/// C: mjs_defaultBody (user/user_init.c:75)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_body(body: *mut mjsBody) {
    // SAFETY: caller guarantees body is a valid, aligned, writable pointer to mjsBody
    unsafe {
        std::ptr::write_bytes(body as *mut u8, 0, std::mem::size_of::<mjsBody>());

        // body frame
        (*body).quat[0] = 1.0;

        // inertial frame
        (*body).ipos[0] = f64::NAN;
        (*body).iquat[0] = 1.0;
        (*body).fullinertia[0] = f64::NAN;
    }
}

/// C: mjs_defaultFrame (user/user_init.c:89)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_frame(frame: *mut mjsFrame) {
    // SAFETY: caller guarantees frame is a valid, aligned, writable pointer to mjsFrame
    unsafe {
        std::ptr::write_bytes(frame as *mut u8, 0, std::mem::size_of::<mjsFrame>());
        (*frame).quat[0] = 1.0;
    }
}

/// C: mjs_defaultJoint (user/user_init.c:96)
/// Calls: mj_defaultSolRefImp
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_joint(joint: *mut mjsJoint) {
    // SAFETY: caller guarantees joint is a valid, aligned, writable pointer to mjsJoint
    unsafe {
        std::ptr::write_bytes(joint as *mut u8, 0, std::mem::size_of::<mjsJoint>());

        // type field is [u8; 8] storing i32 in first 4 bytes
        std::ptr::write((*joint).r#type.as_mut_ptr() as *mut i32, 3); // mjJNT_HINGE = 3
        (*joint).axis[2] = 1.0;
        (*joint).limited = 2; // mjLIMITED_AUTO
        (*joint).actfrclimited = 2; // mjLIMITED_AUTO
        (*joint).align = 2; // mjALIGNFREE_AUTO

        crate::engine::engine_init::mj_default_sol_ref_imp(
            (*joint).solref_limit.as_mut_ptr(),
            (*joint).solimp_limit.as_mut_ptr(),
        );
        crate::engine::engine_init::mj_default_sol_ref_imp(
            (*joint).solref_friction.as_mut_ptr(),
            (*joint).solimp_friction.as_mut_ptr(),
        );
    }
}

/// C: mjs_defaultGeom (user/user_init.c:109)
/// Calls: mj_defaultSolRefImp
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_geom(geom: *mut mjsGeom) {
    // SAFETY: caller guarantees geom is a valid, aligned, writable pointer to mjsGeom
    unsafe {
        std::ptr::write_bytes(geom as *mut u8, 0, std::mem::size_of::<mjsGeom>());

        // type: mjGEOM_SPHERE = 2
        std::ptr::write((*geom).r#type.as_mut_ptr() as *mut i32, 2);

        // frame
        (*geom).quat[0] = 1.0;
        (*geom).fromto[0] = f64::NAN;

        // contact-related
        (*geom).contype = 1;
        (*geom).conaffinity = 1;
        (*geom).condim = 3;
        (*geom).friction[0] = 1.0;
        (*geom).friction[1] = 0.005;
        (*geom).friction[2] = 0.0001;
        (*geom).solmix = 1.0;
        crate::engine::engine_init::mj_default_sol_ref_imp(
            (*geom).solref.as_mut_ptr(),
            (*geom).solimp.as_mut_ptr(),
        );

        // inertia-related
        (*geom).mass = f64::NAN;
        (*geom).density = 1000.0; // water density (1000 Kg / m^3)
        // typeinertia: mjINERTIA_VOLUME = 0 (already zero from memset)

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
    // SAFETY: caller guarantees site is a valid, aligned, writable pointer to mjsSite
    unsafe {
        std::ptr::write_bytes(site as *mut u8, 0, std::mem::size_of::<mjsSite>());

        // type: mjGEOM_SPHERE = 2
        std::ptr::write((*site).r#type.as_mut_ptr() as *mut i32, 2);

        // frame
        (*site).quat[0] = 1.0;
        (*site).size[0] = 0.005;
        (*site).size[1] = 0.005;
        (*site).size[2] = 0.005;
        (*site).fromto[0] = f64::NAN;

        // color: rgba is [u8; 20] blob containing float[4] in first 16 bytes
        let rgba_ptr = (*site).rgba.as_mut_ptr() as *mut f32;
        *rgba_ptr.add(0) = 0.5;
        *rgba_ptr.add(1) = 0.5;
        *rgba_ptr.add(2) = 0.5;
        *rgba_ptr.add(3) = 1.0;
    }
}

/// C: mjs_defaultCamera (user/user_init.c:169)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_camera(camera: *mut mjsCamera) {
    // SAFETY: caller guarantees camera is a valid, aligned, writable pointer to mjsCamera
    unsafe {
        std::ptr::write_bytes(camera as *mut u8, 0, std::mem::size_of::<mjsCamera>());

        // mode: mjCAMLIGHT_FIXED = 0 (already zero from memset)

        // extrinsics
        (*camera).quat[0] = 1.0;

        // intrinsics
        (*camera).fovy = 45.0;
        (*camera).ipd = 0.068;
        (*camera).resolution[0] = 1;
        (*camera).resolution[1] = 1;
        (*camera).output = 1; // mjCAMOUT_RGB = 1
    }
}

/// C: mjs_defaultLight (user/user_init.c:187)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_light(light: *mut mjsLight) {
    // SAFETY: caller guarantees light is a valid, aligned, writable pointer to mjsLight
    unsafe {
        std::ptr::write_bytes(light as *mut u8, 0, std::mem::size_of::<mjsLight>());

        // mode: mjCAMLIGHT_FIXED = 0 (already zero from memset)

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
    // SAFETY: caller guarantees flex is a valid, aligned, writable pointer to mjsFlex
    unsafe {
        std::ptr::write_bytes(flex as *mut u8, 0, std::mem::size_of::<mjsFlex>());

        // contact defaults
        (*flex).contype = 1;
        (*flex).conaffinity = 1;
        (*flex).condim = 3;
        (*flex).friction[0] = 1.0;
        (*flex).friction[1] = 0.005;
        (*flex).friction[2] = 0.0001;
        (*flex).solmix = 1.0;
        crate::engine::engine_init::mj_default_sol_ref_imp(
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
        (*flex).selfcollide = 4; // mjFLEXSELF_AUTO
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
    // SAFETY: caller guarantees mesh is a valid, aligned, writable pointer to mjsMesh
    unsafe {
        std::ptr::write_bytes(mesh as *mut u8, 0, std::mem::size_of::<mjsMesh>());
        (*mesh).refquat[0] = 1.0;
        (*mesh).scale[0] = 1.0;
        (*mesh).scale[1] = 1.0;
        (*mesh).scale[2] = 1.0;
        (*mesh).maxhullvert = -1;
        // inertia: mjMESH_INERTIA_LEGACY = 2, stored as i32 in [u8; 4]
        std::ptr::write((*mesh).inertia.as_mut_ptr() as *mut i32, 2);
        (*mesh).octree_maxdepth = 6;
    }
}

/// C: mjs_defaultHField (user/user_init.c:251)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_h_field(hfield: *mut mjsHField) {
    todo!() // mjs_defaultHField
}

/// C: mjs_defaultSkin (user/user_init.c:257)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_skin(skin: *mut mjsSkin) {
    todo!() // mjs_defaultSkin
}

/// C: mjs_defaultTexture (user/user_init.c:265)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_texture(texture: *mut mjsTexture) {
    todo!() // mjs_defaultTexture
}

/// C: mjs_defaultMaterial (user/user_init.c:280)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_material(material: *mut mjsMaterial) {
    todo!() // mjs_defaultMaterial
}

/// C: mjs_defaultPair (user/user_init.c:292)
/// Calls: mj_defaultSolRefImp
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_pair(pair: *mut mjsPair) {
    // SAFETY: caller guarantees pair is a valid, aligned, writable pointer to mjsPair
    unsafe {
        std::ptr::write_bytes(pair as *mut u8, 0, std::mem::size_of::<mjsPair>());
        (*pair).condim = 3;
        crate::engine::engine_init::mj_default_sol_ref_imp(
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
    // SAFETY: caller guarantees equality is a valid, aligned, writable pointer to mjsEquality
    unsafe {
        std::ptr::write_bytes(equality as *mut u8, 0, std::mem::size_of::<mjsEquality>());
        // type: mjEQ_CONNECT = 0, stored as i32 in [u8; 8] (already zero from memset)
        std::ptr::write((*equality).r#type.as_mut_ptr() as *mut i32, 0);
        (*equality).active = 1;
        crate::engine::engine_init::mj_default_sol_ref_imp(
            (*equality).solref.as_mut_ptr(),
            (*equality).solimp.as_mut_ptr(),
        );
        (*equality).data[1] = 1.0;
        (*equality).data[10] = 1.0; // torque:force ratio
    }
}

/// C: mjs_defaultTendon (user/user_init.c:316)
/// Calls: mj_defaultSolRefImp
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_tendon(tendon: *mut mjsTendon) {
    // SAFETY: caller guarantees tendon is a valid, aligned, writable pointer to mjsTendon
    unsafe {
        std::ptr::write_bytes(tendon as *mut u8, 0, std::mem::size_of::<mjsTendon>());
        (*tendon).limited = 2; // mjLIMITED_AUTO
        (*tendon).springlength[0] = -1.0;
        (*tendon).springlength[1] = -1.0;
        crate::engine::engine_init::mj_default_sol_ref_imp(
            (*tendon).solref_limit.as_mut_ptr(),
            (*tendon).solimp_limit.as_mut_ptr(),
        );
        crate::engine::engine_init::mj_default_sol_ref_imp(
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
    // SAFETY: caller guarantees actuator is a valid, aligned, writable pointer to mjsActuator
    unsafe {
        std::ptr::write_bytes(actuator as *mut u8, 0, std::mem::size_of::<mjsActuator>());

        // gain, bias
        // gaintype: mjGAIN_FIXED = 0 (already zero from memset)
        (*actuator).gainprm[0] = 1.0;
        // biastype: mjBIAS_NONE = 0 (already zero from memset)

        // activation state
        // dyntype: mjDYN_NONE = 0 (already zero from memset)
        (*actuator).dynprm[0] = 1.0;
        (*actuator).actdim = -1;

        // transmission
        // trntype: mjTRN_UNDEFINED = 1000
        std::ptr::write((*actuator).trntype.as_mut_ptr() as *mut i32, 1000);
        (*actuator).gear[0] = 1.0;

        // input/output clamping
        (*actuator).ctrllimited = 2; // mjLIMITED_AUTO
        (*actuator).forcelimited = 2; // mjLIMITED_AUTO
        (*actuator).actlimited = 2; // mjLIMITED_AUTO
    }
}

/// C: mjs_defaultSensor (user/user_init.c:354)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_sensor(sensor: *mut mjsSensor) {
    // SAFETY: caller guarantees sensor is a valid, aligned, writable pointer to mjsSensor
    unsafe {
        std::ptr::write_bytes(sensor as *mut u8, 0, std::mem::size_of::<mjsSensor>());

        // type: mjSENS_TOUCH = 0, stored as i32 in [u8; 4] (already zero from memset)
        std::ptr::write((*sensor).r#type.as_mut_ptr() as *mut i32, 0);
        // datatype: mjDATATYPE_REAL = 0, stored as i32 in [u8; 4]
        std::ptr::write((*sensor).datatype.as_mut_ptr() as *mut i32, 0);
        // needstage: mjSTAGE_ACC = 3, stored as i32 in [u8; 4]
        std::ptr::write((*sensor).needstage.as_mut_ptr() as *mut i32, 3);
    }
}

/// C: mjs_defaultNumeric (user/user_init.c:364)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_numeric(numeric: *mut mjsNumeric) {
    todo!() // mjs_defaultNumeric
}

/// C: mjs_defaultText (user/user_init.c:370)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_text(text: *mut mjsText) {
    todo!() // mjs_defaultText
}

/// C: mjs_defaultTuple (user/user_init.c:376)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_tuple(tuple: *mut mjsTuple) {
    todo!() // mjs_defaultTuple
}

/// C: mjs_defaultKey (user/user_init.c:382)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_key(key: *mut mjsKey) {
    todo!() // mjs_defaultKey
}

/// C: mjs_defaultPlugin (user/user_init.c:388)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_default_plugin(plugin: *mut mjsPlugin) {
    todo!() // mjs_defaultPlugin
}

