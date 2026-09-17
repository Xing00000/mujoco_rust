//! Port of: engine/engine_collision_convex.c
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mjc_setCCDBuffer (engine/engine_collision_convex.c:42)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_set_ccd_buffer(buffer: *mut ()) {
    todo!() // cxx:_mjc_setCCDBuffer
}

/// C: prism_firstdir (engine/engine_collision_convex.c:47)
#[allow(unused_variables, non_snake_case)]
pub fn prism_firstdir(o1: *const (), o2: *const (), vec: *mut ccd_vec3_t) {
    // SAFETY: caller guarantees vec points to valid ccd_vec3_t (holds 3 f64 = 24 bytes)
    unsafe {
        let v = vec as *mut f64;
        *v.add(0) = 0.0;
        *v.add(1) = 0.0;
        *v.add(2) = 1.0;
    }
}

/// C: _libccd_wrapper (engine/engine_collision_convex.c:52)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn libccd_wrapper(m: *const mjModel, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj, con: *mut mjPreContact, margin: f64) -> i32 {
    // C source (engine/engine_collision_convex.c:52-88):
    // Calls ccdMPRPenetration via our C bridge which handles ccd_t setup.
    //
    // Rust trampolines adapt mjccd_support/mjccd_center to the bridge's
    // double[3] callback ABI.

    // Bridge callback trampolines: adapt ccd_vec3_t (double[3]) ↔ ccd_vec3_t
    // The bridge passes dir/vec as *const double[3] / *mut double[3] (same layout).
    unsafe extern "C" fn support_trampoline(obj: *const core::ffi::c_void,
                                             dir: *const f64, vec: *mut f64) {
        // SAFETY: bridge guarantees dir and vec point to 3 f64 each.
        // We reconstruct a ccd_vec3_t on the stack for the existing Rust impl.
        let mut ccd_dir = crate::types::ccd_vec3_t { v: [0u8; 24] };
        let mut ccd_vec = crate::types::ccd_vec3_t { v: [0u8; 24] };
        (ccd_dir.v.as_mut_ptr() as *mut f64).copy_from_nonoverlapping(dir, 3);
        crate::engine::engine_collision_convex::mjccd_support(
            obj as *const (),
            &ccd_dir as *const crate::types::ccd_vec3_t,
            &mut ccd_vec as *mut crate::types::ccd_vec3_t,
        );
        (vec as *mut f64).copy_from_nonoverlapping(
            ccd_vec.v.as_ptr() as *const f64, 3);
    }

    unsafe extern "C" fn center_trampoline(obj: *const core::ffi::c_void,
                                            center: *mut f64) {
        // SAFETY: bridge guarantees center points to 3 f64.
        let mut ccd_center = crate::types::ccd_vec3_t { v: [0u8; 24] };
        crate::engine::engine_collision_convex::mjccd_center(
            obj as *const (),
            &mut ccd_center as *mut crate::types::ccd_vec3_t,
        );
        (center as *mut f64).copy_from_nonoverlapping(
            ccd_center.v.as_ptr() as *const f64, 3);
    }

    // SAFETY: m, obj1, obj2, con are valid pointers from caller.
    unsafe {
        // use_prism_dir = 1 if either geom is hfield
        let use_prism = ((*obj1).geom_type == mjtGeom_mjGEOM_HFIELD as i32
            || (*obj2).geom_type == mjtGeom_mjGEOM_HFIELD as i32) as i32;

        // FFI call to libccd_bridge
        extern "C" {
            fn c2rust_ccd_mpr_penetration(
                obj1: *const core::ffi::c_void,
                obj2: *const core::ffi::c_void,
                mpr_tolerance: f64,
                epa_tolerance: f64,
                max_iterations: u64,
                support1: unsafe extern "C" fn(*const core::ffi::c_void, *const f64, *mut f64),
                support2: unsafe extern "C" fn(*const core::ffi::c_void, *const f64, *mut f64),
                center1: unsafe extern "C" fn(*const core::ffi::c_void, *mut f64),
                center2: unsafe extern "C" fn(*const core::ffi::c_void, *mut f64),
                use_prism_dir: i32,
                result: *mut CcdResult,
            );
        }

        #[repr(C)]
        struct CcdResult {
            status: i32,
            dir_is_origin: i32,
            depth: f64,
            dir: [f64; 3],
            pos: [f64; 3],
        }

        let mut result = CcdResult {
            status: -1,
            dir_is_origin: 0,
            depth: 0.0,
            dir: [0.0; 3],
            pos: [0.0; 3],
        };

        c2rust_ccd_mpr_penetration(
            obj1 as *const core::ffi::c_void,
            obj2 as *const core::ffi::c_void,
            (*m).opt.ccd_tolerance,
            (*m).opt.ccd_tolerance,
            (*m).opt.ccd_iterations as u64,
            support_trampoline,
            support_trampoline,
            center_trampoline,
            center_trampoline,
            use_prism,
            &mut result as *mut CcdResult,
        );

        if result.status != 0 {
            return 0;
        }
        if result.dir_is_origin != 0 {
            return 0;
        }

        // Fill contact — exact C order
        (*con).dist = margin - result.depth;
        (*con).normal[0] = result.dir[0];
        (*con).normal[1] = result.dir[1];
        (*con).normal[2] = result.dir[2];
        (*con).pos[0] = result.pos[0];
        (*con).pos[1] = result.pos[1];
        (*con).pos[2] = result.pos[2];
        (*con).tangent[0] = 0.0;
        (*con).tangent[1] = 0.0;
        (*con).tangent[2] = 0.0;

        1
    }
}

