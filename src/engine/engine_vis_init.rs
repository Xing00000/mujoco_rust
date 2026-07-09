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
    // SAFETY: scn is valid pointer. m may be null (checked before use).
    // All mju_malloc calls return valid aligned memory or null (checked).
    // memcpy via copy_nonoverlapping is safe since src/dst don't overlap and sizes match.
    unsafe {
        // render flag defaults: mjRNDSTRING[i][1][0] == '1'
        const MJNRNDFLAG: usize = 11;
        static RND_DEFAULTS: [u8; MJNRNDFLAG] = [1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1];

        // free previous
        crate::engine::engine_vis_init::mjv_free_scene(scn);

        // allocate geom buffers
        if maxgeom > 0 {
            (*scn).maxgeom = maxgeom;
            (*scn).geoms = crate::engine::engine_util_errmem::mju_malloc(
                maxgeom as usize * core::mem::size_of::<mjvGeom>(),
            ) as *mut mjvGeom;
            (*scn).geomorder = crate::engine::engine_util_errmem::mju_malloc(
                maxgeom as usize * core::mem::size_of::<i32>(),
            ) as *mut i32;

            // check allocation
            if (*scn).geoms.is_null() || (*scn).geomorder.is_null() {
                crate::engine::engine_util_errmem::mju_error(
                    b"could not allocate geom buffers\0".as_ptr() as *const i8,
                );
            }
        }

        // set default OpenGL options
        let mut i: usize = 0;
        while i < MJNRNDFLAG {
            (*scn).flags[i] = RND_DEFAULTS[i];
            i += 1;
        }

        // set default model transformation
        (*scn).scale = 1.0;
        (*scn).rotate[0] = 1.0;

        // set number of flexes
        (*scn).nflex = if !m.is_null() { (*m).nflex as i32 } else { 0 };

        // allocate flex data
        if (*scn).nflex != 0 {
            let nflex = (*scn).nflex;

            // allocate fixed
            (*scn).flexedgeadr = crate::engine::engine_util_errmem::mju_malloc(
                nflex as usize * core::mem::size_of::<i32>(),
            ) as *mut i32;
            (*scn).flexedgenum = crate::engine::engine_util_errmem::mju_malloc(
                nflex as usize * core::mem::size_of::<i32>(),
            ) as *mut i32;
            (*scn).flexvertadr = crate::engine::engine_util_errmem::mju_malloc(
                nflex as usize * core::mem::size_of::<i32>(),
            ) as *mut i32;
            (*scn).flexvertnum = crate::engine::engine_util_errmem::mju_malloc(
                nflex as usize * core::mem::size_of::<i32>(),
            ) as *mut i32;
            (*scn).flexfaceadr = crate::engine::engine_util_errmem::mju_malloc(
                nflex as usize * core::mem::size_of::<i32>(),
            ) as *mut i32;
            (*scn).flexfacenum = crate::engine::engine_util_errmem::mju_malloc(
                nflex as usize * core::mem::size_of::<i32>(),
            ) as *mut i32;
            (*scn).flexfaceused = crate::engine::engine_util_errmem::mju_malloc(
                nflex as usize * core::mem::size_of::<i32>(),
            ) as *mut i32;
            (*scn).flexedge = crate::engine::engine_util_errmem::mju_malloc(
                2 * (*m).nflexedge * core::mem::size_of::<i32>(),
            ) as *mut i32;
            (*scn).flexvert = crate::engine::engine_util_errmem::mju_malloc(
                3 * (*m).nflexvert * core::mem::size_of::<f32>(),
            ) as *mut f32;

            // count max number of flex faces to be rendered
            let mut nface: i32 = 0;
            let mut f: i32 = 0;
            while f < nflex {
                let dim = *(*m).flex_dim.add(f as usize);

                // 1D: 0
                if dim == 0 {
                    *(*scn).flexfacenum.add(f as usize) = 0;
                } else if dim == 2 {
                    // 2D: 2*fragments + 2*elements
                    *(*scn).flexfacenum.add(f as usize) =
                        2 * *(*m).flex_shellnum.add(f as usize)
                        + 2 * *(*m).flex_elemnum.add(f as usize);
                } else {
                    // 3D: max(fragments, 4*maxlayer)
                    let mut maxlayer: i32 = 0;
                    let mut layer: i32 = 0;
                    let mut nlayer: i32 = 1;
                    while nlayer != 0 {
                        // count elements in this layer
                        nlayer = 0;
                        let mut e: i32 = 0;
                        while e < *(*m).flex_elemnum.add(f as usize) {
                            if *(*m).flex_elemlayer.add((*(*m).flex_elemadr.add(f as usize) + e) as usize) == layer {
                                nlayer += 1;
                            }
                            e += 1;
                        }
                        // accumulate max over layers, advance layer
                        if nlayer > maxlayer {
                            maxlayer = nlayer;
                        }
                        layer += 1;
                    }

                    let shells = *(*m).flex_shellnum.add(f as usize);
                    let four_max = 4 * maxlayer;
                    *(*scn).flexfacenum.add(f as usize) = if shells > four_max { shells } else { four_max };
                }

                // accumulate over flexes
                nface += *(*scn).flexfacenum.add(f as usize);
                f += 1;
            }

            // allocate face-related
            (*scn).flexface = if nface != 0 {
                crate::engine::engine_util_errmem::mju_malloc(
                    9 * nface as usize * core::mem::size_of::<f32>(),
                ) as *mut f32
            } else {
                core::ptr::null_mut()
            };
            (*scn).flexnormal = if nface != 0 {
                crate::engine::engine_util_errmem::mju_malloc(
                    9 * nface as usize * core::mem::size_of::<f32>(),
                ) as *mut f32
            } else {
                core::ptr::null_mut()
            };
            (*scn).flextexcoord = if nface != 0 {
                crate::engine::engine_util_errmem::mju_malloc(
                    6 * nface as usize * core::mem::size_of::<f32>(),
                ) as *mut f32
            } else {
                core::ptr::null_mut()
            };

            // check allocation
            if (*scn).flexedgeadr.is_null()
                || (*scn).flexedgenum.is_null()
                || (*scn).flexfaceadr.is_null()
                || (*scn).flexfacenum.is_null()
                || (*scn).flexfaceused.is_null()
                || (*scn).flexvertadr.is_null()
                || (*scn).flexvertnum.is_null()
                || (*scn).flexedge.is_null()
                || (*scn).flexvert.is_null()
                || (nface != 0 && (*scn).flexface.is_null())
                || (nface != 0 && (*scn).flexnormal.is_null())
                || (nface != 0 && (*scn).flextexcoord.is_null())
            {
                crate::engine::engine_util_errmem::mju_error(
                    b"Could not allocate flex buffers\0".as_ptr() as *const i8,
                );
            }

            // copy constant edge and vertex data
            core::ptr::copy_nonoverlapping(
                (*m).flex_edgeadr, (*scn).flexedgeadr, nflex as usize,
            );
            core::ptr::copy_nonoverlapping(
                (*m).flex_edgenum, (*scn).flexedgenum, nflex as usize,
            );
            core::ptr::copy_nonoverlapping(
                (*m).flex_vertadr, (*scn).flexvertadr, nflex as usize,
            );
            core::ptr::copy_nonoverlapping(
                (*m).flex_vertnum, (*scn).flexvertnum, nflex as usize,
            );
            core::ptr::copy_nonoverlapping(
                (*m).flex_edge, (*scn).flexedge, 2 * (*m).nflexedge,
            );

            // compute flexfaceadr
            let mut f: i32 = 0;
            while f < nflex {
                *(*scn).flexfaceadr.add(f as usize) = if f == 0 {
                    0
                } else {
                    *(*scn).flexfaceadr.add((f - 1) as usize) + *(*scn).flexfacenum.add((f - 1) as usize)
                };
                f += 1;
            }
        }

        // set number of skins
        if !m.is_null() {
            (*scn).nskin = (*m).nskin as i32;
        } else {
            (*scn).nskin = 0;
        }

        // allocate skin data
        if (*scn).nskin != 0 {
            let nskin = (*m).nskin as i32;

            // allocate
            (*scn).skinfacenum = crate::engine::engine_util_errmem::mju_malloc(
                nskin as usize * core::mem::size_of::<i32>(),
            ) as *mut i32;
            (*scn).skinvertadr = crate::engine::engine_util_errmem::mju_malloc(
                nskin as usize * core::mem::size_of::<i32>(),
            ) as *mut i32;
            (*scn).skinvertnum = crate::engine::engine_util_errmem::mju_malloc(
                nskin as usize * core::mem::size_of::<i32>(),
            ) as *mut i32;
            (*scn).skinvert = crate::engine::engine_util_errmem::mju_malloc(
                3 * (*m).nskinvert * core::mem::size_of::<f32>(),
            ) as *mut f32;
            (*scn).skinnormal = crate::engine::engine_util_errmem::mju_malloc(
                3 * (*m).nskinvert * core::mem::size_of::<f32>(),
            ) as *mut f32;

            // check allocation
            if (*scn).skinfacenum.is_null()
                || (*scn).skinvertadr.is_null()
                || (*scn).skinvertnum.is_null()
                || (*scn).skinvert.is_null()
                || (*scn).skinnormal.is_null()
            {
                crate::engine::engine_util_errmem::mju_error(
                    b"could not allocate skin buffers\0".as_ptr() as *const i8,
                );
            }

            // copy constant data
            crate::engine::engine_util_misc::mju_copy_int(
                (*scn).skinfacenum, (*m).skin_facenum, nskin,
            );
            crate::engine::engine_util_misc::mju_copy_int(
                (*scn).skinvertadr, (*m).skin_vertadr, nskin,
            );
            crate::engine::engine_util_misc::mju_copy_int(
                (*scn).skinvertnum, (*m).skin_vertnum, nskin,
            );
        }

        // mjvGeom, mjvLight, mjvGLCamera objects are invalid
    }
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

