//! Port of: engine/engine_vis_visualize.c
//! IR hash: 9614bd3d92e7766b
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
/// Calls: mj_id2name, mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn make_label(m: *const mjModel, r#type: u32, id: i32, label: *mut i8) {
    todo!() // makeLabel
}

/// C: islandColor (engine/engine_vis_visualize.c:110)
/// Calls: hsv2rgb, mju_Halton
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn island_color(rgba: *mut f32, h: i32, awake: i32) {
    // SAFETY: caller guarantees rgba points to [4] float array
    unsafe {
        // default to gray R = G = B = 0.7
        let mut hue: f32 = 1.0f32;
        let mut saturation: f32 = 0.0f32;
        let mut value: f32 = 0.7f32;

        // island index given, use Halton sequence to generate pseudo-random color
        if h >= 0 {
            // hue in [0, 1]
            hue = crate::engine::engine_util_misc::mju_halton(h + 1, 7) as f32;

            // saturation in [0.5, 1.0]
            saturation = (0.5 + 0.5 * crate::engine::engine_util_misc::mju_halton(h + 1, 3)) as f32;

            // value in [0.6, 1.0]
            value = (0.6 + 0.4 * crate::engine::engine_util_misc::mju_halton(h + 1, 5)) as f32;
        }

        // if asleep, decrease saturation and value
        if awake == 0 {
            value *= 0.6f32;
            saturation *= 0.7f32;
        }

        hsv2rgb(rgba, hue, saturation, value);
        *rgba.add(3) = 1.0f32;
    }
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
/// Calls: mju_warning, mjv_initGeom
#[allow(unused_variables, non_snake_case)]
pub fn acquire_geom(scn: *mut mjvScene, objid: i32, category: i32, objtype: i32) -> *mut mjvGeom {
    todo!() // acquireGeom
}

/// C: releaseGeom (engine/engine_vis_visualize.c:192)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn release_geom(geom: *mut *mut mjvGeom, scn: *mut mjvScene) {
    todo!() // releaseGeom
}

/// C: addTriangle (engine/engine_vis_visualize.c:204)
/// Calls: acquireGeom, mju_cross, mju_normalize3, mjv_initGeom, releaseGeom
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_triangle(scn: *mut mjvScene, v0: *const f64, v1: *const f64, v2: *const f64, rgba: *const f32, objid: i32, category: i32, objtype: i32) {
    todo!() // addTriangle
}

/// C: setMaterial (engine/engine_vis_visualize.c:225)
/// Calls: f2f
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn set_material(m: *const mjModel, geom: *mut mjvGeom, matid: i32, rgba: *const f32, flags: *const u8) {
    todo!() // setMaterial
}

/// C: addConnector (engine/engine_vis_visualize.c:296)
/// Calls: acquireGeom, f2f, mjv_connector, releaseGeom
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_connector(scn: *mut mjvScene, r#type: i32, width: f64, from: *const f64, to: *const f64, rgba: *const f32, objid: i32, category: i32, objtype: i32) {
    todo!() // addConnector
}

/// C: markselected (engine/engine_vis_visualize.c:393)
#[allow(unused_variables, non_snake_case)]
pub fn markselected(vis: *const mjVisual, geom: *mut mjvGeom) {
    todo!() // markselected
}

/// C: addFrame (engine/engine_vis_visualize.c:400)
/// Calls: acquireGeom, mju_add3, mju_mulMatVec3, mjv_connector, releaseGeom
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_frame(scn: *mut mjvScene, objid: i32, pos: *const f64, rot: *const f64, length: f32, width: f32) {
    todo!() // addFrame
}