/// C: mjc_penetration (engine/engine_collision_convex.c:87)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:__libccd_wrapper, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_add3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_sub3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_zero3, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocByte, cxx:_mjc_ccd, cxx:_mjc_ccdSize, cxx:_mju_normalize3, cxx:_mju_scl3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_penetration(m: *const mjModel, d: *mut mjData, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj, con: *mut mjPreContact, nconmax: i32, margin: f64) -> i32 {
    todo!() // cxx-internal:engine_collision_convex.c.o:_mjc_penetration
}

/// C: mjccd_center (engine/engine_collision_convex.c:137)
/// Calls: cxx:_mjc_center
#[allow(unused_variables, non_snake_case)]
pub fn mjccd_center(obj: *const (), center: *mut ccd_vec3_t) {
    // SAFETY: obj is actually a *const mjCCDObj. center.v is [u8;24] but represents [f64;3].
    unsafe {
        let v_ptr = (*center).v.as_mut_ptr() as *mut f64;
        mjc_center(v_ptr, obj as *const mjCCDObj);
    }
}

/// C: mjc_center (engine/engine_collision_convex.c:142)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_addTo3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy3, cxx:_mju_scl3, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_center(res: *mut f64, obj: *const mjCCDObj) {
    // SAFETY: obj valid, res[3] valid. Union data accessed via raw offsets.
    // hfield.prism at offset 0: mjtNum[6][3]. flex.aabb at offset 16, flex.elemadr at offset 24,
    // flex.vert_xpos at offset 40, flex.vertadr at offset 48.
    unsafe {
        let g = (*obj).geom;
        let f = (*obj).flex;
        let e = (*obj).elem;
        let v = (*obj).vert;

        const MJ_GEOM_HFIELD: i32 = 1;
        if (*obj).geom_type == MJ_GEOM_HFIELD {
            crate::engine::engine_util_blas::mju_zero3(res);
            let prism = (*obj).data._data.as_ptr() as *const f64;
            for i in 0..6_i32 {
                crate::engine::engine_inline::mji_addTo3(res, prism.add((i * 3) as usize));
            }
            crate::engine::engine_util_blas::mju_scl3(res, res, 1.0 / 6.0);
            return;
        }

        // return geom position
        if g >= 0 {
            crate::engine::engine_inline::mji_copy3(res, (*obj).pos.as_ptr());
            return;
        }

        // return flex element position
        if e >= 0 {
            let data_ptr = (*obj).data._data.as_ptr();
            let flex_aabb = *(data_ptr.add(16) as *const *const f64);
            let flex_elemadr = *(data_ptr.add(24) as *const *const i32);
            crate::engine::engine_inline::mji_copy3(
                res, flex_aabb.add(6 * (*flex_elemadr.add(f as usize) + e) as usize));
            return;
        }

        // return flex vertex position
        if f >= 0 {
            let data_ptr = (*obj).data._data.as_ptr();
            let flex_vert_xpos = *(data_ptr.add(40) as *const *const f64);
            let flex_vertadr = *(data_ptr.add(48) as *const *const i32);
            crate::engine::engine_inline::mji_copy3(
                res, flex_vert_xpos.add(3 * (*flex_vertadr.add(f as usize) + v) as usize));
            return;
        }
    }
}

