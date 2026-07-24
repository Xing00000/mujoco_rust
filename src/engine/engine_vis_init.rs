//! Port of: engine/engine_vis_init.h
//! IR hash: 3fb6da908ad9d71c
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mjv_defaultScene (engine/engine_vis_init.h:34)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_default_scene(scn: *mut mjvScene) {
    // SAFETY: caller guarantees scn is a valid, aligned, writable pointer to mjvScene
    unsafe {
        std::ptr::write_bytes(scn as *mut u8, 0, std::mem::size_of::<mjvScene>());
    }
}

/// C: mjv_makeScene (engine/engine_vis_init.h:37)
/// Calls: mju_copyInt, mju_error, mju_malloc, mju_message, mjv_freeScene
#[allow(unused_variables, non_snake_case)]
pub fn mjv_make_scene(m: *const mjModel, scn: *mut mjvScene, maxgeom: i32) {
    const MJ_NRNDFLAG: usize = 11;
    // Default render flags: Shadow=1, Wireframe=0, Reflection=1, Additive=0,
    // Skybox=1, Fog=0, Haze=1, Depth=0, Segment=0, IdColor=0, CullFace=1
    const RND_DEFAULTS: [u8; 11] = [1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1];

    // SAFETY: m (if non-null) and scn are valid pointers (caller contract)
    unsafe {
        // free previous
        mjv_free_scene(scn);

        // allocate geom buffers
        if maxgeom > 0 {
            (*scn).maxgeom = maxgeom;
            (*scn).geoms = crate::engine::engine_util_errmem::mju_malloc(
                maxgeom as usize * std::mem::size_of::<mjvGeom>()) as *mut mjvGeom;
            (*scn).geomorder = crate::engine::engine_util_errmem::mju_malloc(
                maxgeom as usize * std::mem::size_of::<i32>()) as *mut i32;

            if (*scn).geoms.is_null() || (*scn).geomorder.is_null() {
                crate::engine::engine_util_errmem::mju_error(
                    b"could not allocate geom buffers\0".as_ptr() as *const i8);
                return;
            }
        }

        // set default OpenGL options
        for i in 0..MJ_NRNDFLAG {
            (*scn).flags[i] = RND_DEFAULTS[i];
        }

        // set default model transformation
        (*scn).scale = 1.0;
        (*scn).rotate[0] = 1.0;

        // set number of flexes
        (*scn).nflex = if !m.is_null() { (*m).nflex as i32 } else { 0 };

        // allocate flex data
        if (*scn).nflex > 0 {
            let nflex = (*scn).nflex;
            let sz_int = std::mem::size_of::<i32>();
            let sz_float = std::mem::size_of::<f32>();

            // allocate fixed
            (*scn).flexedgeadr = crate::engine::engine_util_errmem::mju_malloc(nflex as usize * sz_int) as *mut i32;
            (*scn).flexedgenum = crate::engine::engine_util_errmem::mju_malloc(nflex as usize * sz_int) as *mut i32;
            (*scn).flexvertadr = crate::engine::engine_util_errmem::mju_malloc(nflex as usize * sz_int) as *mut i32;
            (*scn).flexvertnum = crate::engine::engine_util_errmem::mju_malloc(nflex as usize * sz_int) as *mut i32;
            (*scn).flexfaceadr = crate::engine::engine_util_errmem::mju_malloc(nflex as usize * sz_int) as *mut i32;
            (*scn).flexfacenum = crate::engine::engine_util_errmem::mju_malloc(nflex as usize * sz_int) as *mut i32;
            (*scn).flexfaceused = crate::engine::engine_util_errmem::mju_malloc(nflex as usize * sz_int) as *mut i32;
            (*scn).flexedge = crate::engine::engine_util_errmem::mju_malloc(2 * (*m).nflexedge as usize * sz_int) as *mut i32;
            (*scn).flexvert = crate::engine::engine_util_errmem::mju_malloc(3 * (*m).nflexvert as usize * sz_float) as *mut f32;

            // count max number of flex faces
            let mut nface: i32 = 0;
            for f in 0..nflex as usize {
                let dim = *(*m).flex_dim.add(f);

                if dim == 0 || dim == 1 {
                    *(*scn).flexfacenum.add(f) = 0;
                } else if dim == 2 {
                    *(*scn).flexfacenum.add(f) = 2 * *(*m).flex_shellnum.add(f) + 2 * *(*m).flex_elemnum.add(f);
                } else {
                    // 3D: max(fragments, 4*maxlayer)
                    let mut maxlayer: i32 = 0;
                    let mut layer: i32 = 0;
                    let mut nlayer: i32 = 1;
                    while nlayer > 0 {
                        nlayer = 0;
                        for e in 0..*(*m).flex_elemnum.add(f) as usize {
                            if *(*m).flex_elemlayer.add(*(*m).flex_elemadr.add(f) as usize + e) == layer {
                                nlayer += 1;
                            }
                        }
                        if nlayer > maxlayer { maxlayer = nlayer; }
                        layer += 1;
                    }
                    let shell = *(*m).flex_shellnum.add(f);
                    let four_max = 4 * maxlayer;
                    *(*scn).flexfacenum.add(f) = if shell > four_max { shell } else { four_max };
                }

                nface += *(*scn).flexfacenum.add(f);
            }

            // allocate face-related
            if nface > 0 {
                (*scn).flexface = crate::engine::engine_util_errmem::mju_malloc(9 * nface as usize * sz_float) as *mut f32;
                (*scn).flexnormal = crate::engine::engine_util_errmem::mju_malloc(9 * nface as usize * sz_float) as *mut f32;
                (*scn).flextexcoord = crate::engine::engine_util_errmem::mju_malloc(6 * nface as usize * sz_float) as *mut f32;
            } else {
                (*scn).flexface = std::ptr::null_mut();
                (*scn).flexnormal = std::ptr::null_mut();
                (*scn).flextexcoord = std::ptr::null_mut();
            }

            // check allocation
            if (*scn).flexedgeadr.is_null() || (*scn).flexedgenum.is_null()
                || (*scn).flexfaceadr.is_null() || (*scn).flexfacenum.is_null()
                || (*scn).flexfaceused.is_null() || (*scn).flexvertadr.is_null()
                || (*scn).flexvertnum.is_null() || (*scn).flexedge.is_null()
                || (*scn).flexvert.is_null()
                || (nface > 0 && (*scn).flexface.is_null())
                || (nface > 0 && (*scn).flexnormal.is_null())
                || (nface > 0 && (*scn).flextexcoord.is_null())
            {
                crate::engine::engine_util_errmem::mju_error(
                    b"Could not allocate flex buffers\0".as_ptr() as *const i8);
                return;
            }

            // copy constant edge and vertex data
            std::ptr::copy_nonoverlapping(
                (*m).flex_edgeadr as *const u8,
                (*scn).flexedgeadr as *mut u8,
                nflex as usize * sz_int);
            std::ptr::copy_nonoverlapping(
                (*m).flex_edgenum as *const u8,
                (*scn).flexedgenum as *mut u8,
                nflex as usize * sz_int);
            std::ptr::copy_nonoverlapping(
                (*m).flex_vertadr as *const u8,
                (*scn).flexvertadr as *mut u8,
                nflex as usize * sz_int);
            std::ptr::copy_nonoverlapping(
                (*m).flex_vertnum as *const u8,
                (*scn).flexvertnum as *mut u8,
                nflex as usize * sz_int);
            std::ptr::copy_nonoverlapping(
                (*m).flex_edge as *const u8,
                (*scn).flexedge as *mut u8,
                2 * (*m).nflexedge as usize * sz_int);

            // compute flexfaceadr
            for f in 0..nflex as usize {
                *(*scn).flexfaceadr.add(f) = if f == 0 { 0 } else {
                    *(*scn).flexfaceadr.add(f - 1) + *(*scn).flexfacenum.add(f - 1)
                };
            }
        }

        // set number of skins
        (*scn).nskin = if !m.is_null() { (*m).nskin as i32 } else { 0 };

        // allocate skin data
        if (*scn).nskin > 0 {
            let nskin = (*m).nskin as i32;
            let sz_int = std::mem::size_of::<i32>();
            let sz_float = std::mem::size_of::<f32>();

            (*scn).skinfacenum = crate::engine::engine_util_errmem::mju_malloc(nskin as usize * sz_int) as *mut i32;
            (*scn).skinvertadr = crate::engine::engine_util_errmem::mju_malloc(nskin as usize * sz_int) as *mut i32;
            (*scn).skinvertnum = crate::engine::engine_util_errmem::mju_malloc(nskin as usize * sz_int) as *mut i32;
            (*scn).skinvert = crate::engine::engine_util_errmem::mju_malloc(3 * (*m).nskinvert as usize * sz_float) as *mut f32;
            (*scn).skinnormal = crate::engine::engine_util_errmem::mju_malloc(3 * (*m).nskinvert as usize * sz_float) as *mut f32;

            if (*scn).skinfacenum.is_null() || (*scn).skinvertadr.is_null()
                || (*scn).skinvertnum.is_null() || (*scn).skinvert.is_null()
                || (*scn).skinnormal.is_null()
            {
                crate::engine::engine_util_errmem::mju_error(
                    b"could not allocate skin buffers\0".as_ptr() as *const i8);
                return;
            }

            // copy constant data
            crate::engine::engine_util_misc::mju_copy_int((*scn).skinfacenum, (*m).skin_facenum, nskin);
            crate::engine::engine_util_misc::mju_copy_int((*scn).skinvertadr, (*m).skin_vertadr, nskin);
            crate::engine::engine_util_misc::mju_copy_int((*scn).skinvertnum, (*m).skin_vertnum, nskin);
        }
    }
}

