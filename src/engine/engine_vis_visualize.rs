//! Port of: engine/engine_vis_visualize.c
//! IR hash: 05737965add36adb
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
    // WARNING: signature changed — verify body
    // Previous params: (dest : * mut f32, src : * const f32, n : i32)
    // Previous return: ()
    unsafe { core :: ptr :: copy_nonoverlapping (src , dest , n as usize) ; }
}

/// C: makeLabel (engine/engine_vis_visualize.c:55)
/// Calls: mj_id2name, mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn make_label(m: *const mjModel, r#type: mjtObj, id: i32, label: *mut i8) {
    extern "C" { fn makeLabel_impl(m: *const mjModel, r#type: mjtObj, id: i32, label: *mut i8); }
    // SAFETY: delegates to C implementation; caller guarantees m and label are valid
    unsafe { makeLabel_impl(m, r#type, id, label) }
}

/// C: islandColor (engine/engine_vis_visualize.c:110)
/// Calls: hsv2rgb, mju_Halton
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn island_color(rgba: [f32; 4], h: i32, awake: i32) {
    // NOTE: In C ABI, `float rgba[4]` is passed as a pointer.
    // The Rust signature uses [f32; 4] but this is actually a pointer in the ABI.
    let rgba_ptr = rgba.as_ptr() as *mut f32;

    // default to gray R = G = B = 0.7
    let mut hue: f32 = 1.0;
    let mut saturation: f32 = 0.0;
    let mut value: f32 = 0.7;

    // island index given, use Halton sequence to generate pseudo-random color
    if h >= 0 {
        // hue in [0, 1]
        hue = crate::engine::engine_util_misc::mju_halton(h + 1, 7) as f32;

        // saturation in [0.5, 1.0]
        saturation = 0.5 + 0.5 * crate::engine::engine_util_misc::mju_halton(h + 1, 3) as f32;

        // value in [0.6, 1.0]
        value = 0.6 + 0.4 * crate::engine::engine_util_misc::mju_halton(h + 1, 5) as f32;
    }

    // if asleep, decrease saturation and value
    if awake == 0 {
        value *= 0.6;
        saturation *= 0.7;
    }

    hsv2rgb(rgba_ptr, hue, saturation, value);
    // SAFETY: rgba_ptr points to caller-owned memory (C ABI array param = pointer)
    unsafe { *rgba_ptr.add(3) = 1.0; }
}

/// C: mixcolor (engine/engine_vis_visualize.c:140)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mixcolor(rgba: [f32; 4], r#ref: [f32; 4], flg1: i32, flg2: i32) {
    extern "C" { fn mixcolor_impl(rgba: [f32; 4], r#ref: [f32; 4], flg1: i32, flg2: i32); }
    // SAFETY: delegates to C implementation
    unsafe { mixcolor_impl(rgba, r#ref, flg1, flg2) }
}

/// C: bodycategory (engine/engine_vis_visualize.c:157)
#[allow(unused_variables, non_snake_case)]
pub fn bodycategory(m: *const mjModel, bodyid: i32) -> i32 {
    // SAFETY: pointer dereferences follow C source semantics; caller guarantees m is valid
    // and bodyid is within bounds
    unsafe {
        if *(*m).body_weldid.add(bodyid as usize) == 0
            && *(*m).body_mocapid.add(*(*m).body_rootid.add(bodyid as usize) as usize) == -1
        {
            1 // mjCAT_STATIC
        } else {
            2 // mjCAT_DYNAMIC
        }
    }
}

/// C: acquireGeom (engine/engine_vis_visualize.c:169)
/// Calls: mju_warning, mjv_initGeom
#[allow(unused_variables, non_snake_case)]
pub fn acquire_geom(scn: *mut mjvScene, objid: i32, category: i32, objtype: i32) -> *mut mjvGeom {
    // SAFETY: all pointer dereferences follow C source semantics; caller guarantees scn is valid
    unsafe {
        if (*scn).ngeom >= (*scn).maxgeom {
            if (*scn).status == 0 {
                crate::engine::engine_util_errmem::mju_warning(b"Pre-allocated visual geom buffer is full. Increase maxgeom above %d.\0".as_ptr() as *const i8);
                (*scn).status = 1;
            }
            return std::ptr::null_mut();
        }
        let thisgeom: *mut mjvGeom = (*scn).geoms.add((*scn).ngeom as usize);
        std::ptr::write_bytes(thisgeom, 0, 1);
        mjv_init_geom(thisgeom, -1, std::ptr::null(), std::ptr::null(), std::ptr::null(), std::ptr::null());
        (*thisgeom).objtype = objtype;
        (*thisgeom).objid = objid;
        (*thisgeom).category = category;
        (*thisgeom).segid = (*scn).ngeom;
        thisgeom
    }
}

/// C: releaseGeom (engine/engine_vis_visualize.c:192)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn release_geom(geom: *mut *mut mjvGeom, scn: *mut mjvScene) {
    // SAFETY: caller guarantees geom and scn are valid; mirrors C pointer arithmetic
    unsafe {
        if *geom != (*scn).geoms.add((*scn).ngeom as usize) {
            crate::engine::engine_util_errmem::mju_error(b"Unexpected geom pointer; did you call acquireGeom?\0".as_ptr() as *const i8);
        }
        (*scn).ngeom += 1;
        *geom = std::ptr::null_mut();
    }
}

/// C: addTriangle (engine/engine_vis_visualize.c:204)
/// Calls: acquireGeom, mju_cross, mju_normalize3, mjv_initGeom, releaseGeom
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_triangle(scn: *mut mjvScene, v0: *const f64, v1: *const f64, v2: *const f64, rgba: [f32; 4], objid: i32, category: i32, objtype: i32) {
    extern "C" { fn addTriangle_impl(scn: *mut mjvScene, v0: *const f64, v1: *const f64, v2: *const f64, rgba: [f32; 4], objid: i32, category: i32, objtype: i32); }
    // SAFETY: delegates to C implementation
    unsafe { addTriangle_impl(scn, v0, v1, v2, rgba, objid, category, objtype) }
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
    extern "C" { fn setMaterial_impl(m: *const mjModel, geom: *mut mjvGeom, matid: i32, rgba: *const f32, flags: *const u8); }
    // SAFETY: delegates to C implementation; caller guarantees all pointers are valid
    unsafe { setMaterial_impl(m, geom, matid, rgba, flags) }
}

/// C: addConnector (engine/engine_vis_visualize.c:296)
/// Calls: acquireGeom, f2f, mjv_connector, releaseGeom
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_connector(scn: *mut mjvScene, r#type: i32, width: f64, from: *const f64, to: *const f64, rgba: [f32; 4], objid: i32, category: i32, objtype: i32) {
    let thisgeom = acquire_geom(scn, objid, category, objtype);
    if thisgeom.is_null() {
        return;
    }
    mjv_connector(thisgeom, r#type, width, from, to);
    // SAFETY: thisgeom is valid from acquire_geom; rgba is a C ABI array (pointer to caller memory)
    unsafe {
        f2f((*thisgeom).rgba.as_mut_ptr(), rgba.as_ptr(), 4);
    }
    let mut thisgeom_mut = thisgeom;
    release_geom(&mut thisgeom_mut, scn);
}

/// C: markselected (engine/engine_vis_visualize.c:393)
#[allow(unused_variables, non_snake_case)]
pub fn markselected(vis: *const mjVisual, geom: *mut mjvGeom) {
    extern "C" { fn markselected_impl(vis: *const mjVisual, geom: *mut mjvGeom); }
    // SAFETY: delegates to C implementation; caller guarantees vis and geom are valid
    unsafe { markselected_impl(vis, geom) }
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
    // SAFETY: caller guarantees scn, pos, rot are valid pointers;
    // pos[3], rot[9] are valid arrays. We call functions in the same module.
    unsafe {
        // draw separate geoms for each axis
        for j in 0..3i32 {
            let mut axis: [f64; 3] = [0.0; 3];
            for k in 0..3i32 {
                axis[k as usize] = if j == k { length as f64 } else { 0.0 };
            }

            let mut vec: [f64; 3] = [0.0; 3];
            crate::engine::engine_util_blas::mju_mul_mat_vec3(vec.as_mut_ptr(), rot, axis.as_ptr());

            // create a cylinder
            let mut to: [f64; 3] = [0.0; 3];
            crate::engine::engine_util_blas::mju_add3(to.as_mut_ptr(), pos, vec.as_ptr());

            let thisgeom: *mut mjvGeom = acquire_geom(scn, objid, 4 /* mjCAT_DECOR */, 0 /* mjOBJ_UNKNOWN */);
            if thisgeom.is_null() {
                return;
            }

            mjv_connector(thisgeom, 5 /* mjGEOM_CYLINDER */, width as f64, pos, to.as_ptr());
            for k in 0..3i32 {
                (*thisgeom).rgba[k as usize] = if j == k { 0.9 } else { 0.0 };
            }
            (*thisgeom).rgba[3] = 1.0;
            let mut geom_ptr = thisgeom;
            release_geom(&mut geom_ptr, scn);
        }
    }
}

/// C: getFrustum (engine/engine_vis_visualize.c:434)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn get_frustum(zver: [f32; 2], zhor: [f32; 2], znear: f32, intrinsic: [f32; 4], sensorsize: [f32; 2]) {
    extern "C" { fn getFrustum_impl(zver: [f32; 2], zhor: [f32; 2], znear: f32, intrinsic: [f32; 4], sensorsize: [f32; 2]); }
    // SAFETY: delegates to C implementation, pointers valid per caller
    unsafe { getFrustum_impl(zver, zhor, znear, intrinsic, sensorsize) }
}

/// C: addContactGeoms (engine/engine_vis_visualize.c:565)
/// Calls: acquireGeom, addFrame, f2f, islandColor, mj_contactForce, mj_id2name, mju_add3, mju_copy, mju_mulMatVec, mju_n2f, mju_norm3, mju_scl3, mju_transpose, mju_zero3, mjv_connector, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_contact_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene, catmask: i32) {
    extern "C" { fn addContactGeoms_impl(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene, catmask: i32); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { addContactGeoms_impl(m, d, vopt, scn, catmask) }
}

/// C: addFlexGeoms (engine/engine_vis_visualize.c:748)
/// Calls: acquireGeom, islandColor, makeLabel, markselected, mj_sleepCycle, mjv_initGeom, releaseGeom, setMaterial
#[allow(unused_variables, non_snake_case)]
pub fn add_flex_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    extern "C" { fn addFlexGeoms_impl(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { addFlexGeoms_impl(m, d, vopt, pert, catmask, scn) }
}

/// C: addSkinGeoms (engine/engine_vis_visualize.c:841)
/// Calls: acquireGeom, makeLabel, markselected, mjv_initGeom, releaseGeom, setMaterial
#[allow(unused_variables, non_snake_case)]
pub fn add_skin_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    extern "C" { fn addSkinGeoms_impl(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { addSkinGeoms_impl(m, d, vopt, pert, catmask, scn) }
}

/// C: addGeomGeoms (engine/engine_vis_visualize.c:892)
/// Calls: acquireGeom, bodycategory, islandColor, makeLabel, markselected, mj_sleepCycle, mju_addToScl3, mju_copy3, mju_dot3, mju_n2f, mju_round, mju_transpose, mjv_initGeom, releaseGeom, setMaterial
#[allow(unused_variables, non_snake_case)]
pub fn add_geom_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    extern "C" { fn addGeomGeoms_impl(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { addGeomGeoms_impl(m, d, vopt, pert, catmask, scn) }
}

/// C: addSiteGeoms (engine/engine_vis_visualize.c:1041)
/// Calls: acquireGeom, bodycategory, makeLabel, markselected, mjv_initGeom, releaseGeom, setMaterial
#[allow(unused_variables, non_snake_case)]
pub fn add_site_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    extern "C" { fn addSiteGeoms_impl(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { addSiteGeoms_impl(m, d, vopt, pert, catmask, scn) }
}

/// C: addSpatialTendonGeoms (engine/engine_vis_visualize.c:1141)
/// Calls: acquireGeom, f2f, islandColor, makeLabel, mju_copy3, mjv_catenary, mjv_connector, mjv_isCatenary, releaseGeom, setMaterial
#[allow(unused_variables, non_snake_case)]
pub fn add_spatial_tendon_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, catmask: i32, scn: *mut mjvScene) {
    extern "C" { fn addSpatialTendonGeoms_impl(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, catmask: i32, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { addSpatialTendonGeoms_impl(m, d, vopt, catmask, scn) }
}

/// C: addSliderCrankGeoms (engine/engine_vis_visualize.c:1266)
/// Calls: acquireGeom, f2f, makeLabel, mju_addTo3, mju_dot3, mju_scl3, mju_sub, mjv_connector, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_slider_crank_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, catmask: i32, scn: *mut mjvScene) {
    extern "C" { fn addSliderCrankGeoms_impl(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, catmask: i32, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { addSliderCrankGeoms_impl(m, d, vopt, catmask, scn) }
}

/// C: addGeomFrameGeoms (engine/engine_vis_visualize.c:1334)
/// Calls: addFrame, bodycategory
#[allow(unused_variables, non_snake_case)]
pub fn add_geom_frame_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, catmask: i32, scn: *mut mjvScene) {
    extern "C" { fn addGeomFrameGeoms_impl(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, catmask: i32, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { addGeomFrameGeoms_impl(m, d, vopt, catmask, scn) }
}

/// C: addSiteFrameGeoms (engine/engine_vis_visualize.c:1364)
/// Calls: addFrame, bodycategory
#[allow(unused_variables, non_snake_case)]
pub fn add_site_frame_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, catmask: i32, scn: *mut mjvScene) {
    extern "C" { fn addSiteFrameGeoms_impl(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, catmask: i32, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { addSiteFrameGeoms_impl(m, d, vopt, catmask, scn) }
}

/// C: addBodyBvhGeoms (engine/engine_vis_visualize.c:1394)
/// Calls: acquireGeom, mju_addTo3, mju_mulMatVec3, mjv_initGeom, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_body_bvh_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    extern "C" { fn addBodyBvhGeoms_impl(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { addBodyBvhGeoms_impl(m, d, vopt, scn) }
}

/// C: addFlexBvhGeoms (engine/engine_vis_visualize.c:1449)
/// Calls: acquireGeom, mj_stackAllocInfo, mju_addTo3, mju_copy3, mju_mulMatVec3, mjv_connector, mjv_initGeom, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_flex_bvh_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    extern "C" { fn addFlexBvhGeoms_impl(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { addFlexBvhGeoms_impl(m, d, vopt, scn) }
}

/// C: addMeshBvhGeoms (engine/engine_vis_visualize.c:1581)
/// Calls: acquireGeom, mju_addTo3, mju_mulMatVec3, mjv_initGeom, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_mesh_bvh_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    extern "C" { fn addMeshBvhGeoms_impl(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { addMeshBvhGeoms_impl(m, d, vopt, scn) }
}

/// C: addMeshOctreeGeoms (engine/engine_vis_visualize.c:1634)
/// Calls: acquireGeom, mju_addTo3, mju_mulMatVec3, mjv_initGeom, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_mesh_octree_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    extern "C" { fn addMeshOctreeGeoms_impl(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { addMeshOctreeGeoms_impl(m, d, vopt, scn) }
}

/// C: addTactileSensorGeoms (engine/engine_vis_visualize.c:1673)
/// Calls: addTriangle, mju_addTo3, mju_mat2Quat, mju_max, mju_mulMatVec3
#[allow(unused_variables, non_snake_case)]
pub fn add_tactile_sensor_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    extern "C" { fn addTactileSensorGeoms_impl(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation
    unsafe { addTactileSensorGeoms_impl(m, d, vopt, scn) }
}

/// C: addInertiaGeoms (engine/engine_vis_visualize.c:1745)
/// Calls: acquireGeom, bodycategory, makeLabel, markselected, mju_max, mjv_initGeom, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_inertia_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    extern "C" { fn addInertiaGeoms_impl(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { addInertiaGeoms_impl(m, d, vopt, pert, catmask, scn) }
}

/// C: addPerturbGeoms (engine/engine_vis_visualize.c:1811)
/// Calls: acquireGeom, f2f, mixcolor, mju_addTo3, mju_copy3, mju_mulMatVec3, mju_quat2Mat, mjv_connector, mjv_initGeom, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_perturb_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, scn: *mut mjvScene) {
    extern "C" { fn addPerturbGeoms_impl(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation
    unsafe { addPerturbGeoms_impl(m, d, vopt, pert, scn) }
}

/// C: addWorldBodyFrameGeoms (engine/engine_vis_visualize.c:1900)
/// Calls: addFrame, bodycategory
#[allow(unused_variables, non_snake_case)]
pub fn add_world_body_frame_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, catmask: i32, scn: *mut mjvScene) {
    extern "C" { fn addWorldBodyFrameGeoms_impl(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, catmask: i32, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { addWorldBodyFrameGeoms_impl(m, d, vopt, catmask, scn) }
}

/// C: addSelectionPointGeoms (engine/engine_vis_visualize.c:1928)
/// Calls: acquireGeom, f2f, mju_addTo3, mju_mulMatVec3, mju_n2f, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_selection_point_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, scn: *mut mjvScene) {
    extern "C" { fn addSelectionPointGeoms_impl(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { addSelectionPointGeoms_impl(m, d, vopt, pert, scn) }
}

/// C: addBodyLabelGeoms (engine/engine_vis_visualize.c:1964)
/// Calls: acquireGeom, bodycategory, makeLabel, mju_n2f, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_body_label_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    extern "C" { fn addBodyLabelGeoms_impl(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { addBodyLabelGeoms_impl(m, d, vopt, pert, catmask, scn) }
}

/// C: addJointGeoms (engine/engine_vis_visualize.c:1994)
/// Calls: acquireGeom, f2f, makeLabel, mju_addScl3, mju_message, mju_n2f, mjv_connector, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_joint_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    extern "C" { fn addJointGeoms_impl(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { addJointGeoms_impl(m, d, vopt, scn) }
}

/// C: addActuatorGeoms (engine/engine_vis_visualize.c:2074)
/// Calls: acquireGeom, f2f, makeLabel, mj_actuatorDisabled, mju_addScl3, mju_clip, mju_scl3, mjv_connector, mjv_initGeom, releaseGeom, setMaterial
#[allow(unused_variables, non_snake_case)]
pub fn add_actuator_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    extern "C" { fn addActuatorGeoms_impl(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { addActuatorGeoms_impl(m, d, vopt, scn) }
}

/// C: addIslandLabelGeoms (engine/engine_vis_visualize.c:2283)
/// Calls: acquireGeom, mju_n2f, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_island_label_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    extern "C" { fn addIslandLabelGeoms_impl(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { addIslandLabelGeoms_impl(m, d, vopt, scn) }
}

/// C: addCameraGeoms (engine/engine_vis_visualize.c:2313)
/// Calls: acquireGeom, addConnector, addFrame, addTriangle, f2f, getFrustum, makeLabel, mju_addScl3, mju_addToScl3, mju_n2f, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_camera_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    extern "C" { fn addCameraGeoms_impl(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation
    unsafe { addCameraGeoms_impl(m, d, vopt, scn) }
}

/// C: addLightGeoms (engine/engine_vis_visualize.c:2460)
/// Calls: acquireGeom, addFrame, f2f, makeLabel, mju_addScl3, mju_n2f, mju_quat2Mat, mju_quatZ2Vec, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_light_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    extern "C" { fn addLightGeoms_impl(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { addLightGeoms_impl(m, d, vopt, scn) }
}

/// C: addCenterOfMassGeoms (engine/engine_vis_visualize.c:2509)
/// Calls: acquireGeom, f2f, mju_n2f, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_center_of_mass_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    extern "C" { fn addCenterOfMassGeoms_impl(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { addCenterOfMassGeoms_impl(m, d, vopt, scn) }
}

/// C: addAutoConnectGeoms (engine/engine_vis_visualize.c:2535)
/// Calls: addConnector
#[allow(unused_variables, non_snake_case)]
pub fn add_auto_connect_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    extern "C" { fn addAutoConnectGeoms_impl(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { addAutoConnectGeoms_impl(m, d, vopt, scn) }
}

/// C: addRangefinderGeoms (engine/engine_vis_visualize.c:2570)
/// Calls: acquireGeom, addConnector, f2f, mju_addScl3, mju_camIntrinsics, mju_camPixelRay, mju_copy3, mju_isZero, mju_n2f, mju_raydataSize, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_rangefinder_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    extern "C" { fn addRangefinderGeoms_impl(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { addRangefinderGeoms_impl(m, d, vopt, scn) }
}

/// C: addExternalPerturbGeoms (engine/engine_vis_visualize.c:2729)
/// Calls: addConnector, mju_add3, mju_isZero, mju_norm3, mju_scl3
#[allow(unused_variables, non_snake_case)]
pub fn add_external_perturb_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    extern "C" { fn addExternalPerturbGeoms_impl(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { addExternalPerturbGeoms_impl(m, d, vopt, scn) }
}

/// C: addConstraintGeoms (engine/engine_vis_visualize.c:2760)
/// Calls: acquireGeom, makeLabel, mju_addTo3, mju_copy3, mju_mulMatVec3, mjv_initGeom, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_constraint_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    extern "C" { fn addConstraintGeoms_impl(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { addConstraintGeoms_impl(m, d, vopt, scn) }
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
    extern "C" { fn makeFace_impl(_face: *mut f32, _normal: *mut f32, radius: f64, vertxpos: *const f64, nface: i32, i0: i32, i1: i32, i2: i32); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { makeFace_impl(_face, _normal, radius, vertxpos, nface, i0, i1, i2) }
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
    extern "C" { fn addNormal_impl(vertnorm: *mut f64, vertxpos: *const f64, i0: i32, i1: i32, i2: i32); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { addNormal_impl(vertnorm, vertxpos, i0, i1, i2) }
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
    extern "C" { fn makeSmooth_impl(_face: *mut f32, _normal: *mut f32, radius: f64, flg_flat: u8, vertnorm: *const f64, vertxpos: *const f64, nface: i32, i0: i32, i1: i32, i2: i32); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { makeSmooth_impl(_face, _normal, radius, flg_flat, vertnorm, vertxpos, nface, i0, i1, i2) }
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
    extern "C" { fn makeSide_impl(_face: *mut f32, _normal: *mut f32, radius: f64, vertnorm: *const f64, vertxpos: *const f64, nface: i32, i0: i32, i1: i32); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { makeSide_impl(_face, _normal, radius, vertnorm, vertxpos, nface, i0, i1) }
}

/// C: copyTex (engine/engine_vis_visualize.c:3159)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn copy_tex(dst: *mut f32, src: *const f32, nface: i32, i0: i32, i1: i32, i2: i32) {
    extern "C" { fn copyTex_impl(dst: *mut f32, src: *const f32, nface: i32, i0: i32, i1: i32, i2: i32); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { copyTex_impl(dst, src, nface, i0, i1, i2) }
}

/// C: cosh_sinh (engine/engine_vis_visualize.c:3516)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn cosh_sinh(x: f64, sinh: *mut f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (x : f64, sinh : * mut f64)
    // Previous return: f64
    let expx: f64 = x.exp();
    if !sinh.is_null() {
        // SAFETY: caller guarantees sinh is valid when non-null
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
    // WARNING: signature changed — verify body
    // Previous params: (v : f64, h : f64, length : f64)
    // Previous return: f64
    1.0 / (((length * length - v * v).sqrt()) / h - 1.0).sqrt()
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
    // WARNING: signature changed — verify body
    // Previous params: (b : f64, intercept : f64, grad : * mut f64)
    // Previous return: f64
    let a: f64 = 0.5 / b;
    let mut sinh_val: f64 = 0.0;
    let cosh_val: f64 = cosh_sinh(a, &mut sinh_val as *mut f64);
    if !grad.is_null() {
        // SAFETY: caller guarantees grad is valid when non-null
        unsafe { *grad = (a * cosh_val - sinh_val) * (2.0 * b * sinh_val - 1.0).powf(-1.5); }
    }
    1.0 / (2.0 * b * sinh_val - 1.0).sqrt() - intercept
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
    // WARNING: signature changed — verify body
    // Previous params: (v : f64, h : f64, length : f64)
    // Previous return: f64
    const TOLERANCE: f64 = 1e-9;

    let intercept: f64 = catenary_intercept(v, h, length);

    // initial guess using linear approximation to catenary_residual
    let mut b: f64 = intercept / 24.0_f64.sqrt();

    // Newton steps to convergence (usually ~ 5 steps)
    for _i in 0..50 {
        // get value and gradient
        let mut grad: f64 = 0.0;
        let res: f64 = catenary_residual(b, intercept, &mut grad as *mut f64);

        if res.abs() < TOLERANCE {
            break;
        }

        // Newton step
        let mut step: f64 = -res / grad;

        // backtracking line-search is not essential but can reduce number of iterations
        for _j in 0..10 {
            let new_res: f64 = catenary_residual(b + step, intercept, core::ptr::null_mut());
            if new_res.abs() < res.abs() {
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
    // SAFETY: geom valid. from/to have 3 elements each.
    unsafe {
        const MJGEOM_CAPSULE: i32 = 3;
        const MJGEOM_CYLINDER: i32 = 5;

        let dif: [f64; 3] = [
            *to.add(0) - *from.add(0),
            *to.add(1) - *from.add(1),
            *to.add(2) - *from.add(2),
        ];

        // assign type
        (*geom).r#type = r#type;

        // compute size for XYZ scaling
        (*geom).size[0] = width as f32;
        (*geom).size[1] = width as f32;
        (*geom).size[2] = crate::engine::engine_util_blas::mju_norm3(dif.as_ptr()) as f32;

        // cylinder and capsule are centered, and size[2] is half-length
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

        // set mat to minimal rotation aligning dif with z axis
        let mut quat: [f64; 4] = [0.0; 4];
        let mut mat: [f64; 9] = [0.0; 9];
        crate::engine::engine_util_spatial::mju_quat_z2vec(quat.as_mut_ptr(), dif.as_ptr());
        crate::engine::engine_util_spatial::mju_quat2mat(mat.as_mut_ptr(), quat.as_ptr());
        crate::engine::engine_util_misc::mju_n2f((*geom).mat.as_mut_ptr(), mat.as_ptr(), 9);
    }
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
    // SAFETY: geom is valid. size/pos/mat/rgba may be null (use defaults).
    unsafe {
        const MJGEOM_SPHERE: i32 = 2;
        const MJGEOM_CAPSULE: i32 = 3;
        const MJGEOM_CYLINDER: i32 = 5;

        // assign type
        (*geom).r#type = r#type;

        // set size
        if !size.is_null() {
            if r#type == MJGEOM_SPHERE {
                (*geom).size[0] = *size.add(0) as f32;
                (*geom).size[1] = *size.add(0) as f32;
                (*geom).size[2] = *size.add(0) as f32;
            } else if r#type == MJGEOM_CAPSULE || r#type == MJGEOM_CYLINDER {
                (*geom).size[0] = *size.add(0) as f32;
                (*geom).size[1] = *size.add(0) as f32;
                (*geom).size[2] = *size.add(1) as f32;
            } else {
                crate::engine::engine_util_misc::mju_n2f((*geom).size.as_mut_ptr(), size, 3);
            }
        } else {
            (*geom).size[0] = 0.1;
            (*geom).size[1] = 0.1;
            (*geom).size[2] = 0.1;
        }

        // set pos
        if !pos.is_null() {
            crate::engine::engine_util_misc::mju_n2f((*geom).pos.as_mut_ptr(), pos, 3);
        } else {
            (*geom).pos[0] = 0.0;
            (*geom).pos[1] = 0.0;
            (*geom).pos[2] = 0.0;
        }

        // set mat
        if !mat.is_null() {
            crate::engine::engine_util_misc::mju_n2f((*geom).mat.as_mut_ptr(), mat, 9);
        } else {
            (*geom).mat[0] = 1.0; (*geom).mat[1] = 0.0; (*geom).mat[2] = 0.0;
            (*geom).mat[3] = 0.0; (*geom).mat[4] = 1.0; (*geom).mat[5] = 0.0;
            (*geom).mat[6] = 0.0; (*geom).mat[7] = 0.0; (*geom).mat[8] = 1.0;
        }

        // set rgba
        if !rgba.is_null() {
            f2f((*geom).rgba.as_mut_ptr(), rgba, 4);
        } else {
            (*geom).rgba[0] = 0.5;
            (*geom).rgba[1] = 0.5;
            (*geom).rgba[2] = 0.5;
            (*geom).rgba[3] = 1.0;
        }

        // set defaults
        (*geom).dataid = -1;
        (*geom).matid = -1;
        (*geom).texcoord = 0;
        (*geom).emission = 0.0;
        (*geom).specular = 0.5;
        (*geom).shininess = 0.5;
        (*geom).reflectance = 0.0;
        (*geom).label[0] = 0;
        (*geom).modelrbound = 0.0;
    }
}

/// C: mjv_updateScene (engine/engine_vis_visualize.h:37)
/// Calls: mjp_getPluginAtSlotUnsafe, mjp_pluginCount, mju_message, mjv_addGeoms, mjv_makeLights, mjv_updateActiveFlex, mjv_updateActiveSkin, mjv_updateCamera
#[allow(unused_variables, non_snake_case)]
pub fn mjv_update_scene(m: *const mjModel, d: *mut mjData, opt: *const mjvOption, pert: *const mjvPerturb, cam: *mut mjvCamera, catmask: i32, scn: *mut mjvScene) {
    extern "C" { fn mjv_updateScene_impl(m: *const mjModel, d: *mut mjData, opt: *const mjvOption, pert: *const mjvPerturb, cam: *mut mjvCamera, catmask: i32, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation
    unsafe { mjv_updateScene_impl(m, d, opt, pert, cam, catmask, scn) }
}

/// C: mjv_addGeoms (engine/engine_vis_visualize.h:41)
/// Calls: addActuatorGeoms, addAutoConnectGeoms, addBodyBvhGeoms, addBodyLabelGeoms, addCameraGeoms, addCenterOfMassGeoms, addConstraintGeoms, addContactGeoms, addExternalPerturbGeoms, addFlexBvhGeoms, addFlexGeoms, addGeomFrameGeoms, addGeomGeoms, addInertiaGeoms, addIslandLabelGeoms, addJointGeoms, addLightGeoms, addMeshBvhGeoms, addMeshOctreeGeoms, addPerturbGeoms, addRangefinderGeoms, addSelectionPointGeoms, addSiteFrameGeoms, addSiteGeoms, addSkinGeoms, addSliderCrankGeoms, addSpatialTendonGeoms, addTactileSensorGeoms, addWorldBodyFrameGeoms, mjv_defaultPerturb
#[allow(unused_variables, non_snake_case)]
pub fn mjv_add_geoms(m: *const mjModel, d: *mut mjData, opt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    extern "C" { fn mjv_addGeoms_impl(m: *const mjModel, d: *mut mjData, opt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation
    unsafe { mjv_addGeoms_impl(m, d, opt, pert, catmask, scn) }
}

/// C: mjv_makeLights (engine/engine_vis_visualize.h:45)
/// Calls: f2f, mju_n2f, mjv_cameraInModel
#[allow(unused_variables, non_snake_case)]
pub fn mjv_make_lights(m: *const mjModel, d: *const mjData, scn: *mut mjvScene) {
    extern "C" { fn mjv_makeLights_impl(m: *const mjModel, d: *const mjData, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation
    unsafe { mjv_makeLights_impl(m, d, scn) }
}

/// C: mjv_updateCamera (engine/engine_vis_visualize.h:48)
/// Calls: mju_copy3, mju_message, mjv_cameraFrame, mjv_cameraFrustum
#[allow(unused_variables, non_snake_case)]
pub fn mjv_update_camera(m: *const mjModel, d: *const mjData, cam: *mut mjvCamera, scn: *mut mjvScene) {
    extern "C" {
        fn mjv_updateCamera_impl(m: *const mjModel, d: *const mjData, cam: *mut mjvCamera, scn: *mut mjvScene);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjv_updateCamera_impl(m, d, cam, scn) }
}

/// C: mjv_updateActiveFlex (engine/engine_vis_visualize.h:51)
/// Calls: addNormal, copyTex, makeFace, makeSide, makeSmooth, mj_freeStack, mj_markStack, mj_stackAllocInfo, mju_error, mju_normalize3, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mjv_update_active_flex(m: *const mjModel, d: *mut mjData, scn: *mut mjvScene, opt: *const mjvOption) {
    extern "C" { fn mjv_updateActiveFlex_impl(m: *const mjModel, d: *mut mjData, scn: *mut mjvScene, opt: *const mjvOption); }
    // SAFETY: delegates to C implementation
    unsafe { mjv_updateActiveFlex_impl(m, d, scn, opt) }
}

/// C: mjv_updateSkin (engine/engine_vis_visualize.h:54)
/// Calls: mju_warning, mjv_defaultOption, mjv_updateActiveSkin
#[allow(unused_variables, non_snake_case)]
pub fn mjv_update_skin(m: *const mjModel, d: *const mjData, scn: *mut mjvScene) {

    extern "C" { fn mjv_updateSkin_impl(m: *const mjModel, d: *const mjData, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation
    unsafe { mjv_updateSkin_impl(m, d, scn) }
}

/// C: mjv_updateActiveSkin (engine/engine_vis_visualize.h:57)
/// Calls: mju_addTo3, mju_cross, mju_mulMatVec3, mju_mulQuat, mju_negQuat, mju_quat2Mat, mju_sub3
#[allow(unused_variables, non_snake_case)]
pub fn mjv_update_active_skin(m: *const mjModel, d: *const mjData, scn: *mut mjvScene, opt: *const mjvOption) {
    extern "C" { fn mjv_updateActiveSkin_impl(m: *const mjModel, d: *const mjData, scn: *mut mjvScene, opt: *const mjvOption); }
    // SAFETY: delegates to C implementation
    unsafe { mjv_updateActiveSkin_impl(m, d, scn, opt) }
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
    extern "C" { fn mjv_cameraFrame_impl(headpos: *mut f64, forward: *mut f64, up: *mut f64, right: *mut f64, d: *const mjData, cam: *const mjvCamera); }
    // SAFETY: delegates to C implementation
    unsafe { mjv_cameraFrame_impl(headpos, forward, up, right, d, cam) }
}

/// C: mjv_cameraFrustum (engine/engine_vis_visualize.h:65)
/// Calls: getFrustum, mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_camera_frustum(zver: [f32; 2], zhor: [f32; 2], zclip: [f32; 2], m: *const mjModel, cam: *const mjvCamera) {
    extern "C" { fn mjv_cameraFrustum_impl(zver: [f32; 2], zhor: [f32; 2], zclip: [f32; 2], m: *const mjModel, cam: *const mjvCamera); }
    // SAFETY: delegates to C implementation, pointers valid per caller
    unsafe { mjv_cameraFrustum_impl(zver, zhor, zclip, m, cam) }
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
    // SAFETY: m, d valid. i is a valid tendon index. length is non-null.
    unsafe {
        const MJNPOLY: i32 = 2;
        const MJDSBL_GRAVITY: i32 = 1 << 7;
        const MJTRN_TENDON: i32 = 2;
        const MJMINVAL: f64 = 1e-15;

        let has_stiffness: i32 = (*(*m).tendon_stiffness.add(i as usize) != 0.0
            || crate::engine::engine_util_misc::mju_is_zero(
                (*m).tendon_stiffnesspoly.add((MJNPOLY * i) as usize), MJNPOLY) == 0) as i32;

        // tendon has a deadband spring
        let limitedspring: i32 = (has_stiffness != 0
            && *(*m).tendon_lengthspring.add((2 * i) as usize) == 0.0
            && *(*m).tendon_lengthspring.add((2 * i + 1) as usize) > 0.0) as i32;

        // tendon has a simple length constraint, but is currently not limited
        let ten_length = *(*d).ten_length.add(i as usize);
        let lower = *(*m).tendon_range.add((2 * i) as usize);
        let upper = *(*m).tendon_range.add((2 * i + 1) as usize);
        let limitedconstraint: i32 = (has_stiffness == 0
            && *((*m).tendon_limited as *const u8).add(i as usize) == 1
            && lower == 0.0
            && ten_length < upper) as i32;

        let has_damping: i32 = (*(*m).tendon_damping.add(i as usize) != 0.0
            || crate::engine::engine_util_misc::mju_is_zero(
                (*m).tendon_dampingpoly.add((MJNPOLY * i) as usize), MJNPOLY) == 0) as i32;

        // conditions for drawing a catenary
        let mut draw_catenary: i32 = (
            ((*m).opt.disableflags & MJDSBL_GRAVITY) == 0
            && crate::engine::engine_util_blas::mju_norm3((*m).opt.gravity.as_ptr()) > MJMINVAL
            && *(*m).tendon_num.add(i as usize) == 2
            && (limitedspring != limitedconstraint)
            && has_damping == 0
            && *(*m).tendon_frictionloss.add(i as usize) == 0.0
        ) as i32;

        // no actuator
        if draw_catenary != 0 {
            let mut j: i32 = 0;
            while j < (*m).nu as i32 {
                if *(*m).actuator_trntype.add(j as usize) == MJTRN_TENDON
                    && *(*m).actuator_trnid.add((2 * j) as usize) == i {
                    draw_catenary = 0;
                    break;
                }
                j += 1;
            }
        }

        if draw_catenary != 0 {
            // length of the tendon
            if limitedconstraint != 0 {
                *length = *(*m).tendon_range.add((2 * i + 1) as usize);
            } else {
                *length = *(*m).tendon_lengthspring.add((2 * i + 1) as usize);
            }
        }

        draw_catenary
    }
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
    // SAFETY: x0, x1, gravity have 3 elements. catenary has 3*ncatenary elements.
    unsafe {
        const MJMINVAL: f64 = 1e-15;

        let dist = crate::engine::engine_util_blas::mju_dist3(x0, x1);

        // tendon is stretched longer than length: draw straight line
        if dist > length {
            crate::engine::engine_util_blas::mju_copy3(catenary.add(0), x0);
            crate::engine::engine_util_blas::mju_copy3(catenary.add(3), x1);
            return 2;
        }

        // tendon is shorter than length
        // normalized up vector
        let mut up: [f64; 3] = [0.0; 3];
        crate::engine::engine_util_blas::mju_scl3(up.as_mut_ptr(), gravity, -1.0);
        crate::engine::engine_util_blas::mju_normalize3(up.as_mut_ptr());

        // x0 to x1
        let mut x01: [f64; 3] = [0.0; 3];
        crate::engine::engine_util_blas::mju_sub3(x01.as_mut_ptr(), x1, x0);

        // make across orthonormal to up, points from x0 to x1
        let mut across: [f64; 3] = [0.0; 3];
        crate::engine::engine_util_blas::mju_copy3(across.as_mut_ptr(), x01.as_ptr());
        let mut tmp: [f64; 3] = [0.0; 3];
        crate::engine::engine_util_blas::mju_scl3(
            tmp.as_mut_ptr(), up.as_ptr(),
            crate::engine::engine_util_blas::mju_dot3(up.as_ptr(), across.as_ptr()));
        crate::engine::engine_util_blas::mju_sub_from3(across.as_mut_ptr(), tmp.as_ptr());
        let norm = crate::engine::engine_util_blas::mju_normalize3(across.as_mut_ptr());

        // if across is numerically tiny, just set to 0
        if norm < MJMINVAL {
            crate::engine::engine_util_blas::mju_zero3(across.as_mut_ptr());
        }

        // extents in the suspension plane
        let h = crate::engine::engine_util_blas::mju_dot3(x01.as_ptr(), across.as_ptr());
        let v = crate::engine::engine_util_blas::mju_dot3(x01.as_ptr(), up.as_ptr());

        // near vertical tendon, use hanging bead approximation: 3 points
        if length > 100.0 * h {
            // solve for location of bead hanging on tendon
            let d_up = -0.5 * ((length * length - h * h).sqrt() - v);
            let d_across = h * d_up / (2.0 * d_up - v);

            // start point
            crate::engine::engine_util_blas::mju_copy3(catenary.add(0), x0);

            // midpoint: bead location
            crate::engine::engine_util_blas::mju_copy3(catenary.add(3), x0);
            crate::engine::engine_util_blas::mju_add_to_scl3(catenary.add(3), up.as_ptr(), d_up);
            crate::engine::engine_util_blas::mju_add_to_scl3(catenary.add(3), across.as_ptr(), d_across);

            // end point
            crate::engine::engine_util_blas::mju_copy3(catenary.add(6), x1);

            return 3;
        }

        // compute full catenary: ncatenary points
        // b*h: scaled catenary flatness
        let bh = solve_catenary(v, h, length) * h;

        // horizontal and vertical offsets
        let h_offset = -0.5 * (((length + v) / (length - v)).ln() * bh - h);
        let v_offset = -cosh_sinh(h_offset / bh, core::ptr::null_mut()) * bh;

        // start point
        crate::engine::engine_util_blas::mju_copy3(catenary.add(0), x0);

        // hanging points
        let mut i: i32 = 1;
        while i < ncatenary - 1 {
            // linearly spaced horizontal offset
            let horizontal = i as f64 * h / ncatenary as f64;
            crate::engine::engine_util_blas::mju_add_scl3(catenary.add(3 * i as usize), x0, across.as_ptr(), horizontal);

            // vertical offset, evaluate catenary values
            let vertical = bh * cosh_sinh((horizontal - h_offset) / bh, core::ptr::null_mut()) + v_offset;
            crate::engine::engine_util_blas::mju_add_to_scl3(catenary.add(3 * i as usize), up.as_ptr(), vertical);
            i += 1;
        }

        // end point
        crate::engine::engine_util_blas::mju_copy3(catenary.add(3 * (ncatenary - 1) as usize), x1);

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
    // SAFETY: RGB has 3 elements.
    unsafe {
        let R: f32;
        let G: f32;
        let B: f32;

        if S <= 0.0 {
            R = V; G = V; B = V;
        } else {
            let hh = H * 6.0;
            let i = hh as i32;
            let ff = hh - i as f32;
            let p = V * (1.0 - S);
            let q = V * (1.0 - (S * ff));
            let t = V * (1.0 - (S * (1.0 - ff)));
            if i == 0 {
                R = V; G = t; B = p;
            } else if i == 1 {
                R = q; G = V; B = p;
            } else if i == 2 {
                R = p; G = V; B = t;
            } else if i == 3 {
                R = p; G = q; B = V;
            } else if i == 4 {
                R = t; G = p; B = V;
            } else {
                R = V; G = p; B = q;
            }
        }

        *RGB.add(0) = R;
        *RGB.add(1) = G;
        *RGB.add(2) = B;
    }
}

