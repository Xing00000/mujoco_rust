//! Port of: engine/engine_vis_init.h
//! IR hash: d2209344472ae336
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
    todo!() // mjv_makeScene
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