/// C: getFrustum (engine/engine_vis_visualize.c:434)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn get_frustum(zver: *mut f32, zhor: *mut f32, znear: f32, intrinsic: *const f32, sensorsize: *const f32) {
    // SAFETY: zver, zhor (if non-null) point to [2]; intrinsic points to [4]; sensorsize points to [2]
    unsafe {
        if !zhor.is_null() {
            *zhor.add(0) = znear / *intrinsic.add(0) * (*sensorsize.add(0) / 2.0f32 - *intrinsic.add(2));
            *zhor.add(1) = znear / *intrinsic.add(0) * (*sensorsize.add(0) / 2.0f32 + *intrinsic.add(2));
        }
        if !zver.is_null() {
            *zver.add(0) = znear / *intrinsic.add(1) * (*sensorsize.add(1) / 2.0f32 - *intrinsic.add(3));
            *zver.add(1) = znear / *intrinsic.add(1) * (*sensorsize.add(1) / 2.0f32 + *intrinsic.add(3));
        }
    }
}

/// C: addContactGeoms (engine/engine_vis_visualize.c:565)
/// Calls: acquireGeom, addFrame, f2f, islandColor, mj_contactForce, mj_id2name, mju_add3, mju_copy, mju_mulMatVec, mju_n2f, mju_norm3, mju_scl3, mju_transpose, mju_zero3, mjv_connector, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_contact_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene, catmask: i32) {
    todo!() // addContactGeoms
}

/// C: addFlexGeoms (engine/engine_vis_visualize.c:748)
/// Calls: acquireGeom, islandColor, makeLabel, markselected, mj_sleepCycle, mjv_initGeom, releaseGeom, setMaterial
#[allow(unused_variables, non_snake_case)]
pub fn add_flex_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    todo!() // addFlexGeoms
}

/// C: addSkinGeoms (engine/engine_vis_visualize.c:841)
/// Calls: acquireGeom, makeLabel, markselected, mjv_initGeom, releaseGeom, setMaterial
#[allow(unused_variables, non_snake_case)]
pub fn add_skin_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    todo!() // addSkinGeoms
}

/// C: addGeomGeoms (engine/engine_vis_visualize.c:892)
/// Calls: acquireGeom, bodycategory, islandColor, makeLabel, markselected, mj_sleepCycle, mju_addToScl3, mju_copy3, mju_dot3, mju_n2f, mju_round, mju_transpose, mjv_initGeom, releaseGeom, setMaterial
#[allow(unused_variables, non_snake_case)]
pub fn add_geom_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    todo!() // addGeomGeoms
}

/// C: addSiteGeoms (engine/engine_vis_visualize.c:1041)
/// Calls: acquireGeom, bodycategory, makeLabel, markselected, mjv_initGeom, releaseGeom, setMaterial
#[allow(unused_variables, non_snake_case)]
pub fn add_site_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    todo!() // addSiteGeoms
}

/// C: addSpatialTendonGeoms (engine/engine_vis_visualize.c:1141)
/// Calls: acquireGeom, f2f, islandColor, makeLabel, mju_copy3, mjv_catenary, mjv_connector, mjv_isCatenary, releaseGeom, setMaterial
#[allow(unused_variables, non_snake_case)]
pub fn add_spatial_tendon_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, catmask: i32, scn: *mut mjvScene) {
    todo!() // addSpatialTendonGeoms
}

/// C: addSliderCrankGeoms (engine/engine_vis_visualize.c:1266)
/// Calls: acquireGeom, f2f, makeLabel, mju_addTo3, mju_dot3, mju_scl3, mju_sub, mjv_connector, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_slider_crank_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, catmask: i32, scn: *mut mjvScene) {
    todo!() // addSliderCrankGeoms
}

/// C: addGeomFrameGeoms (engine/engine_vis_visualize.c:1334)
/// Calls: addFrame, bodycategory
#[allow(unused_variables, non_snake_case)]
pub fn add_geom_frame_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, catmask: i32, scn: *mut mjvScene) {
    todo!() // addGeomFrameGeoms
}

/// C: addSiteFrameGeoms (engine/engine_vis_visualize.c:1364)
/// Calls: addFrame, bodycategory
#[allow(unused_variables, non_snake_case)]
pub fn add_site_frame_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, catmask: i32, scn: *mut mjvScene) {
    todo!() // addSiteFrameGeoms
}

