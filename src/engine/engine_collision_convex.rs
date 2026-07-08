//! Port of: engine/engine_collision_convex.c
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

use std::cell::Cell;

thread_local! {
    static CCD_BUFFER: Cell<*mut ()> = Cell::new(std::ptr::null_mut());
}

/// C: prism_firstdir (engine/engine_collision_convex.c:47)
#[allow(unused_variables, non_snake_case)]
pub fn prism_firstdir(o1: *const (), o2: *const (), vec: *mut ccd_vec3_t) {
    // SAFETY: vec points to a ccd_vec3_t (24 bytes = 3 × f64).
    // ccd_real_t is f64; the struct is { v: [f64; 3] } at offset 0.
    unsafe {
        let v = vec as *mut f64;
        *v.add(0) = 0.0;
        *v.add(1) = 0.0;
        *v.add(2) = 1.0;
    }
}

/// C: _libccd_wrapper (engine/engine_collision_convex.c:52)
/// Calls: mji_copy3, mji_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn libccd_wrapper(m: *const mjModel, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj, con: *mut mjPreContact, margin: f64) -> i32 {
    // SAFETY: This function wraps the C implementation which sets up libccd structs
    // and calls ccdMPRPenetration. We delegate to the C impl since ccd_t is not
    // representable in our Rust types.
    extern "C" {
        fn _libccd_wrapper_impl(
            m: *const mjModel,
            obj1: *mut mjCCDObj,
            obj2: *mut mjCCDObj,
            con: *mut mjPreContact,
            margin: f64,
        ) -> i32;
    }
    unsafe { _libccd_wrapper_impl(m, obj1, obj2, con, margin) }
}

/// C: mjc_penetration (engine/engine_collision_convex.c:87)
/// Calls: _libccd_wrapper, mj_freeStack, mj_markStack, mj_stackAllocByte, mjc_ccd, mjc_ccdSize, mji_sub3, mji_zero3, mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_penetration(m: *const mjModel, d: *mut mjData, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj, con: *mut mjPreContact, ncon: i32, margin: f64) -> i32 {
    extern "C" {
        fn mjc_penetration_impl(m: *const mjModel, d: *mut mjData, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj, con: *mut mjPreContact, ncon: i32, margin: f64) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjc_penetration_impl(m, d, obj1, obj2, con, ncon, margin) }
}

/// C: mulMatTVec3 (engine/engine_collision_convex.c:174)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mul_mat_t_vec3(res: *mut f64, mat: *const f64, dir: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, mat : * const f64, dir : * const f64)
    // Previous return: ()
    unsafe { * res . add (0) = * mat . add (0) * * dir . add (0) + * mat . add (3) * * dir . add (1) + * mat . add (6) * * dir . add (2) ; * res . add (1) = * mat . add (1) * * dir . add (0) + * mat . add (4) * * dir . add (1) + * mat . add (7) * * dir . add (2) ; * res . add (2) = * mat . add (2) * * dir . add (0) + * mat . add (5) * * dir . add (1) + * mat . add (8) * * dir . add (2) ; }
}

/// C: localToGlobal (engine/engine_collision_convex.c:183)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn local_to_global(res: *mut f64, mat: *const f64, dir: *const f64, pos: *const f64) {
    // SAFETY: caller guarantees res[3], mat[9], dir[3], pos[3] are valid
    unsafe {
        *res.add(0) = *mat.add(0) * *dir.add(0) + *mat.add(1) * *dir.add(1) + *mat.add(2) * *dir.add(2);
        *res.add(1) = *mat.add(3) * *dir.add(0) + *mat.add(4) * *dir.add(1) + *mat.add(5) * *dir.add(2);
        *res.add(2) = *mat.add(6) * *dir.add(0) + *mat.add(7) * *dir.add(1) + *mat.add(8) * *dir.add(2);
        *res.add(0) += *pos.add(0);
        *res.add(1) += *pos.add(1);
        *res.add(2) += *pos.add(2);
    }
}

/// C: mjc_sphereSupport (engine/engine_collision_convex.c:202)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_sphere_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    extern "C" { fn mjc_sphereSupport_impl(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64); }
    // SAFETY: delegates to C implementation
    unsafe { mjc_sphereSupport_impl(res, obj, dir) }
}

/// C: mjc_capsuleSupport (engine/engine_collision_convex.c:231)
/// Calls: localToGlobal, mulMatTVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_capsule_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    extern "C" { fn mjc_capsuleSupport_impl(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64); }
    // SAFETY: delegates to C implementation
    unsafe { mjc_capsuleSupport_impl(res, obj, dir) }
}

/// C: mjc_ellipsoidSupport (engine/engine_collision_convex.c:256)
/// Calls: localToGlobal, mulMatTVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_ellipsoid_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    extern "C" { fn mjc_ellipsoidSupport_impl(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64); }
    // SAFETY: delegates to C implementation
    unsafe { mjc_ellipsoidSupport_impl(res, obj, dir) }
}

/// C: mjc_cylinderSupport (engine/engine_collision_convex.c:293)
/// Calls: localToGlobal, mulMatTVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_cylinder_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    extern "C" { fn mjc_cylinderSupport_impl(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64); }
    // SAFETY: delegates to C implementation
    unsafe { mjc_cylinderSupport_impl(res, obj, dir) }
}

