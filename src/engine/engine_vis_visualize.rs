//! Port of: engine/engine_vis_visualize.c
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: f2f (engine/engine_vis_visualize.c:49)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn f2f(dest: *mut f32, src: *const f32, n: i32) {
    // SAFETY: caller guarantees dest and src have at least n elements, non-overlapping
    unsafe {
        std::ptr::copy_nonoverlapping(src, dest, n as usize);
    }
}

/// C: makeLabel (engine/engine_vis_visualize.c:55)
/// Calls: cxx:_mj_id2name, cxx:_mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn make_label(m: *const mjModel, r#type: u32, id: i32, label: *mut i8) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_makeLabel
}

/// C: hsv2rgb (engine/engine_vis_visualize.c:76)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn hsv2rgb(RGB: *mut f32, H: f32, S: f32, V: f32) {
    // SAFETY: caller guarantees RGB points to at least 3 floats
    unsafe {
        let R: f32;
        let G: f32;
        let B: f32;

        if S <= 0.0 {
            R = V;
            G = V;
            B = V;
        } else {
            let hh: f32 = H * 6.0;
            let i: i32 = hh as i32;
            let ff: f32 = hh - i as f32;
            let p: f32 = V * (1.0 - S);
            let q: f32 = V * (1.0 - (S * ff));
            let t: f32 = V * (1.0 - (S * (1.0 - ff)));
            if i == 0 {
                R = V;  G = t;  B = p;
            } else if i == 1 {
                R = q;  G = V;  B = p;
            } else if i == 2 {
                R = p;  G = V;  B = t;
            } else if i == 3 {
                R = p;  G = q;  B = V;
            } else if i == 4 {
                R = t;  G = p;  B = V;
            } else {
                R = V;  G = p;  B = q;
            }
        }

        *RGB.add(0) = R;
        *RGB.add(1) = G;
        *RGB.add(2) = B;
    }
}

/// C: islandColor (engine/engine_vis_visualize.c:110)
/// Calls: cxx:_hsv2rgb, cxx:_mju_Halton
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn island_color(rgba: *mut f32, h: i32, awake: i32) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_islandColor
}

/// C: mixcolor (engine/engine_vis_visualize.c:140)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mixcolor(rgba: *mut f32, r#ref: *const f32, flg1: i32, flg2: i32) {
    // SAFETY: caller guarantees rgba and ref point to [4] float arrays
    unsafe {
        *rgba.add(0) = if flg1 != 0 { *r#ref.add(0) } else { 0.0 };
        if flg2 != 0 {
            let v = *rgba.add(0);
            let r1 = *r#ref.add(1);
            *rgba.add(0) = if v > r1 { v } else { r1 };
        }

        *rgba.add(1) = if flg1 != 0 { *r#ref.add(1) } else { 0.0 };
        if flg2 != 0 {
            let v = *rgba.add(1);
            let r0 = *r#ref.add(0);
            *rgba.add(1) = if v > r0 { v } else { r0 };
        }

        *rgba.add(2) = *r#ref.add(2);
        *rgba.add(3) = *r#ref.add(3);
    }
}

/// C: bodycategory (engine/engine_vis_visualize.c:157)
#[allow(unused_variables, non_snake_case)]
pub fn bodycategory(m: *const mjModel, bodyid: i32) -> i32 {
    const MJCAT_STATIC: i32 = 1;
    const MJCAT_DYNAMIC: i32 = 2;
    // SAFETY: m is valid mjModel pointer; body_weldid, body_rootid, body_mocapid are valid arrays indexed by bodyid
    unsafe {
        let rootid = *(*m).body_rootid.add(bodyid as usize);
        if *(*m).body_weldid.add(bodyid as usize) == 0
            && *(*m).body_mocapid.add(rootid as usize) == -1
        {
            MJCAT_STATIC
        } else {
            MJCAT_DYNAMIC
        }
    }
}

/// C: acquireGeom (engine/engine_vis_visualize.c:169)
/// Calls: cxx:_mju_warning, cxx:_mjv_initGeom
#[allow(unused_variables, non_snake_case)]
pub fn acquire_geom(scn: *mut mjvScene, objid: i32, category: i32, objtype: i32) -> *mut mjvGeom {
    todo!() // cxx:_acquireGeom
}

/// C: releaseGeom (engine/engine_vis_visualize.c:192)
/// Calls: cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn release_geom(geom: *mut *mut mjvGeom, scn: *mut mjvScene) {
    todo!() // cxx:_releaseGeom
}

