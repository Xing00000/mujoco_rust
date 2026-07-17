//! Port of: engine/engine_init.c
//! IR hash: d2209344472ae336
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_defaultSolRefImp (engine/engine_init.c:32)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_default_sol_ref_imp(solref: *mut f64, solimp: *mut f64) {
    if !solref.is_null() {
        // SAFETY: solref points to at least 2 f64 (caller contract)
        unsafe {
            *solref.add(0) = 0.02;
            *solref.add(1) = 1.0;
        }
    }
    if !solimp.is_null() {
        // SAFETY: solimp points to at least 5 f64 (caller contract)
        unsafe {
            *solimp.add(0) = 0.9;
            *solimp.add(1) = 0.95;
            *solimp.add(2) = 0.001;
            *solimp.add(3) = 0.5;
            *solimp.add(4) = 2.0;
        }
    }
}

/// C: mj_defaultOption (engine/engine_init.c:51)
/// Calls: mj_defaultSolRefImp
#[allow(unused_variables, non_snake_case)]
pub fn mj_default_option(opt: *mut mjOption) {
    const mjINT_EULER: i32 = 0;
    const mjCONE_PYRAMIDAL: i32 = 0;
    const mjJAC_AUTO: i32 = 2;
    const mjSOL_NEWTON: i32 = 2;

    // SAFETY: caller guarantees opt is a valid, aligned, writable pointer to mjOption
    unsafe {
        // fill opt with zeros in case struct is padded
        std::ptr::write_bytes(opt as *mut u8, 0, std::mem::size_of::<mjOption>());

        // timing parameters
        (*opt).timestep = 0.002;

        // solver parameters
        (*opt).impratio = 1.0;
        (*opt).tolerance = 1e-8;
        (*opt).ls_tolerance = 0.01;
        (*opt).noslip_tolerance = 1e-6;
        (*opt).ccd_tolerance = 1e-6;

        // sleep settings
        (*opt).sleep_tolerance = 1e-4;

        // physical constants
        (*opt).gravity[0] = 0.0;
        (*opt).gravity[1] = 0.0;
        (*opt).gravity[2] = -9.81;
        (*opt).wind[0] = 0.0;
        (*opt).wind[1] = 0.0;
        (*opt).wind[2] = 0.0;
        (*opt).magnetic[0] = 0.0;
        (*opt).magnetic[1] = -0.5;
        (*opt).magnetic[2] = 0.0;
        (*opt).density = 0.0;
        (*opt).viscosity = 0.0;

        // solver overrides
        (*opt).o_margin = 0.0;
        mj_default_sol_ref_imp((*opt).o_solref.as_mut_ptr(), (*opt).o_solimp.as_mut_ptr());
        (*opt).o_friction[0] = 1.0;
        (*opt).o_friction[1] = 1.0;
        (*opt).o_friction[2] = 0.005;
        (*opt).o_friction[3] = 0.0001;
        (*opt).o_friction[4] = 0.0001;

        // discrete options
        (*opt).integrator = mjINT_EULER;
        (*opt).cone = mjCONE_PYRAMIDAL;
        (*opt).jacobian = mjJAC_AUTO;
        (*opt).solver = mjSOL_NEWTON;
        (*opt).iterations = 100;
        (*opt).ls_iterations = 50;
        (*opt).noslip_iterations = 0;
        (*opt).ccd_iterations = 35;
        (*opt).disableflags = 0;
        (*opt).enableflags = 0;
        (*opt).disableactuator = 0;

        // sdf collisions
        (*opt).sdf_initpoints = 40;
        (*opt).sdf_iterations = 10;
    }
}

/// C: setf4 (engine/engine_init.c:124)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn setf4(rgba: *mut f32, r: f32, g: f32, b: f32, a: f32) {
    // SAFETY: caller guarantees rgba points to at least 4 contiguous floats
    unsafe {
        *rgba.add(0) = r;
        *rgba.add(1) = g;
        *rgba.add(2) = b;
        *rgba.add(3) = a;
    }
}

