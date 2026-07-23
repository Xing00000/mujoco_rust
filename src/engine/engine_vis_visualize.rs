//! Port of: engine/engine_vis_visualize.c
//! IR hash: 73a9f665ec0ecfc0
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
    // SAFETY: caller guarantees m valid, label points to buffer of at least 100 bytes
    unsafe {
        extern "C" { fn snprintf(s: *mut i8, n: usize, fmt: *const i8, ...) -> i32; }
        extern "C" { fn strncpy(dst: *mut i8, src: *const i8, n: usize) -> *mut i8; }

        let typestr = crate::engine::engine_util_misc::mju_type2str(r#type as i32);
        let namestr = crate::engine::engine_name::mj_id2name(m, r#type as i32, id);
        let mut txt: [i8; 100] = [0; 100];

        if !namestr.is_null() {
            snprintf(txt.as_mut_ptr(), 100, b"%s\0".as_ptr() as *const i8, namestr);
        } else if !typestr.is_null() {
            snprintf(txt.as_mut_ptr(), 100, b"%s %d\0".as_ptr() as *const i8, typestr, id);
        } else {
            snprintf(txt.as_mut_ptr(), 100, b"%d\0".as_ptr() as *const i8, id);
        }

        strncpy(label, txt.as_ptr(), 100);
        *label.add(99) = 0;
    }
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
    // SAFETY: scn is a valid mjvScene pointer (caller contract)
    unsafe {
        // check for overflow
        if (*scn).ngeom >= (*scn).maxgeom {
            if (*scn).status == 0 {
                crate::engine::engine_util_errmem::mju_warning(
                    b"Pre-allocated visual geom buffer is full. Increase maxgeom above %d.\0".as_ptr() as *const i8);
                (*scn).status = 1;
            }
            return std::ptr::null_mut();
        }

        let thisgeom = (*scn).geoms.add((*scn).ngeom as usize);
        // SAFETY: thisgeom points to valid mjvGeom in the geoms array
        std::ptr::write_bytes(thisgeom as *mut u8, 0, std::mem::size_of::<mjvGeom>());
        mjv_init_geom(thisgeom, 1001, std::ptr::null(), std::ptr::null(), std::ptr::null(), std::ptr::null());  // mjGEOM_NONE=1001
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
    // SAFETY: geom is a valid pointer-to-pointer, scn is valid (caller contract)
    unsafe {
        // check geom being released was most recently acquired
        if *geom != (*scn).geoms.add((*scn).ngeom as usize) {
            crate::engine::engine_util_errmem::mju_error(
                b"Unexpected geom pointer; did you call acquireGeom?\0".as_ptr() as *const i8);
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
pub fn add_triangle(scn: *mut mjvScene, v0: *const f64, v1: *const f64, v2: *const f64, rgba: *const f32, objid: i32, category: i32, objtype: i32) {
    // SAFETY: scn, v0, v1, v2, rgba are valid pointers (caller contract)
    unsafe {
        let mut thisgeom = acquire_geom(scn, objid, category, objtype);
        if thisgeom.is_null() {
            return;
        }
        let e1: [f64; 3] = [*v1.add(0) - *v0.add(0), *v1.add(1) - *v0.add(1), *v1.add(2) - *v0.add(2)];
        let e2: [f64; 3] = [*v2.add(0) - *v0.add(0), *v2.add(1) - *v0.add(1), *v2.add(2) - *v0.add(2)];
        let mut normal: [f64; 3] = [0.0; 3];
        crate::engine::engine_util_spatial::mju_cross(normal.as_mut_ptr(), e1.as_ptr(), e2.as_ptr());
        let mut e1_m = e1;
        let mut e2_m = e2;
        let lengths: [f64; 3] = [
            crate::engine::engine_util_blas::mju_normalize3(e1_m.as_mut_ptr()),
            crate::engine::engine_util_blas::mju_normalize3(e2_m.as_mut_ptr()),
            crate::engine::engine_util_blas::mju_normalize3(normal.as_mut_ptr()),
        ];
        let xmat: [f64; 9] = [
            e1_m[0], e2_m[0], normal[0],
            e1_m[1], e2_m[1], normal[1],
            e1_m[2], e2_m[2], normal[2],
        ];
        mjv_init_geom(thisgeom, 108, lengths.as_ptr(), v0, xmat.as_ptr(), rgba);  // mjGEOM_TRIANGLE=108
        release_geom(&mut thisgeom, scn);
    }
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
    const mjVIS_TEXTURE: usize = 1;
    const mjVIS_TRANSPARENT: usize = 18;
    const mjCAT_DYNAMIC: i32 = 2;

    // SAFETY: caller guarantees m, geom, rgba, flags are valid pointers
    unsafe {
        // set material properties if given
        if matid >= 0 {
            f2f(
                (*geom).rgba.as_mut_ptr(),
                (*m).mat_rgba.add(4 * matid as usize),
                4,
            );
            (*geom).emission = *(*m).mat_emission.add(matid as usize);
            (*geom).specular = *(*m).mat_specular.add(matid as usize);
            (*geom).shininess = *(*m).mat_shininess.add(matid as usize);
            (*geom).reflectance = *(*m).mat_reflectance.add(matid as usize);
        }

        // use rgba if different from default, or no material given
        if *rgba.add(0) != 0.5f32
            || *rgba.add(1) != 0.5f32
            || *rgba.add(2) != 0.5f32
            || *rgba.add(3) != 1.0f32
            || matid < 0
        {
            f2f((*geom).rgba.as_mut_ptr(), rgba, 4);
        }

        // set texture
        if *flags.add(mjVIS_TEXTURE) != 0 && matid >= 0 {
            (*geom).matid = matid;
        }

        // scale alpha for dynamic geoms only
        if *flags.add(mjVIS_TRANSPARENT) != 0 && (*geom).category == mjCAT_DYNAMIC {
            // alpha is at offset 16 in map (5th float: stiffness, stiffnessrot, force, torque, alpha)
            let map_ptr = (*m).vis.map.as_ptr() as *const f32;
            let alpha = *map_ptr.add(4);
            (*geom).rgba[3] *= alpha;
        }
    }
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
    // SAFETY: scn, from, to valid; rgba may be null (caller contract)
    unsafe {
        let mut thisgeom = acquire_geom(scn, objid, category, objtype);
        if thisgeom.is_null() {
            return;
        }
        mjv_connector(thisgeom, r#type, width, from, to);
        if !rgba.is_null() {
            f2f((*thisgeom).rgba.as_mut_ptr(), rgba, 4);
        }
        release_geom(&mut thisgeom, scn);
    }
}

/// C: markselected (engine/engine_vis_visualize.c:393)
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

/// C: addFrame (engine/engine_vis_visualize.c:400)
/// Calls: acquireGeom, mju_add3, mju_mulMatVec3, mjv_connector, releaseGeom
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_frame(scn: *mut mjvScene, objid: i32, pos: *const f64, rot: *const f64, length: f32, width: f32) {
    const MJCAT_DECOR: i32 = 4;
    const MJOBJ_UNKNOWN: i32 = 0;
    const MJGEOM_CYLINDER: i32 = 5;

    // SAFETY: scn, pos, rot are valid pointers (caller contract)
    unsafe {
        // draw separate geoms for each axis
        for j in 0..3_i32 {
            let mut axis: [f64; 3] = [0.0; 3];
            axis[j as usize] = length as f64;

            let mut vec: [f64; 3] = [0.0; 3];
            crate::engine::engine_util_blas::mju_mul_mat_vec3(vec.as_mut_ptr(), rot, axis.as_ptr());

            let mut to: [f64; 3] = [0.0; 3];
            crate::engine::engine_util_blas::mju_add3(to.as_mut_ptr(), pos, vec.as_ptr());

            let mut thisgeom = acquire_geom(scn, objid, MJCAT_DECOR, MJOBJ_UNKNOWN);
            if thisgeom.is_null() {
                return;
            }

            mjv_connector(thisgeom, MJGEOM_CYLINDER, width as f64, pos, to.as_ptr());
            for k in 0..3_usize {
                (*thisgeom).rgba[k] = if j as usize == k { 0.9 } else { 0.0 };
            }
            (*thisgeom).rgba[3] = 1.0;
            release_geom(&mut thisgeom, scn);
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
    const MJVIS_CONTACTPOINT: usize = 14;
    const MJVIS_CONTACTFORCE: usize = 16;
    const MJVIS_CONTACTSPLIT: usize = 17;
    const MJVIS_ISLAND: usize = 15;
    const MJFRAME_CONTACT: i32 = 6;
    const MJLABEL_CONTACTPOINT: i32 = 14;
    const MJLABEL_CONTACTFORCE: i32 = 15;
    const MJOBJ_UNKNOWN: i32 = 0;
    const MJOBJ_GEOM: i32 = 5;
    const MJOBJ_FLEX: i32 = 9;
    const MJCAT_DECOR: i32 = 4;
    const MJGEOM_CYLINDER: i32 = 5;
    const MJGEOM_ARROW: i32 = 100;
    const MJGEOM_ARROW2: i32 = 101;
    const MJMINVAL: f64 = 1e-15;

    // SAFETY: m, d, vopt, scn are valid pointers (caller contract).
    unsafe {
        extern "C" { fn snprintf(s: *mut i8, n: usize, fmt: *const i8, ...) -> i32; }

        if *(*vopt).flags.as_ptr().add(MJVIS_CONTACTPOINT) == 0
            && *(*vopt).flags.as_ptr().add(MJVIS_CONTACTFORCE) == 0
            && (*vopt).frame != MJFRAME_CONTACT
        {
            return;
        }

        let objtype: i32 = MJOBJ_UNKNOWN;
        let category: i32 = MJCAT_DECOR;
        let mut mat: [f64; 9] = [0.0; 9];
        let mut tmp: [f64; 9] = [0.0; 9];
        let mut vec: [f64; 3] = [0.0; 3];
        let mut frc: [f64; 3] = [0.0; 3];
        let mut confrc: [f64; 6] = [0.0; 6];
        let scl = (*m).stat.meansize;
        let scale_ptr = (*m).vis.scale.as_ptr() as *const f32;
        let rgba_ptr = (*m).vis.rgba._data.as_ptr() as *const f32;
        let map_ptr = (*m).vis.map.as_ptr() as *const f32;

        for i in 0..(*d).ncon {
            let con = (*d).contact.add(i as usize);

            // mat = contact rotation matrix (normal along z)
            crate::engine::engine_util_blas::mju_copy(
                tmp.as_mut_ptr(), (*con).frame.as_ptr().add(3), 6);
            crate::engine::engine_util_blas::mju_copy(
                tmp.as_mut_ptr().add(6), (*con).frame.as_ptr(), 3);
            crate::engine::engine_util_blas::mju_transpose(
                mat.as_mut_ptr(), tmp.as_ptr(), 3, 3);

            // contact point
            if *(*vopt).flags.as_ptr().add(MJVIS_CONTACTPOINT) != 0 {
                let mut thisgeom = acquire_geom(scn, i, category, objtype);
                if thisgeom.is_null() {
                    return;
                }

                (*thisgeom).r#type = MJGEOM_CYLINDER;
                // vis.scale: index 1 = contactwidth, index 2 = contactheight
                let contactwidth = *scale_ptr.add(1) as f64 * scl;
                let contactheight = *scale_ptr.add(2) as f64 * scl;
                (*thisgeom).size[0] = contactwidth as f32;
                (*thisgeom).size[1] = contactwidth as f32;
                let halfheight = contactheight as f32;
                let halfdepth = -(*con).dist as f32 / 2.0;
                (*thisgeom).size[2] = if halfheight > halfdepth { halfheight } else { halfdepth };
                crate::engine::engine_util_misc::mju_n2f(
                    (*thisgeom).pos.as_mut_ptr(), (*con).pos.as_ptr(), 3);
                crate::engine::engine_util_misc::mju_n2f(
                    (*thisgeom).mat.as_mut_ptr(), mat.as_ptr(), 9);

                let efc_adr = (*(*d).contact.add(i as usize)).efc_address;

                // override colors if visualizing islands
                if *(*vopt).flags.as_ptr().add(MJVIS_ISLAND) != 0 && efc_adr >= 0 {
                    let h = if (*d).nisland > 0 {
                        *(*d).island_dofadr.add(*(*d).efc_island.add(efc_adr as usize) as usize)
                    } else {
                        -1
                    };
                    island_color((*thisgeom).rgba.as_mut_ptr(), h, 1);
                } else {
                    if efc_adr >= 0 {
                        // rgba index 52 = contactpoint
                        f2f((*thisgeom).rgba.as_mut_ptr(), rgba_ptr.add(52), 4);
                    } else {
                        // rgba index 68 = contactgap
                        f2f((*thisgeom).rgba.as_mut_ptr(), rgba_ptr.add(68), 4);
                    }
                }

                // label contacting geom names or ids
                if (*vopt).label == MJLABEL_CONTACTPOINT {
                    let mut contactlabel: [[i8; 48]; 2] = [[0; 48]; 2];
                    for k in 0..2 {
                        if (*con).geom[k] >= 0 {
                            let geomname = crate::engine::engine_name::mj_id2name(
                                m, MJOBJ_GEOM, (*con).geom[k]);
                            if !geomname.is_null() {
                                snprintf(contactlabel[k].as_mut_ptr(), 48,
                                    b"%s\0".as_ptr() as *const i8, geomname);
                            } else {
                                snprintf(contactlabel[k].as_mut_ptr(), 48,
                                    b"g%d\0".as_ptr() as *const i8, (*con).geom[k]);
                            }
                        } else {
                            let flexname = crate::engine::engine_name::mj_id2name(
                                m, MJOBJ_FLEX, (*con).flex[k]);
                            if !flexname.is_null() {
                                if (*con).elem[k] >= 0 {
                                    snprintf(contactlabel[k].as_mut_ptr(), 48,
                                        b"%s.e%d\0".as_ptr() as *const i8,
                                        flexname, (*con).elem[k]);
                                } else {
                                    snprintf(contactlabel[k].as_mut_ptr(), 48,
                                        b"%s.v%d\0".as_ptr() as *const i8,
                                        flexname, (*con).vert[k]);
                                }
                            } else {
                                if (*con).elem[k] >= 0 {
                                    snprintf(contactlabel[k].as_mut_ptr(), 48,
                                        b"f%d.e%d\0".as_ptr() as *const i8,
                                        (*con).flex[k], (*con).elem[k]);
                                } else {
                                    snprintf(contactlabel[k].as_mut_ptr(), 48,
                                        b"f%d.v%d\0".as_ptr() as *const i8,
                                        (*con).flex[k], (*con).vert[k]);
                                }
                            }
                        }
                    }
                    snprintf((*thisgeom).label.as_mut_ptr(), 100,
                        b"%s | %s\0".as_ptr() as *const i8,
                        contactlabel[0].as_ptr(), contactlabel[1].as_ptr());
                }

                release_geom(&mut thisgeom, scn);
            }

            // mat = contact frame rotation matrix (normal along x)
            crate::engine::engine_util_blas::mju_transpose(
                mat.as_mut_ptr(), (*con).frame.as_ptr(), 3, 3);

            // contact frame
            if (*vopt).frame == MJFRAME_CONTACT {
                // vis.scale: index 12 = framelength, index 13 = framewidth
                let framelength = (*scale_ptr.add(12) as f64 * scl / 2.0) as f32;
                let framewidth = (*scale_ptr.add(13) as f64 * scl / 2.0) as f32;
                add_frame(scn, i, (*con).pos.as_ptr(), mat.as_ptr(), framelength, framewidth);
            }

            // nothing else to do for excluded contacts
            if (*(*d).contact.add(i as usize)).efc_address < 0 {
                continue;
            }

            // get contact force:torque in contact frame
            crate::engine::engine_core_util::mj_contact_force(
                m, d as *const mjData, i, confrc.as_mut_ptr());

            // contact force
            if *(*vopt).flags.as_ptr().add(MJVIS_CONTACTFORCE) != 0 {
                // get force, fill zeros if only normal
                crate::engine::engine_util_blas::mju_zero3(frc.as_mut_ptr());
                let dim_min = if 3 < (*con).dim { 3 } else { (*con).dim };
                crate::engine::engine_util_blas::mju_copy(
                    frc.as_mut_ptr(), confrc.as_ptr(), dim_min);
                if crate::engine::engine_util_blas::mju_norm3(frc.as_ptr()) < MJMINVAL {
                    continue;
                }

                // render combined or split
                let split: u8 = (*(*vopt).flags.as_ptr().add(MJVIS_CONTACTSPLIT) != 0
                    && (*con).dim > 1) as u8;
                let j_start = if split != 0 { 1 } else { 0 };
                let j_end = if split != 0 { 3 } else { 1 };
                for j in j_start..j_end {
                    // set vec to combined, normal or friction force, in world frame
                    if j == 0 {
                        // combined
                        crate::engine::engine_util_blas::mju_mul_mat_vec(
                            vec.as_mut_ptr(), mat.as_ptr(), frc.as_ptr(), 3, 3);
                    } else if j == 1 {
                        // normal
                        vec[0] = mat[0] * frc[0];
                        vec[1] = mat[3] * frc[0];
                        vec[2] = mat[6] * frc[0];
                    } else {
                        // friction
                        vec[0] = mat[1] * frc[1] + mat[2] * frc[2];
                        vec[1] = mat[4] * frc[1] + mat[5] * frc[2];
                        vec[2] = mat[7] * frc[1] + mat[8] * frc[2];
                    }

                    // scale vector: vis.map index 2 = force
                    let force_scale = *map_ptr.add(2) as f64 / (*m).stat.meanmass;
                    crate::engine::engine_util_blas::mju_scl3(
                        vec.as_mut_ptr(), vec.as_ptr(), force_scale);

                    // get bodyflex ids
                    let mut bf: [i32; 2] = [0; 2];
                    for k in 0..2 {
                        bf[k] = if (*con).geom[k] >= 0 {
                            *(*m).geom_bodyid.add((*con).geom[k] as usize)
                        } else {
                            (*m).nbody as i32 + (*con).flex[k]
                        };
                    }

                    // make sure arrow points towards bodyflex with higher id
                    if bf[0] > bf[1] {
                        crate::engine::engine_util_blas::mju_scl3(
                            vec.as_mut_ptr(), vec.as_ptr(), -1.0);
                    }

                    // one-directional arrow for friction and world, symmetric otherwise
                    let mut thisgeom = acquire_geom(scn, i, category, objtype);
                    if thisgeom.is_null() {
                        return;
                    }

                    let from = (*con).pos.as_ptr();
                    let mut to: [f64; 3] = [0.0; 3];
                    crate::engine::engine_util_blas::mju_add3(
                        to.as_mut_ptr(), from, vec.as_ptr());

                    let arrow_type = if bf[0] > 0 && bf[1] > 0 && split == 0 {
                        MJGEOM_ARROW2
                    } else {
                        MJGEOM_ARROW
                    };
                    // vis.scale: index 0 = forcewidth
                    let forcewidth = *scale_ptr.add(0) as f64 * scl;
                    mjv_connector(thisgeom, arrow_type, forcewidth, from, to.as_ptr());

                    // rgba index 60 = contactfriction, index 56 = contactforce
                    let color = if j == 2 { rgba_ptr.add(60) } else { rgba_ptr.add(56) };
                    f2f((*thisgeom).rgba.as_mut_ptr(), color, 4);

                    if (*vopt).label == MJLABEL_CONTACTFORCE
                        && j == (if split != 0 { 1 } else { 0 })
                    {
                        let norm_val = crate::engine::engine_util_blas::mju_norm3(frc.as_ptr());
                        snprintf((*thisgeom).label.as_mut_ptr(), 100,
                            b"%-.3g\0".as_ptr() as *const i8, norm_val);
                    }
                    release_geom(&mut thisgeom, scn);
                }
            }
        }
    }
}

/// C: addFlexGeoms (engine/engine_vis_visualize.c:748)
/// Calls: acquireGeom, islandColor, makeLabel, markselected, mj_sleepCycle, mjv_initGeom, releaseGeom, setMaterial
#[allow(unused_variables, non_snake_case)]
pub fn add_flex_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    const MJ_CAT_DYNAMIC: i32 = 2;
    const MJ_VIS_FLEXVERT: usize = 24;
    const MJ_VIS_FLEXEDGE: usize = 25;
    const MJ_VIS_FLEXFACE: usize = 26;
    const MJ_VIS_FLEXSKIN: usize = 27;
    const MJ_VIS_ISLAND: usize = 15;
    const MJ_OBJ_FLEX: i32 = 9;
    const MJ_GEOM_FLEX: i32 = 105;
    const MJ_LABEL_FLEX: i32 = 10;
    const MJ_NGROUP: i32 = 6;
    const MJ_ENBL_SLEEP: i32 = 1 << 4;

    // SAFETY: m, d, vopt, pert, scn are valid pointers (caller contract).
    unsafe {
        let category = MJ_CAT_DYNAMIC;
        if (category & catmask) == 0 {
            return;
        }
        if (*vopt).flags[MJ_VIS_FLEXVERT] == 0
            && (*vopt).flags[MJ_VIS_FLEXEDGE] == 0
            && (*vopt).flags[MJ_VIS_FLEXFACE] == 0
            && (*vopt).flags[MJ_VIS_FLEXSKIN] == 0
        {
            return;
        }

        for i in 0..(*m).nflex as i32 {
            let group = *(*m).flex_group.add(i as usize);
            let clamped = if 0 > (if (MJ_NGROUP - 1) < group { MJ_NGROUP - 1 } else { group }) {
                0
            } else {
                if (MJ_NGROUP - 1) < group { MJ_NGROUP - 1 } else { group }
            };
            if (*vopt).flexgroup[clamped as usize] == 0 {
                continue;
            }

            let mut thisgeom = acquire_geom(scn, i, category, MJ_OBJ_FLEX);
            if thisgeom.is_null() {
                return;
            }

            // construct geom, pos = first vertex
            mjv_init_geom(
                thisgeom,
                MJ_GEOM_FLEX,
                std::ptr::null(),
                (*d).flexvert_xpos.add(3 * *(*m).flex_vertadr.add(i as usize) as usize),
                std::ptr::null(),
                std::ptr::null(),
            );
            (*thisgeom).size[0] = *(*m).flex_radius.add(i as usize) as f32;
            set_material(
                m,
                thisgeom,
                *(*m).flex_matid.add(i as usize),
                (*m).flex_rgba.add(4 * i as usize),
                (*vopt).flags.as_ptr(),
            );

            // override if visualizing islands
            if (*vopt).flags[MJ_VIS_ISLAND] != 0 {
                // find first dynamic body in flex
                let mut bodyid: i32 = -1;
                if *(*m).flex_interp.add(i as usize) != 0 {
                    let nodeadr = *(*m).flex_nodeadr.add(i as usize);
                    let mut j = 0;
                    while j < *(*m).flex_nodenum.add(i as usize) && bodyid < 0 {
                        let b = *(*m).flex_nodebodyid.add((nodeadr + j) as usize);
                        if *(*m).body_treeid.add(b as usize) >= 0 {
                            bodyid = b;
                        }
                        j += 1;
                    }
                } else {
                    let vertadr = *(*m).flex_vertadr.add(i as usize);
                    let mut j = 0;
                    while j < *(*m).flex_vertnum.add(i as usize) && bodyid < 0 {
                        let b = *(*m).flex_vertbodyid.add((vertadr + j) as usize);
                        if *(*m).body_treeid.add(b as usize) >= 0 {
                            bodyid = b;
                        }
                        j += 1;
                    }
                }

                if bodyid >= 0 {
                    // strip material
                    (*thisgeom).matid = -1;

                    let weld_id = *(*m).body_weldid.add(bodyid as usize);
                    let dof = *(*m).body_dofadr.add(weld_id as usize);
                    let island = if (*d).nisland != 0 { *(*d).dof_island.add(dof as usize) } else { -1 };
                    let mut h = if island >= 0 { *(*d).island_dofadr.add(island as usize) } else { -1 };
                    let awake = *(*d).body_awake.add(bodyid as usize);

                    // if sleep is enabled, color by first tree dof
                    if h == -1 && ((*m).opt.enableflags & MJ_ENBL_SLEEP) != 0 {
                        let mut tree = *(*m).dof_treeid.add(dof as usize);
                        if awake == 0 {
                            tree = crate::engine::engine_sleep::mj_sleep_cycle(
                                (*d).tree_asleep, (*m).ntree as i32, tree);
                        }
                        h = *(*m).tree_dofadr.add(tree as usize);
                    }

                    island_color((*thisgeom).rgba.as_mut_ptr(), h, awake);
                }
            }

            // set texcoord
            if *(*m).flex_texcoordadr.add(i as usize) >= 0 {
                (*thisgeom).texcoord = 1;
            } else {
                (*thisgeom).matid = -1;
            }

            // glow flex if selected
            if (*pert).flexselect == i {
                markselected(
                    &(*m).vis as *const _ as *const mjVisual,
                    thisgeom,
                );
            }

            // skip if alpha is 0
            if (*thisgeom).rgba[3] == 0.0 {
                continue;
            }

            // label
            if (*vopt).label == MJ_LABEL_FLEX {
                make_label(m, MJ_OBJ_FLEX as u32, i, (*thisgeom).label.as_mut_ptr());
            }

            release_geom(&mut thisgeom, scn);
        }
    }
}

/// C: addSkinGeoms (engine/engine_vis_visualize.c:841)
/// Calls: acquireGeom, makeLabel, markselected, mjv_initGeom, releaseGeom, setMaterial
#[allow(unused_variables, non_snake_case)]
pub fn add_skin_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    const MJ_CAT_DYNAMIC: i32 = 2;
    const MJ_VIS_SKIN: usize = 23;
    const MJ_OBJ_SKIN: i32 = 11;
    const MJ_GEOM_SKIN: i32 = 106;
    const MJ_LABEL_SKIN: i32 = 11;
    const MJ_NGROUP: i32 = 6;

    // SAFETY: m, d, vopt, pert, scn are valid pointers (caller contract).
    unsafe {
        let category = MJ_CAT_DYNAMIC;
        if (category & catmask) == 0 {
            return;
        }
        if (*vopt).flags[MJ_VIS_SKIN] == 0 {
            return;
        }

        for i in 0..(*m).nskin as i32 {
            let group = *(*m).skin_group.add(i as usize);
            let clamped = if 0 > (if (MJ_NGROUP - 1) < group { MJ_NGROUP - 1 } else { group }) {
                0
            } else {
                if (MJ_NGROUP - 1) < group { MJ_NGROUP - 1 } else { group }
            };
            if (*vopt).skingroup[clamped as usize] == 0 {
                continue;
            }

            let mut thisgeom = acquire_geom(scn, i, category, MJ_OBJ_SKIN);
            if thisgeom.is_null() {
                return;
            }

            // construct geom, pos = first bone
            let bone_body = *(*m).skin_bonebodyid.add(
                *(*m).skin_boneadr.add(i as usize) as usize);
            mjv_init_geom(
                thisgeom,
                MJ_GEOM_SKIN,
                std::ptr::null(),
                (*d).xpos.add(3 * bone_body as usize),
                std::ptr::null(),
                std::ptr::null(),
            );

            // set material properties
            set_material(
                m,
                thisgeom,
                *(*m).skin_matid.add(i as usize),
                (*m).skin_rgba.add(4 * i as usize),
                (*vopt).flags.as_ptr(),
            );

            // glow skin if selected
            if (*pert).skinselect == i {
                markselected(
                    &(*m).vis as *const _ as *const mjVisual,
                    thisgeom,
                );
            }

            // set texcoord
            if *(*m).skin_texcoordadr.add(i as usize) >= 0 {
                (*thisgeom).texcoord = 1;
            }

            // skip if alpha is 0
            if (*thisgeom).rgba[3] == 0.0 {
                continue;
            }

            // label
            if (*vopt).label == MJ_LABEL_SKIN {
                make_label(m, MJ_OBJ_SKIN as u32, i, (*thisgeom).label.as_mut_ptr());
            }

            release_geom(&mut thisgeom, scn);
        }
    }
}

/// C: addGeomGeoms (engine/engine_vis_visualize.c:892)
/// Calls: acquireGeom, bodycategory, islandColor, makeLabel, markselected, mj_sleepCycle, mju_addToScl3, mju_copy3, mju_dot3, mju_n2f, mju_round, mju_transpose, mjv_initGeom, releaseGeom, setMaterial
#[allow(unused_variables, non_snake_case)]
pub fn add_geom_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    const MJOBJ_GEOM: i32 = 5;
    const MJGEOM_PLANE: i32 = 0;
    const MJGEOM_MESH: i32 = 7;
    const MJGEOM_SDF: i32 = 8;
    const MJNGROUP: i32 = 6;
    const MJLABEL_GEOM: i32 = 6;
    const MJVIS_ISLAND: usize = 25;
    const MJVIS_CONVEXHULL: usize = 14;
    const MJENBL_SLEEP: i32 = 1 << 4;
    const MJMAXPLANEGRID: i32 = 200;

    // SAFETY: m, d, vopt, pert, scn are valid pointers (caller contract).
    unsafe {
        let objtype: i32 = MJOBJ_GEOM;
        let mut planeid: i32 = -1;

        for i in 0..(*m).ngeom as i32 {
            // count planes
            if *(*m).geom_type.add(i as usize) == MJGEOM_PLANE {
                planeid += 1;
            }

            // skip if category is masked
            let category = bodycategory(m, *(*m).geom_bodyid.add(i as usize));
            if (category & catmask) == 0 {
                continue;
            }

            // skip if group is disabled
            let group_idx = {
                let g = *(*m).geom_group.add(i as usize);
                let clamped = if g < 0 { 0 } else if g > MJNGROUP - 1 { MJNGROUP - 1 } else { g };
                clamped as usize
            };
            if (*vopt).geomgroup[group_idx] == 0 {
                continue;
            }

            let mut thisgeom = acquire_geom(scn, i, category, objtype);
            if thisgeom.is_null() {
                return;
            }

            // construct geom
            mjv_init_geom(
                thisgeom,
                *(*m).geom_type.add(i as usize),
                (*m).geom_size.add(3 * i as usize),
                (*d).geom_xpos.add(3 * i as usize),
                (*d).geom_xmat.add(9 * i as usize),
                std::ptr::null(),
            );
            (*thisgeom).dataid = *(*m).geom_dataid.add(i as usize);

            // copy rbound
            (*thisgeom).modelrbound = *(*m).geom_rbound.add(i as usize) as f32;

            // set material properties
            let rgba = (*m).geom_rgba.add(4 * i as usize);
            let geom_matid = *(*m).geom_matid.add(i as usize);
            set_material(m, thisgeom, geom_matid, rgba, (*vopt).flags.as_ptr());

            // override if visualizing islands
            if *(*vopt).flags.as_ptr().add(MJVIS_ISLAND) != 0 {
                let weld_id = *(*m).body_weldid.add(*(*m).geom_bodyid.add(i as usize) as usize);
                if *(*m).body_dofnum.add(weld_id as usize) != 0 {
                    // strip materials off moving geom
                    (*thisgeom).matid = -1;

                    // set hue using first island dof
                    let dof = *(*m).body_dofadr.add(weld_id as usize);
                    let island = if (*d).nisland != 0 {
                        *(*d).dof_island.add(dof as usize)
                    } else {
                        -1
                    };
                    let mut h = if island >= 0 {
                        *(*d).island_dofadr.add(island as usize)
                    } else {
                        -1
                    };
                    let awake = *(*d).body_awake.add(*(*m).geom_bodyid.add(i as usize) as usize);

                    // if sleep is enabled, color by first tree dof
                    if h == -1 && ((*m).opt.enableflags & MJENBL_SLEEP) != 0 {
                        let mut tree = *(*m).dof_treeid.add(dof as usize);
                        if awake == 0 {
                            tree = crate::engine::engine_sleep::mj_sleep_cycle(
                                (*d).tree_asleep, (*m).ntree as i32, tree,
                            );
                        }
                        h = *(*m).tree_dofadr.add(tree as usize);
                    }

                    island_color((*thisgeom).rgba.as_mut_ptr(), h, awake);
                }
            }

            // set texcoord
            if (*(*m).geom_type.add(i as usize) == MJGEOM_MESH
                || *(*m).geom_type.add(i as usize) == MJGEOM_SDF)
                && *(*m).geom_dataid.add(i as usize) >= 0
                && *(*m).mesh_texcoordadr.add(*(*m).geom_dataid.add(i as usize) as usize) >= 0
            {
                (*thisgeom).texcoord = 1;
            }

            // skip if alpha is 0
            if (*thisgeom).rgba[3] == 0.0 {
                continue;
            }

            // glow geoms of selected body
            if (*pert).select > 0
                && (*pert).select == *(*m).geom_bodyid.add(i as usize)
            {
                markselected(&(*m).vis as *const _ as *const mjVisual, thisgeom);
            }

            // vopt->label
            if (*vopt).label == MJLABEL_GEOM {
                make_label(m, MJOBJ_GEOM as u32, i, (*thisgeom).label.as_mut_ptr());
            }

            // mesh: 2*i is original, 2*i+1 is convex hull
            if *(*m).geom_type.add(i as usize) == MJGEOM_MESH
                || *(*m).geom_type.add(i as usize) == MJGEOM_SDF
            {
                (*thisgeom).dataid *= 2;
                if *(*m).mesh_graphadr.add(*(*m).geom_dataid.add(i as usize) as usize) >= 0
                    && *(*vopt).flags.as_ptr().add(MJVIS_CONVEXHULL) != 0
                    && (*(*m).geom_contype.add(i as usize) != 0
                        || *(*m).geom_conaffinity.add(i as usize) != 0)
                {
                    (*thisgeom).dataid += 1;
                }
            }
            // plane
            else if *(*m).geom_type.add(i as usize) == MJGEOM_PLANE {
                (*thisgeom).dataid = planeid;

                // save initial pos
                let mut tmp: [f64; 9] = [0.0; 9];
                crate::engine::engine_util_blas::mju_copy3(
                    tmp.as_mut_ptr(),
                    (*d).geom_xpos.add(3 * i as usize),
                );

                // re-center infinite plane
                if *(*m).geom_size.add(3 * i as usize) <= 0.0
                    || *(*m).geom_size.add(3 * i as usize + 1) <= 0.0
                {
                    // vec = headpos - geompos
                    let mut vec: [f64; 3] = [0.0; 3];
                    for j in 0..3 {
                        vec[j] = 0.5
                            * ((*scn).camera[0].pos[j] as f64
                                + (*scn).camera[1].pos[j] as f64)
                            - *(*d).geom_xpos.add(3 * i as usize + j);
                    }

                    // construct axes
                    let mut ax: [f64; 9] = [0.0; 9];
                    crate::engine::engine_util_blas::mju_transpose(
                        ax.as_mut_ptr(),
                        (*d).geom_xmat.add(9 * i as usize),
                        3,
                        3,
                    );

                    // loop over (x,y)
                    for k in 0..2usize {
                        if *(*m).geom_size.add(3 * i as usize + k) <= 0.0 {
                            // compute zfar: offset 32 in vis.map (13 floats: zfar is at index 8)
                            let map_ptr = (*m).vis.map.as_ptr() as *const f32;
                            let zfar = *map_ptr.add(8) as f64 * (*m).stat.extent;

                            // get size increment
                            let sX: f64;
                            let matid = *(*m).geom_matid.add(i as usize);
                            if matid >= 0
                                && *(*m).mat_texrepeat.add(2 * matid as usize + k) > 0.0
                            {
                                sX = 2.0
                                    / *(*m).mat_texrepeat.add(2 * matid as usize + k) as f64;
                            } else {
                                sX = 2.1 * zfar / (MJMAXPLANEGRID - 2) as f64;
                            }

                            // project on frame, round to integer increment of size
                            let dX_raw = crate::engine::engine_util_blas::mju_dot3(
                                vec.as_ptr(),
                                ax.as_ptr().add(3 * k),
                            );
                            let dX = 2.0
                                * sX
                                * crate::engine::engine_util_misc::mju_round(
                                    0.5 * dX_raw / sX,
                                ) as f64;

                            // translate
                            crate::engine::engine_util_blas::mju_add_to_scl3(
                                tmp.as_mut_ptr(),
                                ax.as_ptr().add(3 * k),
                                dX,
                            );
                        }
                    }
                }

                // set final pos
                crate::engine::engine_util_misc::mju_n2f(
                    (*thisgeom).pos.as_mut_ptr(),
                    tmp.as_ptr(),
                    3,
                );
            }

            release_geom(&mut thisgeom, scn);
        }
    }
}

/// C: addSiteGeoms (engine/engine_vis_visualize.c:1041)
/// Calls: acquireGeom, bodycategory, makeLabel, markselected, mjv_initGeom, releaseGeom, setMaterial
#[allow(unused_variables, non_snake_case)]
pub fn add_site_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    const MJ_OBJ_SITE: i32 = 6;
    const MJ_LABEL_SITE: i32 = 4;
    const MJ_NGROUP: i32 = 6;

    // SAFETY: m, d, vopt, pert, scn are valid pointers (caller contract).
    unsafe {
        for i in 0..(*m).nsite as i32 {
            // skip if category is masked
            let category = bodycategory(m, *(*m).site_bodyid.add(i as usize));
            if (category & catmask) == 0 {
                continue;
            }

            // skip if group disabled
            let group = *(*m).site_group.add(i as usize);
            let clamped = if 0 > (if (MJ_NGROUP - 1) < group { MJ_NGROUP - 1 } else { group }) {
                0
            } else {
                if (MJ_NGROUP - 1) < group { MJ_NGROUP - 1 } else { group }
            };
            if (*vopt).sitegroup[clamped as usize] == 0 {
                continue;
            }

            let mut thisgeom = acquire_geom(scn, i, category, MJ_OBJ_SITE);
            if thisgeom.is_null() {
                return;
            }

            // construct geom
            mjv_init_geom(
                thisgeom,
                *(*m).site_type.add(i as usize),
                (*m).site_size.add(3 * i as usize),
                (*d).site_xpos.add(3 * i as usize),
                (*d).site_xmat.add(9 * i as usize),
                std::ptr::null(),
            );

            // set material if given
            set_material(
                m,
                thisgeom,
                *(*m).site_matid.add(i as usize),
                (*m).site_rgba.add(4 * i as usize),
                (*vopt).flags.as_ptr(),
            );

            // skip if alpha is 0
            if (*thisgeom).rgba[3] == 0.0 {
                continue;
            }

            // glow
            if (*pert).select > 0 && (*pert).select == *(*m).site_bodyid.add(i as usize) {
                markselected(
                    &(*m).vis as *const _ as *const mjVisual,
                    thisgeom,
                );
            }

            // label
            if (*vopt).label == MJ_LABEL_SITE {
                make_label(m, MJ_OBJ_SITE as u32, i, (*thisgeom).label.as_mut_ptr());
            }

            release_geom(&mut thisgeom, scn);
        }
    }
}

/// C: addSpatialTendonGeoms (engine/engine_vis_visualize.c:1141)
/// Calls: acquireGeom, f2f, islandColor, makeLabel, mju_copy3, mjv_catenary, mjv_connector, mjv_isCatenary, releaseGeom, setMaterial
#[allow(unused_variables, non_snake_case)]
pub fn add_spatial_tendon_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, catmask: i32, scn: *mut mjvScene) {
    const MJ_CAT_DYNAMIC: i32 = 2;
    const MJ_VIS_TENDON: usize = 7;
    const MJ_VIS_ISLAND: usize = 15;
    const MJ_OBJ_TENDON: i32 = 18;
    const MJ_GEOM_CAPSULE: i32 = 3;
    const MJ_CNSTR_LIMIT_TENDON: i32 = 4;
    const MJ_LABEL_TENDON: i32 = 7;
    const MJ_NGROUP: i32 = 6;

    // SAFETY: m, d, vopt, scn are valid pointers (caller contract).
    unsafe {
        let category = MJ_CAT_DYNAMIC;
        if (category & catmask) == 0 {
            return;
        }
        if (*vopt).flags[MJ_VIS_TENDON] == 0 {
            return;
        }

        let rgba_floats = (*m).vis.rgba._data.as_ptr() as *const f32;
        let constraint_rgba = rgba_floats.add(19 * 4);

        for i in 0..(*m).ntendon as i32 {
            let group = *(*m).tendon_group.add(i as usize);
            let clamped = if 0 > (if (MJ_NGROUP - 1) < group { MJ_NGROUP - 1 } else { group }) {
                0
            } else {
                if (MJ_NGROUP - 1) < group { MJ_NGROUP - 1 } else { group }
            };
            if (*vopt).tendongroup[clamped as usize] == 0 {
                continue;
            }

            let mut length: f64 = 0.0;
            let draw_catenary = mjv_is_catenary(m, d as *const mjData, i, &mut length);

            // conditions not met: draw straight lines
            if draw_catenary == 0 {
                let wrapadr = *(*d).ten_wrapadr.add(i as usize);
                let wrapnum = *(*d).ten_wrapnum.add(i as usize);
                for j in wrapadr..(wrapadr + wrapnum - 1) {
                    if *(*d).wrap_obj.add(j as usize) != -2
                        && *(*d).wrap_obj.add((j + 1) as usize) != -2
                    {
                        let mut thisgeom = acquire_geom(scn, i, category, MJ_OBJ_TENDON);
                        if thisgeom.is_null() {
                            return;
                        }

                        // determine width
                        let width: f64 = if *(*d).wrap_obj.add(j as usize) >= 0
                            && *(*d).wrap_obj.add((j + 1) as usize) >= 0
                        {
                            0.5 * *(*m).tendon_width.add(i as usize)
                        } else {
                            *(*m).tendon_width.add(i as usize)
                        };

                        // construct geom
                        mjv_connector(
                            thisgeom, MJ_GEOM_CAPSULE, width,
                            (*d).wrap_xpos.add(3 * j as usize),
                            (*d).wrap_xpos.add(3 * (j + 1) as usize),
                        );

                        // set material properties
                        let tendon_matid = *(*m).tendon_matid.add(i as usize);
                        let mut rgba: [f32; 4] = [0.0; 4];
                        f2f(rgba.as_mut_ptr(), (*m).tendon_rgba.add(4 * i as usize), 4);

                        // if tendon has no material and color is default gray, re-color using limit impedance
                        if tendon_matid == -1 && rgba[0] == 0.5 && rgba[1] == 0.5
                            && rgba[2] == 0.5 && rgba[3] == 1.0
                        {
                            let mut imp: f64 = 0.0;
                            let efc_start = (*d).ne + (*d).nf;
                            let efc_end = efc_start + (*d).nl;
                            for k in efc_start..efc_end {
                                if *(*d).efc_type.add(k as usize) == MJ_CNSTR_LIMIT_TENDON
                                    && *(*d).efc_id.add(k as usize) == i
                                {
                                    imp = *(*d).efc_KBIP.add(4 * k as usize + 2);
                                }
                            }

                            rgba[0] = (1.0 - imp as f32) * rgba[0] + imp as f32 * *constraint_rgba.add(0);
                            rgba[1] = (1.0 - imp as f32) * rgba[1] + imp as f32 * *constraint_rgba.add(1);
                            rgba[2] = (1.0 - imp as f32) * rgba[2] + imp as f32 * *constraint_rgba.add(2);
                        }

                        set_material(m, thisgeom, tendon_matid, rgba.as_ptr(), (*vopt).flags.as_ptr());

                        // override if visualizing islands
                        if (*vopt).flags[MJ_VIS_ISLAND] != 0 {
                            (*thisgeom).matid = -1;
                            let mut h: i32 = -1;
                            if (*d).nisland != 0 && *(*d).tendon_efcadr.add(i as usize) >= 0 {
                                h = *(*d).island_dofadr.add(
                                    *(*d).efc_island.add(*(*d).tendon_efcadr.add(i as usize) as usize) as usize);
                            }
                            island_color((*thisgeom).rgba.as_mut_ptr(), h, 1);
                        }

                        // label: only the first segment
                        if (*vopt).label == MJ_LABEL_TENDON && j == wrapadr {
                            make_label(m, MJ_OBJ_TENDON as u32, i, (*thisgeom).label.as_mut_ptr());
                        }

                        release_geom(&mut thisgeom, scn);
                    }
                }
            }
            // special case: catenary
            else {
                // two hanging points
                let mut x0: [f64; 3] = [0.0; 3];
                let mut x1: [f64; 3] = [0.0; 3];
                let wrapadr = *(*d).ten_wrapadr.add(i as usize);
                crate::engine::engine_util_blas::mju_copy3(
                    x0.as_mut_ptr(), (*d).wrap_xpos.add(3 * wrapadr as usize));
                crate::engine::engine_util_blas::mju_copy3(
                    x1.as_mut_ptr(), (*d).wrap_xpos.add(3 * (wrapadr + 1) as usize));

                // get number of points along catenary path (capped at 100)
                let quality_ints = (*m).vis.quality.as_ptr() as *const i32;
                let numslices = *quality_ints.add(2);
                let ncatenary = if numslices + 1 < 100 { numslices + 1 } else { 100 };
                let mut catenary: [f64; 300] = [0.0; 300];

                // points along catenary path
                let npoints = mjv_catenary(
                    x0.as_ptr(), x1.as_ptr(),
                    (*m).opt.gravity.as_ptr(),
                    length, catenary.as_mut_ptr(), ncatenary);

                // draw npoints-1 segments
                for j in 0..(npoints - 1) {
                    let mut thisgeom = acquire_geom(scn, i, category, MJ_OBJ_TENDON);
                    if thisgeom.is_null() {
                        return;
                    }

                    // construct geom
                    mjv_connector(
                        thisgeom, MJ_GEOM_CAPSULE,
                        *(*m).tendon_width.add(i as usize),
                        catenary.as_ptr().add(3 * j as usize),
                        catenary.as_ptr().add(3 * (j + 1) as usize),
                    );

                    // set material if given
                    set_material(m, thisgeom, *(*m).tendon_matid.add(i as usize),
                        (*m).tendon_rgba.add(4 * i as usize), (*vopt).flags.as_ptr());

                    // label: only the first segment
                    if (*vopt).label == MJ_LABEL_TENDON && (npoints / 2) != 0 {
                        make_label(m, MJ_OBJ_TENDON as u32, i, (*thisgeom).label.as_mut_ptr());
                    }

                    release_geom(&mut thisgeom, scn);
                }
            }
        }
    }
}

/// C: addSliderCrankGeoms (engine/engine_vis_visualize.c:1266)
/// Calls: acquireGeom, f2f, makeLabel, mju_addTo3, mju_dot3, mju_scl3, mju_sub, mjv_connector, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_slider_crank_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, catmask: i32, scn: *mut mjvScene) {
    const MJ_CAT_DYNAMIC: i32 = 2;
    const MJ_TRN_SLIDERCRANK: i32 = 2;
    const MJ_OBJ_ACTUATOR: i32 = 19;
    const MJ_GEOM_CYLINDER: i32 = 5;
    const MJ_GEOM_CAPSULE: i32 = 3;
    const MJ_LABEL_ACTUATOR: i32 = 8;

    // SAFETY: m, d, vopt, scn are valid pointers (caller contract).
    unsafe {
        let category = MJ_CAT_DYNAMIC;
        if (category & catmask) == 0 {
            return;
        }

        let scl = (*m).stat.meansize as f32;
        let scale_floats = (*m).vis.scale.as_ptr() as *const f32;
        let slidercrank_scale = *scale_floats.add(15);

        let rgba_floats = (*m).vis.rgba._data.as_ptr() as *const f32;
        let slidercrank_rgba = rgba_floats.add(20 * 4);
        let crankbroken_rgba = rgba_floats.add(21 * 4);

        for i in 0..(*m).nu as i32 {
            if *(*m).actuator_trntype.add(i as usize) != MJ_TRN_SLIDERCRANK {
                continue;
            }

            // get data
            let j = *(*m).actuator_trnid.add(2 * i as usize);      // crank
            let k = *(*m).actuator_trnid.add(2 * i as usize + 1);  // slider
            let rod = *(*m).actuator_cranklength.add(i as usize);
            let mut axis: [f64; 3] = [0.0; 3];
            axis[0] = *(*d).site_xmat.add(9 * k as usize + 2);
            axis[1] = *(*d).site_xmat.add(9 * k as usize + 5);
            axis[2] = *(*d).site_xmat.add(9 * k as usize + 8);

            // compute crank length
            let mut vec: [f64; 3] = [0.0; 3];
            crate::engine::engine_util_blas::mju_sub(
                vec.as_mut_ptr(),
                (*d).site_xpos.add(3 * j as usize),
                (*d).site_xpos.add(3 * k as usize),
                3,
            );
            let mut len = crate::engine::engine_util_blas::mju_dot3(vec.as_ptr(), axis.as_ptr());
            let mut det = len * len + rod * rod
                - crate::engine::engine_util_blas::mju_dot3(vec.as_ptr(), vec.as_ptr());
            let mut broken: u8 = 0;
            if det < 0.0 {
                det = 0.0;
                broken = 1;
            }
            len = len - f64::sqrt(det);

            // compute slider endpoint
            let mut end: [f64; 3] = [0.0; 3];
            crate::engine::engine_util_blas::mju_scl3(end.as_mut_ptr(), axis.as_ptr(), len);
            crate::engine::engine_util_blas::mju_add_to3(
                end.as_mut_ptr(),
                (*d).site_xpos.add(3 * k as usize),
            );

            // render slider
            let mut thisgeom = acquire_geom(scn, i, category, MJ_OBJ_ACTUATOR);
            if thisgeom.is_null() {
                return;
            }

            mjv_connector(
                thisgeom,
                MJ_GEOM_CYLINDER,
                scl as f64 * slidercrank_scale as f64,
                (*d).site_xpos.add(3 * k as usize),
                end.as_ptr(),
            );
            f2f((*thisgeom).rgba.as_mut_ptr(), slidercrank_rgba, 4);
            if (*vopt).label == MJ_LABEL_ACTUATOR {
                make_label(m, MJ_OBJ_ACTUATOR as u32, i, (*thisgeom).label.as_mut_ptr());
            }
            release_geom(&mut thisgeom, scn);

            thisgeom = acquire_geom(scn, i, category, MJ_OBJ_ACTUATOR);
            if thisgeom.is_null() {
                return;
            }

            mjv_connector(
                thisgeom,
                MJ_GEOM_CAPSULE,
                scl as f64 * slidercrank_scale as f64 / 2.0,
                end.as_ptr(),
                (*d).site_xpos.add(3 * j as usize),
            );
            if broken != 0 {
                f2f((*thisgeom).rgba.as_mut_ptr(), crankbroken_rgba, 4);
            } else {
                f2f((*thisgeom).rgba.as_mut_ptr(), slidercrank_rgba, 4);
            }
            release_geom(&mut thisgeom, scn);
        }
    }
}

/// C: addGeomFrameGeoms (engine/engine_vis_visualize.c:1334)
/// Calls: addFrame, bodycategory
#[allow(unused_variables, non_snake_case)]
pub fn add_geom_frame_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, catmask: i32, scn: *mut mjvScene) {
    const MJ_FRAME_GEOM: i32 = 2;
    const MJ_NGROUP: usize = 6;

    // SAFETY: m, d, vopt, scn are valid pointers (caller contract).
    unsafe {
        if (*vopt).frame != MJ_FRAME_GEOM {
            return;
        }

        let scl = (*m).stat.meansize as f32;

        // Access vis.scale as a float array. framelength is index 12, framewidth is index 13.
        let scale_floats = (*m).vis.scale.as_ptr() as *const f32;
        let framelength = *scale_floats.add(12);
        let framewidth = *scale_floats.add(13);

        for i in 0..(*m).ngeom as i32 {
            if bodycategory(m, *(*m).geom_bodyid.add(i as usize)) & catmask == 0 {
                continue;
            }

            let group = *(*m).geom_group.add(i as usize) as usize;
            let clamped_group = if group < MJ_NGROUP { group } else { MJ_NGROUP - 1 };
            if (*vopt).geomgroup[clamped_group] == 0 {
                continue;
            }

            // base element is invisible; don't show decors
            let matid = *(*m).geom_matid.add(i as usize);
            let rgba: *const f32 = if matid >= 0 {
                (*m).mat_rgba.add(4 * matid as usize)
            } else {
                (*m).geom_rgba.add(4 * i as usize)
            };
            if *rgba.add(3) == 0.0 {
                continue;
            }

            // construct geom frame
            let width = framewidth * scl;
            let length = framelength * scl;
            add_frame(scn, i, (*d).geom_xpos.add(3 * i as usize), (*d).geom_xmat.add(9 * i as usize), length, width);
        }
    }
}

/// C: addSiteFrameGeoms (engine/engine_vis_visualize.c:1364)
/// Calls: addFrame, bodycategory
#[allow(unused_variables, non_snake_case)]
pub fn add_site_frame_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, catmask: i32, scn: *mut mjvScene) {
    const MJ_FRAME_SITE: i32 = 3;
    const MJ_NGROUP: usize = 6;

    // SAFETY: m, d, vopt, scn are valid pointers (caller contract).
    unsafe {
        if (*vopt).frame != MJ_FRAME_SITE {
            return;
        }

        let scl = (*m).stat.meansize as f32;

        // Access vis.scale as a float array. framelength is index 12, framewidth is index 13.
        let scale_floats = (*m).vis.scale.as_ptr() as *const f32;
        let framelength = *scale_floats.add(12);
        let framewidth = *scale_floats.add(13);

        for i in 0..(*m).nsite as i32 {
            if bodycategory(m, *(*m).site_bodyid.add(i as usize)) & catmask == 0 {
                continue;
            }

            let group = *(*m).site_group.add(i as usize) as usize;
            let clamped_group = if group < MJ_NGROUP { group } else { MJ_NGROUP - 1 };
            if (*vopt).sitegroup[clamped_group] == 0 {
                continue;
            }

            // base element is invisible; don't show decors
            let matid = *(*m).site_matid.add(i as usize);
            let rgba: *const f32 = if matid >= 0 {
                (*m).mat_rgba.add(4 * matid as usize)
            } else {
                (*m).site_rgba.add(4 * i as usize)
            };
            if *rgba.add(3) == 0.0 {
                continue;
            }

            // construct site frame
            let width = framewidth * scl;
            let length = framelength * scl;
            add_frame(scn, i, (*d).site_xpos.add(3 * i as usize), (*d).site_xmat.add(9 * i as usize), length, width);
        }
    }
}