/// C: addTriangle (engine/engine_vis_visualize.c:204)
/// Calls: cxx:_acquireGeom, cxx:_mju_cross, cxx:_mju_normalize3, cxx:_mjv_initGeom, cxx:_releaseGeom
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_triangle(scn: *mut mjvScene, v0: *const f64, v1: *const f64, v2: *const f64, rgba: *const f32, objid: i32, category: i32, objtype: i32) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addTriangle
}

/// C: setMaterial (engine/engine_vis_visualize.c:225)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_f2f
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn set_material(m: *const mjModel, geom: *mut mjvGeom, matid: i32, rgba: *const f32, flags: *const u8) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_setMaterial
}

/// C: mjv_connector (engine/engine_vis_visualize.c:260)
/// Calls: cxx:_mju_message, cxx:_mju_n2f, cxx:_mju_norm3, cxx:_mju_quat2Mat, cxx:_mju_quatZ2Vec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_connector(geom: *mut mjvGeom, r#type: i32, width: f64, from: *const f64, to: *const f64) {
    const MJGEOM_CAPSULE: i32 = 3;
    const MJGEOM_CYLINDER: i32 = 5;
    const MJGEOM_ARROW: i32 = 100;
    const MJGEOM_ARROW1: i32 = 101;
    const MJGEOM_ARROW2: i32 = 102;
    const MJGEOM_LINE: i32 = 103;

    // SAFETY: geom, from, to are valid pointers (caller contract)
    unsafe {
        let mut quat: [f64; 4] = [0.0; 4];
        let mut mat: [f64; 9] = [0.0; 9];
        let dif: [f64; 3] = [
            *to.add(0) - *from.add(0),
            *to.add(1) - *from.add(1),
            *to.add(2) - *from.add(2),
        ];

        // require connector-compatible type
        if r#type != MJGEOM_CAPSULE && r#type != MJGEOM_CYLINDER &&
           r#type != MJGEOM_ARROW && r#type != MJGEOM_ARROW1 && r#type != MJGEOM_ARROW2 &&
           r#type != MJGEOM_LINE {
            crate::engine::engine_util_errmem::mju_error(
                b"invalid geom type %d for connector\0".as_ptr() as *const i8);
        }

        // assign type
        (*geom).r#type = r#type;

        // compute size for XYZ scaling
        let norm = crate::engine::engine_util_blas::mju_norm3(dif.as_ptr());
        (*geom).size[0] = width as f32;
        (*geom).size[1] = width as f32;
        (*geom).size[2] = norm as f32;

        // cylinder and capsule are centered, size[2] is half-length
        if r#type == MJGEOM_CAPSULE || r#type == MJGEOM_CYLINDER {
            (*geom).pos[0] = (0.5 * (*from.add(0) + *to.add(0))) as f32;
            (*geom).pos[1] = (0.5 * (*from.add(1) + *to.add(1))) as f32;
            (*geom).pos[2] = (0.5 * (*from.add(2) + *to.add(2))) as f32;
            (*geom).size[2] *= 0.5;
        }
        // arrow is not centered
        else {
            (*geom).pos[0] = *from.add(0) as f32;
            (*geom).pos[1] = *from.add(1) as f32;
            (*geom).pos[2] = *from.add(2) as f32;
        }

        // set mat to minimal rotation aligning b-a with z axis
        crate::engine::engine_util_spatial::mju_quatZ2Vec(quat.as_mut_ptr(), dif.as_ptr());
        crate::engine::engine_util_spatial::mju_quat2Mat(mat.as_mut_ptr(), quat.as_ptr());
        crate::engine::engine_util_misc::mju_n2f((*geom).mat.as_mut_ptr(), mat.as_ptr(), 9);
    }
}

/// C: addConnector (engine/engine_vis_visualize.c:300)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_f2f, cxx:_acquireGeom, cxx:_mjv_connector, cxx:_releaseGeom
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_connector(scn: *mut mjvScene, r#type: i32, width: f64, from: *const f64, to: *const f64, rgba: *const f32, objid: i32, category: i32, objtype: i32) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addConnector
}

/// C: mjv_initGeom (engine/engine_vis_visualize.c:315)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_f2f, cxx:_mju_n2f
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_init_geom(geom: *mut mjvGeom, r#type: i32, size: *const f64, pos: *const f64, mat: *const f64, rgba: *const f32) {
    todo!() // cxx:_mjv_initGeom
}