/// C: mulMatTVec3 (engine/engine_collision_convex.c:179)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mul_mat_t_vec3(res: *mut f64, mat: *const f64, dir: *const f64) {
    todo!() // cxx-internal:engine_collision_convex.c.o:_mulMatTVec3
}

/// C: localToGlobal (engine/engine_collision_convex.c:188)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn local_to_global(res: *mut f64, mat: *const f64, dir: *const f64, pos: *const f64) {
    todo!() // cxx-internal:engine_collision_convex.c.o:_localToGlobal
}

/// C: mjc_pointSupport (engine/engine_collision_convex.c:201)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_point_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    todo!() // cxx:_mjc_pointSupport
}

/// C: mjc_sphereSupport (engine/engine_collision_convex.c:207)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:_sphere
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_sphere_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    todo!() // cxx-internal:engine_collision_convex.c.o:_mjc_sphereSupport
}

/// C: mjc_lineSupport (engine/engine_collision_convex.c:219)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_line_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    todo!() // cxx:_mjc_lineSupport
}

/// C: mjc_capsuleSupport (engine/engine_collision_convex.c:236)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:_localToGlobal, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:_mulMatTVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_capsule_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    todo!() // cxx-internal:engine_collision_convex.c.o:_mjc_capsuleSupport
}

/// C: mjc_ellipsoidSupport (engine/engine_collision_convex.c:261)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:_localToGlobal, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:_mulMatTVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_ellipsoid_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    todo!() // cxx-internal:engine_collision_convex.c.o:_mjc_ellipsoidSupport
}

/// C: mjc_cylinderSupport (engine/engine_collision_convex.c:298)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:_localToGlobal, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:_mulMatTVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_cylinder_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    todo!() // cxx-internal:engine_collision_convex.c.o:_mjc_cylinderSupport
}

/// C: mjc_boxSupport (engine/engine_collision_convex.c:323)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:_localToGlobal, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:_mulMatTVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_box_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    todo!() // cxx-internal:engine_collision_convex.c.o:_mjc_boxSupport
}

/// C: dot3f (engine/engine_collision_convex.c:349)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn dot3f(a: *const f64, b: *const f32) -> f64 {
    // SAFETY: a points to 3 f64 elements, b points to 3 f32 elements (caller contract)
    unsafe {
        (*a.add(0)) * (*b.add(0) as f64) + (*a.add(1)) * (*b.add(1) as f64) + (*a.add(2)) * (*b.add(2) as f64)
    }
}

/// C: mjc_meshSupport (engine/engine_collision_convex.c:355)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:_dot3f, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:_localToGlobal, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:_mulMatTVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_mesh_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    todo!() // cxx-internal:engine_collision_convex.c.o:_mjc_meshSupport
}

/// C: mjc_hillclimbSupport (engine/engine_collision_convex.c:397)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:_dot3f, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:_localToGlobal, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:_mulMatTVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_hillclimb_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    todo!() // cxx-internal:engine_collision_convex.c.o:_mjc_hillclimbSupport
}

/// C: mjc_prism_support (engine/engine_collision_convex.c:459)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy3, cxx:_mju_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_prism_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    // SAFETY: obj is a valid mjCCDObj pointer, data.hfield.prism is at offset 0 in the union,
    // stored as mjtNum[6][3] = 6 rows of 3 f64 values. dir[3] and res[3] are valid.
    unsafe {
        // hfield.prism is at offset 0 of the union data blob
        let prism = (*obj).data._data.as_ptr() as *const f64;

        // find best vertex in halfspace determined by dir.z
        let istart: i32 = if *dir.add(2) < 0.0 { 0 } else { 3 };
        let mut ibest: i32 = istart;
        let mut best: f64 = crate::engine::engine_util_blas::mju_dot3(
            prism.add((istart * 3) as usize), dir);

        for i in 1..3_i32 {
            let tmp = crate::engine::engine_util_blas::mju_dot3(
                prism.add(((istart + i) * 3) as usize), dir);
            if tmp > best {
                ibest = istart + i;
                best = tmp;
            }
        }

        // copy best point
        crate::engine::engine_inline::mji_copy3(res, prism.add((ibest * 3) as usize));
    }
}