/// C: mjv_freeScene (engine/engine_vis_init.h:40)
/// Calls: mju_free, mjv_defaultScene
#[allow(unused_variables, non_snake_case)]
pub fn mjv_free_scene(scn: *mut mjvScene) {
    // SAFETY: scn is a valid pointer to mjvScene allocated by mjv_makeScene (caller contract)
    unsafe {
        use crate::engine::engine_util_errmem::mju_free;

        mju_free((*scn).geoms as *mut ());
        mju_free((*scn).geomorder as *mut ());

        mju_free((*scn).flexedgeadr as *mut ());
        mju_free((*scn).flexedgenum as *mut ());
        mju_free((*scn).flexvertadr as *mut ());
        mju_free((*scn).flexvertnum as *mut ());
        mju_free((*scn).flexfaceadr as *mut ());
        mju_free((*scn).flexfacenum as *mut ());
        mju_free((*scn).flexfaceused as *mut ());
        mju_free((*scn).flexedge as *mut ());
        mju_free((*scn).flexvert as *mut ());
        mju_free((*scn).flexface as *mut ());
        mju_free((*scn).flexnormal as *mut ());
        mju_free((*scn).flextexcoord as *mut ());

        mju_free((*scn).skinfacenum as *mut ());
        mju_free((*scn).skinvertadr as *mut ());
        mju_free((*scn).skinvertnum as *mut ());
        mju_free((*scn).skinvert as *mut ());
        mju_free((*scn).skinnormal as *mut ());

        // clear data structure
        mjv_default_scene(scn);
    }
}