/// C: markselected (engine/engine_vis_visualize.c:401)
#[allow(unused_variables, non_snake_case)]
pub fn markselected(vis: *const mjVisual, geom: *mut mjvGeom) {
    // SAFETY: caller guarantees vis and geom are valid pointers
    // glow is at byte offset 28 in global (7th field: cameraid, orthographic, fovy, ipd, azimuth, elevation, linewidth, glow)
    unsafe {
        let global_ptr = (*vis).global.as_ptr();
        let glow = *(global_ptr.add(28) as *const f32);
        (*geom).emission += glow;
    }
}

/// C: addFrame (engine/engine_vis_visualize.c:408)
/// Calls: cxx:_acquireGeom, cxx:_mju_add3, cxx:_mju_mulMatVec3, cxx:_mjv_connector, cxx:_releaseGeom
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_frame(scn: *mut mjvScene, objid: i32, pos: *const f64, rot: *const f64, length: f32, width: f32) {
    todo!() // cxx:_addFrame
}

/// C: getFrustum (engine/engine_vis_visualize.c:442)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn get_frustum(zver: *mut f32, zhor: *mut f32, znear: f32, intrinsic: *const f32, sensorsize: *const f32) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_getFrustum
}

/// C: mjv_cameraFrame (engine/engine_vis_visualize.c:456)
/// Calls: cxx:_mju_addScl3, cxx:_mju_copy3, cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_camera_frame(headpos: *mut f64, forward: *mut f64, up: *mut f64, right: *mut f64, d: *const mjData, cam: *const mjvCamera) {
    todo!() // cxx:_mjv_cameraFrame
}

/// C: mjv_cameraFrustum (engine/engine_vis_visualize.c:527)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_getFrustum, cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_camera_frustum(zver: *mut f32, zhor: *mut f32, zclip: *mut f32, m: *const mjModel, cam: *const mjvCamera) {
    todo!() // cxx:_mjv_cameraFrustum
}

/// C: addContactGeoms (engine/engine_vis_visualize.c:583)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_f2f, cxx-internal:engine_vis_visualize.c.o:_islandColor, cxx:_acquireGeom, cxx:_addFrame, cxx:_mj_contactForce, cxx:_mj_geomSurfaceVelocity, cxx:_mj_id2name, cxx:_mju_add3, cxx:_mju_addScl3, cxx:_mju_addToScl3, cxx:_mju_copy, cxx:_mju_dot3, cxx:_mju_mulMatVec, cxx:_mju_n2f, cxx:_mju_norm3, cxx:_mju_scl3, cxx:_mju_transpose, cxx:_mju_zero3, cxx:_mjv_connector, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_contact_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene, catmask: i32) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addContactGeoms
}

/// C: addFlexGeoms (engine/engine_vis_visualize.c:808)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_markselected, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_setMaterial, cxx-internal:engine_vis_visualize.c.o:_islandColor, cxx-internal:engine_vis_visualize.c.o:_makeLabel, cxx:_acquireGeom, cxx:_mj_sleepCycle, cxx:_mjv_initGeom, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_flex_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addFlexGeoms
}

/// C: addSkinGeoms (engine/engine_vis_visualize.c:901)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_markselected, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_setMaterial, cxx-internal:engine_vis_visualize.c.o:_makeLabel, cxx:_acquireGeom, cxx:_mjv_initGeom, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_skin_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addSkinGeoms
}

/// C: addGeomGeoms (engine/engine_vis_visualize.c:952)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_bodycategory, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_markselected, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_setMaterial, cxx-internal:engine_vis_visualize.c.o:_islandColor, cxx-internal:engine_vis_visualize.c.o:_makeLabel, cxx:_acquireGeom, cxx:_mj_sleepCycle, cxx:_mju_addToScl3, cxx:_mju_copy3, cxx:_mju_dot3, cxx:_mju_n2f, cxx:_mju_round, cxx:_mju_transpose, cxx:_mjv_initGeom, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_geom_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addGeomGeoms
}

/// C: addSiteGeoms (engine/engine_vis_visualize.c:1101)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_bodycategory, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_markselected, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_setMaterial, cxx-internal:engine_vis_visualize.c.o:_makeLabel, cxx:_acquireGeom, cxx:_mjv_initGeom, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_site_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addSiteGeoms
}