/// C: mjc_flexSupport (engine/engine_collision_convex.c:481)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_addScl3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_addToScl3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy3, cxx:_mju_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_flex_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    todo!() // cxx-internal:engine_collision_convex.c.o:_mjc_flexSupport
}

/// C: mjccd_support (engine/engine_collision_convex.c:519)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_addScl3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_addTo3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_addToScl3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_scl3, cxx-internal:engine_collision_convex.c.o:_mjc_prism_support, cxx:_mju_dot3, cxx:_mju_message, cxx:_mju_mulMatTVec3, cxx:_mju_mulMatVec3, cxx:_mju_normalize3, cxx:_mju_sign, cxx:_mju_warning, cxx:_mju_zero3
#[allow(unused_variables, non_snake_case)]
pub fn mjccd_support(obj: *const (), dir: *const ccd_vec3_t, vec: *mut ccd_vec3_t) {
    // SAFETY: obj is actually *mut mjCCDObj. dir.v and vec.v are [u8;24] representing [f64;3].
    // All pointer arithmetic follows the C original exactly.
    unsafe {
        let obj = obj as *mut mjCCDObj;
        let res: *mut f64 = (*vec).v.as_mut_ptr() as *mut f64;
        let dir: *const f64 = (*dir).v.as_ptr() as *const f64;
        let g = (*obj).geom;

        const MJ_MINVAL: f64 = 1E-15;
        const MJ_GEOM_HFIELD: i32 = 1;
        const MJ_GEOM_SPHERE: i32 = 2;
        const MJ_GEOM_CAPSULE: i32 = 3;
        const MJ_GEOM_ELLIPSOID: i32 = 4;
        const MJ_GEOM_CYLINDER: i32 = 5;
        const MJ_GEOM_BOX: i32 = 6;
        const MJ_GEOM_MESH: i32 = 7;
        const MJ_GEOM_SDF: i32 = 8;

        // flex support (g < 0)
        if g < 0 {
            let f = (*obj).flex;
            let data_ptr = (*obj).data._data.as_ptr();
            let flex_elem = *(data_ptr.add(0) as *const *const i32);
            let flex_dim = *(data_ptr.add(8) as *const *const i32);
            let flex_elemdataadr = *(data_ptr.add(32) as *const *const i32);
            let flex_vert_xpos = *(data_ptr.add(40) as *const *const f64);
            let flex_vertadr = *(data_ptr.add(48) as *const *const i32);
            let flex_xradius = *(data_ptr.add(56) as *const *const f64);
            let dim = *flex_dim.add(f as usize);

            // flex element
            if (*obj).elem >= 0 {
                let e = (*obj).elem;
                let edata = flex_elem.add(
                    *flex_elemdataadr.add(f as usize) as usize + (e * (dim + 1)) as usize);
                let vert = flex_vert_xpos.add(3 * *flex_vertadr.add(f as usize) as usize);

                crate::engine::engine_inline::mji_copy3(res, vert.add(3 * *edata.add(0) as usize));
                let mut best = crate::engine::engine_util_blas::mju_dot3(res, dir);
                for i in 1..=dim {
                    let dot = crate::engine::engine_util_blas::mju_dot3(
                        vert.add(3 * *edata.add(i as usize) as usize), dir);
                    if dot > best {
                        best = dot;
                        crate::engine::engine_inline::mji_copy3(
                            res, vert.add(3 * *edata.add(i as usize) as usize));
                    }
                }
                crate::engine::engine_inline::mji_addToScl3(
                    res, dir, *flex_xradius.add(f as usize) + 0.5 * (*obj).margin);
                return;
            }
            // flex vertex
            else {
                let vert = flex_vert_xpos.add(
                    3 * (*flex_vertadr.add(f as usize) + (*obj).vert) as usize);
                crate::engine::engine_inline::mji_addScl3(
                    res, vert, dir, *flex_xradius.add(f as usize) + 0.5 * (*obj).margin);
                return;
            }
        }

        let size = (*obj).size.as_ptr();
        let mut local_dir: [f64; 3] = [0.0; 3];

        // rotate dir to geom local frame
        crate::engine::engine_util_blas::mju_mulMatTVec3(
            local_dir.as_mut_ptr(), (*obj).mat.as_ptr(), dir);

        // compute result according to geom type
        match (*obj).geom_type {
            MJ_GEOM_SPHERE => {
                crate::engine::engine_inline::mji_scl3(res, local_dir.as_ptr(), *size.add(0));
            }

            MJ_GEOM_CAPSULE => {
                crate::engine::engine_inline::mji_scl3(res, local_dir.as_ptr(), *size.add(0));
                *res.add(2) += crate::engine::engine_util_misc::mju_sign(local_dir[2]) * *size.add(1);
            }

            MJ_GEOM_ELLIPSOID => {
                for i in 0..3_usize {
                    *res.add(i) = local_dir[i] * *size.add(i);
                }
                crate::engine::engine_util_blas::mju_normalize3(res);
                for i in 0..3_usize {
                    *res.add(i) *= *size.add(i);
                }
            }

            MJ_GEOM_CYLINDER => {
                let tmp = f64::sqrt(local_dir[0] * local_dir[0] + local_dir[1] * local_dir[1]);
                if tmp > MJ_MINVAL {
                    *res.add(0) = local_dir[0] / tmp * *size.add(0);
                    *res.add(1) = local_dir[1] / tmp * *size.add(0);
                } else {
                    *res.add(0) = 0.0;
                    *res.add(1) = 0.0;
                }
                *res.add(2) = crate::engine::engine_util_misc::mju_sign(local_dir[2]) * *size.add(1);
            }

            MJ_GEOM_BOX => {
                for i in 0..3_usize {
                    *res.add(i) = crate::engine::engine_util_misc::mju_sign(local_dir[i]) * *size.add(i);
                }
            }

            MJ_GEOM_MESH | MJ_GEOM_SDF => {
                let data_ptr = (*obj).data._data.as_ptr();
                let vertdata = *(data_ptr.add(8) as *const *const f32);
                let mut tmp: f64 = -1E+10;
                let mut ibest: i32 = -1;

                // check if graph is available (offset 72 in mesh union = graph pointer)
                let graph = *(data_ptr.add(72) as *const *const i32);

                if graph.is_null() {
                    // no graph: exhaustive search
                    let nvert = *(data_ptr as *const i32);
                    for i in 0..nvert {
                        let vdot = local_dir[0] * *vertdata.add(3 * i as usize + 0) as f64
                                 + local_dir[1] * *vertdata.add(3 * i as usize + 1) as f64
                                 + local_dir[2] * *vertdata.add(3 * i as usize + 2) as f64;
                        if vdot > tmp {
                            tmp = vdot;
                            ibest = i;
                        }
                    }
                    (*obj).meshindex = ibest;
                } else {
                    // hill-climb using graph
                    let numvert = *graph.add(0);
                    let vert_edgeadr = graph.add(2);
                    let vert_globalid = graph.add(2 + numvert as usize);
                    let edge_localid = graph.add(2 + 2 * numvert as usize);

                    ibest = if (*obj).meshindex < 0 { 0 } else { (*obj).meshindex };
                    tmp = local_dir[0] * *vertdata.add(3 * *vert_globalid.add(ibest as usize) as usize + 0) as f64
                        + local_dir[1] * *vertdata.add(3 * *vert_globalid.add(ibest as usize) as usize + 1) as f64
                        + local_dir[2] * *vertdata.add(3 * *vert_globalid.add(ibest as usize) as usize + 2) as f64;

                    let mut change: i32 = 1;
                    while change != 0 {
                        change = 0;
                        let mut i = *vert_edgeadr.add(ibest as usize);
                        loop {
                            let locid = *edge_localid.add(i as usize);
                            if locid < 0 { break; }
                            let vdot = local_dir[0] * *vertdata.add(3 * *vert_globalid.add(locid as usize) as usize) as f64
                                     + local_dir[1] * *vertdata.add(3 * *vert_globalid.add(locid as usize) as usize + 1) as f64
                                     + local_dir[2] * *vertdata.add(3 * *vert_globalid.add(locid as usize) as usize + 2) as f64;
                            if vdot > tmp {
                                tmp = vdot;
                                ibest = locid;
                                change = 1;
                            }
                            i += 1;
                        }
                    }
                    (*obj).meshindex = ibest;
                    ibest = *vert_globalid.add(ibest as usize);
                }

                if ibest < 0 {
                    crate::engine::engine_util_errmem::mju_warning(
                        b"mesh_support could not find support vertex\0".as_ptr() as *const i8);
                    crate::engine::engine_util_blas::mju_zero3(res);
                } else {
                    for i in 0..3_usize {
                        *res.add(i) = *vertdata.add(3 * ibest as usize + i) as f64;
                    }
                }
            }

            MJ_GEOM_HFIELD => {
                mjc_prism_support(res, obj, dir);
                return;
            }

            _ => {
                crate::engine::engine_util_errmem::mju_warning(
                    b"ccd support function is undefined for geom type\0".as_ptr() as *const i8);
            }
        }

        // add local_dir*margin/2 to result
        for i in 0..3_usize {
            *res.add(i) += local_dir[i] * (*obj).margin / 2.0;
        }

        // rotate result to global frame
        crate::engine::engine_util_blas::mju_mulMatVec3(res, (*obj).mat.as_ptr(), res);

        // add geom position
        crate::engine::engine_inline::mji_addTo3(res, (*obj).pos.as_ptr());
    }
}