/// C: addBodyBvhGeoms (engine/engine_vis_visualize.c:1394)
/// Calls: acquireGeom, mju_addTo3, mju_mulMatVec3, mjv_initGeom, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_body_bvh_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    const MJ_VIS_BODYBVH: usize = 28;
    const MJ_CAT_DECOR: i32 = 4;
    const MJ_OBJ_UNKNOWN: i32 = 0;
    const MJ_GEOM_LINEBOX: i32 = 104;

    // SAFETY: m, d, vopt, scn are valid pointers (caller contract).
    unsafe {
        if (*vopt).flags[MJ_VIS_BODYBVH] == 0 {
            return;
        }

        for i in 0..(*m).nbvhstatic as i32 {
            let isleaf = *(*m).bvh_child.add(2 * i as usize) == -1
                && *(*m).bvh_child.add(2 * i as usize + 1) == -1;
            if *(*m).bvh_depth.add(i as usize) != (*vopt).bvh_depth {
                if !isleaf || *(*m).bvh_depth.add(i as usize) > (*vopt).bvh_depth {
                    continue;
                }
            }

            // find body number
            let mut bodyid: i32 = 0;
            let geomid = *(*m).bvh_nodeid.add(i as usize);
            while i >= *(*m).body_bvhadr.add(bodyid as usize) + *(*m).body_bvhnum.add(bodyid as usize) {
                bodyid += 1;
                if bodyid >= (*m).nbody as i32 {
                    break;
                }
            }

            // stop after body bvh are finished
            if bodyid >= (*m).nbody as i32 {
                break;
            }

            // get xpos, xmat, size
            let xpos: *const f64 = if isleaf {
                (*d).geom_xpos.add(3 * geomid as usize)
            } else {
                (*d).xipos.add(3 * bodyid as usize)
            };
            let xmat: *const f64 = if isleaf {
                (*d).geom_xmat.add(9 * geomid as usize)
            } else {
                (*d).ximat.add(9 * bodyid as usize)
            };
            let size: *const f64 = if isleaf {
                (*m).geom_aabb.add(6 * geomid as usize + 3)
            } else {
                (*m).bvh_aabb.add(6 * i as usize + 3)
            };

            // offset xpos with aabb center (not always at frame origin)
            let center: *const f64 = if isleaf {
                (*m).geom_aabb.add(6 * geomid as usize)
            } else {
                (*m).bvh_aabb.add(6 * i as usize)
            };
            let mut pos: [f64; 3] = [0.0; 3];
            crate::engine::engine_util_blas::mju_mul_mat_vec3(pos.as_mut_ptr(), xmat, center);
            crate::engine::engine_util_blas::mju_add_to3(pos.as_mut_ptr(), xpos);

            // set box color
            // vis.rgba: bv is index 22, bvactive is index 23
            let rgba_floats = (*m).vis.rgba._data.as_ptr() as *const f32;
            let mut rgba: *const f32 = rgba_floats.add(22 * 4);

            // vis.global.bvactive is at byte offset 48 (i32 at index 12 in global)
            let global_ints = (*m).vis.global.as_ptr() as *const i32;
            if *global_ints.add(12) != 0 && *(*d).bvh_active.add(i as usize) {
                rgba = rgba_floats.add(23 * 4);
            }

            let mut thisgeom = acquire_geom(scn, i, MJ_CAT_DECOR, MJ_OBJ_UNKNOWN);
            if thisgeom.is_null() {
                return;
            }

            mjv_init_geom(thisgeom, MJ_GEOM_LINEBOX, size, pos.as_ptr(), xmat, rgba);
            release_geom(&mut thisgeom, scn);
        }
    }
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
    const MJ_VIS_MESHBVH: usize = 29;
    const MJ_CAT_DECOR: i32 = 4;
    const MJ_OBJ_UNKNOWN: i32 = 0;
    const MJ_GEOM_LINEBOX: i32 = 104;
    const MJ_GEOM_SDF: i32 = 8;

    // SAFETY: m, d, vopt, scn are valid pointers (caller contract).
    unsafe {
        if (*vopt).flags[MJ_VIS_MESHBVH] == 0 {
            return;
        }

        let rgba_floats = (*m).vis.rgba._data.as_ptr() as *const f32;
        let bv_rgba = rgba_floats.add(22 * 4);
        let bvactive_rgba = rgba_floats.add(23 * 4);
        let global_ints = (*m).vis.global.as_ptr() as *const i32;
        let bvactive_flag = *global_ints.add(12) != 0;

        for geomid in 0..(*m).ngeom as i32 {
            let meshid = *(*m).geom_dataid.add(geomid as usize);
            // skip if not a mesh or if there is an octree
            if meshid == -1 || *(*m).geom_type.add(geomid as usize) as i32 == MJ_GEOM_SDF
                || *(*m).mesh_octadr.add(meshid as usize) >= 0
            {
                continue;
            }

            for b in 0..*(*m).mesh_bvhnum.add(meshid as usize) {
                let i = b + *(*m).mesh_bvhadr.add(meshid as usize);
                let isleaf = *(*m).bvh_child.add(2 * i as usize) == -1
                    && *(*m).bvh_child.add(2 * i as usize + 1) == -1;
                if *(*m).bvh_depth.add(i as usize) != (*vopt).bvh_depth {
                    if !isleaf || *(*m).bvh_depth.add(i as usize) > (*vopt).bvh_depth {
                        continue;
                    }
                }

                // box color
                let mut rgba: *const f32 = bv_rgba;
                if bvactive_flag {
                    if *(*d).bvh_active.add(i as usize) {
                        rgba = bvactive_rgba;
                    } else {
                        continue;
                    }
                }

                // get xpos, xmat, size
                let xpos = (*d).geom_xpos.add(3 * geomid as usize);
                let xmat = (*d).geom_xmat.add(9 * geomid as usize);
                let size = (*m).bvh_aabb.add(6 * i as usize + 3);

                // offset xpos with aabb center
                let center = (*m).bvh_aabb.add(6 * i as usize);
                let mut pos: [f64; 3] = [0.0; 3];
                crate::engine::engine_util_blas::mju_mul_mat_vec3(pos.as_mut_ptr(), xmat, center);
                crate::engine::engine_util_blas::mju_add_to3(pos.as_mut_ptr(), xpos);

                let mut thisgeom = acquire_geom(scn, i, MJ_CAT_DECOR, MJ_OBJ_UNKNOWN);
                if thisgeom.is_null() {
                    return;
                }
                mjv_init_geom(thisgeom, MJ_GEOM_LINEBOX, size, pos.as_ptr(), xmat, rgba);
                release_geom(&mut thisgeom, scn);
            }
        }
    }
}