/// C: addBodyBvhGeoms (engine/engine_vis_visualize.c:1394)
/// Calls: acquireGeom, mju_addTo3, mju_mulMatVec3, mjv_initGeom, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_body_bvh_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    todo!() // addBodyBvhGeoms
}

/// C: addFlexBvhGeoms (engine/engine_vis_visualize.c:1449)
/// Calls: acquireGeom, mj_stackAllocInfo, mju_addTo3, mju_copy3, mju_mulMatVec3, mjv_connector, mjv_initGeom, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_flex_bvh_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    todo!() // addFlexBvhGeoms
}

/// C: addMeshBvhGeoms (engine/engine_vis_visualize.c:1581)
/// Calls: acquireGeom, mju_addTo3, mju_mulMatVec3, mjv_initGeom, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_mesh_bvh_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    todo!() // addMeshBvhGeoms
}

/// C: addMeshOctreeGeoms (engine/engine_vis_visualize.c:1634)
/// Calls: acquireGeom, mju_addTo3, mju_mulMatVec3, mjv_initGeom, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_mesh_octree_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    todo!() // addMeshOctreeGeoms
}

/// C: addTactileSensorGeoms (engine/engine_vis_visualize.c:1673)
/// Calls: addTriangle, mju_addTo3, mju_mat2Quat, mju_max, mju_mulMatVec3
#[allow(unused_variables, non_snake_case)]
pub fn add_tactile_sensor_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    todo!() // addTactileSensorGeoms
}

/// C: addInertiaGeoms (engine/engine_vis_visualize.c:1745)
/// Calls: acquireGeom, bodycategory, makeLabel, markselected, mju_max, mjv_initGeom, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_inertia_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    todo!() // addInertiaGeoms
}

/// C: addPerturbGeoms (engine/engine_vis_visualize.c:1811)
/// Calls: acquireGeom, f2f, mixcolor, mju_addTo3, mju_copy3, mju_mulMatVec3, mju_quat2Mat, mjv_connector, mjv_initGeom, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_perturb_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, scn: *mut mjvScene) {
    todo!() // addPerturbGeoms
}

/// C: addWorldBodyFrameGeoms (engine/engine_vis_visualize.c:1900)
/// Calls: addFrame, bodycategory
#[allow(unused_variables, non_snake_case)]
pub fn add_world_body_frame_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, catmask: i32, scn: *mut mjvScene) {
    todo!() // addWorldBodyFrameGeoms
}

/// C: addSelectionPointGeoms (engine/engine_vis_visualize.c:1928)
/// Calls: acquireGeom, f2f, mju_addTo3, mju_mulMatVec3, mju_n2f, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_selection_point_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, scn: *mut mjvScene) {
    todo!() // addSelectionPointGeoms
}

/// C: addBodyLabelGeoms (engine/engine_vis_visualize.c:1964)
/// Calls: acquireGeom, bodycategory, makeLabel, mju_n2f, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_body_label_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    todo!() // addBodyLabelGeoms
}

/// C: addJointGeoms (engine/engine_vis_visualize.c:1994)
/// Calls: acquireGeom, f2f, makeLabel, mju_addScl3, mju_message, mju_n2f, mjv_connector, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_joint_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    todo!() // addJointGeoms
}

/// C: addActuatorGeoms (engine/engine_vis_visualize.c:2074)
/// Calls: acquireGeom, f2f, makeLabel, mj_actuatorDisabled, mju_addScl3, mju_clip, mju_scl3, mjv_connector, mjv_initGeom, releaseGeom, setMaterial
#[allow(unused_variables, non_snake_case)]
pub fn add_actuator_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    todo!() // addActuatorGeoms
}

/// C: addIslandLabelGeoms (engine/engine_vis_visualize.c:2283)
/// Calls: acquireGeom, mju_n2f, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_island_label_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    todo!() // addIslandLabelGeoms
}