/// C: mjv_isCatenary (engine/engine_vis_visualize.c:1162)
/// Calls: cxx:_mju_isZero, cxx:_mju_norm3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_is_catenary(m: *const mjModel, d: *const mjData, i: i32, length: *mut f64) -> i32 {
    todo!() // cxx:_mjv_isCatenary
}

/// C: addSpatialTendonGeoms (engine/engine_vis_visualize.c:1217)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_f2f, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_setMaterial, cxx-internal:engine_vis_visualize.c.o:_islandColor, cxx-internal:engine_vis_visualize.c.o:_makeLabel, cxx:_acquireGeom, cxx:_mju_copy3, cxx:_mjv_catenary, cxx:_mjv_connector, cxx:_mjv_isCatenary, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_spatial_tendon_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, catmask: i32, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addSpatialTendonGeoms
}

/// C: addSliderCrankGeoms (engine/engine_vis_visualize.c:1342)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_f2f, cxx-internal:engine_vis_visualize.c.o:_makeLabel, cxx:_acquireGeom, cxx:_mju_addTo3, cxx:_mju_dot3, cxx:_mju_scl3, cxx:_mju_sub, cxx:_mjv_connector, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_slider_crank_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, catmask: i32, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addSliderCrankGeoms
}

/// C: addGeomFrameGeoms (engine/engine_vis_visualize.c:1410)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_bodycategory, cxx:_addFrame
#[allow(unused_variables, non_snake_case)]
pub fn add_geom_frame_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, catmask: i32, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addGeomFrameGeoms
}

/// C: addSiteFrameGeoms (engine/engine_vis_visualize.c:1440)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_bodycategory, cxx:_addFrame
#[allow(unused_variables, non_snake_case)]
pub fn add_site_frame_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, catmask: i32, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addSiteFrameGeoms
}

/// C: addBodyBvhGeoms (engine/engine_vis_visualize.c:1470)
/// Calls: cxx:_acquireGeom, cxx:_mju_addTo3, cxx:_mju_mulMatVec3, cxx:_mjv_initGeom, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_body_bvh_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addBodyBvhGeoms
}

/// C: addFlexBvhGeoms (engine/engine_vis_visualize.c:1525)
/// Calls: cxx:_acquireGeom, cxx:_mj_stackAllocInfo, cxx:_mju_addTo3, cxx:_mju_copy3, cxx:_mju_mulMatVec3, cxx:_mjv_connector, cxx:_mjv_initGeom, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_flex_bvh_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addFlexBvhGeoms
}

/// C: addMeshBvhGeoms (engine/engine_vis_visualize.c:1657)
/// Calls: cxx:_acquireGeom, cxx:_mju_addTo3, cxx:_mju_mulMatVec3, cxx:_mjv_initGeom, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_mesh_bvh_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addMeshBvhGeoms
}

/// C: addMeshOctreeGeoms (engine/engine_vis_visualize.c:1710)
/// Calls: cxx:_acquireGeom, cxx:_mju_addTo3, cxx:_mju_mulMatVec3, cxx:_mjv_initGeom, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_mesh_octree_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addMeshOctreeGeoms
}

/// C: addTactileSensorGeoms (engine/engine_vis_visualize.c:1749)
/// Calls: cxx-internal:engine_vis_visualize.c.o:_addTriangle, cxx:_mju_addTo3, cxx:_mju_mat2Quat, cxx:_mju_max, cxx:_mju_mulMatVec3
#[allow(unused_variables, non_snake_case)]
pub fn add_tactile_sensor_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addTactileSensorGeoms
}

/// C: addInertiaGeoms (engine/engine_vis_visualize.c:1821)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_bodycategory, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_markselected, cxx-internal:engine_vis_visualize.c.o:_makeLabel, cxx:_acquireGeom, cxx:_mju_max, cxx:_mjv_initGeom, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_inertia_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addInertiaGeoms
}

/// C: addPerturbGeoms (engine/engine_vis_visualize.c:1887)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_f2f, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_mixcolor, cxx:_acquireGeom, cxx:_mju_addTo3, cxx:_mju_copy3, cxx:_mju_mulMatVec3, cxx:_mju_quat2Mat, cxx:_mjv_connector, cxx:_mjv_initGeom, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_perturb_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addPerturbGeoms
}