/// C: addMeshOctreeGeoms (engine/engine_vis_visualize.c:1634)
/// Calls: acquireGeom, mju_addTo3, mju_mulMatVec3, mjv_initGeom, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_mesh_octree_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    const MJ_VIS_MESHBVH: usize = 29;
    const MJ_CAT_DECOR: i32 = 4;
    const MJ_OBJ_UNKNOWN: i32 = 0;
    const MJ_GEOM_LINEBOX: i32 = 104;
    const MJ_GEOM_HFIELD: i32 = 1;

    // SAFETY: m, d, vopt, scn are valid pointers (caller contract).
    unsafe {
        if (*vopt).flags[MJ_VIS_MESHBVH] == 0 {
            return;
        }

        let rgba_floats = (*m).vis.rgba._data.as_ptr() as *const f32;
        let bv_rgba = rgba_floats.add(22 * 4);

        for geomid in 0..(*m).ngeom as i32 {
            let meshid = *(*m).geom_dataid.add(geomid as usize);
            if meshid == -1 || *(*m).geom_type.add(geomid as usize) as i32 == MJ_GEOM_HFIELD
                || *(*m).mesh_octadr.add(meshid as usize) == -1
            {
                continue;
            }

            for b in 0..*(*m).mesh_octnum.add(meshid as usize) {
                let i = b + *(*m).mesh_octadr.add(meshid as usize);
                if *(*m).oct_depth.add(i as usize) != (*vopt).bvh_depth {
                    continue;
                }

                let mut thisgeom = acquire_geom(scn, i, MJ_CAT_DECOR, MJ_OBJ_UNKNOWN);
                if thisgeom.is_null() {
                    return;
                }

                let rgba: *const f32 = bv_rgba;
                let xpos = (*d).geom_xpos.add(3 * geomid as usize);
                let xmat = (*d).geom_xmat.add(9 * geomid as usize);
                let size = (*m).oct_aabb.add(6 * i as usize + 3);

                // offset xpos with aabb center
                let center = (*m).oct_aabb.add(6 * i as usize);
                let mut pos: [f64; 3] = [0.0; 3];
                crate::engine::engine_util_blas::mju_mul_mat_vec3(pos.as_mut_ptr(), xmat, center);
                crate::engine::engine_util_blas::mju_add_to3(pos.as_mut_ptr(), xpos);

                mjv_init_geom(thisgeom, MJ_GEOM_LINEBOX, size, pos.as_ptr(), xmat, rgba);
                release_geom(&mut thisgeom, scn);
            }
        }
    }
}