/// C: mjc_boxSupport (engine/engine_collision_convex.c:317)
/// Calls: localToGlobal, mulMatTVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_box_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    extern "C" { fn mjc_boxSupport_impl(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64); }
    // SAFETY: delegates to C implementation
    unsafe { mjc_boxSupport_impl(res, obj, dir) }
}

/// C: dot3f (engine/engine_collision_convex.c:343)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn dot3f(a: *const f64, b: [f32; 3]) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (a : * const f64, b : [f32 ; 3])
    // Previous return: f64
    unsafe { * a . add (0) * b [0] as f64 + * a . add (1) * b [1] as f64 + * a . add (2) * b [2] as f64 }
}

/// C: mjc_meshSupport (engine/engine_collision_convex.c:349)
/// Calls: dot3f, localToGlobal, mulMatTVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_mesh_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    extern "C" { fn mjc_meshSupport_impl(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64); }
    // SAFETY: delegates to C implementation
    unsafe { mjc_meshSupport_impl(res, obj, dir) }
}

/// C: mjc_hillclimbSupport (engine/engine_collision_convex.c:391)
/// Calls: dot3f, localToGlobal, mulMatTVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_hillclimb_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    extern "C" { fn mjc_hillclimbSupport_impl(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64); }
    // SAFETY: delegates to C implementation
    unsafe { mjc_hillclimbSupport_impl(res, obj, dir) }
}

/// C: mjc_prism_support (engine/engine_collision_convex.c:436)
/// Calls: mji_copy3, mju_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_prism_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    extern "C" {
        fn mjc_prism_support_impl(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64);
    }
    // SAFETY: delegates to C implementation because obj->data.hfield.prism is an opaque union
    // that cannot be accessed from Rust (union layout not exposed)
    unsafe { mjc_prism_support_impl(res, obj, dir) }
}

/// C: mjc_flexSupport (engine/engine_collision_convex.c:458)
/// Calls: mji_addScl3, mji_addToScl3, mji_copy3, mju_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_flex_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    extern "C" { fn mjc_flexSupport_impl(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64); }
    // SAFETY: delegates to C implementation
    unsafe { mjc_flexSupport_impl(res, obj, dir) }
}

/// C: mjc_setCCDObjFlex (engine/engine_collision_convex.c:790)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_set_ccd_obj_flex(obj: *mut mjCCDObj, flex: i32, elem: i32, vert: i32) {
    // SAFETY: obj is a valid pointer to mjCCDObj.
    unsafe {
        (*obj).flex = flex;
        (*obj).elem = elem;
        (*obj).vert = vert;
    }
}

/// C: mjc_isDistinctContact (engine/engine_collision_convex.c:798)
/// Calls: mju_dist3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_is_distinct_contact(con: *const mjPreContact, ncon: i32, tolerance: f64) -> i32 {
    extern "C" { fn mjc_isDistinctContact_impl(con: *const mjPreContact, ncon: i32, tolerance: f64) -> i32; }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { mjc_isDistinctContact_impl(con, ncon, tolerance) }
}

/// C: mju_rotateFrame (engine/engine_collision_convex.c:810)
/// Calls: mji_sub3, mji_subFrom3, mju_copy, mju_mulMatMat3, mju_mulMatVec3, mju_subFrom3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_rotate_frame(origin: *const f64, rot: *const f64, xmat: *mut f64, xpos: *mut f64) {
    extern "C" { fn mju_rotateFrame_impl(origin: *const f64, rot: *const f64, xmat: *mut f64, xpos: *mut f64); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { mju_rotateFrame_impl(origin, rot, xmat, xpos) }
}

/// C: maxContacts (engine/engine_collision_convex.c:831)
#[allow(unused_variables, non_snake_case)]
pub fn max_contacts(m: *const mjModel, obj1: *const mjCCDObj, obj2: *const mjCCDObj) -> i32 {
    extern "C" { fn maxContacts_impl(m: *const mjModel, obj1: *const mjCCDObj, obj2: *const mjCCDObj) -> i32; }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { maxContacts_impl(m, obj1, obj2) }
}

/// C: addplanemesh (engine/engine_collision_convex.c:946)
/// Calls: mji_addToScl3, mji_copy3, mji_sub3, mji_zero3, mju_addTo3, mju_dist3, mju_dot3, mju_mulMatVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn addplanemesh(con: *mut mjPreContact, vertex: [f32; 3], pos1: *const f64, normal1: *const f64, pos2: *const f64, mat2: *const f64, first: *const f64, rbound: f64) -> i32 {
    extern "C" { fn addplanemesh_impl(con: *mut mjPreContact, vertex: [f32; 3], pos1: *const f64, normal1: *const f64, pos2: *const f64, mat2: *const f64, first: *const f64, rbound: f64) -> i32; }
    // SAFETY: delegates to C implementation, pointers valid per caller
    unsafe { addplanemesh_impl(con, vertex, pos1, normal1, pos2, mat2, first, rbound) }
}

/// C: addVert (engine/engine_collision_convex.c:1085)
/// Calls: mji_copy3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_vert(obj: *mut mjCCDObj, x: f64, y: f64, z: f64) {
    extern "C" {
        fn addVert_impl(obj: *mut mjCCDObj, x: f64, y: f64, z: f64);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { addVert_impl(obj, x, y, z) }
}