/// C: addWorldBodyFrameGeoms (engine/engine_vis_visualize.c:1976)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_bodycategory, cxx:_addFrame
#[allow(unused_variables, non_snake_case)]
pub fn add_world_body_frame_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, catmask: i32, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addWorldBodyFrameGeoms
}

/// C: addSelectionPointGeoms (engine/engine_vis_visualize.c:2004)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_f2f, cxx:_acquireGeom, cxx:_mju_addTo3, cxx:_mju_mulMatVec3, cxx:_mju_n2f, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_selection_point_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addSelectionPointGeoms
}

/// C: addBodyLabelGeoms (engine/engine_vis_visualize.c:2040)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_bodycategory, cxx-internal:engine_vis_visualize.c.o:_makeLabel, cxx:_acquireGeom, cxx:_mju_n2f, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_body_label_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addBodyLabelGeoms
}

/// C: addJointGeoms (engine/engine_vis_visualize.c:2070)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_f2f, cxx-internal:engine_vis_visualize.c.o:_makeLabel, cxx:_acquireGeom, cxx:_mju_addScl3, cxx:_mju_message, cxx:_mju_n2f, cxx:_mjv_connector, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_joint_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addJointGeoms
}

/// C: addActuatorGeoms (engine/engine_vis_visualize.c:2150)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_f2f, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_setMaterial, cxx-internal:engine_vis_visualize.c.o:_makeLabel, cxx:_acquireGeom, cxx:_mj_actuatorDisabled, cxx:_mju_addScl3, cxx:_mju_clip, cxx:_mju_scl3, cxx:_mjv_connector, cxx:_mjv_initGeom, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_actuator_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addActuatorGeoms
}

/// C: addIslandLabelGeoms (engine/engine_vis_visualize.c:2359)
/// Calls: cxx:_acquireGeom, cxx:_mju_n2f, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_island_label_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addIslandLabelGeoms
}

/// C: addCameraGeoms (engine/engine_vis_visualize.c:2389)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_f2f, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_getFrustum, cxx-internal:engine_vis_visualize.c.o:_addConnector, cxx-internal:engine_vis_visualize.c.o:_addTriangle, cxx-internal:engine_vis_visualize.c.o:_makeLabel, cxx:_acquireGeom, cxx:_addFrame, cxx:_mju_addScl3, cxx:_mju_addToScl3, cxx:_mju_n2f, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_camera_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addCameraGeoms
}

/// C: addLightGeoms (engine/engine_vis_visualize.c:2536)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_f2f, cxx-internal:engine_vis_visualize.c.o:_makeLabel, cxx:_acquireGeom, cxx:_addFrame, cxx:_mju_addScl3, cxx:_mju_n2f, cxx:_mju_quat2Mat, cxx:_mju_quatZ2Vec, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_light_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addLightGeoms
}

/// C: addCenterOfMassGeoms (engine/engine_vis_visualize.c:2585)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_f2f, cxx:_acquireGeom, cxx:_mju_n2f, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_center_of_mass_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addCenterOfMassGeoms
}

/// C: addAutoConnectGeoms (engine/engine_vis_visualize.c:2611)
/// Calls: cxx-internal:engine_vis_visualize.c.o:_addConnector
#[allow(unused_variables, non_snake_case)]
pub fn add_auto_connect_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addAutoConnectGeoms
}

/// C: addRangefinderGeoms (engine/engine_vis_visualize.c:2646)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_f2f, cxx-internal:engine_vis_visualize.c.o:_addConnector, cxx:_acquireGeom, cxx:_mju_addScl3, cxx:_mju_camIntrinsics, cxx:_mju_camPixelRay, cxx:_mju_copy3, cxx:_mju_isZero, cxx:_mju_n2f, cxx:_mju_raydataSize, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_rangefinder_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addRangefinderGeoms
}

/// C: addExternalPerturbGeoms (engine/engine_vis_visualize.c:2805)
/// Calls: cxx-internal:engine_vis_visualize.c.o:_addConnector, cxx:_mju_add3, cxx:_mju_isZero, cxx:_mju_norm3, cxx:_mju_scl3
#[allow(unused_variables, non_snake_case)]
pub fn add_external_perturb_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addExternalPerturbGeoms
}

/// C: addConstraintGeoms (engine/engine_vis_visualize.c:2836)
/// Calls: cxx-internal:engine_vis_visualize.c.o:_makeLabel, cxx:_acquireGeom, cxx:_mju_addTo3, cxx:_mju_copy3, cxx:_mju_mulMatVec3, cxx:_mjv_initGeom, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_constraint_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addConstraintGeoms
}