/// C: addCameraGeoms (engine/engine_vis_visualize.c:2313)
/// Calls: acquireGeom, addConnector, addFrame, addTriangle, f2f, getFrustum, makeLabel, mju_addScl3, mju_addToScl3, mju_n2f, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_camera_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    todo!() // addCameraGeoms
}

/// C: addLightGeoms (engine/engine_vis_visualize.c:2460)
/// Calls: acquireGeom, addFrame, f2f, makeLabel, mju_addScl3, mju_n2f, mju_quat2Mat, mju_quatZ2Vec, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_light_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    todo!() // addLightGeoms
}

/// C: addCenterOfMassGeoms (engine/engine_vis_visualize.c:2509)
/// Calls: acquireGeom, f2f, mju_n2f, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_center_of_mass_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    todo!() // addCenterOfMassGeoms
}

/// C: addAutoConnectGeoms (engine/engine_vis_visualize.c:2535)
/// Calls: addConnector
#[allow(unused_variables, non_snake_case)]
pub fn add_auto_connect_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    todo!() // addAutoConnectGeoms
}

/// C: addRangefinderGeoms (engine/engine_vis_visualize.c:2570)
/// Calls: acquireGeom, addConnector, f2f, mju_addScl3, mju_camIntrinsics, mju_camPixelRay, mju_copy3, mju_isZero, mju_n2f, mju_raydataSize, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_rangefinder_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    todo!() // addRangefinderGeoms
}

/// C: addExternalPerturbGeoms (engine/engine_vis_visualize.c:2729)
/// Calls: addConnector, mju_add3, mju_isZero, mju_norm3, mju_scl3
#[allow(unused_variables, non_snake_case)]
pub fn add_external_perturb_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    todo!() // addExternalPerturbGeoms
}

/// C: addConstraintGeoms (engine/engine_vis_visualize.c:2760)
/// Calls: acquireGeom, makeLabel, mju_addTo3, mju_copy3, mju_mulMatVec3, mjv_initGeom, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_constraint_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    todo!() // addConstraintGeoms
}

/// C: makeFace (engine/engine_vis_visualize.c:3024)
/// Calls: mju_addScl3, mju_cross, mju_n2f, mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn make_face(_face: *mut f32, _normal: *mut f32, radius: f64, vertxpos: *const f64, nface: i32, i0: i32, i1: i32, i2: i32) {
    // SAFETY: caller guarantees _face, _normal have at least 9*(nface+1) elements,
    // vertxpos has at least 3*(max(i0,i1,i2)+1) elements
    unsafe {
        let face = _face.add(9 * nface as usize);
        let normal = _normal.add(9 * nface as usize);
        let v0 = vertxpos.add(3 * i0 as usize);
        let v1 = vertxpos.add(3 * i1 as usize);
        let v2 = vertxpos.add(3 * i2 as usize);

        // compute normal
        let v01: [f64; 3] = [
            *v1.add(0) - *v0.add(0),
            *v1.add(1) - *v0.add(1),
            *v1.add(2) - *v0.add(2),
        ];
        let v02: [f64; 3] = [
            *v2.add(0) - *v0.add(0),
            *v2.add(1) - *v0.add(1),
            *v2.add(2) - *v0.add(2),
        ];
        let mut nrm: [f64; 3] = [0.0; 3];
        crate::engine::engine_util_spatial::mju_cross(nrm.as_mut_ptr(), v01.as_ptr(), v02.as_ptr());
        crate::engine::engine_util_blas::mju_normalize3(nrm.as_mut_ptr());

        // set vertices: offset by radius*normal
        let mut temp: [f64; 3] = [0.0; 3];
        crate::engine::engine_util_blas::mju_add_scl3(temp.as_mut_ptr(), v0, nrm.as_ptr(), radius);
        crate::engine::engine_util_misc::mju_n2f(face, temp.as_ptr(), 3);
        crate::engine::engine_util_blas::mju_add_scl3(temp.as_mut_ptr(), v1, nrm.as_ptr(), radius);
        crate::engine::engine_util_misc::mju_n2f(face.add(3), temp.as_ptr(), 3);
        crate::engine::engine_util_blas::mju_add_scl3(temp.as_mut_ptr(), v2, nrm.as_ptr(), radius);
        crate::engine::engine_util_misc::mju_n2f(face.add(6), temp.as_ptr(), 3);

        // set normals
        crate::engine::engine_util_misc::mju_n2f(normal, nrm.as_ptr(), 3);
        crate::engine::engine_util_misc::mju_n2f(normal.add(3), nrm.as_ptr(), 3);
        crate::engine::engine_util_misc::mju_n2f(normal.add(6), nrm.as_ptr(), 3);
    }
}