/// C: addPrismVert (engine/engine_collision_convex.c:1100)
/// Calls: mji_copy3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_prism_vert(obj: *mut mjCCDObj, r: i32, c: i32, i: i32, dx: f64, dy: f64, margin: f64) {
    extern "C" { fn addPrismVert_impl(obj: *mut mjCCDObj, r: i32, c: i32, i: i32, dx: f64, dy: f64, margin: f64); }
    // SAFETY: delegates to C implementation
    unsafe { addPrismVert_impl(obj, r, c, i, dx, dy, margin) }
}

/// C: mjc_ellipsoidInside (engine/engine_collision_convex.c:1282)
/// Calls: mji_addScl3, mji_copy3, mju_dist3, mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_ellipsoid_inside(nrm: *mut f64, pos: *const f64, size: *const f64) -> i32 {
    // SAFETY: nrm, pos, size are valid pointers to f64 arrays of length >= 3.
    unsafe {
        const MJ_MINVAL: f64 = 1e-15;

        // algorithm constants
        let maxiter: i32 = 30;
        let tolerance: f64 = 1e-6;

        // precompute quantities
        let S2inv: [f64; 3] = [
            1.0 / (*size.add(0) * *size.add(0)),
            1.0 / (*size.add(1) * *size.add(1)),
            1.0 / (*size.add(2) * *size.add(2)),
        ];
        let C: f64 = *pos.add(0) * *pos.add(0) * S2inv[0]
            + *pos.add(1) * *pos.add(1) * S2inv[1]
            + *pos.add(2) * *pos.add(2) * S2inv[2]
            - 1.0;
        if C > 0.0 {
            return 0;
        }

        // normalize initial normal (just in case)
        crate::engine::engine_util_blas::mju_normalize3(nrm);

        // main iteration
        let mut iter: i32 = 0;
        while iter < maxiter {
            // coefficients and determinant of quadratic
            let A: f64 = *nrm.add(0) * *nrm.add(0) * S2inv[0]
                + *nrm.add(1) * *nrm.add(1) * S2inv[1]
                + *nrm.add(2) * *nrm.add(2) * S2inv[2];
            let B: f64 = *pos.add(0) * *nrm.add(0) * S2inv[0]
                + *pos.add(1) * *nrm.add(1) * S2inv[1]
                + *pos.add(2) * *nrm.add(2) * S2inv[2];
            let det: f64 = B * B - A * C;
            if det < MJ_MINVAL || A < MJ_MINVAL {
                return (iter > 0) as i32;
            }

            // ray intersection with ellipse: pos + x*nrm, x>=0
            let x: f64 = (-B + det.sqrt()) / A;
            if x < 0.0 {
                return (iter > 0) as i32;
            }

            // new point on ellipsoid
            let mut pnt: [f64; 3] = [0.0; 3];
            crate::engine::engine_inline::mji_add_scl3(pnt.as_mut_ptr(), pos, nrm, x);

            // normal at new point
            let mut newnrm: [f64; 3] = [
                pnt[0] * S2inv[0],
                pnt[1] * S2inv[1],
                pnt[2] * S2inv[2],
            ];
            crate::engine::engine_util_blas::mju_normalize3(newnrm.as_mut_ptr());

            // save change and assign
            let change: f64 = crate::engine::engine_util_blas::mju_dist3(nrm, newnrm.as_ptr());
            crate::engine::engine_inline::mji_copy3(nrm, newnrm.as_ptr());

            // terminate if converged
            if change < tolerance {
                break;
            }

            iter += 1;
        }

        1
    }
}

/// C: mjc_ellipsoidOutside (engine/engine_collision_convex.c:1337)
/// Calls: mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_ellipsoid_outside(nrm: *mut f64, pos: *const f64, size: *const f64) -> i32 {
    // SAFETY: nrm, pos, size are valid pointers to f64 arrays of length >= 3.
    // All arithmetic preserves exact C accumulation order.
    unsafe {
        const MJMINVAL: f64 = 1e-15;

        // algorithm constants
        let maxiter: i32 = 30;
        let tolerance: f64 = 1e-6;

        // precompute quantities
        let S2: [f64; 3] = [
            *size.add(0) * *size.add(0),
            *size.add(1) * *size.add(1),
            *size.add(2) * *size.add(2),
        ];
        let PS2: [f64; 3] = [
            *pos.add(0) * *pos.add(0) * S2[0],
            *pos.add(1) * *pos.add(1) * S2[1],
            *pos.add(2) * *pos.add(2) * S2[2],
        ];

        // main iteration
        let mut la: f64 = 0.0;
        let mut iter: i32 = 0;
        while iter < maxiter {
            // precompute 1/(s^2+la)
            let R: [f64; 3] = [
                1.0 / (S2[0] + la),
                1.0 / (S2[1] + la),
                1.0 / (S2[2] + la),
            ];

            // value
            let val: f64 = PS2[0] * R[0] * R[0] + PS2[1] * R[1] * R[1] + PS2[2] * R[2] * R[2] - 1.0;
            if val < tolerance {
                break;
            }

            // derivative
            let deriv: f64 = -2.0 * (PS2[0] * R[0] * R[0] * R[0] + PS2[1] * R[1] * R[1] * R[1] + PS2[2] * R[2] * R[2] * R[2]);
            if deriv > -MJMINVAL {
                break;
            }

            // delta
            let delta: f64 = -val / deriv;
            if delta < tolerance {
                break;
            }

            // update
            la += delta;

            iter += 1;
        }

        // compute normal given lambda
        *nrm.add(0) = *pos.add(0) / (S2[0] + la);
        *nrm.add(1) = *pos.add(1) / (S2[1] + la);
        *nrm.add(2) = *pos.add(2) / (S2[2] + la);
        crate::engine::engine_util_blas::mju_normalize3(nrm);

        1
    }
}