/// C: mjv_addGeoms (engine/engine_vis_visualize.c:2897)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_addActuatorGeoms, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_addAutoConnectGeoms, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_addBodyBvhGeoms, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_addBodyLabelGeoms, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_addCameraGeoms, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_addCenterOfMassGeoms, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_addConstraintGeoms, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_addContactGeoms, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_addExternalPerturbGeoms, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_addFlexBvhGeoms, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_addFlexGeoms, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_addGeomFrameGeoms, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_addGeomGeoms, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_addInertiaGeoms, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_addIslandLabelGeoms, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_addJointGeoms, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_addLightGeoms, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_addMeshBvhGeoms, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_addMeshOctreeGeoms, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_addPerturbGeoms, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_addRangefinderGeoms, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_addSelectionPointGeoms, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_addSiteFrameGeoms, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_addSiteGeoms, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_addSkinGeoms, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_addSliderCrankGeoms, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_addSpatialTendonGeoms, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_addTactileSensorGeoms, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_addWorldBodyFrameGeoms, cxx:_mjv_defaultPerturb
#[allow(unused_variables, non_snake_case)]
pub fn mjv_add_geoms(m: *const mjModel, d: *mut mjData, opt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    todo!() // cxx:_mjv_addGeoms
}

/// C: mjv_makeLights (engine/engine_vis_visualize.c:2949)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_f2f, cxx:_mju_n2f, cxx:_mjv_cameraInModel
#[allow(unused_variables, non_snake_case)]
pub fn mjv_make_lights(m: *const mjModel, d: *const mjData, scn: *mut mjvScene) {
    todo!() // cxx:_mjv_makeLights
}

/// C: mjv_updateCamera (engine/engine_vis_visualize.c:3025)
/// Calls: cxx:_mju_copy3, cxx:_mju_message, cxx:_mjv_cameraFrame, cxx:_mjv_cameraFrustum
#[allow(unused_variables, non_snake_case)]
pub fn mjv_update_camera(m: *const mjModel, d: *const mjData, cam: *mut mjvCamera, scn: *mut mjvScene) {
    todo!() // cxx:_mjv_updateCamera
}

/// C: makeFace (engine/engine_vis_visualize.c:3101)
/// Calls: cxx:_mju_addScl3, cxx:_mju_cross, cxx:_mju_n2f, cxx:_mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn make_face(_face: *mut f32, _normal: *mut f32, radius: f64, vertxpos: *const f64, nface: i32, i0: i32, i1: i32, i2: i32) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_makeFace
}

/// C: addNormal (engine/engine_vis_visualize.c:3133)
/// Calls: cxx:_mju_addTo3, cxx:_mju_cross, cxx:_mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_normal(vertnorm: *mut f64, vertxpos: *const f64, i0: i32, i1: i32, i2: i32) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addNormal
}

/// C: makeSmooth (engine/engine_vis_visualize.c:3153)
/// Calls: cxx:_mju_cross, cxx:_mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn make_smooth(_face: *mut f32, _normal: *mut f32, radius: f64, flg_flat: u8, vertnorm: *const f64, vertxpos: *const f64, nface: i32, i0: i32, i1: i32, i2: i32) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_makeSmooth
}

/// C: makeSide (engine/engine_vis_visualize.c:3200)
/// Calls: cxx:_mju_cross, cxx:_mju_normalize3, cxx:_mju_scl3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn make_side(_face: *mut f32, _normal: *mut f32, radius: f64, vertnorm: *const f64, vertxpos: *const f64, nface: i32, i0: i32, i1: i32) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_makeSide
}

/// C: copyTex (engine/engine_vis_visualize.c:3236)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn copy_tex(dst: *mut f32, src: *const f32, nface: i32, i0: i32, i1: i32, i2: i32) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_copyTex
}

/// C: mjv_updateActiveFlex (engine/engine_vis_visualize.c:3251)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_addNormal, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_copyTex, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_makeSide, cxx-internal:engine_vis_visualize.c.o:_makeFace, cxx-internal:engine_vis_visualize.c.o:_makeSmooth, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_error, cxx:_mju_normalize3, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mjv_update_active_flex(m: *const mjModel, d: *mut mjData, scn: *mut mjvScene, opt: *const mjvOption) {
    todo!() // cxx:_mjv_updateActiveFlex
}