/// C: mjc_initCCDObj (engine/engine_collision_convex.c:727)
/// Calls: cxx:_mju_copy, cxx:_mju_zero4
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_init_ccd_obj(obj: *mut mjCCDObj, m: *const mjModel, d: *const mjData, g: i32, margin: f64) {
    todo!() // cxx:_mjc_initCCDObj
}

/// C: mjc_setCCDObjFlex (engine/engine_collision_convex.c:815)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_set_ccd_obj_flex(obj: *mut mjCCDObj, flex: i32, elem: i32, vert: i32) {
    todo!() // cxx-internal:engine_collision_convex.c.o:_mjc_setCCDObjFlex
}

/// C: mjc_isDistinctContact (engine/engine_collision_convex.c:823)
/// Calls: cxx:_mju_dist3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_is_distinct_contact(con: *const mjPreContact, ncon: i32, tolerance: f64) -> i32 {
    todo!() // cxx-internal:engine_collision_convex.c.o:_mjc_isDistinctContact
}

/// C: mju_rotateFrame (engine/engine_collision_convex.c:835)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_sub3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_subFrom3, cxx:_mju_copy, cxx:_mju_mulMatMat3, cxx:_mju_mulMatVec3, cxx:_mju_subFrom3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_rotate_frame(origin: *const f64, rot: *const f64, xmat: *mut f64, xpos: *mut f64) {
    todo!() // cxx-internal:engine_collision_convex.c.o:_mju_rotateFrame
}