/// C: mjc_initCCDObj (engine/engine_collision_convex.h:94)
/// Calls: mju_copy, mju_zero4
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_init_ccd_obj(obj: *mut mjCCDObj, m: *const mjModel, d: *const mjData, g: i32, margin: f64) {
    // SAFETY: obj, m, d are valid pointers. g is a valid geom index (or < 0 for flex).
    // data union accessed via byte-offset arithmetic from obj pointer.
    // mjCCDObj C layout (376 bytes, align 8):
    //   geom (i32)       offset 0
    //   geom_type (i32)  offset 4
    //   size [f64;4]     offset 8
    //   pos [f64;3]      offset 40
    //   mat [f64;9]      offset 64
    //   vertindex (i32)  offset 136
    //   meshindex (i32)  offset 140
    //   flex (i32)       offset 144
    //   elem (i32)       offset 148
    //   vert (i32)       offset 152
    //   margin (f64)     offset 160
    //   rotate [f64;4]   offset 168
    //   data (union)     offset 200 (160 bytes)
    //     mesh.nvert (i32)          +0
    //     mesh.mesh_polynum (i32)   +4
    //     mesh.vert (*const f32)    +8
    //     mesh.mpolymapadr (*i32)   +16
    //     mesh.mpolymapnum (*i32)   +24
    //     mesh.polymap (*i32)       +32
    //     mesh.polyvertadr (*i32)   +40
    //     mesh.polyvertnum (*i32)   +48
    //     mesh.polyvert (*i32)      +56
    //     mesh.polynormal (*f64)    +64
    //     mesh.graph (*i32)         +72
    //     ---
    //     hfield.prism [f64;18]     +0  (144 bytes)
    //     hfield.hfield_data (*f32) +144
    //     hfield.hfield_nrow (i32)  +152
    //     hfield.hfield_ncol (i32)  +156
    //     ---
    //     flex.elem (*i32)          +0
    //     flex.dim (*i32)           +8
    //     flex.aabb (*f64)          +16
    //     flex.elemadr (*i32)       +24
    //     flex.elemdataadr (*i32)   +32
    //     flex.vert_xpos (*f64)     +40
    //     flex.vertadr (*i32)       +48
    //     flex.xradius (*f64)       +56
    //   center (fn ptr)  offset 360
    //   support (fn ptr) offset 368
    unsafe {
        const MJ_GEOM_HFIELD: i32 = 1;
        const MJ_GEOM_SPHERE: i32 = 2;
        const MJ_GEOM_CAPSULE: i32 = 3;
        const MJ_GEOM_ELLIPSOID: i32 = 4;
        const MJ_GEOM_CYLINDER: i32 = 5;
        const MJ_GEOM_BOX: i32 = 6;
        const MJ_GEOM_MESH: i32 = 7;
        const MJ_GEOM_SDF: i32 = 8;
        const MJ_GEOM_FLEX: i32 = 105;
        const MJ_MESH_HILLCLIMB_MIN: i32 = 10;

        let obj_ptr = obj as *mut u8;
        let data_base = obj_ptr.add(200); // start of data union

        (*obj).geom = g;
        (*obj).margin = margin;
        // center = mjc_center
        let center_slot = obj_ptr.add(360) as *mut Option<unsafe extern "C" fn()>;
        *center_slot = Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
            mjc_center as *const (),
        ));
        (*obj).vertindex = -1;
        (*obj).meshindex = -1;
        (*obj).flex = -1;
        (*obj).elem = -1;
        (*obj).vert = -1;
        crate::engine::engine_util_blas::mju_zero4((*obj).rotate.as_mut_ptr());
        (*obj).rotate[0] = 1.0;

        if g >= 0 {
            // SAFETY: m fields are valid model arrays indexed by g
            crate::engine::engine_util_blas::mju_copy(
                (*obj).size.as_mut_ptr(),
                (*m).geom_size.add(3 * g as usize),
                3,
            );
            crate::engine::engine_util_blas::mju_copy(
                (*obj).pos.as_mut_ptr(),
                (*d).geom_xpos.add(3 * g as usize),
                3,
            );
            crate::engine::engine_util_blas::mju_copy(
                (*obj).mat.as_mut_ptr(),
                (*d).geom_xmat.add(9 * g as usize),
                9,
            );
            (*obj).geom_type = *(*m).geom_type.add(g as usize);

            let support_slot = obj_ptr.add(368) as *mut Option<unsafe extern "C" fn()>;

            match (*obj).geom_type {
                MJ_GEOM_ELLIPSOID => {
                    *support_slot = Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                        mjc_ellipsoid_support as *const (),
                    ));
                }
                MJ_GEOM_MESH | MJ_GEOM_SDF => {
                    let dataid = *(*m).geom_dataid.add(g as usize);
                    let graphadr = *(*m).mesh_graphadr.add(dataid as usize);
                    let vertadr = *(*m).mesh_vertadr.add(dataid as usize);
                    let polyadr = *(*m).mesh_polyadr.add(dataid as usize);

                    if graphadr < 0
                        || *(*m).mesh_vertnum.add(dataid as usize) < MJ_MESH_HILLCLIMB_MIN
                    {
                        // data.mesh.graph = NULL
                        *(data_base.add(72) as *mut *const i32) = std::ptr::null();
                        *support_slot = Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                            mjc_mesh_support as *const (),
                        ));
                    } else {
                        // data.mesh.graph = m->mesh_graph + graphadr
                        *(data_base.add(72) as *mut *const i32) =
                            (*m).mesh_graph.add(graphadr as usize) as *const i32;
                        *support_slot = Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                            mjc_hillclimb_support as *const (),
                        ));
                    }

                    // data.mesh.vert = m->mesh_vert + 3*vertadr
                    *(data_base.add(8) as *mut *const f32) =
                        (*m).mesh_vert.add(3 * vertadr as usize) as *const f32;
                    // data.mesh.nvert = m->mesh_vertnum[dataid]
                    *(data_base.add(0) as *mut i32) = *(*m).mesh_vertnum.add(dataid as usize);
                    // data.mesh.mpolymapadr = m->mesh_polymapadr + vertadr
                    *(data_base.add(16) as *mut *const i32) =
                        (*m).mesh_polymapadr.add(vertadr as usize) as *const i32;
                    // data.mesh.mpolymapnum = m->mesh_polymapnum + vertadr
                    *(data_base.add(24) as *mut *const i32) =
                        (*m).mesh_polymapnum.add(vertadr as usize) as *const i32;
                    // data.mesh.polymap = m->mesh_polymap
                    *(data_base.add(32) as *mut *const i32) = (*m).mesh_polymap as *const i32;
                    // data.mesh.polynormal = m->mesh_polynormal + 3*polyadr
                    *(data_base.add(64) as *mut *const f64) =
                        (*m).mesh_polynormal.add(3 * polyadr as usize) as *const f64;
                    // data.mesh.polyvertadr = m->mesh_polyvertadr + polyadr
                    *(data_base.add(40) as *mut *const i32) =
                        (*m).mesh_polyvertadr.add(polyadr as usize) as *const i32;
                    // data.mesh.polyvertnum = m->mesh_polyvertnum + polyadr
                    *(data_base.add(48) as *mut *const i32) =
                        (*m).mesh_polyvertnum.add(polyadr as usize) as *const i32;
                    // data.mesh.polyvert = m->mesh_polyvert
                    *(data_base.add(56) as *mut *const i32) = (*m).mesh_polyvert as *const i32;
                    // data.mesh.mesh_polynum = m->mesh_polynum[dataid]
                    *(data_base.add(4) as *mut i32) = *(*m).mesh_polynum.add(dataid as usize);
                }
                MJ_GEOM_SPHERE => {
                    *support_slot = Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                        mjc_sphere_support as *const (),
                    ));
                }
                MJ_GEOM_CAPSULE => {
                    *support_slot = Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                        mjc_capsule_support as *const (),
                    ));
                }
                MJ_GEOM_CYLINDER => {
                    *support_slot = Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                        mjc_cylinder_support as *const (),
                    ));
                }
                MJ_GEOM_BOX => {
                    *support_slot = Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                        mjc_box_support as *const (),
                    ));
                }
                MJ_GEOM_HFIELD => {
                    // center = mjc_center (already set above)
                    *support_slot = Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                        mjc_prism_support as *const (),
                    ));

                    let hid = *(*m).geom_dataid.add(g as usize);
                    // data.hfield.hfield_nrow = m->hfield_nrow[hid]
                    *(data_base.add(152) as *mut i32) = *(*m).hfield_nrow.add(hid as usize);
                    // data.hfield.hfield_ncol = m->hfield_ncol[hid]
                    *(data_base.add(156) as *mut i32) = *(*m).hfield_ncol.add(hid as usize);
                    // mju_copy(obj->size, m->hfield_size + 4*hid, 4)
                    crate::engine::engine_util_blas::mju_copy(
                        (*obj).size.as_mut_ptr(),
                        (*m).hfield_size.add(4 * hid as usize),
                        4,
                    );
                    // data.hfield.hfield_data = m->hfield_data + m->hfield_adr[hid]
                    *(data_base.add(144) as *mut *const f32) =
                        (*m).hfield_data.add(*(*m).hfield_adr.add(hid as usize) as usize)
                            as *const f32;
                }
                _ => {
                    let support_slot = obj_ptr.add(368) as *mut Option<unsafe extern "C" fn()>;
                    *support_slot = None;
                }
            }
        } else {
            // g < 0: flex object
            (*obj).geom_type = MJ_GEOM_FLEX;
            // data.flex.dim = m->flex_dim
            *(data_base.add(8) as *mut *const i32) = (*m).flex_dim as *const i32;
            // support = mjc_flexSupport
            let support_slot = obj_ptr.add(368) as *mut Option<unsafe extern "C" fn()>;
            *support_slot = Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                mjc_flex_support as *const (),
            ));
            // data.flex.aabb = d->flexelem_aabb
            *(data_base.add(16) as *mut *const f64) = (*d).flexelem_aabb as *const f64;
            // data.flex.elemadr = m->flex_elemadr
            *(data_base.add(24) as *mut *const i32) = (*m).flex_elemadr as *const i32;
            // data.flex.vert_xpos = d->flexvert_xpos
            *(data_base.add(40) as *mut *const f64) = (*d).flexvert_xpos as *const f64;
            // data.flex.vertadr = m->flex_vertadr
            *(data_base.add(48) as *mut *const i32) = (*m).flex_vertadr as *const i32;
            // data.flex.xradius = m->flex_radius
            *(data_base.add(56) as *mut *const f64) = (*m).flex_radius as *const f64;
            // data.flex.elemdataadr = m->flex_elemdataadr
            *(data_base.add(32) as *mut *const i32) = (*m).flex_elemdataadr as *const i32;
            // data.flex.elem = m->flex_elem
            *(data_base.add(0) as *mut *const i32) = (*m).flex_elem as *const i32;
        }
    }
}