/// C: addTactileSensorGeoms (engine/engine_vis_visualize.c:1673)
/// Calls: addTriangle, mju_addTo3, mju_mat2Quat, mju_max, mju_mulMatVec3
#[allow(unused_variables, non_snake_case)]
pub fn add_tactile_sensor_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    const MJ_VIS_CONTACTPOINT: usize = 14;
    const MJ_SENS_TACTILE: i32 = 46;
    const MJ_CAT_DECOR: i32 = 4;
    const MJ_OBJ_SENSOR: i32 = 20;
    const MJMINVAL: f64 = 1e-15;

    // SAFETY: m, d, vopt, scn are valid pointers (caller contract).
    unsafe {
        if (*vopt).flags[MJ_VIS_CONTACTPOINT] == 0 {
            return;
        }

        for id in 0..(*m).nsensor as i32 {
            if *(*m).sensor_type.add(id as usize) != MJ_SENS_TACTILE {
                continue;
            }

            // get mesh id and geom frame
            let mesh_id = *(*m).sensor_objid.add(id as usize);
            let geom_id = *(*m).sensor_refid.add(id as usize);
            let geom_pos = (*d).geom_xpos.add(3 * geom_id as usize);
            let geom_mat = (*d).geom_xmat.add(9 * geom_id as usize);

            // get sensor data
            let sensordata = (*d).sensordata.add(*(*m).sensor_adr.add(id as usize) as usize);
            let vertnum = *(*m).mesh_vertnum.add(mesh_id as usize);
            let nchannel = *(*m).sensor_dim.add(id as usize) / vertnum;

            // get maximum absolute normal force
            let mut maxval: f64 = 0.0;
            for j in 0..vertnum {
                let v = (*sensordata.add(j as usize)).abs();
                if v > maxval {
                    maxval = v;
                }
            }

            // if no normal force readings, quick return
            if maxval == 0.0 || *(*m).geom_rbound.add(geom_id as usize) < MJMINVAL {
                continue;
            }

            // draw geoms
            let mesh_vert = (*m).mesh_vert.add(3 * *(*m).mesh_vertadr.add(mesh_id as usize) as usize);
            let face = (*m).mesh_face.add(3 * *(*m).mesh_faceadr.add(mesh_id as usize) as usize);
            let facenum = *(*m).mesh_facenum.add(mesh_id as usize);

            for i in 0..facenum {
                // triangle in global frame
                let mut pos: [[f64; 3]; 3] = [[0.0; 3]; 3];
                for j in 0..3usize {
                    let vi = *face.add(3 * i as usize + j);
                    let mut v: [f64; 3] = [
                        *mesh_vert.add(3 * vi as usize) as f64,
                        *mesh_vert.add(3 * vi as usize + 1) as f64,
                        *mesh_vert.add(3 * vi as usize + 2) as f64,
                    ];
                    crate::engine::engine_util_blas::mju_mul_mat_vec3(
                        pos[j].as_mut_ptr(), geom_mat, v.as_ptr());
                    crate::engine::engine_util_blas::mju_add_to3(
                        pos[j].as_mut_ptr(), geom_pos);
                }

                // color
                let mut rgba: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
                let mut nval: [f64; 3] = [0.0; 3];
                let min_nc = if nchannel < 3 { nchannel } else { 3 };
                for r in 0..min_nc {
                    for j in 0..3i32 {
                        let val = *sensordata.add(
                            (r * vertnum + *face.add(3 * i as usize + j as usize)) as usize);
                        rgba[r as usize] += (val.abs() / maxval) as f32;
                        if val != 0.0 {
                            nval[r as usize] += 1.0;
                        }
                    }
                    if nval[r as usize] != 0.0 {
                        rgba[r as usize] /= nval[r as usize] as f32;
                    }
                }

                if rgba[0] == 0.0 && rgba[1] == 0.0 && rgba[2] == 0.0 {
                    rgba[3] = 0.1;
                }

                // draw triangles
                add_triangle(
                    scn,
                    pos[0].as_ptr(), pos[1].as_ptr(), pos[2].as_ptr(),
                    rgba.as_ptr(), id, MJ_CAT_DECOR, MJ_OBJ_SENSOR,
                );
            }
        }
    }
}

/// C: addInertiaGeoms (engine/engine_vis_visualize.c:1745)
/// Calls: acquireGeom, bodycategory, makeLabel, markselected, mju_max, mjv_initGeom, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_inertia_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    const MJ_VIS_INERTIA: usize = 10;
    const MJ_VIS_SCLINERTIA: usize = 11;
    const MJ_CAT_DECOR: i32 = 4;
    const MJ_OBJ_BODY: i32 = 1;
    const MJ_GEOM_ELLIPSOID: i32 = 4;
    const MJ_GEOM_BOX: i32 = 6;
    const MJ_LABEL_BODY: i32 = 1;
    const MJ_LABEL_SELECTION: i32 = 12;
    const MJMINVAL: f64 = 1e-15;
    const MJ_PI: f64 = std::f64::consts::PI;

    // SAFETY: m, d, vopt, pert, scn are valid pointers (caller contract).
    unsafe {
        if (*vopt).flags[MJ_VIS_INERTIA] == 0 {
            return;
        }

        // vis.global.ellipsoidinertia is at byte offset 44 (i32 at index 11 in global)
        let global_ints = (*m).vis.global.as_ptr() as *const i32;
        let ellipsoid = *global_ints.add(11) == 1;

        for i in 1..(*m).nbody as i32 {
            if *(*m).body_mass.add(i as usize) <= MJMINVAL {
                continue;
            }
            if (bodycategory(m, i) & catmask) == 0 {
                continue;
            }

            let mut thisgeom = acquire_geom(scn, i, MJ_CAT_DECOR, MJ_OBJ_BODY);
            if thisgeom.is_null() {
                return;
            }

            let Ixx = *(*m).body_inertia.add(3 * i as usize);
            let Iyy = *(*m).body_inertia.add(3 * i as usize + 1);
            let Izz = *(*m).body_inertia.add(3 * i as usize + 2);
            let mass = *(*m).body_mass.add(i as usize);
            let scale_inertia: f64 = if ellipsoid { f64::sqrt(5.0) } else { f64::sqrt(3.0) };

            let mut sz: [f64; 3] = [0.0; 3];
            sz[0] = f64::sqrt((Iyy + Izz - Ixx) / (2.0 * mass)) * scale_inertia;
            sz[1] = f64::sqrt((Ixx + Izz - Iyy) / (2.0 * mass)) * scale_inertia;
            sz[2] = f64::sqrt((Ixx + Iyy - Izz) / (2.0 * mass)) * scale_inertia;

            // scale with mass if enabled
            if (*vopt).flags[MJ_VIS_SCLINERTIA] != 0 {
                // density = mass / volume
                let scale_volume: f64 = if ellipsoid { 4.0 / 3.0 * MJ_PI } else { 8.0 };
                let volume = scale_volume * sz[0] * sz[1] * sz[2];
                let density = mass / crate::engine::engine_util_misc::mju_max(MJMINVAL, volume);

                // scale = root3(density)
                let scale = f64::powf(density * 0.001, 1.0 / 3.0);

                // scale sizes
                sz[0] *= scale;
                sz[1] *= scale;
                sz[2] *= scale;
            }

            // construct geom
            let geom_type = if ellipsoid { MJ_GEOM_ELLIPSOID } else { MJ_GEOM_BOX };
            // vis.rgba.inertia is index 3 in the rgba float array (each entry is 4 floats)
            let rgba_floats = (*m).vis.rgba._data.as_ptr() as *const f32;
            let inertia_rgba = rgba_floats.add(3 * 4);
            mjv_init_geom(
                thisgeom,
                geom_type,
                sz.as_ptr(),
                (*d).xipos.add(3 * i as usize),
                (*d).ximat.add(9 * i as usize),
                inertia_rgba,
            );

            // glow
            if (*pert).select == i {
                markselected(
                    &(*m).vis as *const _ as *const mjVisual,
                    thisgeom,
                );
            }

            // label
            if (*vopt).label == MJ_LABEL_BODY
                || ((*vopt).label == MJ_LABEL_SELECTION && (*pert).select == i)
            {
                make_label(m, MJ_OBJ_BODY as u32, i, (*thisgeom).label.as_mut_ptr());
            }

            release_geom(&mut thisgeom, scn);
        }
    }
}

/// C: addPerturbGeoms (engine/engine_vis_visualize.c:1811)
/// Calls: acquireGeom, f2f, mixcolor, mju_addTo3, mju_copy3, mju_mulMatVec3, mju_quat2Mat, mjv_connector, mjv_initGeom, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_perturb_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, scn: *mut mjvScene) {
    const MJ_VIS_PERTOBJ: usize = 13;
    const MJ_CAT_DECOR: i32 = 4;
    const MJ_OBJ_UNKNOWN: i32 = 0;
    const MJ_GEOM_CAPSULE: i32 = 3;
    const MJ_GEOM_SPHERE: i32 = 2;
    const MJ_GEOM_LINEBOX: i32 = 104;
    const MJ_PERT_TRANSLATE: i32 = 1;
    const MJ_PERT_ROTATE: i32 = 2;

    // SAFETY: m, d, vopt, pert, scn are valid pointers (caller contract).
    unsafe {
        if (*vopt).flags[MJ_VIS_PERTOBJ] == 0 {
            return;
        }
        if (*pert).select <= 0 {
            return;
        }

        let scl = (*m).stat.meansize as f32;
        let scale_floats = (*m).vis.scale.as_ptr() as *const f32;
        let constraint_scale = *scale_floats.add(14); // constraint is index 14

        // vis.rgba: constraint is index 19, inertia is index 3
        let rgba_floats = (*m).vis.rgba._data.as_ptr() as *const f32;
        let constraint_rgba = rgba_floats.add(19 * 4);
        let inertia_rgba = rgba_floats.add(3 * 4);

        if ((*pert).active | (*pert).active2) & MJ_PERT_TRANSLATE != 0 {
            let mut thisgeom = acquire_geom(scn, (*pert).select, MJ_CAT_DECOR, MJ_OBJ_UNKNOWN);
            if thisgeom.is_null() {
                return;
            }

            // compute selection point in world coordinates
            let mut selpos: [f64; 3] = [0.0; 3];
            crate::engine::engine_util_blas::mju_mul_mat_vec3(
                selpos.as_mut_ptr(),
                (*d).xmat.add(9 * (*pert).select as usize),
                (*pert).localpos.as_ptr(),
            );
            crate::engine::engine_util_blas::mju_add_to3(
                selpos.as_mut_ptr(),
                (*d).xpos.add(3 * (*pert).select as usize),
            );

            // construct geom
            let width = scl as f64 * constraint_scale as f64;
            mjv_connector(thisgeom, MJ_GEOM_CAPSULE, width, selpos.as_ptr(), (*pert).refselpos.as_ptr());

            // prepare color
            let mut rgba: [f32; 4] = [0.0; 4];
            mixcolor(
                rgba.as_mut_ptr(),
                constraint_rgba,
                (if ((*pert).active & MJ_PERT_TRANSLATE) > 0 { 1 } else { 0 }),
                (if ((*pert).active2 & MJ_PERT_TRANSLATE) > 0 { 1 } else { 0 }),
            );
            f2f((*thisgeom).rgba.as_mut_ptr(), rgba.as_ptr(), 4);
            release_geom(&mut thisgeom, scn);

            // add small sphere at end-effector
            thisgeom = acquire_geom(scn, (*pert).select, MJ_CAT_DECOR, MJ_OBJ_UNKNOWN);
            if thisgeom.is_null() {
                return;
            }

            let mut sz: [f64; 3] = [0.0; 3];
            sz[0] = 2.0 * width;
            sz[1] = sz[0];
            sz[2] = sz[0];

            let mut mat: [f64; 9] = [0.0; 9];
            crate::engine::engine_util_spatial::mju_quat2mat(mat.as_mut_ptr(), (*pert).refquat.as_ptr());
            mjv_init_geom(
                thisgeom,
                MJ_GEOM_SPHERE,
                sz.as_ptr(),
                (*pert).refselpos.as_ptr(),
                mat.as_ptr(),
                rgba.as_ptr(),
            );
            release_geom(&mut thisgeom, scn);
        }

        if ((*pert).active | (*pert).active2) & MJ_PERT_ROTATE != 0 {
            let mut thisgeom = acquire_geom(scn, (*pert).select, MJ_CAT_DECOR, MJ_OBJ_UNKNOWN);
            if thisgeom.is_null() {
                return;
            }

            // prepare color, use inertia color
            let mut rgba: [f32; 4] = [0.0; 4];
            mixcolor(
                rgba.as_mut_ptr(),
                inertia_rgba,
                (if ((*pert).active & MJ_PERT_ROTATE) > 0 { 1 } else { 0 }),
                (if ((*pert).active2 & MJ_PERT_ROTATE) > 0 { 1 } else { 0 }),
            );

            // construct geom: if body has a collision aabb, use that
            let mut pos: [f64; 3] = [0.0; 3];
            let mut sz: [f64; 3] = [0.0; 3];
            if *(*m).body_bvhnum.add((*pert).select as usize) != 0 {
                let aabb = (*m).bvh_aabb.add(6 * *(*m).body_bvhadr.add((*pert).select as usize) as usize);
                crate::engine::engine_util_blas::mju_copy3(sz.as_mut_ptr(), aabb.add(3));
                crate::engine::engine_util_blas::mju_mul_mat_vec3(
                    pos.as_mut_ptr(),
                    (*d).ximat.add(9 * (*pert).select as usize),
                    aabb,
                );
            }
            // otherwise box of size meansize
            else {
                sz[0] = scl as f64;
                sz[1] = scl as f64;
                sz[2] = scl as f64;
            }

            let mut mat: [f64; 9] = [0.0; 9];
            crate::engine::engine_util_spatial::mju_quat2mat(mat.as_mut_ptr(), (*pert).refquat.as_ptr());
            crate::engine::engine_util_blas::mju_add_to3(
                pos.as_mut_ptr(),
                (*d).xipos.add(3 * (*pert).select as usize),
            );
            mjv_init_geom(thisgeom, MJ_GEOM_LINEBOX, sz.as_ptr(), pos.as_ptr(), mat.as_ptr(), rgba.as_ptr());
            release_geom(&mut thisgeom, scn);
        }
    }
}