/// C: mj_defaultVisual (engine/engine_init.c:133)
/// Calls: setf4
#[allow(unused_variables, non_snake_case)]
pub fn mj_default_visual(vis: *mut mjVisual) {
    // SAFETY: caller guarantees vis is a valid, aligned, writable pointer to mjVisual
    unsafe {
        std::ptr::write_bytes(vis as *mut u8, 0, std::mem::size_of::<mjVisual>());

        // global — opaque [u8; 52]
        // Layout: cameraid(i32) orthographic(i32) fovy(f32) ipd(f32) azimuth(f32) elevation(f32)
        //         linewidth(f32) glow(f32) realtime(f32) offwidth(i32) offheight(i32)
        //         ellipsoidinertia(i32) bvactive(i32)
        let g = (*vis).global.as_mut_ptr();
        *(g.add(0) as *mut i32) = -1;          // cameraid
        // orthographic = 0 (already zeroed)
        *(g.add(8) as *mut f32) = 45.0;        // fovy
        *(g.add(12) as *mut f32) = 0.068;      // ipd
        *(g.add(16) as *mut f32) = 90.0;       // azimuth
        *(g.add(20) as *mut f32) = -45.0;      // elevation
        *(g.add(24) as *mut f32) = 1.0;        // linewidth
        *(g.add(28) as *mut f32) = 0.3;        // glow
        *(g.add(32) as *mut f32) = 1.0;        // realtime
        *(g.add(36) as *mut i32) = 640;        // offwidth
        *(g.add(40) as *mut i32) = 480;        // offheight
        // ellipsoidinertia = 0 (already zeroed)
        *(g.add(48) as *mut i32) = 1;          // bvactive

        // quality — opaque [u8; 20]
        // Layout: shadowsize(i32) offsamples(i32) numslices(i32) numstacks(i32) numquads(i32)
        let q = (*vis).quality.as_mut_ptr();
        *(q.add(0) as *mut i32) = 4096;        // shadowsize
        *(q.add(4) as *mut i32) = 4;           // offsamples
        *(q.add(8) as *mut i32) = 28;          // numslices
        *(q.add(12) as *mut i32) = 16;         // numstacks
        *(q.add(16) as *mut i32) = 4;          // numquads

        // headlight — opaque struct (40 bytes)
        // Layout: ambient[3](f32) diffuse[3](f32) specular[3](f32) active(i32)
        let h = (*vis).headlight._data.as_mut_ptr();
        *(h.add(0) as *mut f32) = 0.1;         // ambient[0]
        *(h.add(4) as *mut f32) = 0.1;         // ambient[1]
        *(h.add(8) as *mut f32) = 0.1;         // ambient[2]
        *(h.add(12) as *mut f32) = 0.4;        // diffuse[0]
        *(h.add(16) as *mut f32) = 0.4;        // diffuse[1]
        *(h.add(20) as *mut f32) = 0.4;        // diffuse[2]
        *(h.add(24) as *mut f32) = 0.5;        // specular[0]
        *(h.add(28) as *mut f32) = 0.5;        // specular[1]
        *(h.add(32) as *mut f32) = 0.5;        // specular[2]
        *(h.add(36) as *mut i32) = 1;          // active

        // map — opaque [u8; 52]
        // Layout: stiffness(f32) stiffnessrot(f32) force(f32) torque(f32) alpha(f32)
        //         fogstart(f32) fogend(f32) znear(f32) zfar(f32) haze(f32)
        //         shadowclip(f32) shadowscale(f32) actuatortendon(f32)
        let mp = (*vis).map.as_mut_ptr();
        *(mp.add(0) as *mut f32) = 100.0;      // stiffness
        *(mp.add(4) as *mut f32) = 500.0;      // stiffnessrot
        *(mp.add(8) as *mut f32) = 0.005;      // force
        *(mp.add(12) as *mut f32) = 0.1;       // torque
        *(mp.add(16) as *mut f32) = 0.3;       // alpha
        *(mp.add(20) as *mut f32) = 3.0;       // fogstart
        *(mp.add(24) as *mut f32) = 10.0;      // fogend
        *(mp.add(28) as *mut f32) = 0.01;      // znear
        *(mp.add(32) as *mut f32) = 50.0;      // zfar
        *(mp.add(36) as *mut f32) = 0.3;       // haze
        *(mp.add(40) as *mut f32) = 1.0;       // shadowclip
        *(mp.add(44) as *mut f32) = 0.6;       // shadowscale
        *(mp.add(48) as *mut f32) = 2.0;       // actuatortendon

        // scale — opaque [u8; 68]
        // Layout: 17 × f32 in order: forcewidth, contactwidth, contactheight, connect, com,
        //         camera, light, selectpoint, jointlength, jointwidth, actuatorlength,
        //         actuatorwidth, framelength, framewidth, constraint, slidercrank, frustum
        let sc = (*vis).scale.as_mut_ptr();
        *(sc.add(0) as *mut f32) = 0.1;        // forcewidth
        *(sc.add(4) as *mut f32) = 0.3;        // contactwidth
        *(sc.add(8) as *mut f32) = 0.1;        // contactheight
        *(sc.add(12) as *mut f32) = 0.2;       // connect
        *(sc.add(16) as *mut f32) = 0.4;       // com
        *(sc.add(20) as *mut f32) = 0.3;       // camera
        *(sc.add(24) as *mut f32) = 0.3;       // light
        *(sc.add(28) as *mut f32) = 0.2;       // selectpoint
        *(sc.add(32) as *mut f32) = 1.0;       // jointlength
        *(sc.add(36) as *mut f32) = 0.1;       // jointwidth
        *(sc.add(40) as *mut f32) = 0.7;       // actuatorlength
        *(sc.add(44) as *mut f32) = 0.2;       // actuatorwidth
        *(sc.add(48) as *mut f32) = 1.0;       // framelength
        *(sc.add(52) as *mut f32) = 0.1;       // framewidth
        *(sc.add(56) as *mut f32) = 0.1;       // constraint
        *(sc.add(60) as *mut f32) = 0.2;       // slidercrank
        *(sc.add(64) as *mut f32) = 10.0;      // frustum

        // rgba — opaque struct (400 bytes = 25 colors × 4 floats × 4 bytes)
        // Use setf4 helper for each color
        let rgba = (*vis).rgba._data.as_mut_ptr();
        let mut offset: usize = 0;
        // fog
        setf4(rgba.add(offset) as *mut f32, 0.0, 0.0, 0.0, 1.0); offset += 16;
        // haze
        setf4(rgba.add(offset) as *mut f32, 1.0, 1.0, 1.0, 1.0); offset += 16;
        // force
        setf4(rgba.add(offset) as *mut f32, 1.0, 0.5, 0.5, 1.0); offset += 16;
        // inertia
        setf4(rgba.add(offset) as *mut f32, 0.8, 0.2, 0.2, 0.6); offset += 16;
        // joint
        setf4(rgba.add(offset) as *mut f32, 0.2, 0.6, 0.8, 1.0); offset += 16;
        // actuator
        setf4(rgba.add(offset) as *mut f32, 0.2, 0.25, 0.2, 1.0); offset += 16;
        // actuatornegative
        setf4(rgba.add(offset) as *mut f32, 0.2, 0.6, 0.9, 1.0); offset += 16;
        // actuatorpositive
        setf4(rgba.add(offset) as *mut f32, 0.9, 0.4, 0.2, 1.0); offset += 16;
        // com
        setf4(rgba.add(offset) as *mut f32, 0.9, 0.9, 0.9, 1.0); offset += 16;
        // camera
        setf4(rgba.add(offset) as *mut f32, 0.6, 0.9, 0.6, 1.0); offset += 16;
        // light
        setf4(rgba.add(offset) as *mut f32, 0.6, 0.6, 0.9, 1.0); offset += 16;
        // selectpoint
        setf4(rgba.add(offset) as *mut f32, 0.9, 0.9, 0.1, 1.0); offset += 16;
        // connect
        setf4(rgba.add(offset) as *mut f32, 0.2, 0.2, 0.8, 1.0); offset += 16;
        // contactpoint
        setf4(rgba.add(offset) as *mut f32, 0.9, 0.6, 0.2, 1.0); offset += 16;
        // contactforce
        setf4(rgba.add(offset) as *mut f32, 0.7, 0.9, 0.9, 1.0); offset += 16;
        // contactfriction
        setf4(rgba.add(offset) as *mut f32, 0.9, 0.8, 0.4, 1.0); offset += 16;
        // contacttorque
        setf4(rgba.add(offset) as *mut f32, 0.9, 0.7, 0.9, 1.0); offset += 16;
        // contactgap
        setf4(rgba.add(offset) as *mut f32, 0.5, 0.8, 0.9, 1.0); offset += 16;
        // rangefinder
        setf4(rgba.add(offset) as *mut f32, 1.0, 1.0, 0.1, 1.0); offset += 16;
        // constraint
        setf4(rgba.add(offset) as *mut f32, 0.9, 0.0, 0.0, 1.0); offset += 16;
        // slidercrank
        setf4(rgba.add(offset) as *mut f32, 0.5, 0.3, 0.8, 1.0); offset += 16;
        // crankbroken
        setf4(rgba.add(offset) as *mut f32, 0.9, 0.0, 0.0, 1.0); offset += 16;
        // frustum
        setf4(rgba.add(offset) as *mut f32, 1.0, 1.0, 0.0, 0.2); offset += 16;
        // bv
        setf4(rgba.add(offset) as *mut f32, 0.0, 1.0, 0.0, 0.5); offset += 16;
        // bvactive
        setf4(rgba.add(offset) as *mut f32, 1.0, 0.0, 0.0, 0.5);
    }
}