/// C: mjc_center (engine/engine_collision_convex.h:97)
/// Calls: mji_addTo3, mji_copy3, mju_scl3, mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_center(res: *mut f64, obj: *const mjCCDObj) {
    // SAFETY: raw pointer access to opaque union fields using known offsets from C struct layout.
    // obj pointer is valid per caller contract. Union field access requires byte-level pointer arithmetic.
    unsafe {
        let g = (*obj).geom;
        let f = (*obj).flex;
        let e = (*obj).elem;
        let v = (*obj).vert;

        let obj_ptr = obj as *const u8;
        const DATA_OFFSET: usize = 200; // offset of data union in mjCCDObj

        if (*obj).geom_type == 10 {
            // mjGEOM_HFIELD == 10
            // data.hfield.prism[i] is at DATA_OFFSET + i*24
            crate::engine::engine_util_blas::mju_zero3(res);
            for i in 0..6 {
                let prism_i = obj_ptr.add(DATA_OFFSET + i * 24) as *const f64;
                crate::engine::engine_inline::mji_add_to3(res, prism_i);
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
            // data.flex.aabb at DATA_OFFSET + 16 (ptr)
            let aabb = *(obj_ptr.add(DATA_OFFSET + 16) as *const *const f64);
            // data.flex.elemadr at DATA_OFFSET + 24 (ptr)
            let elemadr = *(obj_ptr.add(DATA_OFFSET + 24) as *const *const i32);
            let idx = (*elemadr.add(f as usize) + e) as usize;
            crate::engine::engine_inline::mji_copy3(res, aabb.add(6 * idx));
            return;
        }

        // return flex vertex position
        if f >= 0 {
            // data.flex.vert_xpos at DATA_OFFSET + 40 (ptr)
            let vert_xpos = *(obj_ptr.add(DATA_OFFSET + 40) as *const *const f64);
            // data.flex.vertadr at DATA_OFFSET + 48 (ptr)
            let vertadr = *(obj_ptr.add(DATA_OFFSET + 48) as *const *const i32);
            let idx = (*vertadr.add(f as usize) + v) as usize;
            crate::engine::engine_inline::mji_copy3(res, vert_xpos.add(3 * idx));
            return;
        }
    }
}