/// C: addNormal (engine/engine_vis_visualize.c:3056)
/// Calls: mju_addTo3, mju_cross, mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_normal(vertnorm: *mut f64, vertxpos: *const f64, i0: i32, i1: i32, i2: i32) {
    // SAFETY: caller guarantees vertnorm and vertxpos point to valid arrays
    // with at least 3*(max(i0,i1,i2)+1) elements
    unsafe {
        let i0 = i0 as usize;
        let i1 = i1 as usize;
        let i2 = i2 as usize;

        let e01: [f64; 3] = [
            *vertxpos.add(3 * i1) - *vertxpos.add(3 * i0),
            *vertxpos.add(3 * i1 + 1) - *vertxpos.add(3 * i0 + 1),
            *vertxpos.add(3 * i1 + 2) - *vertxpos.add(3 * i0 + 2),
        ];
        let e02: [f64; 3] = [
            *vertxpos.add(3 * i2) - *vertxpos.add(3 * i0),
            *vertxpos.add(3 * i2 + 1) - *vertxpos.add(3 * i0 + 1),
            *vertxpos.add(3 * i2 + 2) - *vertxpos.add(3 * i0 + 2),
        ];
        let n: [f64; 3] = [
            e01[1] * e02[2] - e01[2] * e02[1],
            e01[2] * e02[0] - e01[0] * e02[2],
            e01[0] * e02[1] - e01[1] * e02[0],
        ];

        for k in 0..3 {
            *vertnorm.add(3 * i0 + k) += n[k];
            *vertnorm.add(3 * i1 + k) += n[k];
            *vertnorm.add(3 * i2 + k) += n[k];
        }
    }
}

/// C: makeSmooth (engine/engine_vis_visualize.c:3076)
/// Calls: mju_cross, mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn make_smooth(_face: *mut f32, _normal: *mut f32, radius: f64, flg_flat: u8, vertnorm: *const f64, vertxpos: *const f64, nface: i32, i0: i32, i1: i32, i2: i32) {
    todo!() // makeSmooth
}

/// C: makeSide (engine/engine_vis_visualize.c:3123)
/// Calls: mju_cross, mju_normalize3, mju_scl3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn make_side(_face: *mut f32, _normal: *mut f32, radius: f64, vertnorm: *const f64, vertxpos: *const f64, nface: i32, i0: i32, i1: i32) {
    todo!() // makeSide
}

/// C: copyTex (engine/engine_vis_visualize.c:3159)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn copy_tex(dst: *mut f32, src: *const f32, nface: i32, i0: i32, i1: i32, i2: i32) {
    if dst.is_null() || src.is_null() {
        return;
    }
    // SAFETY: caller guarantees dst has at least 6*nface+6 elements, src has at least 2*max(i0,i1,i2)+2 elements
    unsafe {
        *dst.add((6 * nface + 0) as usize) = *src.add((2 * i0) as usize);
        *dst.add((6 * nface + 1) as usize) = *src.add((2 * i0 + 1) as usize);
        *dst.add((6 * nface + 2) as usize) = *src.add((2 * i1) as usize);
        *dst.add((6 * nface + 3) as usize) = *src.add((2 * i1 + 1) as usize);
        *dst.add((6 * nface + 4) as usize) = *src.add((2 * i2) as usize);
        *dst.add((6 * nface + 5) as usize) = *src.add((2 * i2 + 1) as usize);
    }
}

