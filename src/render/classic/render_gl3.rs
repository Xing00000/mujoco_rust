//! Port of: render/classic/render_gl3.c
//! IR hash: 9614bd3d92e7766b
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: isBehind (render/classic/render_gl3.c:36)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn is_behind(headpos: *const f32, pos: *const f32, mat: *const f32) -> i32 {
    // SAFETY: caller guarantees headpos[3], pos[3], mat[9] are valid
    unsafe {
        if (*headpos.add(0) - *pos.add(0)) * *mat.add(2)
         + (*headpos.add(1) - *pos.add(1)) * *mat.add(5)
         + (*headpos.add(2) - *pos.add(2)) * *mat.add(8) < 0.0f32 {
            1
        } else {
            0
        }
    }
}

/// C: isReflective (render/classic/render_gl3.c:45)
#[allow(unused_variables, non_snake_case)]
pub fn is_reflective(geom: *const mjvGeom) -> i32 {
    // SAFETY: caller guarantees geom is a valid pointer to mjvGeom
    unsafe {
        if ((*geom).r#type == 0 || (*geom).r#type == 6)  // mjGEOM_PLANE or mjGEOM_BOX
            && (*geom).transparent == 0
            && (*geom).reflectance > 0.0 {
            1
        } else {
            0
        }
    }
}

/// C: settexture (render/classic/render_gl3.c:62)
/// Calls: mjr_setf4, mju_max
#[allow(unused_variables, non_snake_case)]
pub fn settexture(r#type: i32, state: i32, con: *const mjrContext, geom: *const mjvGeom) {
    todo!() // settexture
}

/// C: renderGeom (render/classic/render_gl3.c:217)
/// Calls: isBehind, mjr_setf4, mju_Halton, mju_negQuat, mju_normalize3, mju_quat2Mat, mju_quatZ2Vec, settexture
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn render_geom(geom: *const mjvGeom, mode: i32, headpos: *const f32, scn: *const mjvScene, con: *const mjrContext) {
    todo!() // renderGeom
}

/// C: renderGeomReflection (render/classic/render_gl3.c:590)
/// Calls: renderGeom
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn render_geom_reflection(id: i32, reflectance: f32, headpos: *mut f32, scn: *mut mjvScene, con: *const mjrContext) {
    todo!() // renderGeomReflection
}

/// C: initGL3 (render/classic/render_gl3.c:614)
/// Calls: PNGImage::Width, mjCMesh::Face
#[allow(unused_variables, non_snake_case)]
pub fn init_gl3(scn: *const mjvScene, con: *const mjrContext) {
    todo!() // initGL3
}

/// C: initLights (render/classic/render_gl3.c:662)
#[allow(unused_variables, non_snake_case)]
pub fn init_lights(scn: *mut mjvScene) {
    const GL_LIGHT_MODEL_AMBIENT: u32 = 0x0B53;
    const GL_LIGHT_MODEL_TWO_SIDE: u32 = 0x0B52;
    const GL_LIGHT_MODEL_LOCAL_VIEWER: u32 = 0x0B51;
    const GL_LIGHT0: u32 = 0x4000;
    const GL_AMBIENT: u32 = 0x1200;
    const GL_DIFFUSE: u32 = 0x1201;
    const GL_SPECULAR: u32 = 0x1202;
    const GL_SPOT_EXPONENT: u32 = 0x1205;
    const GL_SPOT_CUTOFF: u32 = 0x1206;
    const GL_CONSTANT_ATTENUATION: u32 = 0x1207;
    const GL_LINEAR_ATTENUATION: u32 = 0x1208;
    const GL_QUADRATIC_ATTENUATION: u32 = 0x1209;

    const mjLIGHT_DIRECTIONAL: i32 = 1;
    const mjLIGHT_SPOT: i32 = 0;

    extern "C" {
        fn glLightModelfv(pname: u32, params: *const f32);
        fn glLightModeli(pname: u32, param: i32);
        fn glLightfv(light: u32, pname: u32, params: *const f32);
        fn glLightf(light: u32, pname: u32, param: f32);
        fn glDisable(cap: u32);
    }

    // SAFETY: scn is a valid mjvScene pointer; GL functions are linked from mujoco (caller contract)
    unsafe {
        // create some ambient light if no lights are present
        let global: f32 = if (*scn).nlight != 0 { 0.0 } else { 0.3 };
        let rgba_global: [f32; 4] = [global, global, global, 1.0];

        // init light model
        glLightModelfv(GL_LIGHT_MODEL_AMBIENT, rgba_global.as_ptr());
        glLightModeli(GL_LIGHT_MODEL_TWO_SIDE, 0);
        glLightModeli(GL_LIGHT_MODEL_LOCAL_VIEWER, 1);

        // set light properties
        for i in 0..(*scn).nlight as usize {
            let light = &(*scn).lights[i];

            // colors
            glLightfv(GL_LIGHT0 + i as u32, GL_AMBIENT, light.ambient.as_ptr());
            glLightfv(GL_LIGHT0 + i as u32, GL_DIFFUSE, light.diffuse.as_ptr());
            glLightfv(GL_LIGHT0 + i as u32, GL_SPECULAR, light.specular.as_ptr());

            // parameters for directional light
            if light.r#type == mjLIGHT_DIRECTIONAL {
                glLightf(GL_LIGHT0 + i as u32, GL_SPOT_EXPONENT, 0.0);
                glLightf(GL_LIGHT0 + i as u32, GL_SPOT_CUTOFF, 180.0);
                glLightf(GL_LIGHT0 + i as u32, GL_CONSTANT_ATTENUATION, 1.0);
                glLightf(GL_LIGHT0 + i as u32, GL_LINEAR_ATTENUATION, 0.0);
                glLightf(GL_LIGHT0 + i as u32, GL_QUADRATIC_ATTENUATION, 0.0);
            }
            // parameters for spot light
            else if light.r#type == mjLIGHT_SPOT {
                glLightf(GL_LIGHT0 + i as u32, GL_SPOT_EXPONENT, light.exponent);
                glLightf(GL_LIGHT0 + i as u32, GL_SPOT_CUTOFF, light.cutoff);
                glLightf(GL_LIGHT0 + i as u32, GL_CONSTANT_ATTENUATION, light.attenuation[0]);
                glLightf(GL_LIGHT0 + i as u32, GL_LINEAR_ATTENUATION, light.attenuation[1]);
                glLightf(GL_LIGHT0 + i as u32, GL_QUADRATIC_ATTENUATION, light.attenuation[2]);
            }
            // else: ignore unsupported light types: mjLIGHT_POINT, mjLIGHT_IMAGE
        }

        // disable all lights (enable selectively in render)
        for i in 0..(*scn).nlight as u32 {
            glDisable(GL_LIGHT0 + i);
        }
    }
}