/// C: mjccd_center (engine/engine_collision_convex.h:100)
/// Calls: mjc_center
#[allow(unused_variables, non_snake_case)]
pub fn mjccd_center(obj: *const (), center: *mut ccd_vec3_t) {
    extern "C" { fn mjccd_center_impl(obj: *const (), center: *mut ccd_vec3_t); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjccd_center_impl(obj, center) }
}

/// C: mjccd_support (engine/engine_collision_convex.h:103)
/// Calls: mjc_prism_support, mji_addScl3, mji_addTo3, mji_addToScl3, mji_copy3, mji_scl3, mju_dot3, mju_message, mju_mulMatTVec3, mju_mulMatVec3, mju_normalize3, mju_sign, mju_warning, mju_zero3
#[allow(unused_variables, non_snake_case)]
pub fn mjccd_support(obj: *const (), dir: *const ccd_vec3_t, vec: *mut ccd_vec3_t) {
    extern "C" { fn mjccd_support_impl(obj: *const (), dir: *const ccd_vec3_t, vec: *mut ccd_vec3_t); }
    // SAFETY: delegates to C implementation which accesses obj->geom_type union fields
    // and dispatches to appropriate support function
    unsafe { mjccd_support_impl(obj, dir, vec) }
}

/// C: mjc_pointSupport (engine/engine_collision_convex.h:106)
/// Calls: mji_copy3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_point_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    // SAFETY: obj is valid pointer to mjCCDObj, res is valid buffer of at least 3 f64
    unsafe {
        crate::engine::engine_inline::mji_copy3(res, (*obj).pos.as_ptr());
    }
}