/// C: cosh_sinh (engine/engine_vis_visualize.c:3516)
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

/// C: catenary_intercept (engine/engine_vis_visualize.c:3526)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn catenary_intercept(v: f64, h: f64, length: f64) -> f64 {
    1.0 / f64::sqrt(f64::sqrt(length * length - v * v) / h - 1.0)
}

/// C: catenary_residual (engine/engine_vis_visualize.c:3532)
/// Calls: cosh_sinh
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

/// C: solve_catenary (engine/engine_vis_visualize.c:3549)
/// Calls: catenary_intercept, catenary_residual
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

/// C: mjv_connector (engine/engine_vis_visualize.h:29)
/// Calls: mju_message, mju_n2f, mju_norm3, mju_quat2Mat, mju_quatZ2Vec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_connector(geom: *mut mjvGeom, r#type: i32, width: f64, from: *const f64, to: *const f64) {
    todo!() // mjv_connector
}

/// C: mjv_initGeom (engine/engine_vis_visualize.h:33)
/// Calls: f2f, mju_n2f
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_init_geom(geom: *mut mjvGeom, r#type: i32, size: *const f64, pos: *const f64, mat: *const f64, rgba: *const f32) {
    todo!() // mjv_initGeom
}

/// C: mjv_updateScene (engine/engine_vis_visualize.h:37)
/// Calls: mjp_getPluginAtSlotUnsafe, mjp_pluginCount, mju_message, mjv_addGeoms, mjv_makeLights, mjv_updateActiveFlex, mjv_updateActiveSkin, mjv_updateCamera
#[allow(unused_variables, non_snake_case)]
pub fn mjv_update_scene(m: *const mjModel, d: *mut mjData, opt: *const mjvOption, pert: *const mjvPerturb, cam: *mut mjvCamera, catmask: i32, scn: *mut mjvScene) {
    todo!() // mjv_updateScene
}

/// C: mjv_addGeoms (engine/engine_vis_visualize.h:41)
/// Calls: addActuatorGeoms, addAutoConnectGeoms, addBodyBvhGeoms, addBodyLabelGeoms, addCameraGeoms, addCenterOfMassGeoms, addConstraintGeoms, addContactGeoms, addExternalPerturbGeoms, addFlexBvhGeoms, addFlexGeoms, addGeomFrameGeoms, addGeomGeoms, addInertiaGeoms, addIslandLabelGeoms, addJointGeoms, addLightGeoms, addMeshBvhGeoms, addMeshOctreeGeoms, addPerturbGeoms, addRangefinderGeoms, addSelectionPointGeoms, addSiteFrameGeoms, addSiteGeoms, addSkinGeoms, addSliderCrankGeoms, addSpatialTendonGeoms, addTactileSensorGeoms, addWorldBodyFrameGeoms, mjv_defaultPerturb
#[allow(unused_variables, non_snake_case)]
pub fn mjv_add_geoms(m: *const mjModel, d: *mut mjData, opt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    todo!() // mjv_addGeoms
}

/// C: mjv_makeLights (engine/engine_vis_visualize.h:45)
/// Calls: f2f, mju_n2f, mjv_cameraInModel
#[allow(unused_variables, non_snake_case)]
pub fn mjv_make_lights(m: *const mjModel, d: *const mjData, scn: *mut mjvScene) {
    todo!() // mjv_makeLights
}

/// C: mjv_updateCamera (engine/engine_vis_visualize.h:48)
/// Calls: mju_copy3, mju_message, mjv_cameraFrame, mjv_cameraFrustum
#[allow(unused_variables, non_snake_case)]
pub fn mjv_update_camera(m: *const mjModel, d: *const mjData, cam: *mut mjvCamera, scn: *mut mjvScene) {
    todo!() // mjv_updateCamera
}

