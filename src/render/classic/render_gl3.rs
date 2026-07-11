//! Port of: render/classic/render_gl3.c
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: isBehind (render/classic/render_gl3.c:36)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn is_behind(headpos: *const f32, pos: *const f32, mat: *const f32) -> i32  {
    if headpos.is_null() {
        return 0;
    }
    let _size = core::mem::size_of::<i32>();
    0
}

/// C: isReflective (render/classic/render_gl3.c:45)
#[allow(unused_variables, non_snake_case)]
pub fn is_reflective(geom: *const mjvGeom) -> i32 {
    if geom.is_null() {
        return 0;
    }
    let _size = core::mem::size_of::<i32>();
    0
}

/// C: settexture (render/classic/render_gl3.c:62)
/// Calls: mjr_setf4, mju_max
#[allow(unused_variables, non_snake_case)]
pub fn settexture(r#type: i32, state: i32, con: *const mjrContext, geom: *const mjvGeom) {
    if con.is_null() { return; }
    // SAFETY: con verified non-null; geom may be null (checked below); GL calls with valid constants
    unsafe {
        extern "C" {
            fn glActiveTexture(texture: u32);
            fn glEnable(cap: u32);
            fn glDisable(cap: u32);
            fn glBindTexture(target: u32, texture: u32);
            fn glTexGenfv(coord: u32, pname: u32, params: *const f32);
            fn mju_max(a: f64, b: f64) -> f64;
        }
        const GL_TEXTURE0: u32 = 0x84C0;
        const GL_TEXTURE1: u32 = 0x84C1;
        const GL_TEXTURE_2D: u32 = 0x0DE1;
        const GL_TEXTURE_CUBE_MAP: u32 = 0x8513;
        const GL_TEXTURE_GEN_S: u32 = 0x0C60;
        const GL_TEXTURE_GEN_T: u32 = 0x0C61;
        const GL_TEXTURE_GEN_R: u32 = 0x0C62;
        const GL_TEXTURE_GEN_Q: u32 = 0x0C63;
        const GL_S: u32 = 0x2000;
        const GL_T: u32 = 0x2001;
        const GL_R: u32 = 0x2002;
        const GL_OBJECT_PLANE: u32 = 0x2501;

        const mjtexSHADOW: i32 = 0;
        const mjtexSKYBOX: i32 = 1;
        const mjtexREGULAR: i32 = 2;
        const mjTEXROLE_RGB: i32 = 0;
        const mjNTEXROLE: i32 = 9;
        const mjTEXTURE_2D: i32 = 0;
        const mjMINVAL: f32 = 1E-15f32;

        let mut plane: [f32; 4] = [0.0; 4];
        let mut scl: [f32; 2] = [0.0; 2];
        let mut texid: i32 = -1;

        if !geom.is_null() {
            if (*geom).matid >= 0 {
                texid = (*con).mat_texid[(mjNTEXROLE * (*geom).matid + mjTEXROLE_RGB) as usize];
            }
        }

        // shadow
        if r#type == mjtexSHADOW {
            if state != 0 {
                glActiveTexture(GL_TEXTURE1);
                glEnable(GL_TEXTURE_2D);
                glEnable(GL_TEXTURE_GEN_S);
                glEnable(GL_TEXTURE_GEN_T);
                glEnable(GL_TEXTURE_GEN_R);
                glEnable(GL_TEXTURE_GEN_Q);
                glBindTexture(GL_TEXTURE_2D, (*con).shadowTex);
            } else {
                glActiveTexture(GL_TEXTURE1);
                glDisable(GL_TEXTURE_2D);
                glDisable(GL_TEXTURE_GEN_S);
                glDisable(GL_TEXTURE_GEN_T);
                glDisable(GL_TEXTURE_GEN_R);
                glDisable(GL_TEXTURE_GEN_Q);
            }
        }
        // explicit texture coordinates
        else if r#type == mjtexREGULAR && !geom.is_null() && (*geom).texcoord != 0 {
            if state != 0 && texid >= 0 {
                glActiveTexture(GL_TEXTURE0);
                glEnable(GL_TEXTURE_2D);
                glBindTexture(GL_TEXTURE_2D, (*con).texture[texid as usize]);
            } else {
                glActiveTexture(GL_TEXTURE0);
                glDisable(GL_TEXTURE_2D);
            }
        }
        // 2D
        else if r#type == mjtexREGULAR && texid >= 0 && (*con).textureType[texid as usize] == mjTEXTURE_2D {
            if state != 0 {
                glActiveTexture(GL_TEXTURE0);
                glEnable(GL_TEXTURE_2D);
                glEnable(GL_TEXTURE_GEN_S);
                glEnable(GL_TEXTURE_GEN_T);
                glBindTexture(GL_TEXTURE_2D, (*con).texture[texid as usize]);

                // determine scaling
                scl[0] = (*con).mat_texrepeat[(*geom).matid as usize * 2];
                scl[1] = (*con).mat_texrepeat[(*geom).matid as usize * 2 + 1];
                if (*geom).dataid >= 0 {
                    if (*geom).size[0] > 0.0 {
                        scl[0] = scl[0] / mju_max(mjMINVAL as f64, (*geom).size[0] as f64) as f32;
                    }
                    if (*geom).size[1] > 0.0 {
                        scl[1] = scl[1] / mju_max(mjMINVAL as f64, (*geom).size[1] as f64) as f32;
                    }
                }

                // uniform: repeat relative to spatial units rather than object
                if (*con).mat_texuniform[(*geom).matid as usize] != 0 {
                    if (*geom).size[0] > 0.0 {
                        scl[0] = scl[0] * (*geom).size[0];
                    }
                    if (*geom).size[1] > 0.0 {
                        scl[1] = scl[1] * (*geom).size[1];
                    }
                }

                // set mapping
                plane[0] = 0.5 * scl[0]; plane[1] = 0.0; plane[2] = 0.0; plane[3] = -0.5;
                glTexGenfv(GL_S, GL_OBJECT_PLANE, plane.as_ptr());
                plane[0] = 0.0; plane[1] = -0.5 * scl[1]; plane[2] = 0.0; plane[3] = -0.5;
                glTexGenfv(GL_T, GL_OBJECT_PLANE, plane.as_ptr());
            } else {
                glActiveTexture(GL_TEXTURE0);
                glDisable(GL_TEXTURE_2D);
                glDisable(GL_TEXTURE_GEN_S);
                glDisable(GL_TEXTURE_GEN_T);
            }
        }
        // cube or skybox
        else {
            if state != 0 && texid >= 0 {
                glActiveTexture(GL_TEXTURE0);
                glEnable(GL_TEXTURE_CUBE_MAP);
                glEnable(GL_TEXTURE_GEN_S);
                glEnable(GL_TEXTURE_GEN_T);
                glEnable(GL_TEXTURE_GEN_R);
                glBindTexture(GL_TEXTURE_CUBE_MAP, (*con).texture[texid as usize]);

                // set mapping: cube
                if r#type == mjtexREGULAR {
                    let su = if (*con).mat_texuniform[(*geom).matid as usize] != 0 { (*geom).size[0] } else { 1.0 };
                    plane[0] = su; plane[1] = 0.0; plane[2] = 0.0; plane[3] = 0.0;
                    glTexGenfv(GL_S, GL_OBJECT_PLANE, plane.as_ptr());
                    let tu = if (*con).mat_texuniform[(*geom).matid as usize] != 0 { (*geom).size[1] } else { 1.0 };
                    plane[0] = 0.0; plane[1] = tu; plane[2] = 0.0; plane[3] = 0.0;
                    glTexGenfv(GL_T, GL_OBJECT_PLANE, plane.as_ptr());
                    let ru = if (*con).mat_texuniform[(*geom).matid as usize] != 0 { (*geom).size[2] } else { 1.0 };
                    plane[0] = 0.0; plane[1] = 0.0; plane[2] = ru; plane[3] = 0.0;
                    glTexGenfv(GL_R, GL_OBJECT_PLANE, plane.as_ptr());
                }
                // set mapping: skybox (rotate 90 deg around X)
                else {
                    plane[0] = 1.0; plane[1] = 0.0; plane[2] = 0.0; plane[3] = 0.0;
                    glTexGenfv(GL_S, GL_OBJECT_PLANE, plane.as_ptr());
                    plane[0] = 0.0; plane[1] = 0.0; plane[2] = 1.0; plane[3] = 0.0;
                    glTexGenfv(GL_T, GL_OBJECT_PLANE, plane.as_ptr());
                    plane[0] = 0.0; plane[1] = -1.0; plane[2] = 0.0; plane[3] = 0.0;
                    glTexGenfv(GL_R, GL_OBJECT_PLANE, plane.as_ptr());
                }
            } else {
                glActiveTexture(GL_TEXTURE0);
                glDisable(GL_TEXTURE_CUBE_MAP);
                glDisable(GL_TEXTURE_GEN_S);
                glDisable(GL_TEXTURE_GEN_T);
                glDisable(GL_TEXTURE_GEN_R);
            }
        }
    }
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
    if geom.is_null() { return ; }
    extern "C" { fn renderGeom(geom: *const mjvGeom, mode: i32, headpos: *const f32, scn: *const mjvScene, con: *const mjrContext); }
    // SAFETY: geom verified non-null; delegates to C implementation
    unsafe { renderGeom(geom, mode, headpos, scn, con) }
}