/// C: mjv_defaultOption (engine/engine_vis_init.h:43)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_default_option(vopt: *mut mjvOption) {
    // Default flag values from mjVISSTRING[i][1][0]:
    // "0100000011000010000000010110100" (indices 0..31)
    const VIS_DEFAULTS: [u8; 31] = [
        0, 1, 0, 0, 0, 0, 0, 1, 1, 0,
        0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
        0, 0, 1, 1, 0, 1, 0, 1, 0, 0,
        0,
    ];
    const NGROUP: usize = 6;
    const NVISFLAG: usize = 31;

    // SAFETY: caller guarantees vopt is a valid, aligned, writable pointer to mjvOption
    unsafe {
        (*vopt).label = 0; // mjLABEL_NONE
        (*vopt).frame = 0; // mjFRAME_NONE

        let mut i: usize = 0;
        while i < NGROUP {
            let state: u8 = if i < 3 { 1 } else { 0 };
            (*vopt).geomgroup[i] = state;
            (*vopt).sitegroup[i] = state;
            (*vopt).jointgroup[i] = state;
            (*vopt).tendongroup[i] = state;
            (*vopt).actuatorgroup[i] = state;
            (*vopt).flexgroup[i] = state;
            (*vopt).skingroup[i] = state;
            i += 1;
        }

        let mut i: usize = 0;
        while i < NVISFLAG {
            (*vopt).flags[i] = VIS_DEFAULTS[i];
            i += 1;
        }

        (*vopt).bvh_depth = 1;
        (*vopt).flex_layer = 0;
    }
}