/// C: mjv_updateSkin (engine/engine_vis_visualize.c:3410)
/// Calls: cxx:_mju_warning, cxx:_mjv_defaultOption, cxx:_mjv_updateActiveSkin
#[allow(unused_variables, non_snake_case)]
pub fn mjv_update_skin(m: *const mjModel, d: *const mjData, scn: *mut mjvScene) {
    todo!() // cxx:_mjv_updateSkin
}

/// C: mjv_updateActiveSkin (engine/engine_vis_visualize.c:3419)
/// Calls: cxx:_mju_addTo3, cxx:_mju_cross, cxx:_mju_mulMatVec3, cxx:_mju_mulQuat, cxx:_mju_negQuat, cxx:_mju_quat2Mat, cxx:_mju_sub3
#[allow(unused_variables, non_snake_case)]
pub fn mjv_update_active_skin(m: *const mjModel, d: *const mjData, scn: *mut mjvScene, opt: *const mjvOption) {
    todo!() // cxx:_mjv_updateActiveSkin
}

/// C: mjv_updateScene (engine/engine_vis_visualize.c:3548)
/// Calls: cxx:_mjp_getPluginAtSlotUnsafe, cxx:_mjp_pluginCount, cxx:_mju_message, cxx:_mjv_addGeoms, cxx:_mjv_makeLights, cxx:_mjv_updateActiveFlex, cxx:_mjv_updateActiveSkin, cxx:_mjv_updateCamera
#[allow(unused_variables, non_snake_case)]
pub fn mjv_update_scene(m: *const mjModel, d: *mut mjData, opt: *const mjvOption, pert: *const mjvPerturb, cam: *mut mjvCamera, catmask: i32, scn: *mut mjvScene) {
    todo!() // cxx:_mjv_updateScene
}

/// C: cosh_sinh (engine/engine_vis_visualize.c:3593)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn cosh_sinh(x: f64, sinh: *mut f64) -> f64 {
    let expx: f64 = f64::exp(x);
    if !sinh.is_null() {
        // SAFETY: caller guarantees sinh points to a valid f64 when non-null
        unsafe { *sinh = 0.5 * (expx - 1.0 / expx); }
    }
    0.5 * (expx + 1.0 / expx)
}

/// C: catenary_intercept (engine/engine_vis_visualize.c:3603)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn catenary_intercept(v: f64, h: f64, length: f64) -> f64 {
    1.0 / f64::sqrt(f64::sqrt(length * length - v * v) / h - 1.0)
}

/// C: catenary_residual (engine/engine_vis_visualize.c:3609)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_cosh_sinh
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn catenary_residual(b: f64, intercept: f64, grad: *mut f64) -> f64 {
    let a: f64 = 0.5 / b;
    let mut sinh_val: f64 = 0.0;
    let cosh_val: f64 = cosh_sinh(a, &mut sinh_val);
    if !grad.is_null() {
        // SAFETY: caller guarantees grad points to a valid f64 when non-null
        unsafe {
            *grad = (a * cosh_val - sinh_val) * f64::powf(2.0 * b * sinh_val - 1.0, -1.5);
        }
    }
    1.0 / f64::sqrt(2.0 * b * sinh_val - 1.0) - intercept
}

/// C: solve_catenary (engine/engine_vis_visualize.c:3626)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_catenary_intercept, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_catenary_residual
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn solve_catenary(v: f64, h: f64, length: f64) -> f64 {
    const TOLERANCE: f64 = 1e-9;

    let intercept: f64 = catenary_intercept(v, h, length);

    // initial guess using linear approximation to catenary_residual
    let mut b: f64 = intercept / f64::sqrt(24.0);

    // Newton steps to convergence (usually ~ 5 steps)
    for _i in 0..50 {
        // get value and gradient
        let mut grad: f64 = 0.0;
        let res: f64 = catenary_residual(b, intercept, &mut grad as *mut f64);

        if f64::abs(res) < TOLERANCE {
            break;
        }

        // Newton step
        let mut step: f64 = -res / grad;

        // backtracking line-search
        for _j in 0..10 {
            let new_res: f64 = catenary_residual(b + step, intercept, std::ptr::null_mut());
            if f64::abs(new_res) < f64::abs(res) {
                break;
            } else {
                step *= 0.5;
            }
        }

        // take step
        b += step;
    }

    b
}