/// C: maxContacts (engine/engine_collision_convex.c:856)
#[allow(unused_variables, non_snake_case)]
pub fn max_contacts(m: *const mjModel, obj1: *const mjCCDObj, obj2: *const mjCCDObj) -> i32 {
    todo!() // cxx-internal:engine_collision_convex.c.o:_maxContacts
}

/// C: mjc_Convex (engine/engine_collision_convex.c:882)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:_maxContacts, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:_mjc_isDistinctContact, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:_mju_rotateFrame, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_axisAngle2Quat, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy9, cxx-internal:engine_collision_convex.c.o:_mjc_penetration, cxx:_mjc_fixNormal, cxx:_mjc_initCCDObj, cxx:_mju_makeFrame, cxx:_mju_min, cxx:_mju_quat2Mat, cxx:_mju_transpose, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_convex(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    todo!() // cxx:_mjc_Convex
}

/// C: sub3f (engine/engine_collision_convex.c:966)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn sub3f(res: *mut f64, a: *const f32, b: *const f32) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:_sub3f
}

/// C: area4f (engine/engine_collision_convex.c:973)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:_sub3f, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_cross, cxx:_mju_norm3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn area4f(hull: *const f32, idx: *const i32, a: i32, b: i32, c: i32, d: i32) -> f64 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:_area4f
}