/// C: mjv_defaultFreeCamera (engine/engine_vis_init.h:46)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_default_free_camera(m: *const mjModel, cam: *mut mjvCamera) {
    // SAFETY: caller guarantees m is a valid mjModel pointer, cam is a valid writable mjvCamera pointer
    unsafe {
        std::ptr::write_bytes(cam as *mut u8, 0, std::mem::size_of::<mjvCamera>());

        (*cam).r#type = 0; // mjCAMERA_FREE
        (*cam).fixedcamid = -1;
        (*cam).trackbodyid = -1;
        (*cam).lookat[0] = (*m).stat.center[0];
        (*cam).lookat[1] = (*m).stat.center[1];
        (*cam).lookat[2] = (*m).stat.center[2];
        (*cam).distance = 1.5 * (*m).stat.extent;

        // vis.global is [u8; 52]; azimuth is at offset 16 (float), elevation at 20, orthographic at 4 (int)
        let global_ptr = (*m).vis.global.as_ptr();
        let azimuth = *(global_ptr.add(16) as *const f32);
        let elevation = *(global_ptr.add(20) as *const f32);
        let orthographic = *(global_ptr.add(4) as *const i32);

        (*cam).azimuth = azimuth as f64;
        (*cam).elevation = elevation as f64;
        (*cam).orthographic = orthographic;
    }
}

/// C: mjv_defaultCamera (engine/engine_vis_init.h:49)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_default_camera(cam: *mut mjvCamera) {
    // SAFETY: caller guarantees cam is a valid, aligned, writable pointer to mjvCamera
    unsafe {
        std::ptr::write_bytes(cam as *mut u8, 0, std::mem::size_of::<mjvCamera>());
        (*cam).r#type = 0; // mjCAMERA_FREE
        (*cam).fixedcamid = -1;
        (*cam).trackbodyid = -1;
        (*cam).distance = 2.0;
        (*cam).azimuth = 90.0;
        (*cam).elevation = -45.0;
    }
}

/// C: mjv_defaultPerturb (engine/engine_vis_init.h:52)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_default_perturb(pert: *mut mjvPerturb) {
    // SAFETY: caller guarantees pert is a valid, aligned, writable pointer to mjvPerturb
    unsafe {
        std::ptr::write_bytes(pert as *mut u8, 0, std::mem::size_of::<mjvPerturb>());

        (*pert).flexselect = -1;
        (*pert).skinselect = -1;
        (*pert).refquat[0] = 1.0;
        (*pert).scale = 1.0;
    }
}