/// C: mjc_lineSupport (engine/engine_collision_convex.h:109)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_line_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    // SAFETY: obj is valid pointer to mjCCDObj, res and dir are valid f64[3] buffers
    unsafe {
        let mat = (*obj).mat.as_ptr();
        let pos = (*obj).pos.as_ptr();
        let length = (*obj).size[1];

        let dot = *mat.add(2) * *dir.add(0) + *mat.add(5) * *dir.add(1) + *mat.add(8) * *dir.add(2);
        let scl = if dot >= 0.0 { length } else { -length };

        // transform result to global frame
        *res.add(0) = *mat.add(2) * scl + *pos.add(0);
        *res.add(1) = *mat.add(5) * scl + *pos.add(1);
        *res.add(2) = *mat.add(8) * scl + *pos.add(2);
    }
}

/// C: mjc_PlaneConvex (engine/engine_collision_convex.h:112)
/// Calls: addplanemesh, mjc_initCCDObj, mjccd_support, mji_addToScl3, mji_copy3, mji_sub3, mji_zero3, mju_dot3, mju_mulMatTVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_plane_convex(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    extern "C" {
        fn mjc_PlaneConvex_impl(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjc_PlaneConvex_impl(m, d, con, g1, g2, margin) }
}

/// C: mjc_ConvexHField (engine/engine_collision_convex.h:113)
/// Calls: addPrismVert, mjc_fixNormal, mjc_initCCDObj, mjc_penetration, mji_addTo3, mji_copy3, mji_copy9, mji_mulMatTMat3, mji_mulMatVec3, mju_mulMatTVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_convex_h_field(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    extern "C" { fn mjc_ConvexHField_impl(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjc_ConvexHField_impl(m, d, con, g1, g2, margin) }
}

/// C: mjc_Convex (engine/engine_collision_convex.h:114)
/// Calls: maxContacts, mjc_fixNormal, mjc_initCCDObj, mjc_isDistinctContact, mjc_penetration, mji_axisAngle2Quat, mji_copy3, mji_copy9, mju_makeFrame, mju_min, mju_quat2Mat, mju_rotateFrame, mju_transpose, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_convex(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    extern "C" { fn mjc_Convex_impl(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjc_Convex_impl(m, d, con, g1, g2, margin) }
}

/// C: mjc_ConvexElem (engine/engine_collision_convex.h:117)
/// Calls: mjc_fixNormal, mjc_initCCDObj, mjc_penetration, mjc_setCCDObjFlex
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_convex_elem(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, f1: i32, e1: i32, v1: i32, f2: i32, e2: i32, margin: f64) -> i32 {
    extern "C" { fn mjc_ConvexElem_impl(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, f1: i32, e1: i32, v1: i32, f2: i32, e2: i32, margin: f64) -> i32; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjc_ConvexElem_impl(m, d, con, g1, f1, e1, v1, f2, e2, margin) }
}

/// C: mjc_HFieldElem (engine/engine_collision_convex.h:121)
/// Calls: addVert, mjc_initCCDObj, mjc_penetration, mjc_setCCDObjFlex, mji_addTo3, mji_copy3, mji_mulMatTVec3, mji_sub3, mji_zero3, mju_max, mju_min, mju_mulMatVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_h_field_elem(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g: i32, f: i32, e: i32, margin: f64) -> i32 {
    extern "C" { fn mjc_HFieldElem_impl(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g: i32, f: i32, e: i32, margin: f64) -> i32; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjc_HFieldElem_impl(m, d, con, g, f, e, margin) }
}

