//! Port of: engine/engine_vis_init.h
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mjv_defaultScene (engine/engine_vis_init.h:34)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_default_scene(scn: *mut mjvScene) {
    // SAFETY: scn is a valid pointer to mjvScene. Zeroing all bytes is correct for this C struct.
    unsafe {
        core::ptr::write_bytes(scn, 0, 1);
    }
}

/// C: mjv_makeScene (engine/engine_vis_init.h:37)
/// Calls: mju_copyInt, mju_error, mju_malloc, mju_message, mjv_freeScene
#[allow(unused_variables, non_snake_case)]
pub fn mjv_make_scene(m: *const mjModel, scn: *mut mjvScene, maxgeom: i32) {

    extern "C" { fn mjv_makeScene_impl(m: *const mjModel, scn: *mut mjvScene, maxgeom: i32); }
    // SAFETY: delegates to C implementation
    unsafe { mjv_makeScene_impl(m, scn, maxgeom) }
}

/// C: mjv_freeScene (engine/engine_vis_init.h:40)
/// Calls: mju_free, mjv_defaultScene
#[allow(unused_variables, non_snake_case)]
pub fn mjv_free_scene(scn: *mut mjvScene) {
    // SAFETY: scn is valid pointer; mju_free handles NULL gracefully.
    // Frees all buffers allocated by mjv_makeScene, then zeroes the struct.
    unsafe {
        crate::engine::engine_util_errmem::mju_free((*scn).geoms as *mut ());
        crate::engine::engine_util_errmem::mju_free((*scn).geomorder as *mut ());

        crate::engine::engine_util_errmem::mju_free((*scn).flexedgeadr as *mut ());
        crate::engine::engine_util_errmem::mju_free((*scn).flexedgenum as *mut ());
        crate::engine::engine_util_errmem::mju_free((*scn).flexvertadr as *mut ());
        crate::engine::engine_util_errmem::mju_free((*scn).flexvertnum as *mut ());
        crate::engine::engine_util_errmem::mju_free((*scn).flexfaceadr as *mut ());
        crate::engine::engine_util_errmem::mju_free((*scn).flexfacenum as *mut ());
        crate::engine::engine_util_errmem::mju_free((*scn).flexfaceused as *mut ());
        crate::engine::engine_util_errmem::mju_free((*scn).flexedge as *mut ());
        crate::engine::engine_util_errmem::mju_free((*scn).flexvert as *mut ());
        crate::engine::engine_util_errmem::mju_free((*scn).flexface as *mut ());
        crate::engine::engine_util_errmem::mju_free((*scn).flexnormal as *mut ());
        crate::engine::engine_util_errmem::mju_free((*scn).flextexcoord as *mut ());

        crate::engine::engine_util_errmem::mju_free((*scn).skinfacenum as *mut ());
        crate::engine::engine_util_errmem::mju_free((*scn).skinvertadr as *mut ());
        crate::engine::engine_util_errmem::mju_free((*scn).skinvertnum as *mut ());
        crate::engine::engine_util_errmem::mju_free((*scn).skinvert as *mut ());
        crate::engine::engine_util_errmem::mju_free((*scn).skinnormal as *mut ());

        // clear data structure
        mjv_default_scene(scn);
    }
}

/// C: mjv_defaultOption (engine/engine_vis_init.h:43)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_default_option(vopt: *mut mjvOption) {
    // SAFETY: vopt is a valid pointer per caller contract.
    // mjVISSTRING initial values hardcoded from C source (31 flags):
    //   0,1,0,0,0,0,0,1,1,0,0,0,0,1,0,0,0,0,0,0,0,0,1,1,0,1,0,1,0,0,0
    unsafe {
        const MJNGROUP: i32 = 6;
        const MJNVISFLAG: i32 = 31;

        // mjVISSTRING[i][1][0] == '1' → flag values
        static VIS_INIT: [u8; 31] = [
            0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0,
            0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 0,
        ];

        (*vopt).label = 0; // mjLABEL_NONE
        (*vopt).frame = 0; // mjFRAME_NONE

        let mut i: i32 = 0;
        while i < MJNGROUP {
            let state: u8 = if i < 3 { 1 } else { 0 };
            (*vopt).geomgroup[i as usize] = state;
            (*vopt).sitegroup[i as usize] = state;
            (*vopt).jointgroup[i as usize] = state;
            (*vopt).tendongroup[i as usize] = state;
            (*vopt).actuatorgroup[i as usize] = state;
            (*vopt).flexgroup[i as usize] = state;
            (*vopt).skingroup[i as usize] = state;
            i += 1;
        }

        let mut i: i32 = 0;
        while i < MJNVISFLAG {
            (*vopt).flags[i as usize] = VIS_INIT[i as usize];
            i += 1;
        }

        (*vopt).bvh_depth = 1;
        (*vopt).flex_layer = 0;
    }
}