/// C: addWorldBodyFrameGeoms (engine/engine_vis_visualize.c:1900)
/// Calls: addFrame, bodycategory
#[allow(unused_variables, non_snake_case)]
pub fn add_world_body_frame_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, catmask: i32, scn: *mut mjvScene) {
    const MJ_FRAME_BODY: i32 = 1;
    const MJ_FRAME_WORLD: i32 = 7;
    const MJ_VIS_INERTIA: usize = 10;

    // SAFETY: m, d, vopt, scn are valid pointers (caller contract).
    unsafe {
        let scl = (*m).stat.meansize as f32;
        let scale_floats = (*m).vis.scale.as_ptr() as *const f32;
        let framelength = *scale_floats.add(12);
        let framewidth = *scale_floats.add(13);

        let start = if (*vopt).frame == MJ_FRAME_WORLD { 0 } else { 1 };
        let end = if (*vopt).frame == MJ_FRAME_BODY { (*m).nbody as i32 } else { 1 };

        for i in start..end {
            // skip if body is static and static bodies are masked
            if i > 0 && (bodycategory(m, i) & !catmask) != 0 {
                continue;
            }

            // set length(1) and width(0) of the axis cylinders
            let sz_length: f32;
            let sz_width: f32;
            if i == 0 {
                sz_length = framelength * scl * 2.0;
                sz_width = framewidth * scl * 2.0;
            } else {
                sz_length = framelength * scl;
                sz_width = framewidth * scl;
            }

            let xmat: *const f64 = if (*vopt).flags[MJ_VIS_INERTIA] != 0 {
                (*d).ximat.add(9 * i as usize)
            } else {
                (*d).xmat.add(9 * i as usize)
            };
            let xpos: *const f64 = if (*vopt).flags[MJ_VIS_INERTIA] != 0 {
                (*d).xipos.add(3 * i as usize)
            } else {
                (*d).xpos.add(3 * i as usize)
            };
            add_frame(scn, i, xpos, xmat, sz_length, sz_width);
        }
    }
}

/// C: addSelectionPointGeoms (engine/engine_vis_visualize.c:1928)
/// Calls: acquireGeom, f2f, mju_addTo3, mju_mulMatVec3, mju_n2f, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_selection_point_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, scn: *mut mjvScene) {
    const MJ_VIS_SELECT: usize = 21;
    const MJ_CAT_DECOR: i32 = 4;
    const MJ_OBJ_UNKNOWN: i32 = 0;
    const MJ_GEOM_SPHERE: i32 = 2;
    const MJ_LABEL_SELPNT: i32 = 13;

    // SAFETY: m, d, vopt, pert, scn are valid pointers (caller contract).
    unsafe {
        if (*pert).select <= 0 {
            return;
        }
        if (*vopt).flags[MJ_VIS_SELECT] == 0 {
            return;
        }

        let scl = (*m).stat.meansize as f32;
        let scale_floats = (*m).vis.scale.as_ptr() as *const f32;
        let selectpoint_scale = *scale_floats.add(7); // selectpoint is index 7

        // compute selection point in world coordinates
        let mut selpos: [f64; 3] = [0.0; 3];
        crate::engine::engine_util_blas::mju_mul_mat_vec3(
            selpos.as_mut_ptr(),
            (*d).xmat.add(9 * (*pert).select as usize),
            (*pert).localpos.as_ptr(),
        );
        crate::engine::engine_util_blas::mju_add_to3(
            selpos.as_mut_ptr(),
            (*d).xpos.add(3 * (*pert).select as usize),
        );

        let mut thisgeom = acquire_geom(scn, (*pert).select, MJ_CAT_DECOR, MJ_OBJ_UNKNOWN);
        if thisgeom.is_null() {
            return;
        }

        (*thisgeom).r#type = MJ_GEOM_SPHERE;
        let sz = scl * selectpoint_scale;
        (*thisgeom).size[0] = sz;
        (*thisgeom).size[1] = sz;
        (*thisgeom).size[2] = sz;
        crate::engine::engine_util_misc::mju_n2f(
            (*thisgeom).pos.as_mut_ptr(), selpos.as_ptr(), 3);

        static IDENTITY: [f64; 9] = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
        crate::engine::engine_util_misc::mju_n2f(
            (*thisgeom).mat.as_mut_ptr(), IDENTITY.as_ptr(), 9);

        // vis.rgba.selectpoint is index 11 in the rgba float array (each entry is 4 floats)
        let rgba_floats = (*m).vis.rgba._data.as_ptr() as *const f32;
        let selectpoint_rgba = rgba_floats.add(11 * 4);
        f2f((*thisgeom).rgba.as_mut_ptr(), selectpoint_rgba, 4);

        if (*vopt).label == MJ_LABEL_SELPNT {
            extern "C" { fn snprintf(s: *mut i8, n: usize, fmt: *const i8, ...) -> i32; }
            snprintf(
                (*thisgeom).label.as_mut_ptr(), 100,
                b"%.3f %.3f %.3f (local %.3f %.3f %.3f)\0".as_ptr() as *const i8,
                selpos[0], selpos[1], selpos[2],
                (*pert).localpos[0], (*pert).localpos[1], (*pert).localpos[2],
            );
        }

        release_geom(&mut thisgeom, scn);
    }
}

/// C: addBodyLabelGeoms (engine/engine_vis_visualize.c:1964)
/// Calls: acquireGeom, bodycategory, makeLabel, mju_n2f, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_body_label_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    const MJ_VIS_INERTIA: usize = 10;
    const MJ_LABEL_SELECTION: i32 = 12;
    const MJ_LABEL_BODY: i32 = 1;
    const MJ_CAT_DECOR: i32 = 4;
    const MJ_OBJ_UNKNOWN: i32 = 0;
    const MJ_GEOM_LABEL: i32 = 107;
    const MJ_OBJ_BODY: u32 = 1;

    // SAFETY: m, d, vopt, pert, scn are valid pointers (caller contract).
    unsafe {
        if (*vopt).flags[MJ_VIS_INERTIA] != 0 {
            return;
        }
        if (*vopt).label != MJ_LABEL_SELECTION && (*vopt).label != MJ_LABEL_BODY {
            return;
        }

        for i in 1..(*m).nbody as i32 {
            if (*vopt).label == MJ_LABEL_SELECTION && (*pert).select != i {
                continue;
            }
            if bodycategory(m, i) & !catmask != 0 {
                continue;
            }

            let mut thisgeom = acquire_geom(scn, i, MJ_CAT_DECOR, MJ_OBJ_UNKNOWN);
            if thisgeom.is_null() {
                return;
            }

            (*thisgeom).r#type = MJ_GEOM_LABEL;
            crate::engine::engine_util_misc::mju_n2f(
                (*thisgeom).pos.as_mut_ptr(), (*d).xpos.add(3 * i as usize), 3,
            );
            crate::engine::engine_util_misc::mju_n2f(
                (*thisgeom).mat.as_mut_ptr(), (*d).xmat.add(9 * i as usize), 9,
            );
            make_label(m, MJ_OBJ_BODY, i, (*thisgeom).label.as_mut_ptr());
            release_geom(&mut thisgeom, scn);
        }
    }
}

/// C: addJointGeoms (engine/engine_vis_visualize.c:1994)
/// Calls: acquireGeom, f2f, makeLabel, mju_addScl3, mju_message, mju_n2f, mjv_connector, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_joint_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    const MJ_VIS_JOINT: usize = 2;
    const MJ_CAT_DECOR: i32 = 4;
    const MJ_OBJ_JOINT: i32 = 3;
    const MJ_GEOM_BOX: i32 = 6;
    const MJ_GEOM_SPHERE: i32 = 2;
    const MJ_GEOM_ARROW: i32 = 100;
    const MJ_GEOM_ARROW1: i32 = 101;
    const MJ_JNT_FREE: i32 = 0;
    const MJ_JNT_BALL: i32 = 1;
    const MJ_JNT_SLIDE: i32 = 2;
    const MJ_JNT_HINGE: i32 = 3;
    const MJ_CNSTR_LIMIT_JOINT: i32 = 3;
    const MJ_LABEL_JOINT: i32 = 2;
    const MJ_NGROUP: i32 = 6;

    // SAFETY: m, d, vopt, scn are valid pointers (caller contract).
    unsafe {
        if (*vopt).flags[MJ_VIS_JOINT] == 0 {
            return;
        }

        let scl = (*m).stat.meansize as f32;
        let scale_floats = (*m).vis.scale.as_ptr() as *const f32;
        let jointlength = *scale_floats.add(8);
        let jointwidth = *scale_floats.add(9);

        // rgba: joint = index 4, constraint = index 19
        let rgba_floats = (*m).vis.rgba._data.as_ptr() as *const f32;
        let joint_rgba = rgba_floats.add(4 * 4);
        let constraint_rgba = rgba_floats.add(19 * 4);

        for i in 0..(*m).njnt as i32 {
            let group = *(*m).jnt_group.add(i as usize);
            let clamped = if 0 > (if (MJ_NGROUP - 1) < group { MJ_NGROUP - 1 } else { group }) {
                0
            } else {
                if (MJ_NGROUP - 1) < group { MJ_NGROUP - 1 } else { group }
            };
            if (*vopt).jointgroup[clamped as usize] == 0 {
                continue;
            }

            let mut thisgeom = acquire_geom(scn, i, MJ_CAT_DECOR, MJ_OBJ_JOINT);
            if thisgeom.is_null() {
                return;
            }

            // set sz = {width, length} of the connectors
            let sz_length = jointlength * scl;
            let sz_width = jointwidth * scl;

            // set type, size, pos, mat depending on joint type
            let j = *(*m).jnt_bodyid.add(i as usize);
            let jnt_type = *(*m).jnt_type.add(i as usize);

            if jnt_type == MJ_JNT_FREE {
                (*thisgeom).r#type = MJ_GEOM_BOX;
                let s = 0.3 * sz_length;
                (*thisgeom).size[0] = s;
                (*thisgeom).size[1] = s;
                (*thisgeom).size[2] = s;
                crate::engine::engine_util_misc::mju_n2f(
                    (*thisgeom).pos.as_mut_ptr(), (*d).xanchor.add(3 * i as usize), 3);
                crate::engine::engine_util_misc::mju_n2f(
                    (*thisgeom).mat.as_mut_ptr(), (*d).xmat.add(9 * j as usize), 9);
            } else if jnt_type == MJ_JNT_BALL {
                (*thisgeom).r#type = MJ_GEOM_SPHERE;
                let s = 0.3 * sz_length;
                (*thisgeom).size[0] = s;
                (*thisgeom).size[1] = s;
                (*thisgeom).size[2] = s;
                crate::engine::engine_util_misc::mju_n2f(
                    (*thisgeom).pos.as_mut_ptr(), (*d).xanchor.add(3 * i as usize), 3);
                crate::engine::engine_util_misc::mju_n2f(
                    (*thisgeom).mat.as_mut_ptr(), (*d).xmat.add(9 * j as usize), 9);
            } else if jnt_type == MJ_JNT_SLIDE || jnt_type == MJ_JNT_HINGE {
                let from = (*d).xanchor.add(3 * i as usize);
                let mut to: [f64; 3] = [0.0; 3];
                crate::engine::engine_util_blas::mju_add_scl3(
                    to.as_mut_ptr(), from, (*d).xaxis.add(3 * i as usize), sz_length as f64);
                let geom_type = if jnt_type == MJ_JNT_SLIDE { MJ_GEOM_ARROW } else { MJ_GEOM_ARROW1 };
                mjv_connector(thisgeom, geom_type, sz_width as f64, from, to.as_ptr());
            } else {
                crate::engine::engine_util_errmem::mju_error(
                    b"unknown joint type\0".as_ptr() as *const i8);
            }

            // loop over limit constraints, get impedance if this joint is limited
            let mut imp: f64 = 0.0;
            let efc_start = (*d).ne + (*d).nf;
            let efc_end = efc_start + (*d).nl;
            for k in efc_start..efc_end {
                if *(*d).efc_type.add(k as usize) == MJ_CNSTR_LIMIT_JOINT
                    && *(*d).efc_id.add(k as usize) == i
                {
                    imp = *(*d).efc_KBIP.add(4 * k as usize + 2);
                }
            }

            // use impedance to mix joint and constraint colors
            let mut rgba: [f32; 4] = [0.0; 4];
            rgba[0] = (1.0 - imp as f32) * *joint_rgba.add(0) + imp as f32 * *constraint_rgba.add(0);
            rgba[1] = (1.0 - imp as f32) * *joint_rgba.add(1) + imp as f32 * *constraint_rgba.add(1);
            rgba[2] = (1.0 - imp as f32) * *joint_rgba.add(2) + imp as f32 * *constraint_rgba.add(2);
            rgba[3] = 1.0;
            f2f((*thisgeom).rgba.as_mut_ptr(), rgba.as_ptr(), 4);

            // label
            if (*vopt).label == MJ_LABEL_JOINT {
                make_label(m, MJ_OBJ_JOINT as u32, i, (*thisgeom).label.as_mut_ptr());
            }

            release_geom(&mut thisgeom, scn);
        }
    }
}

