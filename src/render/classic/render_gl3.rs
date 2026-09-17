//! Port of: render/classic/render_gl3.c
//! IR hash: fd5288ad7242f001
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
    todo!() // cxx-internal:render_gl3.c.o:_isBehind
}

/// C: isReflective (render/classic/render_gl3.c:45)
#[allow(unused_variables, non_snake_case)]
pub fn is_reflective(geom: *const mjvGeom) -> i32 {
    todo!() // cxx-internal:render_gl3.c.o:_isReflective
}

/// C: isBuiltinWithUV (render/classic/render_gl3.c:62)
#[allow(unused_variables, non_snake_case)]
pub fn is_builtin_with_uv(r#type: i32) -> i32 {
    todo!() // cxx-internal:render_gl3.c.o:_isBuiltinWithUV
}

/// C: settexture (render/classic/render_gl3.c:77)
/// Calls: cxx-internal:render_gl3.c.o:_isBuiltinWithUV, cxx:_mjr_setf4, cxx:_mju_max
#[allow(unused_variables, non_snake_case)]
pub fn settexture(r#type: i32, state: i32, con: *const mjrContext, geom: *const mjvGeom) {
    const mjtexSHADOW: i32 = 0;
    const mjtexSKYBOX: i32 = 1;
    const mjtexREGULAR: i32 = 2;
    const mjNTEXROLE: i32 = 10;
    const mjTEXROLE_RGB: i32 = 1;
    const mjTEXTURE_2D: i32 = 0;

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
    const MJ_MINVAL: f32 = 1E-15_f32;

    extern "C" {
        fn glActiveTexture(texture: u32);
        fn glEnable(cap: u32);
        fn glDisable(cap: u32);
        fn glBindTexture(target: u32, texture: u32);
        fn glTexGenfv(coord: u32, pname: u32, params: *const f32);
    }

    // SAFETY: con and geom are valid pointers per caller contract. GL functions are
    // linked from the mujoco C library.
    unsafe {
        let mut texid: i32 = -1;
        if !geom.is_null() {
            if (*geom).matid >= 0 {
                texid = (*con).mat_texid[(mjNTEXROLE * (*geom).matid) as usize + mjTEXROLE_RGB as usize];
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
        else if r#type == mjtexREGULAR && (*geom).texcoord != 0 {
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

                // determine scaling, adjust for pre-scaled geoms
                let mut scl: [f32; 2] = [
                    (*con).mat_texrepeat[(*geom).matid as usize * 2],
                    (*con).mat_texrepeat[(*geom).matid as usize * 2 + 1],
                ];
                if (*geom).dataid >= 0 {
                    if (*geom).size[0] > 0.0 {
                        scl[0] = scl[0] / crate::engine::engine_util_misc::mju_max(MJ_MINVAL as f64, (*geom).size[0] as f64) as f32;
                    }
                    if (*geom).size[1] > 0.0 {
                        scl[1] = scl[1] / crate::engine::engine_util_misc::mju_max(MJ_MINVAL as f64, (*geom).size[1] as f64) as f32;
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
                let mut plane: [f32; 4] = [0.0; 4];
                crate::render::classic::render_util::mjr_setf4(plane.as_mut_ptr(), 0.5 * scl[0], 0.0, 0.0, -0.5);
                glTexGenfv(GL_S, GL_OBJECT_PLANE, plane.as_ptr());
                crate::render::classic::render_util::mjr_setf4(plane.as_mut_ptr(), 0.0, -0.5 * scl[1], 0.0, -0.5);
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

                let mut plane: [f32; 4] = [0.0; 4];
                // set mapping: cube
                if r#type == mjtexREGULAR {
                    crate::render::classic::render_util::mjr_setf4(plane.as_mut_ptr(),
                        if (*con).mat_texuniform[(*geom).matid as usize] != 0 { (*geom).size[0] } else { 1.0 }, 0.0, 0.0, 0.0);
                    glTexGenfv(GL_S, GL_OBJECT_PLANE, plane.as_ptr());
                    crate::render::classic::render_util::mjr_setf4(plane.as_mut_ptr(),
                        0.0, if (*con).mat_texuniform[(*geom).matid as usize] != 0 { (*geom).size[1] } else { 1.0 }, 0.0, 0.0);
                    glTexGenfv(GL_T, GL_OBJECT_PLANE, plane.as_ptr());
                    crate::render::classic::render_util::mjr_setf4(plane.as_mut_ptr(),
                        0.0, 0.0, if (*con).mat_texuniform[(*geom).matid as usize] != 0 { (*geom).size[2] } else { 1.0 }, 0.0);
                    glTexGenfv(GL_R, GL_OBJECT_PLANE, plane.as_ptr());
                }
                // set mapping: skybox (rotate 90 deg around X)
                else {
                    crate::render::classic::render_util::mjr_setf4(plane.as_mut_ptr(), 1.0, 0.0, 0.0, 0.0);
                    glTexGenfv(GL_S, GL_OBJECT_PLANE, plane.as_ptr());
                    crate::render::classic::render_util::mjr_setf4(plane.as_mut_ptr(), 0.0, 0.0, 1.0, 0.0);
                    glTexGenfv(GL_T, GL_OBJECT_PLANE, plane.as_ptr());
                    crate::render::classic::render_util::mjr_setf4(plane.as_mut_ptr(), 0.0, -1.0, 0.0, 0.0);
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

/// C: renderGeom (render/classic/render_gl3.c:268)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl3.c:_isBehind, cxx-internal:render_gl3.c.o:_settexture, cxx:_mjr_setf4, cxx:_mju_Halton, cxx:_mju_negQuat, cxx:_mju_normalize3, cxx:_mju_quat2Mat, cxx:_mju_quatZ2Vec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn render_geom(geom: *const mjvGeom, mode: i32, headpos: *const f32, scn: *const mjvScene, con: *const mjrContext) {
    todo!() // cxx-internal:render_gl3.c.o:_renderGeom
}

/// C: renderGeomReflection (render/classic/render_gl3.c:644)
/// Calls: cxx-internal:render_gl3.c.o:_renderGeom
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn render_geom_reflection(id: i32, reflectance: f32, headpos: *mut f32, scn: *mut mjvScene, con: *const mjrContext) {
    todo!() // cxx:_renderGeomReflection
}

/// C: initGL3 (render/classic/render_gl3.c:668)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:__ZNK12_GLOBAL__N_18PNGImage5WidthEv, cxx:__ZNK7mjCMesh4FaceEv
#[allow(unused_variables, non_snake_case)]
pub fn init_gl3(scn: *const mjvScene, con: *const mjrContext) {
    todo!() // cxx-internal:render_gl3.c.o:_initGL3
}

/// C: isSupportedLight (render/classic/render_gl3.c:716)
#[allow(unused_variables, non_snake_case)]
pub fn is_supported_light(light: *const mjvLight) -> i32 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl3.c:_isSupportedLight
}

/// C: initLights (render/classic/render_gl3.c:723)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl3.c:_isSupportedLight
#[allow(unused_variables, non_snake_case)]
pub fn init_lights(scn: *mut mjvScene) {
    todo!() // cxx-internal:render_gl3.c.o:_initLights
}

/// C: setView (render/classic/render_gl3.c:779)
/// Calls: cxx:_mjr_lookAt, cxx:_mjr_transform, cxx:_mjv_averageCamera
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn set_view(view: i32, viewport: mjrRect, scn: *const mjvScene, con: *const mjrContext, camProject: *mut f32, camView: *mut f32) {
    todo!() // cxx-internal:render_gl3.c.o:_setView
}

/// C: geomcmp (render/classic/render_gl3.c:846)
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

/// C: geomSort (render/classic/render_gl3.c:861)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl3.c:_geomcmp
#[allow(unused_variables, non_snake_case)]
pub fn geom_sort(arr: *mut i32, buf: *mut i32, n: i32, context: *mut ()) {
    todo!() // cxx-internal:render_gl3.c.o:_geomSort
}

/// C: adjustLight (render/classic/render_gl3.c:866)
/// Calls: cxx:_mjr_setf4
#[allow(unused_variables, non_snake_case)]
pub fn adjust_light(thislight: *const mjvLight, n: i32) {
    todo!() // cxx-internal:render_gl3.c.o:_adjustLight
}

/// C: mjr_render (render/classic/render_gl3.c:886)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl3.c:_adjustLight, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl3.c:_geomSort, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl3.c:_initGL3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl3.c:_initLights, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl3.c:_isBehind, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl3.c:_isReflective, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl3.c:_isSupportedLight, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl3.c:_setView, cxx-internal:render_gl3.c.o:_renderGeom, cxx-internal:render_gl3.c.o:_settexture, cxx:_mjr_getrow4, cxx:_mjr_lookAt, cxx:_mjr_mulMat44, cxx:_mjr_orthoVec, cxx:_mjr_perspective, cxx:_mjr_reflect, cxx:_mjr_restoreBuffer, cxx:_mjr_textActual, cxx:_mju_error, cxx:_mju_free, cxx:_mju_malloc, cxx:_mju_min, cxx:_mju_n2f, cxx:_mjv_averageCamera, cxx:_mjv_cameraInModel, cxx:_mjv_rbound, cxx:_renderGeomReflection
#[allow(unused_variables, non_snake_case)]
pub fn mjr_render(viewport: mjrRect, scn: *mut mjvScene, con: *const mjrContext) {
    todo!() // cxx:_mjr_render
}

/// C: mjr_finish (render/classic/render_gl3.c:1618)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_finish() {
    todo!() // cxx:_mjr_finish
}

/// C: mjr_getError (render/classic/render_gl3.c:1625)
/// Calls: cxx:__ZNK8mjCModel8GetErrorEv
#[allow(unused_variables, non_snake_case)]
pub fn mjr_get_error() -> i32 {
    todo!() // cxx:_mjr_getError
}