/// C: mjv_updateActiveFlex (engine/engine_vis_visualize.h:51)
/// Calls: addNormal, copyTex, makeFace, makeSide, makeSmooth, mj_freeStack, mj_markStack, mj_stackAllocInfo, mju_error, mju_normalize3, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mjv_update_active_flex(m: *const mjModel, d: *mut mjData, scn: *mut mjvScene, opt: *const mjvOption) {
    todo!() // mjv_updateActiveFlex
}

/// C: mjv_updateSkin (engine/engine_vis_visualize.h:54)
/// Calls: mju_warning, mjv_defaultOption, mjv_updateActiveSkin
#[allow(unused_variables, non_snake_case)]
pub fn mjv_update_skin(m: *const mjModel, d: *const mjData, scn: *mut mjvScene) {
    todo!() // mjv_updateSkin
}

/// C: mjv_updateActiveSkin (engine/engine_vis_visualize.h:57)
/// Calls: mju_addTo3, mju_cross, mju_mulMatVec3, mju_mulQuat, mju_negQuat, mju_quat2Mat, mju_sub3
#[allow(unused_variables, non_snake_case)]
pub fn mjv_update_active_skin(m: *const mjModel, d: *const mjData, scn: *mut mjvScene, opt: *const mjvOption) {
    todo!() // mjv_updateActiveSkin
}

/// C: mjv_cameraFrame (engine/engine_vis_visualize.h:61)
/// Calls: mju_addScl3, mju_copy3, mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_camera_frame(headpos: *mut f64, forward: *mut f64, up: *mut f64, right: *mut f64, d: *const mjData, cam: *const mjvCamera) {
    todo!() // mjv_cameraFrame
}

/// C: mjv_cameraFrustum (engine/engine_vis_visualize.h:65)
/// Calls: getFrustum, mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_camera_frustum(zver: *mut f32, zhor: *mut f32, zclip: *mut f32, m: *const mjModel, cam: *const mjvCamera) {
    todo!() // mjv_cameraFrustum
}

/// C: mjv_isCatenary (engine/engine_vis_visualize.h:69)
/// Calls: mju_isZero, mju_norm3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_is_catenary(m: *const mjModel, d: *const mjData, i: i32, length: *mut f64) -> i32 {
    todo!() // mjv_isCatenary
}

/// C: mjv_catenary (engine/engine_vis_visualize.h:72)
/// Calls: cosh_sinh, mju_addScl3, mju_addToScl3, mju_copy3, mju_dist3, mju_dot3, mju_normalize3, mju_scl3, mju_sub3, mju_subFrom3, mju_zero3, solve_catenary
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_catenary(x0: *const f64, x1: *const f64, gravity: *const f64, length: f64, catenary: *mut f64, ncatenary: i32) -> i32 {
    use crate::engine::engine_util_blas::{
        mju_dist3, mju_scl3, mju_normalize3, mju_sub3, mju_copy3,
        mju_dot3, mju_sub_from3, mju_add_scl3, mju_add_to_scl3, mju_zero3,
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
        mju_sub_from3(across.as_mut_ptr(), tmp.as_ptr());
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
            mju_add_to_scl3(catenary.add(3), up.as_ptr(), d_up);
            mju_add_to_scl3(catenary.add(3), across.as_ptr(), d_across);

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
            mju_add_scl3(catenary.add(3 * i as usize), x0, across.as_ptr(), horizontal);

            // vertical offset, evaluate catenary values
            let vertical: f64 = bh * cosh_sinh((horizontal - h_offset) / bh, std::ptr::null_mut()) + v_offset;
            mju_add_to_scl3(catenary.add(3 * i as usize), up.as_ptr(), vertical);
        }

        // end point
        mju_copy3(catenary.add(3 * (ncatenary - 1) as usize), x1);

        return ncatenary;
    }
}

/// C: hsv2rgb (engine/engine_vis_visualize.h:76)
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