/// C: mj_defaultLROpt (engine/engine_init.c:234)
#[allow(unused_variables, non_snake_case)]
pub fn mj_default_lr_opt(opt: *mut mjLROpt) {
    // SAFETY: opt is a valid pointer to mjLROpt (caller contract)
    unsafe {
        (*opt).mode = 1; // mjLRMODE_MUSCLE
        (*opt).useexisting = 1;
        (*opt).uselimit = 0;

        (*opt).accel = 20.0;
        (*opt).maxforce = 0.0;
        (*opt).timeconst = 1.0;
        (*opt).timestep = 0.01;
        (*opt).inttotal = 10.0;
        (*opt).interval = 2.0;
        (*opt).tolrange = 0.05;
    }
}

/// C: mj_defaultStatistic (engine/engine_init.h:30)
/// Calls: mju_zero3
#[allow(unused_variables, non_snake_case)]
pub fn mj_default_statistic(stat: *mut mjStatistic) {
    // SAFETY: stat is a valid mjStatistic pointer (caller contract).
    unsafe {
        crate::engine::engine_util_blas::mju_zero3((*stat).center.as_mut_ptr());
        (*stat).extent = 2.0;
        (*stat).meaninertia = 1.0;
        (*stat).meanmass = 1.0;
        (*stat).meansize = 0.2;
    }
}