/// C: addActuatorGeoms (engine/engine_vis_visualize.c:2074)
/// Calls: acquireGeom, f2f, makeLabel, mj_actuatorDisabled, mju_addScl3, mju_clip, mju_scl3, mjv_connector, mjv_initGeom, releaseGeom, setMaterial
#[allow(unused_variables, non_snake_case)]
pub fn add_actuator_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    const MJVIS_ACTUATOR: usize = 4;
    const MJVIS_ACTIVATION: usize = 5;
    const MJCAT_DECOR: i32 = 4;
    const MJOBJ_ACTUATOR: i32 = 19;
    const MJLABEL_ACTUATOR: i32 = 8;
    const MJTRN_JOINT: i32 = 0;
    const MJTRN_JOINTINPARENT: i32 = 1;
    const MJTRN_SITE: i32 = 4;
    const MJTRN_TENDON: i32 = 3;
    const MJTRN_BODY: i32 = 5;
    const MJJNT_FREE: i32 = 0;
    const MJJNT_BALL: i32 = 1;
    const MJJNT_SLIDE: i32 = 2;
    const MJJNT_HINGE: i32 = 3;
    const MJGEOM_PLANE: i32 = 0;
    const MJGEOM_HFIELD: i32 = 1;
    const MJGEOM_SPHERE: i32 = 2;
    const MJGEOM_CAPSULE: i32 = 3;
    const MJGEOM_BOX: i32 = 6;
    const MJGEOM_MESH: i32 = 7;
    const MJGEOM_SDF: i32 = 8;
    const MJGEOM_ARROW: i32 = 100;
    const MJGEOM_ARROW1: i32 = 101;
    const MJMINVAL: f64 = 1e-15;
    const MJNGROUP: i32 = 6;

    // SAFETY: m, d, vopt, scn are valid pointers (caller contract).
    unsafe {
        if *(*vopt).flags.as_ptr().add(MJVIS_ACTUATOR) == 0 {
            return;
        }

        let scl = (*m).stat.meansize as f32;
        let scale_ptr = (*m).vis.scale.as_ptr() as *const f32;
        let rgba_ptr = (*m).vis.rgba._data.as_ptr() as *const f32;
        let map_ptr = (*m).vis.map.as_ptr() as *const f32;

        for i in 0..(*m).nu as i32 {
            let group = *(*m).actuator_group.add(i as usize);
            let clamped_group = if group < 0 { 0 } else if group > MJNGROUP - 1 { MJNGROUP - 1 } else { group };
            if (*vopt).actuatorgroup[clamped_group as usize] == 0 {
                continue;
            }
            if crate::engine::engine_support::mj_actuator_disabled(m, i) != 0 {
                continue;
            }

            // determine extended range
            let mut rng: [f64; 3] = [-1.0, 0.0, 1.0];
            let mut rmin: f64 = -1.0;
            let mut rmax: f64 = 1.0;
            let mut act: f64 = 0.0;
            if *(*m).actuator_ctrllimited.add(i as usize) {
                rmin = *(*m).actuator_ctrlrange.add(2 * i as usize);
                rmax = *(*m).actuator_ctrlrange.add(2 * i as usize + 1);
            } else if *(*vopt).flags.as_ptr().add(MJVIS_ACTIVATION) != 0
                && *(*m).actuator_actlimited.add(i as usize)
            {
                rmin = *(*m).actuator_actrange.add(2 * i as usize);
                rmax = *(*m).actuator_actrange.add(2 * i as usize + 1);
            }
            if rmin >= 0.0 {
                rng[0] = -1.0; rng[1] = rmin; rng[2] = rmax;
            } else if rmax <= 0.0 {
                rng[0] = rmin; rng[1] = rmax; rng[2] = 1.0;
            } else {
                rng[0] = rmin; rng[1] = 0.0; rng[2] = rmax;
            }

            if rng[1] - rng[0] < MJMINVAL {
                rng[0] = rng[1] - MJMINVAL;
            }
            if rng[2] - rng[1] < MJMINVAL {
                rng[2] = rng[1] + MJMINVAL;
            }

            // clamp act to extended range
            if *(*vopt).flags.as_ptr().add(MJVIS_ACTIVATION) != 0
                && *(*m).actuator_dyntype.add(i as usize) != 0
            {
                let act_idx = *(*m).actuator_actadr.add(i as usize)
                    + *(*m).actuator_actnum.add(i as usize) - 1;
                act = crate::engine::engine_util_misc::mju_clip(
                    *(*d).act.add(act_idx as usize), rng[0], rng[2]);
            } else {
                act = crate::engine::engine_util_misc::mju_clip(
                    *(*d).ctrl.add(i as usize), rng[0], rng[2]);
            }

            // compute interpolants
            let amin: f32;
            let amean: f32;
            let amax: f32;
            if act <= rng[1] {
                let denom = if MJMINVAL > (rng[1] - rng[0]) { MJMINVAL } else { rng[1] - rng[0] };
                amin = ((rng[1] - act) / denom) as f32;
                amean = 1.0 - amin;
                amax = 0.0;
            } else {
                let denom = if MJMINVAL > (rng[2] - rng[1]) { MJMINVAL } else { rng[2] - rng[1] };
                amax = ((act - rng[1]) / denom) as f32;
                amean = 1.0 - amax;
                amin = 0.0;
            }

            // interpolated color: vis.rgba actuatornegative=24, actuator=20, actuatorpositive=28
            let mut rgba: [f32; 4] = [0.0; 4];
            for j in 0..4 {
                rgba[j] = amin * *rgba_ptr.add(24 + j)
                    + amean * *rgba_ptr.add(20 + j)
                    + amax * *rgba_ptr.add(28 + j);
            }

            // get transmission object id
            let j = *(*m).actuator_trnid.add(2 * i as usize);
            let trntype = *(*m).actuator_trntype.add(i as usize);

            // slide and hinge joint actuators, or site
            if trntype == MJTRN_JOINT || trntype == MJTRN_JOINTINPARENT || trntype == MJTRN_SITE {
                let mut thisgeom = acquire_geom(scn, i, MJCAT_DECOR, MJOBJ_ACTUATOR);
                if thisgeom.is_null() {
                    return;
                }

                let mut sz: [f64; 3] = [0.0; 3];
                if trntype == MJTRN_SITE {
                    crate::engine::engine_util_blas::mju_scl3(
                        sz.as_mut_ptr(), (*m).site_size.add(3 * j as usize), 1.05);
                    mjv_init_geom(
                        thisgeom, *(*m).site_type.add(j as usize), sz.as_ptr(),
                        (*d).site_xpos.add(3 * j as usize),
                        (*d).site_xmat.add(9 * j as usize),
                        (*thisgeom).rgba.as_ptr());
                } else if *(*m).jnt_type.add(j as usize) == MJJNT_HINGE
                    || *(*m).jnt_type.add(j as usize) == MJJNT_SLIDE
                {
                    // vis.scale: index 10 = actuatorlength, 11 = actuatorwidth
                    sz[1] = *scale_ptr.add(10) as f64 * scl as f64;
                    sz[0] = *scale_ptr.add(11) as f64 * scl as f64;

                    let from = (*d).xanchor.add(3 * j as usize);
                    let mut to: [f64; 3] = [0.0; 3];
                    crate::engine::engine_util_blas::mju_add_scl3(
                        to.as_mut_ptr(), from, (*d).xaxis.add(3 * j as usize), sz[1]);
                    let arrow_type = if *(*m).jnt_type.add(j as usize) == MJJNT_SLIDE {
                        MJGEOM_ARROW
                    } else {
                        MJGEOM_ARROW1
                    };
                    mjv_connector(thisgeom, arrow_type, sz[0], from, to.as_ptr());
                } else if *(*m).jnt_type.add(j as usize) == MJJNT_BALL
                    || *(*m).jnt_type.add(j as usize) == MJJNT_FREE
                {
                    // vis.scale: index 8 = jointlength
                    sz[0] = *scale_ptr.add(8) as f64 * scl as f64 * 0.33;
                    sz[1] = sz[0];
                    sz[2] = sz[0];
                    let geom_type = if *(*m).jnt_type.add(j as usize) == MJJNT_BALL {
                        MJGEOM_SPHERE
                    } else {
                        MJGEOM_BOX
                    };
                    mjv_init_geom(
                        thisgeom, geom_type, sz.as_ptr(),
                        (*d).xanchor.add(3 * j as usize),
                        (*d).xmat.add(9 * *(*m).jnt_bodyid.add(j as usize) as usize),
                        (*thisgeom).rgba.as_ptr());
                }

                f2f((*thisgeom).rgba.as_mut_ptr(), rgba.as_ptr(), 4);
                if (*vopt).label == MJLABEL_ACTUATOR {
                    make_label(m, MJOBJ_ACTUATOR as u32, i, (*thisgeom).label.as_mut_ptr());
                }
                release_geom(&mut thisgeom, scn);
            }
            // body actuators
            else if trntype == MJTRN_BODY {
                let geomnum = *(*m).body_geomnum.add(j as usize);
                let geomadr = *(*m).body_geomadr.add(j as usize);
                for k in geomadr..(geomadr + geomnum) {
                    let geomtype = *(*m).geom_type.add(k as usize);
                    if geomtype != MJGEOM_PLANE && geomtype != MJGEOM_HFIELD
                        && geomtype != MJGEOM_MESH && geomtype != MJGEOM_SDF
                    {
                        let mut thisgeom = acquire_geom(scn, i, MJCAT_DECOR, MJOBJ_ACTUATOR);
                        if thisgeom.is_null() {
                            return;
                        }
                        let mut sz: [f64; 3] = [0.0; 3];
                        crate::engine::engine_util_blas::mju_scl3(
                            sz.as_mut_ptr(), (*m).geom_size.add(3 * k as usize), 1.05);
                        mjv_init_geom(
                            thisgeom, *(*m).geom_type.add(k as usize), sz.as_ptr(),
                            (*d).geom_xpos.add(3 * k as usize),
                            (*d).geom_xmat.add(9 * k as usize),
                            (*thisgeom).rgba.as_ptr());
                        f2f((*thisgeom).rgba.as_mut_ptr(), rgba.as_ptr(), 4);
                        release_geom(&mut thisgeom, scn);
                    }
                }
            }
            // spatial tendon actuators
            else if trntype == MJTRN_TENDON && *(*d).ten_wrapnum.add(j as usize) != 0 {
                let wrapadr = *(*d).ten_wrapadr.add(j as usize);
                let wrapnum = *(*d).ten_wrapnum.add(j as usize);
                for k in wrapadr..(wrapadr + wrapnum - 1) {
                    if *(*d).wrap_obj.add(k as usize) != -2
                        && *(*d).wrap_obj.add((k + 1) as usize) != -2
                    {
                        let mut thisgeom = acquire_geom(scn, i, MJCAT_DECOR, MJOBJ_ACTUATOR);
                        if thisgeom.is_null() {
                            return;
                        }

                        let mut width: f64;
                        if *(*d).wrap_obj.add(k as usize) >= 0
                            && *(*d).wrap_obj.add((k + 1) as usize) >= 0
                        {
                            width = 0.5 * *(*m).tendon_width.add(j as usize);
                        } else {
                            width = *(*m).tendon_width.add(j as usize);
                        }
                        // vis.map: index 12 = actuatortendon
                        width *= *map_ptr.add(12) as f64;

                        mjv_connector(
                            thisgeom, MJGEOM_CAPSULE, width,
                            (*d).wrap_xpos.add(3 * k as usize),
                            (*d).wrap_xpos.add(3 * (k + 1) as usize));
                        set_material(
                            m, thisgeom, *(*m).tendon_matid.add(j as usize),
                            (*m).tendon_rgba.add(4 * j as usize), (*vopt).flags.as_ptr());
                        f2f((*thisgeom).rgba.as_mut_ptr(), rgba.as_ptr(), 4);

                        if (*vopt).label == MJLABEL_ACTUATOR && k == wrapadr {
                            make_label(m, MJOBJ_ACTUATOR as u32, i, (*thisgeom).label.as_mut_ptr());
                        }
                        release_geom(&mut thisgeom, scn);
                    }
                }
            }
        }
    }
}

/// C: addIslandLabelGeoms (engine/engine_vis_visualize.c:2283)
/// Calls: acquireGeom, mju_n2f, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_island_label_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    const MJ_LABEL_ISLAND: i32 = 15;
    const MJ_CAT_DECOR: i32 = 4;
    const MJ_OBJ_UNKNOWN: i32 = 0;
    const MJ_GEOM_LABEL: i32 = 107;

    // SAFETY: m, d, vopt, scn are valid pointers (caller contract).
    unsafe {
        if (*vopt).label != MJ_LABEL_ISLAND || (*d).nisland == 0 {
            return;
        }

        extern "C" { fn snprintf(s: *mut i8, n: usize, fmt: *const i8, ...) -> i32; }

        for i in 1..(*m).nbody as i32 {
            let weld_id = *(*m).body_weldid.add(i as usize);
            if *(*m).body_dofnum.add(weld_id as usize) == 0 {
                continue;
            }
            let islandid = *(*d).dof_island.add(*(*m).body_dofadr.add(weld_id as usize) as usize);
            if islandid <= -1 {
                continue;
            }

            let mut thisgeom = acquire_geom(scn, i, MJ_CAT_DECOR, MJ_OBJ_UNKNOWN);
            if thisgeom.is_null() {
                return;
            }

            (*thisgeom).r#type = MJ_GEOM_LABEL;
            crate::engine::engine_util_misc::mju_n2f(
                (*thisgeom).pos.as_mut_ptr(), (*d).xipos.add(3 * i as usize), 3,
            );
            crate::engine::engine_util_misc::mju_n2f(
                (*thisgeom).mat.as_mut_ptr(), (*d).ximat.add(9 * i as usize), 9,
            );
            snprintf(
                (*thisgeom).label.as_mut_ptr(), 100,
                b"%d\0".as_ptr() as *const i8, islandid,
            );

            release_geom(&mut thisgeom, scn);
        }
    }
}

/// C: addCameraGeoms (engine/engine_vis_visualize.c:2313)
/// Calls: acquireGeom, addConnector, addFrame, addTriangle, f2f, getFrustum, makeLabel, mju_addScl3, mju_addToScl3, mju_n2f, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_camera_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    const MJVIS_CAMERA: usize = 3;
    const MJCAT_DECOR: i32 = 4;
    const MJOBJ_CAMERA: i32 = 7;
    const MJGEOM_BOX: i32 = 6;
    const MJGEOM_CYLINDER: i32 = 5;
    const MJGEOM_LINE: i32 = 103;
    const MJLABEL_CAMERA: i32 = 5;
    const MJFRAME_CAMERA: i32 = 4;
    const MJPROJ_ORTHOGRAPHIC: i32 = 1;
    const MJPI: f64 = std::f64::consts::PI;

    // SAFETY: m, d, vopt, scn are valid pointers (caller contract).
    unsafe {
        if *(*vopt).flags.as_ptr().add(MJVIS_CAMERA) == 0 {
            return;
        }

        let scl = (*m).stat.meansize as f32;
        let scale_ptr = (*m).vis.scale.as_ptr() as *const f32;
        let rgba_ptr = (*m).vis.rgba._data.as_ptr() as *const f32;
        let map_ptr = (*m).vis.map.as_ptr() as *const f32;

        for i in 0..(*m).ncam as i32 {
            // copy camera rgba (index 36 in rgba)
            let mut cam_rgba: [f32; 4] = [0.0; 4];
            f2f(cam_rgba.as_mut_ptr(), rgba_ptr.add(36), 4);

            // draw frustum if resolution larger than (1, 1)
            if *(*m).cam_resolution.add(2 * i as usize) > 1
                || *(*m).cam_resolution.add(2 * i as usize + 1) > 1
            {
                cam_rgba[3] = 0.3;

                // frustum rgba (index 88)
                let rgba_frustum = rgba_ptr.add(88);
                let mut vnear: [[f64; 3]; 4] = [[0.0; 3]; 4];
                let mut vfar: [[f64; 3]; 4] = [[0.0; 3]; 4];
                let mut center: [f64; 3] = [0.0; 3];
                // vis.map: index 7 = znear
                let znear = *map_ptr.add(7) as f64 * (*m).stat.extent;
                // vis.scale: index 16 = frustum
                let mut zfar = *scale_ptr.add(16) as f64 * scl as f64;
                let mut zver: [f32; 2] = [0.0; 2];
                let mut zhor: [f32; 2] = [0.0; 2];
                let orthographic = *(*m).cam_projection.add(i as usize) == MJPROJ_ORTHOGRAPHIC;

                // get frustum
                if orthographic {
                    let aspect = *(*m).cam_resolution.add(2 * i as usize) as f32
                        / *(*m).cam_resolution.add(2 * i as usize + 1) as f32;
                    let fovy = *(*m).cam_fovy.add(i as usize) as f32;
                    zver[0] = fovy / 2.0;
                    zver[1] = fovy / 2.0;
                    zhor[0] = fovy * aspect / 2.0;
                    zhor[1] = fovy * aspect / 2.0;
                } else if *(*m).cam_sensorsize.add(2 * i as usize) != 0.0
                    && *(*m).cam_sensorsize.add(2 * i as usize + 1) != 0.0
                {
                    get_frustum(
                        zver.as_mut_ptr(), zhor.as_mut_ptr(), znear as f32,
                        (*m).cam_intrinsic.add(4 * i as usize),
                        (*m).cam_sensorsize.add(2 * i as usize),
                    );
                } else {
                    let aspect = *(*m).cam_resolution.add(2 * i as usize) as f32
                        / *(*m).cam_resolution.add(2 * i as usize + 1) as f32;
                    let fovy = *(*m).cam_fovy.add(i as usize);
                    let tan_val = (fovy * MJPI / 360.0).tan();
                    zver[0] = (znear * tan_val) as f32;
                    zver[1] = zver[0];
                    zhor[0] = zver[0] * aspect;
                    zhor[1] = zhor[0];
                }

                // frustum frame
                let cam_xpos = (*d).cam_xpos.add(3 * i as usize);
                let cam_xmat = (*d).cam_xmat.add(9 * i as usize);
                let x: [f64; 3] = [*cam_xmat.add(0), *cam_xmat.add(3), *cam_xmat.add(6)];
                let y: [f64; 3] = [*cam_xmat.add(1), *cam_xmat.add(4), *cam_xmat.add(7)];
                let z: [f64; 3] = [*cam_xmat.add(2), *cam_xmat.add(5), *cam_xmat.add(8)];

                // vertices of the near plane
                crate::engine::engine_util_blas::mju_add_scl3(
                    center.as_mut_ptr(), cam_xpos, z.as_ptr(), -znear);
                crate::engine::engine_util_blas::mju_add_scl3(
                    vnear[0].as_mut_ptr(), center.as_ptr(), x.as_ptr(), -(zhor[0] as f64));
                crate::engine::engine_util_blas::mju_add_scl3(
                    vnear[1].as_mut_ptr(), center.as_ptr(), x.as_ptr(), zhor[1] as f64);
                crate::engine::engine_util_blas::mju_add_scl3(
                    vnear[2].as_mut_ptr(), center.as_ptr(), x.as_ptr(), zhor[1] as f64);
                crate::engine::engine_util_blas::mju_add_scl3(
                    vnear[3].as_mut_ptr(), center.as_ptr(), x.as_ptr(), -(zhor[0] as f64));
                crate::engine::engine_util_blas::mju_add_to_scl3(
                    vnear[0].as_mut_ptr(), y.as_ptr(), -(zver[0] as f64));
                crate::engine::engine_util_blas::mju_add_to_scl3(
                    vnear[1].as_mut_ptr(), y.as_ptr(), -(zver[0] as f64));
                crate::engine::engine_util_blas::mju_add_to_scl3(
                    vnear[2].as_mut_ptr(), y.as_ptr(), zver[1] as f64);
                crate::engine::engine_util_blas::mju_add_to_scl3(
                    vnear[3].as_mut_ptr(), y.as_ptr(), zver[1] as f64);

                // vertices of the far plane
                if !orthographic {
                    zhor[0] *= (zfar / znear) as f32;
                    zhor[1] *= (zfar / znear) as f32;
                    zver[0] *= (zfar / znear) as f32;
                    zver[1] *= (zfar / znear) as f32;
                } else {
                    zfar = (zhor[0] as f64 + zver[0] as f64) / 2.0;
                }
                crate::engine::engine_util_blas::mju_add_scl3(
                    center.as_mut_ptr(), cam_xpos, z.as_ptr(), -zfar);
                crate::engine::engine_util_blas::mju_add_scl3(
                    vfar[0].as_mut_ptr(), center.as_ptr(), x.as_ptr(), -(zhor[0] as f64));
                crate::engine::engine_util_blas::mju_add_scl3(
                    vfar[1].as_mut_ptr(), center.as_ptr(), x.as_ptr(), zhor[1] as f64);
                crate::engine::engine_util_blas::mju_add_scl3(
                    vfar[2].as_mut_ptr(), center.as_ptr(), x.as_ptr(), zhor[1] as f64);
                crate::engine::engine_util_blas::mju_add_scl3(
                    vfar[3].as_mut_ptr(), center.as_ptr(), x.as_ptr(), -(zhor[0] as f64));
                crate::engine::engine_util_blas::mju_add_to_scl3(
                    vfar[0].as_mut_ptr(), y.as_ptr(), -(zver[0] as f64));
                crate::engine::engine_util_blas::mju_add_to_scl3(
                    vfar[1].as_mut_ptr(), y.as_ptr(), -(zver[0] as f64));
                crate::engine::engine_util_blas::mju_add_to_scl3(
                    vfar[2].as_mut_ptr(), y.as_ptr(), zver[1] as f64);
                crate::engine::engine_util_blas::mju_add_to_scl3(
                    vfar[3].as_mut_ptr(), y.as_ptr(), zver[1] as f64);

                // triangulation and wireframe
                for e in 0..4i32 {
                    let e1 = ((e + 1) % 4) as usize;
                    add_triangle(
                        scn, vnear[e as usize].as_ptr(), vfar[e as usize].as_ptr(),
                        vnear[e1].as_ptr(), rgba_frustum, i, MJCAT_DECOR, MJOBJ_CAMERA);
                    add_triangle(
                        scn, vfar[e as usize].as_ptr(), vfar[e1].as_ptr(),
                        vnear[e1].as_ptr(), rgba_frustum, i, MJCAT_DECOR, MJOBJ_CAMERA);
                    add_connector(
                        scn, MJGEOM_LINE, 3.0, vnear[e as usize].as_ptr(),
                        vnear[e1].as_ptr(), rgba_frustum, i, MJCAT_DECOR, MJOBJ_CAMERA);
                    add_connector(
                        scn, MJGEOM_LINE, 3.0, vfar[e as usize].as_ptr(),
                        vfar[e1].as_ptr(), rgba_frustum, i, MJCAT_DECOR, MJOBJ_CAMERA);
                    add_connector(
                        scn, MJGEOM_LINE, 3.0, vnear[e as usize].as_ptr(),
                        vfar[e as usize].as_ptr(), rgba_frustum, i, MJCAT_DECOR, MJOBJ_CAMERA);
                }
            }

            let mut thisgeom = acquire_geom(scn, i, MJCAT_DECOR, MJOBJ_CAMERA);
            if thisgeom.is_null() {
                return;
            }

            // construct geom: camera body
            // vis.scale: index 5 = camera
            let cam_scale = *scale_ptr.add(5);
            (*thisgeom).r#type = MJGEOM_BOX;
            (*thisgeom).size[0] = scl * cam_scale * 1.0;
            (*thisgeom).size[1] = scl * cam_scale * 0.8;
            (*thisgeom).size[2] = scl * cam_scale * 0.4;
            crate::engine::engine_util_misc::mju_n2f(
                (*thisgeom).pos.as_mut_ptr(), (*d).cam_xpos.add(3 * i as usize), 3);
            crate::engine::engine_util_misc::mju_n2f(
                (*thisgeom).mat.as_mut_ptr(), (*d).cam_xmat.add(9 * i as usize), 9);
            f2f((*thisgeom).rgba.as_mut_ptr(), cam_rgba.as_ptr(), 4);

            if (*vopt).label == MJLABEL_CAMERA {
                make_label(m, MJOBJ_CAMERA as u32, i, (*thisgeom).label.as_mut_ptr());
            }

            release_geom(&mut thisgeom, scn);

            thisgeom = acquire_geom(scn, i, MJCAT_DECOR, MJOBJ_CAMERA);
            if thisgeom.is_null() {
                return;
            }

            // construct geom: lens
            let cam_xpos_i = (*d).cam_xpos.add(3 * i as usize);
            let cam_xmat_i = (*d).cam_xmat.add(9 * i as usize);
            (*thisgeom).pos[0] = (*cam_xpos_i.add(0)
                - scl as f64 * cam_scale as f64 * 0.6 * *cam_xmat_i.add(2)) as f32;
            (*thisgeom).pos[1] = (*cam_xpos_i.add(1)
                - scl as f64 * cam_scale as f64 * 0.6 * *cam_xmat_i.add(5)) as f32;
            (*thisgeom).pos[2] = (*cam_xpos_i.add(2)
                - scl as f64 * cam_scale as f64 * 0.6 * *cam_xmat_i.add(8)) as f32;
            (*thisgeom).r#type = MJGEOM_CYLINDER;
            (*thisgeom).size[0] = scl * cam_scale * 0.4;
            (*thisgeom).size[1] = scl * cam_scale * 0.4;
            (*thisgeom).size[2] = scl * cam_scale * 0.3;
            crate::engine::engine_util_misc::mju_n2f(
                (*thisgeom).mat.as_mut_ptr(), cam_xmat_i, 9);
            f2f((*thisgeom).rgba.as_mut_ptr(), cam_rgba.as_ptr(), 4);
            for k in 0..3 {
                (*thisgeom).rgba[k] *= 0.5;
            }

            release_geom(&mut thisgeom, scn);

            if (*vopt).frame != MJFRAME_CAMERA {
                continue;
            }
            // vis.scale: index 13 = framewidth, index 12 = framelength
            let width = *scale_ptr.add(13) as f64 * scl as f64;
            let length = *scale_ptr.add(12) as f64 * scl as f64;
            add_frame(scn, i, cam_xpos_i, cam_xmat_i, length as f32, width as f32);
        }
    }
}