/// C: mjv_catenary (engine/engine_vis_visualize.c:3665)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_cosh_sinh, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:_solve_catenary, cxx:_mju_addScl3, cxx:_mju_addToScl3, cxx:_mju_copy3, cxx:_mju_dist3, cxx:_mju_dot3, cxx:_mju_normalize3, cxx:_mju_scl3, cxx:_mju_sub3, cxx:_mju_subFrom3, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_catenary(x0: *const f64, x1: *const f64, gravity: *const f64, length: f64, catenary: *mut f64, ncatenary: i32) -> i32 {
    use crate::engine::engine_util_blas::{
        mju_dist3, mju_scl3, mju_normalize3, mju_sub3, mju_copy3,
        mju_dot3, mju_subFrom3, mju_addScl3, mju_addToScl3, mju_zero3,
    };
    const MJ_MINVAL: f64 = 1E-15_f64;

    // SAFETY: x0, x1, gravity are valid f64[3]; catenary is a valid buffer for 3*ncatenary f64s
    unsafe {
        let dist: f64 = mju_dist3(x0, x1);

        // tendon is stretched longer than length: draw straight line
        if dist > length {
            mju_copy3(catenary.add(0), x0);
            mju_copy3(catenary.add(3), x1);
            return 2;
        }

        // tendon is shorter than length
        // normalized up vector
        let mut up: [f64; 3] = [0.0; 3];
        mju_scl3(up.as_mut_ptr(), gravity, -1.0);
        mju_normalize3(up.as_mut_ptr());

        // x0 to x1
        let mut x01: [f64; 3] = [0.0; 3];
        mju_sub3(x01.as_mut_ptr(), x1, x0);

        // make across orthonormal to up, points from x0 to x1
        let mut across: [f64; 3] = [0.0; 3];
        mju_copy3(across.as_mut_ptr(), x01.as_ptr());
        let mut tmp: [f64; 3] = [0.0; 3];
        mju_scl3(tmp.as_mut_ptr(), up.as_ptr(), mju_dot3(up.as_ptr(), across.as_ptr()));
        mju_subFrom3(across.as_mut_ptr(), tmp.as_ptr());
        let norm: f64 = mju_normalize3(across.as_mut_ptr());

        // if across is numerically tiny, just set to 0
        if norm < MJ_MINVAL {
            mju_zero3(across.as_mut_ptr());
        }

        // extents in the suspension plane
        let h: f64 = mju_dot3(x01.as_ptr(), across.as_ptr());
        let v: f64 = mju_dot3(x01.as_ptr(), up.as_ptr());

        // near vertical tendon, use hanging bead approximation: 3 points
        if length > 100.0 * h {
            // solve for location of bead hanging on tendon
            let d_up: f64 = -0.5 * (f64::sqrt(length * length - h * h) - v);
            let d_across: f64 = h * d_up / (2.0 * d_up - v);

            // start point
            mju_copy3(catenary.add(0), x0);

            // midpoint: bead location
            mju_copy3(catenary.add(3), x0);
            mju_addToScl3(catenary.add(3), up.as_ptr(), d_up);
            mju_addToScl3(catenary.add(3), across.as_ptr(), d_across);

            // end point
            mju_copy3(catenary.add(6), x1);

            return 3;
        }

        // compute full catenary: ncatenary points
        // b*h: scaled catenary flatness
        let bh: f64 = solve_catenary(v, h, length) * h;

        // horizontal and vertical offsets
        let h_offset: f64 = -0.5 * (f64::ln((length + v) / (length - v)) * bh - h);
        let v_offset: f64 = -cosh_sinh(h_offset / bh, std::ptr::null_mut()) * bh;

        // start point
        mju_copy3(catenary.add(0), x0);

        // hanging points
        for i in 1..(ncatenary - 1) {
            // linearly spaced horizontal offset
            let horizontal: f64 = (i as f64) * h / (ncatenary as f64);
            mju_addScl3(catenary.add(3 * i as usize), x0, across.as_ptr(), horizontal);

            // vertical offset, evaluate catenary values
            let vertical: f64 = bh * cosh_sinh((horizontal - h_offset) / bh, std::ptr::null_mut()) + v_offset;
            mju_addToScl3(catenary.add(3 * i as usize), up.as_ptr(), vertical);
        }

        // end point
        mju_copy3(catenary.add(3 * (ncatenary - 1) as usize), x1);

        return ncatenary;
    }
}