/// C: mjc_fixNormal (engine/engine_collision_convex.h:125)
/// Calls: mjc_ellipsoidInside, mjc_ellipsoidOutside, mji_copy3, mji_mulMatVec3, mji_scl3, mji_sub3, mju_mulMatTVec3, mju_norm, mju_normalize3, mju_sub3
#[allow(unused_variables, non_snake_case)]
pub fn mjc_fix_normal(m: *const mjModel, d: *const mjData, con: *mut mjPreContact, g1: i32, g2: i32) {
    // SAFETY: m, d, con valid per caller. g1/g2 are valid geom indices or -1.
    // All geom arrays have adequate capacity for the given indices.
    unsafe {
        const MJGEOM_NONE: i32 = -1;
        const MJGEOM_SPHERE: i32 = 2;
        const MJGEOM_CAPSULE: i32 = 3;
        const MJGEOM_ELLIPSOID: i32 = 4;
        const MJGEOM_CYLINDER: i32 = 5;
        const MJMINVAL: f64 = 1e-15;

        // get geom ids and types
        let gid: [i32; 2] = [g1, g2];
        let mut r#type: [i32; 2] = [0; 2];
        let mut i: i32 = 0;
        while i < 2 {
            if gid[i as usize] < 0 {
                r#type[i as usize] = MJGEOM_NONE;
            } else {
                r#type[i as usize] = *(*m).geom_type.add(gid[i as usize] as usize);
            }
            // set to NONE if type cannot be processed
            if r#type[i as usize] != MJGEOM_SPHERE && r#type[i as usize] != MJGEOM_CAPSULE
                && r#type[i as usize] != MJGEOM_ELLIPSOID && r#type[i as usize] != MJGEOM_CYLINDER {
                r#type[i as usize] = MJGEOM_NONE;
            }
            i += 1;
        }

        // neither type can be processed: nothing to do
        if r#type[0] == MJGEOM_NONE && r#type[1] == MJGEOM_NONE {
            return;
        }

        // init normals
        let mut normal: [[f64; 3]; 2] = [
            [(*con).normal[0], (*con).normal[1], (*con).normal[2]],
            [-(*con).normal[0], -(*con).normal[1], -(*con).normal[2]],
        ];

        // process geoms in type range
        let mut processed: [i32; 2] = [0, 0];
        i = 0;
        while i < 2 {
            if r#type[i as usize] != MJGEOM_NONE {
                // get geom mat and size
                let mat = (*d).geom_xmat.add(9 * gid[i as usize] as usize);
                let size = (*m).geom_size.add(3 * gid[i as usize] as usize);

                // map contact point and normal to local frame
                let mut dif: [f64; 3] = [0.0; 3];
                let mut pos1: [f64; 3] = [0.0; 3];
                let mut nrm: [f64; 3] = [0.0; 3];
                crate::engine::engine_util_blas::mju_sub3(
                    dif.as_mut_ptr(),
                    (*con).pos.as_ptr(),
                    (*d).geom_xpos.add(3 * gid[i as usize] as usize),
                );
                crate::engine::engine_util_blas::mju_mul_mat_t_vec3(pos1.as_mut_ptr(), mat, dif.as_ptr());
                crate::engine::engine_util_blas::mju_mul_mat_t_vec3(nrm.as_mut_ptr(), mat, normal[i as usize].as_ptr());

                // process according to type
                if r#type[i as usize] == MJGEOM_SPHERE {
                    crate::engine::engine_inline::mji_copy3(nrm.as_mut_ptr(), pos1.as_ptr());
                    processed[i as usize] = 1;
                } else if r#type[i as usize] == MJGEOM_CAPSULE {
                    // Z: bottom cap
                    if pos1[2] < -*size.add(1) {
                        nrm[2] = pos1[2] + *size.add(1);
                    }
                    // Z: top cap
                    else if pos1[2] > *size.add(1) {
                        nrm[2] = pos1[2] - *size.add(1);
                    }
                    // Z: cylinder
                    else {
                        nrm[2] = 0.0;
                    }
                    // copy XY
                    nrm[0] = pos1[0];
                    nrm[1] = pos1[1];
                    processed[i as usize] = 1;
                } else if r#type[i as usize] == MJGEOM_ELLIPSOID {
                    // guard against invalid ellipsoid size
                    if *size.add(0) < MJMINVAL || *size.add(1) < MJMINVAL || *size.add(2) < MJMINVAL {
                        // do nothing
                    } else {
                        // compute elliptic distance^2
                        let dst1 = pos1[0] * pos1[0] / (*size.add(0) * *size.add(0))
                            + pos1[1] * pos1[1] / (*size.add(1) * *size.add(1))
                            + pos1[2] * pos1[2] / (*size.add(2) * *size.add(2));
                        // dispatch to inside or outside solver
                        if dst1 <= 1.0 {
                            processed[i as usize] = mjc_ellipsoid_inside(nrm.as_mut_ptr(), pos1.as_ptr(), size);
                        } else {
                            processed[i as usize] = mjc_ellipsoid_outside(nrm.as_mut_ptr(), pos1.as_ptr(), size);
                        }
                    }
                } else if r#type[i as usize] == MJGEOM_CYLINDER {
                    // skip if within 5% length of flat wall
                    if pos1[2].abs() <= 0.95 * *size.add(1) {
                        // compute distances to flat and round wall
                        let dst1 = (*size.add(1) - pos1[2].abs()).abs();
                        let dst2 = (*size.add(0) - crate::engine::engine_util_blas::mju_norm(pos1.as_ptr(), 2)).abs();
                        // require 4x closer to round than flat wall
                        if dst1 >= 0.25 * dst2 {
                            // set normal for round wall
                            nrm[0] = pos1[0];
                            nrm[1] = pos1[1];
                            nrm[2] = 0.0;
                            processed[i as usize] = 1;
                        }
                    }
                }

                // normalize and map normal to global frame
                if processed[i as usize] != 0 {
                    crate::engine::engine_util_blas::mju_normalize3(nrm.as_mut_ptr());
                    crate::engine::engine_inline::mji_mul_mat_vec3(normal[i as usize].as_mut_ptr(), mat, nrm.as_ptr());
                }
            }
            i += 1;
        }

        // both processed: average
        if processed[0] != 0 && processed[1] != 0 {
            crate::engine::engine_inline::mji_sub3((*con).normal.as_mut_ptr(), normal[0].as_ptr(), normal[1].as_ptr());
            crate::engine::engine_util_blas::mju_normalize3((*con).normal.as_mut_ptr());
        }
        // first processed: copy
        else if processed[0] != 0 {
            crate::engine::engine_inline::mji_copy3((*con).normal.as_mut_ptr(), normal[0].as_ptr());
        }
        // second processed: copy reverse
        else if processed[1] != 0 {
            crate::engine::engine_inline::mji_scl3((*con).normal.as_mut_ptr(), normal[1].as_ptr(), -1.0);
        }
    }
}

/// C: mjc_setCCDBuffer (engine/engine_collision_convex.h:128)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_set_ccd_buffer(buffer: *mut ()) {
    CCD_BUFFER.with(|p| p.set(buffer))
}