/// C: addLightGeoms (engine/engine_vis_visualize.c:2460)
/// Calls: acquireGeom, addFrame, f2f, makeLabel, mju_addScl3, mju_n2f, mju_quat2Mat, mju_quatZ2Vec, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_light_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    const MJ_VIS_LIGHT: usize = 6;
    const MJ_CAT_DECOR: i32 = 4;
    const MJ_OBJ_LIGHT: i32 = 8;
    const MJ_GEOM_CYLINDER: i32 = 5;
    const MJ_FRAME_LIGHT: i32 = 5;
    const MJ_LABEL_LIGHT: i32 = 6;

    // SAFETY: m, d, vopt, scn are valid pointers (caller contract).
    unsafe {
        if (*vopt).flags[MJ_VIS_LIGHT] == 0 {
            return;
        }

        let scl = (*m).stat.meansize as f32;
        let scale_floats = (*m).vis.scale.as_ptr() as *const f32;
        let light_scale = *scale_floats.add(6);
        let framelength = *scale_floats.add(12);
        let framewidth = *scale_floats.add(13);

        let rgba_floats = (*m).vis.rgba._data.as_ptr() as *const f32;
        let light_rgba = rgba_floats.add(10 * 4);

        for i in 0..(*m).nlight as i32 {
            // make light frame
            let mut quat: [f64; 4] = [0.0; 4];
            crate::engine::engine_util_spatial::mju_quat_z2vec(
                quat.as_mut_ptr(), (*d).light_xdir.add(3 * i as usize));

            let mut mat: [f64; 9] = [0.0; 9];
            crate::engine::engine_util_spatial::mju_quat2mat(mat.as_mut_ptr(), quat.as_ptr());

            // make light position: offset backward, to avoid casting shadow
            let mut vec: [f64; 3] = [0.0; 3];
            crate::engine::engine_util_blas::mju_add_scl3(
                vec.as_mut_ptr(),
                (*d).light_xpos.add(3 * i as usize),
                (*d).light_xdir.add(3 * i as usize),
                -(scl as f64) * light_scale as f64 - 0.0001,
            );

            let mut thisgeom = acquire_geom(scn, i, MJ_CAT_DECOR, MJ_OBJ_LIGHT);
            if thisgeom.is_null() {
                return;
            }

            // construct geom
            (*thisgeom).r#type = MJ_GEOM_CYLINDER;
            (*thisgeom).size[0] = scl * light_scale * 0.8;
            (*thisgeom).size[1] = scl * light_scale * 0.8;
            (*thisgeom).size[2] = scl * light_scale * 1.0;
            crate::engine::engine_util_misc::mju_n2f(
                (*thisgeom).pos.as_mut_ptr(), vec.as_ptr(), 3);
            crate::engine::engine_util_misc::mju_n2f(
                (*thisgeom).mat.as_mut_ptr(), mat.as_ptr(), 9);
            f2f((*thisgeom).rgba.as_mut_ptr(), light_rgba, 4);

            // label
            if (*vopt).label == MJ_LABEL_LIGHT {
                make_label(m, MJ_OBJ_LIGHT as u32, i, (*thisgeom).label.as_mut_ptr());
            }

            release_geom(&mut thisgeom, scn);

            if (*vopt).frame != MJ_FRAME_LIGHT {
                continue;
            }
            let width = framewidth * scl;
            let length = framelength * scl;
            add_frame(scn, i, (*d).light_xpos.add(3 * i as usize), mat.as_ptr(), length, width);
        }
    }
}

/// C: addCenterOfMassGeoms (engine/engine_vis_visualize.c:2509)
/// Calls: acquireGeom, f2f, mju_n2f, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_center_of_mass_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    const MJ_VIS_COM: usize = 20;
    const MJ_CAT_DECOR: i32 = 4;
    const MJ_OBJ_UNKNOWN: i32 = 0;
    const MJ_GEOM_SPHERE: i32 = 4;
    static IDENTITY: [f64; 9] = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];

    // SAFETY: m, d, vopt, scn are valid pointers (caller contract).
    unsafe {
        if (*vopt).flags[MJ_VIS_COM] == 0 {
            return;
        }

        let scl = (*m).stat.meansize as f32;

        // vis.scale.com is index 4 in the scale float array
        let scale_floats = (*m).vis.scale.as_ptr() as *const f32;
        let com_scale = *scale_floats.add(4);

        // vis.rgba.com is index 8 in the rgba float array (each entry is 4 floats)
        let rgba_floats = (*m).vis.rgba._data.as_ptr() as *const f32;
        let com_rgba = rgba_floats.add(8 * 4);

        for i in 1..(*m).nbody as i32 {
            if *(*m).body_rootid.add(i as usize) == i {
                let mut thisgeom = acquire_geom(scn, i, MJ_CAT_DECOR, MJ_OBJ_UNKNOWN);
                if thisgeom.is_null() {
                    return;
                }

                (*thisgeom).r#type = MJ_GEOM_SPHERE;
                let sz = scl * com_scale;
                (*thisgeom).size[0] = sz;
                (*thisgeom).size[1] = sz;
                (*thisgeom).size[2] = sz;
                crate::engine::engine_util_misc::mju_n2f(
                    (*thisgeom).pos.as_mut_ptr(), (*d).subtree_com.add(3 * i as usize), 3,
                );
                crate::engine::engine_util_misc::mju_n2f(
                    (*thisgeom).mat.as_mut_ptr(), IDENTITY.as_ptr(), 9,
                );
                f2f((*thisgeom).rgba.as_mut_ptr(), com_rgba, 4);
                release_geom(&mut thisgeom, scn);
            }
        }
    }
}

/// C: addAutoConnectGeoms (engine/engine_vis_visualize.c:2535)
/// Calls: addConnector
#[allow(unused_variables, non_snake_case)]
pub fn add_auto_connect_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    const MJ_VIS_AUTOCONNECT: usize = 19;
    const MJ_GEOM_CAPSULE: i32 = 3;
    const MJ_CAT_DECOR: i32 = 4;
    const MJ_OBJ_UNKNOWN: i32 = 0;

    // SAFETY: m, d, vopt, scn are valid pointers (caller contract).
    unsafe {
        if (*vopt).flags[MJ_VIS_AUTOCONNECT] == 0 {
            return;
        }

        let scl = (*m).stat.meansize as f32;

        // vis.scale.connect is index 3 in the scale float array
        let scale_floats = (*m).vis.scale.as_ptr() as *const f32;
        let connect_scale = *scale_floats.add(3);

        // vis.rgba.connect is index 12 in the rgba float array (each entry is 4 floats)
        let rgba_floats = (*m).vis.rgba._data.as_ptr() as *const f32;
        let connect_rgba = rgba_floats.add(12 * 4);

        for i in 1..(*m).nbody as i32 {
            // do not connect to world
            if *(*m).body_parentid.add(i as usize) == 0 {
                continue;
            }

            // start at body com, connect joint centers in reverse order
            let mut cur: *const f64 = (*d).xipos.add(3 * i as usize);
            if *(*m).body_jntnum.add(i as usize) != 0 {
                let jntadr = *(*m).body_jntadr.add(i as usize);
                let jntnum = *(*m).body_jntnum.add(i as usize);
                let mut j = jntadr + jntnum - 1;
                while j >= jntadr {
                    let nxt: *const f64 = (*d).xanchor.add(3 * j as usize);

                    // construct geom
                    add_connector(
                        scn, MJ_GEOM_CAPSULE, (scl * connect_scale) as f64,
                        cur, nxt, connect_rgba, i, MJ_CAT_DECOR, MJ_OBJ_UNKNOWN,
                    );

                    cur = nxt;
                    j -= 1;
                }
            }

            // connect first joint (or com) to parent com
            let first: *const f64 = (*d).xipos.add(3 * *(*m).body_parentid.add(i as usize) as usize);
            add_connector(
                scn, MJ_GEOM_CAPSULE, (scl * connect_scale) as f64,
                cur, first, connect_rgba, i, MJ_CAT_DECOR, MJ_OBJ_UNKNOWN,
            );
        }
    }
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
    const MJ_VIS_PERTFORCE: usize = 12;
    const MJ_GEOM_ARROW: i32 = 100;
    const MJ_CAT_DECOR: i32 = 4;
    const MJ_OBJ_UNKNOWN: i32 = 0;
    const MJMINVAL: f64 = 1e-15;

    // SAFETY: m, d, vopt, scn are valid pointers (caller contract).
    unsafe {
        if (*vopt).flags[MJ_VIS_PERTFORCE] == 0 {
            return;
        }

        let scl = (*m).stat.meansize as f32;

        // vis.scale.forcewidth is index 0 in the scale float array
        let scale_floats = (*m).vis.scale.as_ptr() as *const f32;
        let forcewidth = *scale_floats.add(0);

        // vis.map.force is index 2 in the map float array
        let map_floats = (*m).vis.map.as_ptr() as *const f32;
        let map_force = *map_floats.add(2);

        // vis.rgba.force is index 2 in the rgba float array (each entry is 4 floats)
        let rgba_floats = (*m).vis.rgba._data.as_ptr() as *const f32;
        let force_rgba = rgba_floats.add(2 * 4);

        for i in 1..(*m).nbody as i32 {
            if crate::engine::engine_util_misc::mju_is_zero((*d).xfrc_applied.add(6 * i as usize), 6) != 0 {
                continue;
            }

            // force perturbation
            let xfrc: *const f64 = (*d).xfrc_applied.add(6 * i as usize);
            if crate::engine::engine_util_blas::mju_norm3(xfrc) <= MJMINVAL {
                continue;
            }

            let from: *const f64 = (*d).xipos.add(3 * i as usize);

            // map force to spatial vector in world frame
            let mut vec: [f64; 3] = [0.0; 3];
            crate::engine::engine_util_blas::mju_scl3(
                vec.as_mut_ptr(), xfrc, map_force as f64 / (*m).stat.meanmass,
            );
            let mut to: [f64; 3] = [0.0; 3];
            crate::engine::engine_util_blas::mju_add3(to.as_mut_ptr(), from, vec.as_ptr());

            add_connector(
                scn, MJ_GEOM_ARROW, (forcewidth * scl) as f64,
                from, to.as_ptr(), force_rgba, i, MJ_CAT_DECOR, MJ_OBJ_UNKNOWN,
            );
        }
    }
}