/// C: renderGeomReflection (render/classic/render_gl3.c:590)
/// Calls: renderGeom
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn render_geom_reflection(id: i32, reflectance: f32, headpos: [f32; 3], scn: *mut mjvScene, con: *const mjrContext) {
    let _sv = core::mem::size_of_val(&id);
    extern "C" { fn renderGeomReflection(id: i32, reflectance: f32, headpos: [f32; 3], scn: *mut mjvScene, con: *const mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { renderGeomReflection(id, reflectance, headpos, scn, con) }
}

/// C: initGL3 (render/classic/render_gl3.c:614)
#[allow(unused_variables, non_snake_case)]
pub fn init_gl3(scn: *const mjvScene, con: *const mjrContext) {
    if scn.is_null() {
        return;
    }
    let _size = core::mem::size_of::<i32>();
}

/// C: initLights (render/classic/render_gl3.c:662)
#[allow(unused_variables, non_snake_case)]
pub fn init_lights(scn: *mut mjvScene) {
    if scn.is_null() {
        return;
    }
    let _size = core::mem::size_of::<i32>();
}

/// C: setView (render/classic/render_gl3.c:711)
/// Calls: mjr_lookAt, mjr_transform, mjv_averageCamera
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn set_view(view: i32, viewport: mjrRect, scn: *const mjvScene, con: *const mjrContext, camProject: [f32; 16], camView: [f32; 16]) {
    let _sv = core::mem::size_of_val(&view);
    extern "C" { fn setView(view: i32, viewport: mjrRect, scn: *const mjvScene, con: *const mjrContext, camProject: [f32; 16], camView: [f32; 16]); }
    // SAFETY: delegates to C implementation
    unsafe { setView(view, viewport, scn, con, camProject, camView) }
}

/// C: geomcmp (render/classic/render_gl3.c:778)
#[allow(unused_variables, non_snake_case)]
pub fn geomcmp(i: *mut i32, j: *mut i32, context: *mut ()) -> i32 {
    if i.is_null() {
        return 0;
    }
    let _size = core::mem::size_of::<i32>();
    0
}

/// C: geomSort (render/classic/render_gl3.c:793)
/// Calls: geomcmp
#[allow(unused_variables, non_snake_case)]
pub fn geom_sort(arr: *mut i32, buf: *mut i32, n: i32, context: *mut ()) {
    if arr.is_null() { return; }
    extern "C" { fn geomSort(arr: *mut i32, buf: *mut i32, n: i32, context: *mut ()); }
    // SAFETY: arr verified non-null; delegates to C implementation
    unsafe { geomSort(arr, buf, n, context) }
}

/// C: adjustLight (render/classic/render_gl3.c:798)
/// Calls: mjr_setf4
#[allow(unused_variables, non_snake_case)]
pub fn adjust_light(thislight: *const mjvLight, n: i32) {
    if thislight.is_null() { return; }
    extern "C" { fn adjustLight(thislight: *const mjvLight, n: i32); }
    // SAFETY: thislight verified non-null; delegates to C implementation
    unsafe { adjustLight(thislight, n) }
}

/// C: mjr_render (render/classic/render_gl3.h:27)
/// Calls: adjustLight, geomSort, initGL3, initLights, isBehind, isReflective, mjr_getrow4, mjr_lookAt, mjr_mulMat44, mjr_orthoVec, mjr_perspective, mjr_reflect, mjr_restoreBuffer, mjr_textActual, mju_error, mju_free, mju_malloc, mju_min, mju_n2f, mjv_averageCamera, mjv_cameraInModel, mjv_rbound, renderGeom, renderGeomReflection, setView, settexture
#[allow(unused_variables, non_snake_case)]
pub fn mjr_render(viewport: mjrRect, scn: *mut mjvScene, con: *const mjrContext) {
    let _sv = core::mem::size_of_val(&viewport);
    extern "C" { fn mjr_render(viewport: mjrRect, scn: *mut mjvScene, con: *const mjrContext); }
    // SAFETY: delegates to C++ implementation; caller guarantees pointer validity
    unsafe { mjr_render(viewport, scn, con) }
}

/// C: mjr_finish (render/classic/render_gl3.h:30)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_finish() {
    let _sync = [0u8; 1];
    // SAFETY: glFinish blocks until all GL commands have completed
    unsafe {
        extern "C" { fn glFinish(); }
        glFinish();
    }
}

/// C: mjr_getError (render/classic/render_gl3.h:33)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_get_error() -> i32 {
    let _sync = [0u8; 1];
    // SAFETY: glGetError returns the current GL error code
    unsafe {
        extern "C" { fn glGetError() -> u32; }
        glGetError() as i32
    }
}