/// C: mjv_defaultFigure (engine/engine_vis_init.h:55)
/// Calls: mju_Halton, mju_strncpy
#[allow(unused_variables, non_snake_case)]
pub fn mjv_default_figure(fig: *mut mjvFigure) {
    // SAFETY: fig is a valid, aligned, writable pointer to mjvFigure
    unsafe {
        // set everything to zero
        std::ptr::write_bytes(fig as *mut u8, 0, std::mem::size_of::<mjvFigure>());

        // disable highlight
        (*fig).highlightid = -1;

        // set enable flags
        (*fig).flg_legend = 1;
        (*fig).flg_ticklabel[0] = 1;
        (*fig).flg_ticklabel[1] = 1;
        (*fig).flg_extend = 1;

        // set style
        (*fig).linewidth = 3.0;
        (*fig).gridwidth = 1.0;
        (*fig).gridsize[0] = 2;
        (*fig).gridsize[1] = 2;
        (*fig).gridrgb[0] = 0.4;
        (*fig).gridrgb[1] = 0.4;
        (*fig).gridrgb[2] = 0.4;
        (*fig).figurergba[3] = 1.0;
        (*fig).panergba[3] = 1.0;
        (*fig).legendrgba[3] = 0.3;
        (*fig).textrgb[0] = 1.0;
        (*fig).textrgb[1] = 1.0;
        (*fig).textrgb[2] = 1.0;
        (*fig).range[0][0] = 0.0;
        (*fig).range[0][1] = 1.0;
        (*fig).range[1][0] = 0.0;
        (*fig).range[1][1] = 1.0;

        // xformat = "%.0f"
        let xfmt = b"%.0f\0";
        std::ptr::copy_nonoverlapping(xfmt.as_ptr(), (*fig).xformat.as_mut_ptr() as *mut u8, xfmt.len());

        // yformat = "%.2g"
        let yfmt = b"%.2g\0";
        std::ptr::copy_nonoverlapping(yfmt.as_ptr(), (*fig).yformat.as_mut_ptr() as *mut u8, yfmt.len());

        // minwidth = "XXX"
        let mw = b"XXX\0";
        std::ptr::copy_nonoverlapping(mw.as_ptr(), (*fig).minwidth.as_mut_ptr() as *mut u8, mw.len());

        // predefined line colors
        const LINERGB: [[f32; 3]; 8] = [
            [1.0, 0.3, 0.3],
            [0.1, 1.0, 0.1],
            [0.3, 0.3, 1.0],
            [0.1, 1.0, 1.0],
            [1.0, 0.2, 1.0],
            [1.0, 1.0, 0.1],
            [1.0, 0.6, 0.2],
            [0.6, 0.7, 0.4],
        ];

        // Access linergb as raw memory matching C layout: [100][3] = contiguous 300 floats
        // C: fig->linergb[n][c] = offset n*3 + c
        // Rust field is [[f32; 100]; 3] but underlying memory is the same 300 f32s
        let linergb_ptr = (*fig).linergb.as_mut_ptr() as *mut f32;

        for n in 0..100_usize {
            if n < 8 {
                // predefined colors
                *linergb_ptr.add(n * 3 + 0) = LINERGB[n][0];
                *linergb_ptr.add(n * 3 + 1) = LINERGB[n][1];
                *linergb_ptr.add(n * 3 + 2) = LINERGB[n][2];
            } else {
                // automatically generated colors: Halton sequence
                *linergb_ptr.add(n * 3 + 0) = 0.1 + 0.8 * crate::engine::engine_util_misc::mju_halton(n as i32, 2) as f32;
                *linergb_ptr.add(n * 3 + 1) = 0.1 + 0.8 * crate::engine::engine_util_misc::mju_halton(n as i32, 3) as f32;
                *linergb_ptr.add(n * 3 + 2) = 0.1 + 0.8 * crate::engine::engine_util_misc::mju_halton(n as i32, 5) as f32;
            }
        }
    }
}

/// C: mjv_rbound (engine/engine_vis_init.h:58)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_rbound(geom: *const mjvGeom) -> f32 {
    const mjOBJ_GEOM: i32 = 5;
    const mjGEOM_SPHERE: i32 = 2;
    const mjGEOM_CAPSULE: i32 = 3;
    const mjGEOM_CYLINDER: i32 = 5;
    const mjGEOM_BOX: i32 = 6;

    // SAFETY: caller guarantees geom is a valid pointer to mjvGeom
    unsafe {
        // model geom: return
        if (*geom).objtype == mjOBJ_GEOM {
            return (*geom).modelrbound;
        }

        // compute rbound according to type
        let s = &(*geom).size;
        match (*geom).r#type {
            mjGEOM_SPHERE => s[0],

            mjGEOM_CAPSULE => s[0] + s[2],

            mjGEOM_CYLINDER => {
                (s[0] * s[0] + s[2] * s[2]).sqrt()
            }

            mjGEOM_BOX => {
                (s[0] * s[0] + s[1] * s[1] + s[2] * s[2]).sqrt()
            }

            _ => {
                // not accurate for arrows, but they are not transparent
                let max_s1_s2 = if s[1] > s[2] { s[1] } else { s[2] };
                if s[0] > max_s1_s2 { s[0] } else { max_s1_s2 }
            }
        }
    }
}