/// C: addConstraintGeoms (engine/engine_vis_visualize.c:2760)
/// Calls: acquireGeom, makeLabel, mju_addTo3, mju_copy3, mju_mulMatVec3, mjv_initGeom, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_constraint_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    const MJVIS_CONSTRAINT: usize = 9;
    const MJEQ_CONNECT: i32 = 0;
    const MJEQ_WELD: i32 = 1;
    const MJOBJ_SITE: i32 = 6;
    const MJNEQDATA: i32 = 11;
    const MJCAT_DECOR: i32 = 4;
    const MJOBJ_EQUALITY: i32 = 17;
    const MJGEOM_SPHERE: i32 = 3;
    const MJLABEL_CONSTRAINT: i32 = 9;

    // SAFETY: m, d, vopt, scn are valid pointers (caller contract).
    unsafe {
        if *(*vopt).flags.as_ptr().add(MJVIS_CONSTRAINT) == 0 {
            return;
        }

        let scl = (*m).stat.meansize as f32;

        for i in 0..(*m).neq as i32 {
            let is_weld = (*(*m).eq_type.add(i as usize) == MJEQ_WELD) as i32;
            let is_connect = (*(*m).eq_type.add(i as usize) == MJEQ_CONNECT) as i32;
            if *(*d).eq_active.add(i as usize) && (is_connect != 0 || is_weld != 0) {
                let mut vec: [f64; 3] = [0.0; 3];
                let mut end: [f64; 3] = [0.0; 3];
                let xmat_j: *const f64;
                let xmat_k: *const f64;
                let j = *(*m).eq_obj1id.add(i as usize);
                let k = *(*m).eq_obj2id.add(i as usize);

                if *(*m).eq_objtype.add(i as usize) == MJOBJ_SITE {
                    crate::engine::engine_util_blas::mju_copy3(
                        vec.as_mut_ptr(), (*d).site_xpos.add(3 * j as usize));
                    crate::engine::engine_util_blas::mju_copy3(
                        end.as_mut_ptr(), (*d).site_xpos.add(3 * k as usize));
                    xmat_j = (*d).site_xmat.add(9 * j as usize);
                    xmat_k = (*d).site_xmat.add(9 * k as usize);
                } else {
                    crate::engine::engine_util_blas::mju_mul_mat_vec3(
                        vec.as_mut_ptr(),
                        (*d).xmat.add(9 * j as usize),
                        (*m).eq_data.add((MJNEQDATA * i + 3 * is_weld) as usize),
                    );
                    crate::engine::engine_util_blas::mju_add_to3(
                        vec.as_mut_ptr(), (*d).xpos.add(3 * j as usize));
                    crate::engine::engine_util_blas::mju_mul_mat_vec3(
                        end.as_mut_ptr(),
                        (*d).xmat.add(9 * k as usize),
                        (*m).eq_data.add((MJNEQDATA * i + 3 * is_connect) as usize),
                    );
                    crate::engine::engine_util_blas::mju_add_to3(
                        end.as_mut_ptr(), (*d).xpos.add(3 * k as usize));
                    xmat_j = (*d).xmat.add(9 * j as usize);
                    xmat_k = (*d).xmat.add(9 * k as usize);
                }

                // construct geom: size
                let mut sz: [f64; 3] = [0.0; 3];
                // vis.scale: index 14 = constraint
                let scale_ptr = (*m).vis.scale.as_ptr() as *const f32;
                sz[0] = (scl * *scale_ptr.add(14)) as f64;

                // vis.rgba: connect at index 48, constraint at index 76
                let rgba_ptr = (*m).vis.rgba._data.as_ptr() as *const f32;

                let mut thisgeom = acquire_geom(scn, i, MJCAT_DECOR, MJOBJ_EQUALITY);
                if thisgeom.is_null() {
                    return;
                }
                mjv_init_geom(
                    thisgeom, MJGEOM_SPHERE, sz.as_ptr(), vec.as_ptr(),
                    xmat_j, rgba_ptr.add(48),
                );
                if (*vopt).label == MJLABEL_CONSTRAINT {
                    make_label(m, MJOBJ_EQUALITY as u32, i, (*thisgeom).label.as_mut_ptr());
                }
                release_geom(&mut thisgeom, scn);

                thisgeom = acquire_geom(scn, i, MJCAT_DECOR, MJOBJ_EQUALITY);
                if thisgeom.is_null() {
                    return;
                }
                mjv_init_geom(
                    thisgeom, MJGEOM_SPHERE, sz.as_ptr(), end.as_ptr(),
                    xmat_k, rgba_ptr.add(76),
                );
                if (*vopt).label == MJLABEL_CONSTRAINT {
                    make_label(m, MJOBJ_EQUALITY as u32, i, (*thisgeom).label.as_mut_ptr());
                }
                release_geom(&mut thisgeom, scn);
            }
        }
    }
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
    // SAFETY: caller guarantees _face, _normal have 9*(nface+1) elements;
    //         vertnorm and vertxpos have at least 3*max(i0,i1,i2)+3 elements
    unsafe {
        let face = _face.add(9 * nface as usize);
        let normal = _normal.add(9 * nface as usize);
        let ind: [i32; 3] = [i0, i1, i2];
        let sign: f64 = if radius > 0.0 { 1.0 } else { -1.0 };

        // flat shading
        if flg_flat != 0 {
            // compute face normal
            let v0 = vertxpos.add(3 * i0 as usize);
            let v1 = vertxpos.add(3 * i1 as usize);
            let v2 = vertxpos.add(3 * i2 as usize);
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

            // set all vertex normals equal to face normal
            for k in 0..3 {
                *normal.add(3 * k + 0) = (sign * nrm[0]) as f32;
                *normal.add(3 * k + 1) = (sign * nrm[1]) as f32;
                *normal.add(3 * k + 2) = (sign * nrm[2]) as f32;
            }
        }
        // smooth shading
        else {
            for k in 0..3 {
                *normal.add(3 * k + 0) = (sign * *vertnorm.add(3 * ind[k] as usize + 0)) as f32;
                *normal.add(3 * k + 1) = (sign * *vertnorm.add(3 * ind[k] as usize + 1)) as f32;
                *normal.add(3 * k + 2) = (sign * *vertnorm.add(3 * ind[k] as usize + 2)) as f32;
            }
        }

        // set positions: vertices offset by radius*normal
        for k in 0..3 {
            *face.add(3 * k + 0) = (*vertxpos.add(3 * ind[k] as usize + 0)
                + radius * *vertnorm.add(3 * ind[k] as usize + 0)) as f32;
            *face.add(3 * k + 1) = (*vertxpos.add(3 * ind[k] as usize + 1)
                + radius * *vertnorm.add(3 * ind[k] as usize + 1)) as f32;
            *face.add(3 * k + 2) = (*vertxpos.add(3 * ind[k] as usize + 2)
                + radius * *vertnorm.add(3 * ind[k] as usize + 2)) as f32;
        }
    }
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
    // SAFETY: caller guarantees _face, _normal have 9*(nface+1) elements;
    //         vertnorm and vertxpos have at least 3*max(i0,i1)+3 elements
    unsafe {
        let face = _face.add(9 * nface as usize);
        let normal = _normal.add(9 * nface as usize);

        // compute normal
        let v0 = vertxpos.add(3 * i0 as usize);
        let v1 = vertxpos.add(3 * i1 as usize);
        let v01: [f64; 3] = [
            *v1.add(0) - *v0.add(0),
            *v1.add(1) - *v0.add(1),
            *v1.add(2) - *v0.add(2),
        ];
        let mut nrm: [f64; 3] = [0.0; 3];
        crate::engine::engine_util_spatial::mju_cross(nrm.as_mut_ptr(), v01.as_ptr(), vertnorm.add(3 * i1 as usize));
        if radius < 0.0 {
            crate::engine::engine_util_blas::mju_scl3(nrm.as_mut_ptr(), nrm.as_ptr(), -1.0);
        }
        crate::engine::engine_util_blas::mju_normalize3(nrm.as_mut_ptr());

        // set normals
        for k in 0..3 {
            *normal.add(3 * k + 0) = nrm[0] as f32;
            *normal.add(3 * k + 1) = nrm[1] as f32;
            *normal.add(3 * k + 2) = nrm[2] as f32;
        }

        // set positions
        let ind: [i32; 3] = [i0, i1, i1];
        for k in 0..3 {
            let sign: f64 = if k == 1 { -1.0 } else { 1.0 };
            *face.add(3 * k + 0) = (*vertxpos.add(3 * ind[k] as usize + 0)
                + sign * radius * *vertnorm.add(3 * ind[k] as usize + 0)) as f32;
            *face.add(3 * k + 1) = (*vertxpos.add(3 * ind[k] as usize + 1)
                + sign * radius * *vertnorm.add(3 * ind[k] as usize + 1)) as f32;
            *face.add(3 * k + 2) = (*vertxpos.add(3 * ind[k] as usize + 2)
                + sign * radius * *vertnorm.add(3 * ind[k] as usize + 2)) as f32;
        }
    }
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
    // SAFETY: geom is a valid mjvGeom pointer. size/pos/mat/rgba may be null.
    unsafe {
        (*geom).r#type = r#type;

        // set size
        if !size.is_null() {
            match r#type as u32 {
                mjtGeom_mjGEOM_SPHERE => {
                    (*geom).size[0] = *size.add(0) as f32;
                    (*geom).size[1] = *size.add(0) as f32;
                    (*geom).size[2] = *size.add(0) as f32;
                }
                mjtGeom_mjGEOM_CAPSULE => {
                    (*geom).size[0] = *size.add(0) as f32;
                    (*geom).size[1] = *size.add(0) as f32;
                    (*geom).size[2] = *size.add(1) as f32;
                }
                mjtGeom_mjGEOM_CYLINDER => {
                    (*geom).size[0] = *size.add(0) as f32;
                    (*geom).size[1] = *size.add(0) as f32;
                    (*geom).size[2] = *size.add(1) as f32;
                }
                _ => {
                    crate::engine::engine_util_misc::mju_n2f((*geom).size.as_mut_ptr(), size, 3);
                }
            }
        } else {
            (*geom).size[0] = 0.1_f32;
            (*geom).size[1] = 0.1_f32;
            (*geom).size[2] = 0.1_f32;
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
            (*geom).mat[0] = 1.0;
            (*geom).mat[1] = 0.0;
            (*geom).mat[2] = 0.0;
            (*geom).mat[3] = 0.0;
            (*geom).mat[4] = 1.0;
            (*geom).mat[5] = 0.0;
            (*geom).mat[6] = 0.0;
            (*geom).mat[7] = 0.0;
            (*geom).mat[8] = 1.0;
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
    // SAFETY: All pointers are valid (caller contract). Performs nested loops over skin data.
    unsafe {
        const MJ_NGROUP: i32 = 6;
        const MJ_MINVAL_F64: f64 = 1E-15_f64;

        for i in 0..(*m).nskin as i32 {
            let vertadr = *(*m).skin_vertadr.add(i as usize);
            let vertnum = *(*m).skin_vertnum.add(i as usize);
            let faceadr = *(*m).skin_faceadr.add(i as usize);
            let facenum = *(*m).skin_facenum.add(i as usize);

            // clear positions and normals
            std::ptr::write_bytes((*scn).skinvert.add(3 * vertadr as usize), 0, 3 * vertnum as usize);
            std::ptr::write_bytes((*scn).skinnormal.add(3 * vertadr as usize), 0, 3 * vertnum as usize);

            // update only if visible
            let group = *(*m).skin_group.add(i as usize);
            let clamped = if 0 > (if (MJ_NGROUP - 1) < group { MJ_NGROUP - 1 } else { group }) {
                0
            } else {
                if (MJ_NGROUP - 1) < group { MJ_NGROUP - 1 } else { group }
            };
            if (*opt).skingroup[clamped as usize] != 0 {
                // accumulate positions from all bones
                let bone_end = *(*m).skin_boneadr.add(i as usize) + *(*m).skin_bonenum.add(i as usize);
                for j in *(*m).skin_boneadr.add(i as usize)..bone_end {
                    // get bind pose
                    let mut bindpos: [f64; 3] = [
                        *(*m).skin_bonebindpos.add(3 * j as usize) as f64,
                        *(*m).skin_bonebindpos.add(3 * j as usize + 1) as f64,
                        *(*m).skin_bonebindpos.add(3 * j as usize + 2) as f64,
                    ];
                    let mut bindquat: [f64; 4] = [
                        *(*m).skin_bonebindquat.add(4 * j as usize) as f64,
                        *(*m).skin_bonebindquat.add(4 * j as usize + 1) as f64,
                        *(*m).skin_bonebindquat.add(4 * j as usize + 2) as f64,
                        *(*m).skin_bonebindquat.add(4 * j as usize + 3) as f64,
                    ];

                    // compute rotation
                    let bodyid = *(*m).skin_bonebodyid.add(j as usize);
                    let mut quat: [f64; 4] = [0.0; 4];
                    let mut quatneg: [f64; 4] = [0.0; 4];
                    let mut rotate: [f64; 9] = [0.0; 9];
                    crate::engine::engine_util_spatial::mju_neg_quat(quatneg.as_mut_ptr(), bindquat.as_ptr());
                    crate::engine::engine_util_spatial::mju_mul_quat(quat.as_mut_ptr(), (*d).xquat.add(4 * bodyid as usize), quatneg.as_ptr());
                    crate::engine::engine_util_spatial::mju_quat2mat(rotate.as_mut_ptr(), quat.as_ptr());

                    // compute translation
                    let mut translate: [f64; 3] = [0.0; 3];
                    crate::engine::engine_util_blas::mju_mul_mat_vec3(translate.as_mut_ptr(), rotate.as_ptr(), bindpos.as_ptr());
                    crate::engine::engine_util_blas::mju_sub3(translate.as_mut_ptr(), (*d).xpos.add(3 * bodyid as usize), translate.as_ptr());

                    // process all bone vertices
                    let vert_end = *(*m).skin_bonevertadr.add(j as usize) + *(*m).skin_bonevertnum.add(j as usize);
                    for k in *(*m).skin_bonevertadr.add(j as usize)..vert_end {
                        let vid = *(*m).skin_bonevertid.add(k as usize);
                        let vweight = *(*m).skin_bonevertweight.add(k as usize);

                        // get original position
                        let pos: [f64; 3] = [
                            *(*m).skin_vert.add(3 * (vertadr + vid) as usize) as f64,
                            *(*m).skin_vert.add(3 * (vertadr + vid) as usize + 1) as f64,
                            *(*m).skin_vert.add(3 * (vertadr + vid) as usize + 2) as f64,
                        ];

                        // transform
                        let mut pos1: [f64; 3] = [0.0; 3];
                        crate::engine::engine_util_blas::mju_mul_mat_vec3(pos1.as_mut_ptr(), rotate.as_ptr(), pos.as_ptr());
                        crate::engine::engine_util_blas::mju_add_to3(pos1.as_mut_ptr(), translate.as_ptr());

                        // accumulate position: float += float * (float)double
                        // C: scn->skinvert[idx] += vweight*(float)pos1[k]
                        let base = 3 * (vertadr + vid) as usize;
                        *(*scn).skinvert.add(base) += vweight * (pos1[0] as f32);
                        *(*scn).skinvert.add(base + 1) += vweight * (pos1[1] as f32);
                        *(*scn).skinvert.add(base + 2) += vweight * (pos1[2] as f32);
                    }
                }

                // compute vertex normals from face normals
                for k in faceadr..(faceadr + facenum) {
                    // get face vertex indices
                    let vid: [i32; 3] = [
                        *(*m).skin_face.add(3 * k as usize),
                        *(*m).skin_face.add(3 * k as usize + 1),
                        *(*m).skin_face.add(3 * k as usize + 2),
                    ];

                    // get triangle edges
                    let mut vec01: [f64; 3] = [0.0; 3];
                    let mut vec02: [f64; 3] = [0.0; 3];
                    for r in 0..3_i32 {
                        vec01[r as usize] = *(*scn).skinvert.add((3 * (vertadr + vid[1]) + r) as usize) as f64
                            - *(*scn).skinvert.add((3 * (vertadr + vid[0]) + r) as usize) as f64;
                        vec02[r as usize] = *(*scn).skinvert.add((3 * (vertadr + vid[2]) + r) as usize) as f64
                            - *(*scn).skinvert.add((3 * (vertadr + vid[0]) + r) as usize) as f64;
                    }

                    // compute face normal
                    let mut nrm: [f64; 3] = [0.0; 3];
                    crate::engine::engine_util_spatial::mju_cross(nrm.as_mut_ptr(), vec01.as_ptr(), vec02.as_ptr());

                    // add normal to each vertex with weight = area
                    for r in 0..3_i32 {
                        for t in 0..3_i32 {
                            // C: float += double => (float)((double)float + double)
                            let ptr = (*scn).skinnormal.add((3 * (vertadr + vid[r as usize]) + t) as usize);
                            *ptr = (*ptr as f64 + nrm[t as usize]) as f32;
                        }
                    }
                }

                // normalize normals
                for k in vertadr..(vertadr + vertnum) {
                    let s = f32::sqrt(
                        *(*scn).skinnormal.add(3 * k as usize) * *(*scn).skinnormal.add(3 * k as usize)
                        + *(*scn).skinnormal.add(3 * k as usize + 1) * *(*scn).skinnormal.add(3 * k as usize + 1)
                        + *(*scn).skinnormal.add(3 * k as usize + 2) * *(*scn).skinnormal.add(3 * k as usize + 2)
                    );
                    let scl: f32 = (1.0_f64 / (if MJ_MINVAL_F64 > s as f64 { MJ_MINVAL_F64 } else { s as f64 })) as f32;

                    *(*scn).skinnormal.add(3 * k as usize) *= scl;
                    *(*scn).skinnormal.add(3 * k as usize + 1) *= scl;
                    *(*scn).skinnormal.add(3 * k as usize + 2) *= scl;
                }

                // inflate
                if *(*m).skin_inflate.add(i as usize) != 0.0 {
                    let inflate = *(*m).skin_inflate.add(i as usize);
                    for k in vertadr..(vertadr + vertnum) {
                        *(*scn).skinvert.add(3 * k as usize) += inflate * *(*scn).skinnormal.add(3 * k as usize);
                        *(*scn).skinvert.add(3 * k as usize + 1) += inflate * *(*scn).skinnormal.add(3 * k as usize + 1);
                        *(*scn).skinvert.add(3 * k as usize + 2) += inflate * *(*scn).skinnormal.add(3 * k as usize + 2);
                    }
                }
            }
        }
    }
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
    const MJ_CAMERA_FREE: i32 = 0;
    const MJ_CAMERA_TRACKING: i32 = 1;
    const MJ_CAMERA_FIXED: i32 = 2;
    const MJ_PI: f64 = std::f64::consts::PI;

    // SAFETY: d, cam are valid pointers (caller contract). headpos/forward/up/right may be null.
    unsafe {
        let cam_type = (*cam).r#type;

        if cam_type == MJ_CAMERA_FREE || cam_type == MJ_CAMERA_TRACKING {
            let ca = f64::cos((*cam).azimuth / 180.0 * MJ_PI);
            let sa = f64::sin((*cam).azimuth / 180.0 * MJ_PI);
            let ce = f64::cos((*cam).elevation / 180.0 * MJ_PI);
            let se = f64::sin((*cam).elevation / 180.0 * MJ_PI);
            if !forward.is_null() {
                *forward.add(0) = ce * ca;
                *forward.add(1) = ce * sa;
                *forward.add(2) = se;
            }
            if !up.is_null() {
                *up.add(0) = -se * ca;
                *up.add(1) = -se * sa;
                *up.add(2) = ce;
            }
            if !right.is_null() {
                *right.add(0) = sa;
                *right.add(1) = -ca;
                *right.add(2) = 0.0;
            }
            if !headpos.is_null() {
                crate::engine::engine_util_blas::mju_add_scl3(
                    headpos, (*cam).lookat.as_ptr(), forward, -(*cam).distance);
            }
        } else if cam_type == MJ_CAMERA_FIXED {
            let cid = (*cam).fixedcamid;
            let mat = (*d).cam_xmat.add(9 * cid as usize);
            if !forward.is_null() {
                *forward.add(0) = -*mat.add(2);
                *forward.add(1) = -*mat.add(5);
                *forward.add(2) = -*mat.add(8);
            }
            if !up.is_null() {
                *up.add(0) = *mat.add(1);
                *up.add(1) = *mat.add(4);
                *up.add(2) = *mat.add(7);
            }
            if !right.is_null() {
                *right.add(0) = *mat.add(0);
                *right.add(1) = *mat.add(3);
                *right.add(2) = *mat.add(6);
            }
            if !headpos.is_null() {
                crate::engine::engine_util_blas::mju_copy3(
                    headpos, (*d).cam_xpos.add(3 * cid as usize));
            }
        } else {
            crate::engine::engine_util_errmem::mju_error(
                b"unknown camera type\0".as_ptr() as *const i8);
        }
    }
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
    const MJ_CAMERA_FREE: i32 = 0;
    const MJ_CAMERA_TRACKING: i32 = 1;
    const MJ_CAMERA_FIXED: i32 = 2;
    const MJ_PROJ_ORTHOGRAPHIC: i32 = 1;
    const MJ_PI: f64 = std::f64::consts::PI;

    // SAFETY: m, cam are valid pointers. zver, zhor, zclip may be null.
    unsafe {
        let mut fovy: f64;
        let mut orthographic: i32 = 0;
        let mut intrinsic: *const f32 = std::ptr::null();
        let mut sensorsize: *const f32 = std::ptr::null();

        let cam_type = (*cam).r#type;
        if cam_type == MJ_CAMERA_FREE || cam_type == MJ_CAMERA_TRACKING {
            // vis.global: orthographic at byte 4 (i32), fovy at byte 8 (f32)
            let global_ints = (*m).vis.global.as_ptr() as *const i32;
            orthographic = *global_ints.add(1);
            let global_floats = (*m).vis.global.as_ptr().add(8) as *const f32;
            fovy = *global_floats as f64;
        } else if cam_type == MJ_CAMERA_FIXED {
            let cid = (*cam).fixedcamid;
            if cid < 0 || cid as i64 >= (*m).ncam {
                crate::engine::engine_util_errmem::mju_error(
                    b"fixed camera id is outside valid range\0".as_ptr() as *const i8);
                return;
            }
            orthographic = if *(*m).cam_projection.add(cid as usize) == MJ_PROJ_ORTHOGRAPHIC { 1 } else { 0 };
            fovy = *(*m).cam_fovy.add(cid as usize);

            // if positive sensorsize, get sensorsize and intrinsic
            if *(*m).cam_sensorsize.add(2 * cid as usize + 1) != 0.0 {
                sensorsize = (*m).cam_sensorsize.add(2 * cid as usize);
                intrinsic = (*m).cam_intrinsic.add(4 * cid as usize);
            }
        } else {
            crate::engine::engine_util_errmem::mju_error(
                b"unknown camera type\0".as_ptr() as *const i8);
            return;
        }

        // vis.map: znear at index 7, zfar at index 8
        let map_floats = (*m).vis.map.as_ptr() as *const f32;
        let znear = *map_floats.add(7) * (*m).stat.extent as f32;

        if orthographic != 0 {
            if !zver.is_null() {
                *zver.add(0) = fovy as f32 / 2.0;
                *zver.add(1) = fovy as f32 / 2.0;
            }
            if !zhor.is_null() {
                *zhor.add(0) = 0.0;
                *zhor.add(1) = 0.0;
            }
        } else if !intrinsic.is_null() {
            get_frustum(zver, zhor, znear, intrinsic, sensorsize);
        } else {
            if !zver.is_null() {
                let half = znear * f64::tan(fovy * MJ_PI / 360.0) as f32;
                *zver.add(0) = half;
                *zver.add(1) = half;
            }
            if !zhor.is_null() {
                *zhor.add(0) = 0.0;
                *zhor.add(1) = 0.0;
            }
        }

        if !zclip.is_null() {
            *zclip.add(0) = znear;
            *zclip.add(1) = *map_floats.add(8) * (*m).stat.extent as f32;
        }
    }
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
    // SAFETY: m, d are valid model/data pointers. i is valid tendon index.
    // length is a valid output pointer.
    unsafe {
        const MJ_NPOLY: i32 = 2;
        const MJ_MINVAL: f64 = 1E-15_f64;
        const MJ_DSBL_GRAVITY: i32 = 1 << 7;
        const MJ_TRN_TENDON: i32 = 3;

        let has_stiffness: i32 = (*(*m).tendon_stiffness.add(i as usize) != 0.0
            || crate::engine::engine_util_misc::mju_is_zero(
                (*m).tendon_stiffnesspoly.add((MJ_NPOLY * i) as usize),
                MJ_NPOLY,
            ) == 0) as i32;

        let limitedspring: i32 = (has_stiffness != 0
            && *(*m).tendon_lengthspring.add(2 * i as usize) == 0.0
            && *(*m).tendon_lengthspring.add(2 * i as usize + 1) > 0.0) as i32;

        let ten_length: f64 = *(*d).ten_length.add(i as usize);
        let lower: f64 = *(*m).tendon_range.add(2 * i as usize);
        let upper: f64 = *(*m).tendon_range.add(2 * i as usize + 1);

        let limitedconstraint: i32 = (has_stiffness == 0
            && *(*m).tendon_limited.add(i as usize)
            && lower == 0.0
            && ten_length < upper) as i32;

        let has_damping: i32 = (*(*m).tendon_damping.add(i as usize) != 0.0
            || crate::engine::engine_util_misc::mju_is_zero(
                (*m).tendon_dampingpoly.add((MJ_NPOLY * i) as usize),
                MJ_NPOLY,
            ) == 0) as i32;

        // mjDISABLED(mjDSBL_GRAVITY) = (m->opt.disableflags & mjDSBL_GRAVITY)
        let mut draw_catenary: i32 = (((*m).opt.disableflags & MJ_DSBL_GRAVITY) == 0
            && crate::engine::engine_util_blas::mju_norm3((*m).opt.gravity.as_ptr()) > MJ_MINVAL
            && *(*m).tendon_num.add(i as usize) == 2
            && (limitedspring != limitedconstraint)
            && has_damping == 0
            && *(*m).tendon_frictionloss.add(i as usize) == 0.0) as i32;

        // no actuator
        if draw_catenary != 0 {
            for j in 0..(*m).nu as i32 {
                if *(*m).actuator_trntype.add(j as usize) == MJ_TRN_TENDON
                    && *(*m).actuator_trnid.add(2 * j as usize) == i
                {
                    draw_catenary = 0;
                    break;
                }
            }
        }

        if draw_catenary != 0 {
            if limitedconstraint != 0 {
                *length = *(*m).tendon_range.add(2 * i as usize + 1);
            } else {
                *length = *(*m).tendon_lengthspring.add(2 * i as usize + 1);
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