/// C: setView (render/classic/render_gl3.c:711)
/// Calls: mjr_lookAt, mjr_transform, mjv_averageCamera
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn set_view(view: i32, viewport: mjrRect, scn: *const mjvScene, con: *const mjrContext, camProject: *mut f32, camView: *mut f32) {
    todo!() // setView
}

/// C: geomcmp (render/classic/render_gl3.c:778)
#[allow(unused_variables, non_snake_case)]
pub fn geomcmp(i: *mut i32, j: *mut i32, context: *mut ()) -> i32 {
    // SAFETY: i, j are valid pointers to i32 indices; context is a valid mjvGeom array (caller contract)
    unsafe {
        let geom = context as *const mjvGeom;
        let d1: f32 = (*geom.offset(*i as isize)).camdist;
        let d2: f32 = (*geom.offset(*j as isize)).camdist;

        if d1 < d2 {
            -1
        } else if d1 == d2 {
            0
        } else {
            1
        }
    }
}

/// C: geomSort (render/classic/render_gl3.c:793)
/// Calls: geomcmp
#[allow(unused_variables, non_snake_case)]
pub fn geom_sort(arr: *mut i32, buf: *mut i32, n: i32, context: *mut ()) {
    todo!() // geomSort
}

/// C: adjustLight (render/classic/render_gl3.c:798)
/// Calls: mjr_setf4
#[allow(unused_variables, non_snake_case)]
pub fn adjust_light(thislight: *const mjvLight, n: i32) {
    const GL_LIGHT0: u32 = 0x4000;
    const GL_POSITION: u32 = 0x1203;
    const GL_SPOT_DIRECTION: u32 = 0x1204;
    const mjLIGHT_DIRECTIONAL: i32 = 1;
    const mjLIGHT_SPOT: i32 = 0;

    extern "C" {
        fn glLightfv(light: u32, pname: u32, params: *const f32);
    }

    // SAFETY: caller guarantees thislight is a valid pointer; GL calls are linked from mujoco
    unsafe {
        let mut temp: [f32; 4] = [0.0; 4];

        if (*thislight).r#type == mjLIGHT_DIRECTIONAL {
            crate::render::classic::render_util::mjr_setf4(
                temp.as_mut_ptr(),
                -(*thislight).dir[0], -(*thislight).dir[1], -(*thislight).dir[2], 0.0,
            );
            glLightfv(GL_LIGHT0 + n as u32, GL_POSITION, temp.as_ptr());
        } else if (*thislight).r#type == mjLIGHT_SPOT {
            crate::render::classic::render_util::mjr_setf4(
                temp.as_mut_ptr(),
                (*thislight).dir[0], (*thislight).dir[1], (*thislight).dir[2], 0.0,
            );
            glLightfv(GL_LIGHT0 + n as u32, GL_SPOT_DIRECTION, temp.as_ptr());
            crate::render::classic::render_util::mjr_setf4(
                temp.as_mut_ptr(),
                (*thislight).pos[0], (*thislight).pos[1], (*thislight).pos[2], 1.0,
            );
            glLightfv(GL_LIGHT0 + n as u32, GL_POSITION, temp.as_ptr());
        }
    }
}

/// C: mjr_render (render/classic/render_gl3.h:27)
/// Calls: adjustLight, geomSort, initGL3, initLights, isBehind, isReflective, mjr_getrow4, mjr_lookAt, mjr_mulMat44, mjr_orthoVec, mjr_perspective, mjr_reflect, mjr_restoreBuffer, mjr_textActual, mju_error, mju_free, mju_malloc, mju_min, mju_n2f, mjv_averageCamera, mjv_cameraInModel, mjv_rbound, renderGeom, renderGeomReflection, setView, settexture
#[allow(unused_variables, non_snake_case)]
pub fn mjr_render(viewport: mjrRect, scn: *mut mjvScene, con: *const mjrContext) {
    todo!() // mjr_render
}

/// C: mjr_finish (render/classic/render_gl3.h:30)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_finish() {
    todo ! ()
}

/// C: mjr_getError (render/classic/render_gl3.h:33)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_get_error() -> i32 {
    todo ! ()
}