/// C: hull4f (engine/engine_collision_convex.c:984)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:_area4f
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn hull4f(res: *mut i32, hull: *const f32, idx: *const i32, nhull: i32, a: i32) -> i32 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:_hull4f
}

/// C: mjc_PlaneConvex (engine/engine_collision_convex.c:1022)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:_hull4f, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:_localToGlobal, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_addToScl3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_sub3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_zero3, cxx:_mjc_initCCDObj, cxx:_mju_dot3, cxx:_mju_mulMatTVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_plane_convex(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    todo!() // cxx:_mjc_PlaneConvex
}

/// C: addVert (engine/engine_collision_convex.c:1135)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_vert(obj: *mut mjCCDObj, x: f64, y: f64, z: f64) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:_addVert
}

/// C: addPrismVert (engine/engine_collision_convex.c:1150)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_prism_vert(obj: *mut mjCCDObj, r: i32, c: i32, i: i32, dx: f64, dy: f64, margin: f64) {
    todo!() // cxx-internal:engine_collision_convex.c.o:_addPrismVert
}

/// C: mjc_ConvexHField (engine/engine_collision_convex.c:1171)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:_addPrismVert, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_addTo3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy9, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_mulMatVec3, cxx-internal:engine_collision_convex.c.o:_mjc_penetration, cxx-internal:engine_collision_convex.c.o:_mji_mulMatTMat3, cxx:_mjc_fixNormal, cxx:_mjc_initCCDObj, cxx:_mju_mulMatTVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_convex_h_field(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    todo!() // cxx:_mjc_ConvexHField
}

/// C: mjc_ellipsoidInside (engine/engine_collision_convex.c:1332)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_addScl3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy3, cxx:_mju_dist3, cxx:_mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_ellipsoid_inside(nrm: *mut f64, pos: *const f64, size: *const f64) -> i32 {
    todo!() // cxx-internal:engine_collision_convex.c.o:_mjc_ellipsoidInside
}

/// C: mjc_ellipsoidOutside (engine/engine_collision_convex.c:1387)
/// Calls: cxx:_mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_ellipsoid_outside(nrm: *mut f64, pos: *const f64, size: *const f64) -> i32 {
    todo!() // cxx-internal:engine_collision_convex.c.o:_mjc_ellipsoidOutside
}

/// C: mjc_fixNormal (engine/engine_collision_convex.c:1436)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:_mjc_ellipsoidInside, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:_mjc_ellipsoidOutside, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_mulMatVec3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_scl3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_sub3, cxx:_mju_mulMatTVec3, cxx:_mju_norm, cxx:_mju_normalize3, cxx:_mju_sub3
#[allow(unused_variables, non_snake_case)]
pub fn mjc_fix_normal(m: *const mjModel, d: *const mjData, con: *mut mjPreContact, g1: i32, g2: i32) {
    todo!() // cxx:_mjc_fixNormal
}

/// C: mjc_ConvexElem (engine/engine_collision_convex.c:1586)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:_mjc_setCCDObjFlex, cxx-internal:engine_collision_convex.c.o:_mjc_penetration, cxx:_mjc_fixNormal, cxx:_mjc_initCCDObj
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_convex_elem(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, f1: i32, e1: i32, v1: i32, f2: i32, e2: i32, margin: f64) -> i32 {
    todo!() // cxx:_mjc_ConvexElem
}

/// C: mjc_HFieldElem (engine/engine_collision_convex.c:1615)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:_mjc_setCCDObjFlex, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_addTo3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_mulMatTVec3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_sub3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_zero3, cxx-internal:engine_collision_convex.c.o:_mjc_penetration, cxx:_mjc_initCCDObj, cxx:_mju_max, cxx:_mju_min, cxx:_mju_mulMatVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_h_field_elem(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g: i32, f: i32, e: i32, margin: f64) -> i32 {
    todo!() // cxx:_mjc_HFieldElem
}