/// C: mjv_defaultFreeCamera (engine/engine_vis_init.h:46)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_default_free_camera(m: *const mjModel, cam: *mut mjvCamera) {
    // SAFETY: m and cam are valid pointers per caller contract.
    // vis.global is opaque in Rust types, so we use raw offsets matching C layout:
    //   global.orthographic = offset 4 (i32)
    //   global.azimuth      = offset 16 (f32)
    //   global.elevation    = offset 20 (f32)
    unsafe {
        core::ptr::write_bytes(cam, 0, 1);

        let vis_ptr = core::ptr::addr_of!((*m).vis) as *const u8;
        let global_orthographic = *(vis_ptr.add(4) as *const i32);
        let global_azimuth = *(vis_ptr.add(16) as *const f32);
        let global_elevation = *(vis_ptr.add(20) as *const f32);

        (*cam).r#type = 0; // mjCAMERA_FREE
        (*cam).fixedcamid = -1;
        (*cam).trackbodyid = -1;
        (*cam).lookat[0] = (*m).stat.center[0];
        (*cam).lookat[1] = (*m).stat.center[1];
        (*cam).lookat[2] = (*m).stat.center[2];
        (*cam).distance = 1.5 * (*m).stat.extent;
        (*cam).azimuth = global_azimuth as f64;
        (*cam).elevation = global_elevation as f64;
        (*cam).orthographic = global_orthographic;
    }
}

/// C: mjv_defaultCamera (engine/engine_vis_init.h:49)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_default_camera(cam: *mut mjvCamera) {
    // SAFETY: cam is a valid pointer. Zero first, then set specific fields per C source.
    unsafe {
        core::ptr::write_bytes(cam, 0, 1);
        (*cam).r#type = 0; // mjCAMERA_FREE = 0
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
    // SAFETY: pert is a valid pointer. Zero first, then set specific fields per C source.
    unsafe {
        core::ptr::write_bytes(pert, 0, 1);
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
    // SAFETY: fig is a valid pointer per caller contract.
    // linergb in C is float[100][3], in Rust [[f32;100];3] — layout differs.
    // We use raw pointer arithmetic matching C's row-major [line][channel] layout.
    unsafe {
        const MJMAXLINE: i32 = 100;

        // predefined line colors (matching C static const _linergb)
        static LINERGB: [[f32; 3]; 8] = [
            [1.0, 0.3, 0.3],
            [0.1, 1.0, 0.1],
            [0.3, 0.3, 1.0],
            [0.1, 1.0, 1.0],
            [1.0, 0.2, 1.0],
            [1.0, 1.0, 0.1],
            [1.0, 0.6, 0.2],
            [0.6, 0.7, 0.4],
        ];

        // set everything to zero
        core::ptr::write_bytes(fig, 0, 1);

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

        // mjSTRNCPY(fig->xformat, "%.0f") → strncpy with n=20
        crate::engine::engine_util_misc::mju_strncpy(
            (*fig).xformat.as_mut_ptr(), b"%.0f\0".as_ptr() as *const i8, 20);
        crate::engine::engine_util_misc::mju_strncpy(
            (*fig).yformat.as_mut_ptr(), b"%.2g\0".as_ptr() as *const i8, 20);
        crate::engine::engine_util_misc::mju_strncpy(
            (*fig).minwidth.as_mut_ptr(), b"XXX\0".as_ptr() as *const i8, 20);

        // set line colors using raw pointer arithmetic (C layout: float[100][3])
        let linergb_ptr = (*fig).linergb.as_mut_ptr() as *mut f32;
        let mut n: i32 = 0;
        while n < MJMAXLINE {
            if n < 8 {
                // predefined colors
                *linergb_ptr.add((n * 3 + 0) as usize) = LINERGB[n as usize][0];
                *linergb_ptr.add((n * 3 + 1) as usize) = LINERGB[n as usize][1];
                *linergb_ptr.add((n * 3 + 2) as usize) = LINERGB[n as usize][2];
            } else {
                // automatically generated colors: Halton sequence
                *linergb_ptr.add((n * 3 + 0) as usize) =
                    0.1 + 0.8 * crate::engine::engine_util_misc::mju_halton(n, 2) as f32;
                *linergb_ptr.add((n * 3 + 1) as usize) =
                    0.1 + 0.8 * crate::engine::engine_util_misc::mju_halton(n, 3) as f32;
                *linergb_ptr.add((n * 3 + 2) as usize) =
                    0.1 + 0.8 * crate::engine::engine_util_misc::mju_halton(n, 5) as f32;
            }
            n += 1;
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
    // SAFETY: geom is a valid pointer per caller contract; read-only access to struct fields.
    unsafe {
        const MJOBJ_GEOM: i32 = 5;
        const MJGEOM_SPHERE: i32 = 2;
        const MJGEOM_CAPSULE: i32 = 3;
        const MJGEOM_CYLINDER: i32 = 5;
        const MJGEOM_BOX: i32 = 6;

        // model geom: return
        if (*geom).objtype == MJOBJ_GEOM {
            return (*geom).modelrbound;
        }

        // compute rbound according to type
        let s: *const f32 = (*geom).size.as_ptr();
        match (*geom).r#type {
            MJGEOM_SPHERE => {
                *s.add(0)
            }
            MJGEOM_CAPSULE => {
                *s.add(0) + *s.add(2)
            }
            MJGEOM_CYLINDER => {
                (*s.add(0) * *s.add(0) + *s.add(2) * *s.add(2)).sqrt()
            }
            MJGEOM_BOX => {
                (*s.add(0) * *s.add(0) + *s.add(1) * *s.add(1) + *s.add(2) * *s.add(2)).sqrt()
            }
            _ => {
                // not accurate for arrows, but they are not transparent
                let m01 = if *s.add(0) > *s.add(1) { *s.add(0) } else { *s.add(1) };
                if m01 > *s.add(2) { m01 } else { *s.add(2) }
            }
        }
    }
}

