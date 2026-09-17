//! Port of: engine/engine_collision_sdf.c
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: boxProjection (engine/engine_collision_sdf.c:35)
/// Calls: cxx:_mju_max
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn box_projection(point: *mut f64, r#box: *const f64) -> f64 {
    todo!() // cxx:_boxProjection
}

/// C: findOct (engine/engine_collision_sdf.c:69)
/// Calls: cxx:_mju_error
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn find_oct(w: *mut f64, dw: *mut [f64; 3], oct_aabb: *const f64, oct_child: *const i32, p: *const f64) -> i32 {
    todo!() // cxx-internal:engine_collision_sdf.c.o:_findOct
}

/// C: oct_distance (engine/engine_collision_sdf.c:138)
/// Calls: cxx-internal:engine_collision_sdf.c.o:_findOct, cxx:_boxProjection, cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn oct_distance(m: *const mjModel, p: *const f64, meshid: i32) -> f64 {
    // SAFETY: m is valid mjModel, p is valid f64[3] (caller contract)
    unsafe {
        let octadr = *(*m).mesh_octadr.add(meshid as usize);
        let oct_child = (*m).oct_child.add(8 * octadr as usize);
        let oct_aabb = (*m).oct_aabb.add(6 * octadr as usize);
        let oct_coeff = (*m).oct_coeff.add(8 * octadr as usize);

        if octadr == -1 {
            crate::engine::engine_util_errmem::mju_error(
                b"Octree not found in mesh %d\0".as_ptr() as *const i8);
            return 0.0;
        }

        let mut w: [f64; 8] = [0.0; 8];
        let mut point: [f64; 3] = [*p.add(0), *p.add(1), *p.add(2)];
        let box_dist = boxProjection(point.as_mut_ptr(), oct_aabb);
        let node = findOct(w.as_mut_ptr(), std::ptr::null_mut(), oct_aabb, oct_child, point.as_ptr());

        let mut sdf: f64 = 0.0;
        for i in 0..8 {
            sdf += w[i] * *oct_coeff.add(8 * node as usize + i);
        }

        if box_dist > 0.0 { sdf + box_dist } else { sdf }
    }
}

/// C: oct_gradient (engine/engine_collision_sdf.c:162)
/// Calls: cxx-internal:engine_collision_sdf.c.o:_findOct, cxx:_boxProjection, cxx:_mju_message, cxx:_mju_zero3, cxx:_oct_distance
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn oct_gradient(m: *const mjModel, grad: *mut f64, point: *const f64, meshid: i32) {
    // SAFETY: m is valid, grad is f64[3], point is f64[3] (caller contract)
    unsafe {
        crate::engine::engine_util_blas::mju_zero3(grad);
        let mut p: [f64; 3] = [*point.add(0), *point.add(1), *point.add(2)];

        let octadr = *(*m).mesh_octadr.add(meshid as usize);
        let oct_child = (*m).oct_child.add(8 * octadr as usize);
        let oct_aabb = (*m).oct_aabb.add(6 * octadr as usize);
        let oct_coeff = (*m).oct_coeff.add(8 * octadr as usize);

        if octadr == -1 {
            crate::engine::engine_util_errmem::mju_error(
                b"Octree not found in mesh %d\0".as_ptr() as *const i8);
        }

        // analytic in the interior
        if boxProjection(p.as_mut_ptr(), oct_aabb) <= 0.0 {
            let mut dw: [[f64; 3]; 8] = [[0.0; 3]; 8];
            let node = findOct(std::ptr::null_mut(), dw.as_mut_ptr(), oct_aabb, oct_child, p.as_ptr());
            for j in 0..8 {
                *grad.add(0) += dw[j][0] * *oct_coeff.add(8 * node as usize + j);
                *grad.add(1) += dw[j][1] * *oct_coeff.add(8 * node as usize + j);
                *grad.add(2) += dw[j][2] * *oct_coeff.add(8 * node as usize + j);
            }
            return;
        }

        // finite difference in the exterior
        let eps: f64 = 1e-8;
        let dist0 = oct_distance(m, point, meshid);
        let pointX: [f64; 3] = [*point.add(0) + eps, *point.add(1), *point.add(2)];
        let distX = oct_distance(m, pointX.as_ptr(), meshid);
        let pointY: [f64; 3] = [*point.add(0), *point.add(1) + eps, *point.add(2)];
        let distY = oct_distance(m, pointY.as_ptr(), meshid);
        let pointZ: [f64; 3] = [*point.add(0), *point.add(1), *point.add(2) + eps];
        let distZ = oct_distance(m, pointZ.as_ptr(), meshid);

        *grad.add(0) = (distX - dist0) / eps;
        *grad.add(1) = (distY - dist0) / eps;
        *grad.add(2) = (distZ - dist0) / eps;
    }
}

/// C: radialField3d (engine/engine_collision_sdf.c:205)
/// Calls: cxx:_mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn radial_field3d(field: *mut f64, a: *const f64, x: *const f64, size: *const f64) {
    todo!() // cxx-internal:engine_collision_sdf.c.o:_radialField3d
}

/// C: geomDistance (engine/engine_collision_sdf.c:218)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_sdf.c:_radialField3d, cxx:_mju_clip, cxx:_mju_max, cxx:_mju_message, cxx:_mju_min, cxx:_mju_norm, cxx:_mju_norm3, cxx:_oct_distance
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn geom_distance(m: *const mjModel, d: *const mjData, p: *const mjpPlugin, i: i32, x: *const f64, r#type: u32) -> f64 {
    todo!() // cxx-internal:engine_collision_sdf.c.o:_geomDistance
}

/// C: geomGradient (engine/engine_collision_sdf.c:295)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_sdf.c:_radialField3d, cxx:_mju_clip, cxx:_mju_copy3, cxx:_mju_max, cxx:_mju_message, cxx:_mju_norm, cxx:_mju_norm3, cxx:_mju_normalize3, cxx:_mju_zero3, cxx:_oct_gradient
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn geom_gradient(gradient: *mut f64, m: *const mjModel, d: *const mjData, p: *const mjpPlugin, i: i32, x: *const f64, r#type: u32) {
    todo!() // cxx-internal:engine_collision_sdf.c.o:_geomGradient
}

/// C: mjc_distance (engine/engine_collision_sdf.c:417)
/// Calls: cxx-internal:engine_collision_sdf.c.o:_geomDistance, cxx:_mju_addTo3, cxx:_mju_max, cxx:_mju_message, cxx:_mju_mulMatVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_distance(m: *const mjModel, d: *const mjData, s: *const mjSDF, x: *const f64) -> f64 {
    // mjSDFTYPE: SINGLE=0, INTERSECTION=1, MIDSURFACE=2, COLLISION=3
    // mjSDF.type is stored as [u8; 8] — first 4 bytes are the i32 enum value
    // SAFETY: s is a valid mjSDF pointer, x is f64[3] (caller contract)
    unsafe {
        let mut y: [f64; 3] = [0.0; 3];
        let sdf_type = *(std::ptr::addr_of!((*s).r#type) as *const i32);

        match sdf_type {
            0 => {  // SINGLE
                geomDistance(m, d, *(*s).plugin.add(0), *(*s).id.add(0), x, *(*s).geomtype.add(0))
            }

            1 => {  // INTERSECTION
                crate::engine::engine_util_blas::mju_mulMatVec3(y.as_mut_ptr(), (*s).relmat, x);
                crate::engine::engine_util_blas::mju_addTo3(y.as_mut_ptr(), (*s).relpos);
                let d0 = geomDistance(m, d, *(*s).plugin.add(0), *(*s).id.add(0), x, *(*s).geomtype.add(0));
                let d1 = geomDistance(m, d, *(*s).plugin.add(1), *(*s).id.add(1), y.as_ptr(), *(*s).geomtype.add(1));
                if d0 > d1 { d0 } else { d1 }
            }

            2 => {  // MIDSURFACE
                crate::engine::engine_util_blas::mju_mulMatVec3(y.as_mut_ptr(), (*s).relmat, x);
                crate::engine::engine_util_blas::mju_addTo3(y.as_mut_ptr(), (*s).relpos);
                geomDistance(m, d, *(*s).plugin.add(0), *(*s).id.add(0), x, *(*s).geomtype.add(0)) -
                geomDistance(m, d, *(*s).plugin.add(1), *(*s).id.add(1), y.as_ptr(), *(*s).geomtype.add(1))
            }

            3 => {  // COLLISION
                crate::engine::engine_util_blas::mju_mulMatVec3(y.as_mut_ptr(), (*s).relmat, x);
                crate::engine::engine_util_blas::mju_addTo3(y.as_mut_ptr(), (*s).relpos);
                let a = geomDistance(m, d, *(*s).plugin.add(0), *(*s).id.add(0), x, *(*s).geomtype.add(0));
                let b = geomDistance(m, d, *(*s).plugin.add(1), *(*s).id.add(1), y.as_ptr(), *(*s).geomtype.add(1));
                let max_ab = if a > b { a } else { b };
                a + b + max_ab.abs()
            }

            _ => {
                crate::engine::engine_util_errmem::mju_error(
                    b"SDF type not available\0".as_ptr() as *const i8);
                0.0
            }
        }
    }
}

/// C: mjc_gradient (engine/engine_collision_sdf.c:452)
/// Calls: cxx-internal:engine_collision_sdf.c.o:_geomDistance, cxx-internal:engine_collision_sdf.c.o:_geomGradient, cxx:_mju_addTo3, cxx:_mju_addToScl3, cxx:_mju_max, cxx:_mju_message, cxx:_mju_mulMatTVec3, cxx:_mju_mulMatVec3, cxx:_mju_normalize3, cxx:_mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_gradient(m: *const mjModel, d: *const mjData, s: *const mjSDF, gradient: *mut f64, x: *const f64) {
    // SAFETY: s is valid mjSDF, gradient is f64[3], x is f64[3] (caller contract)
    unsafe {
        let mut y: [f64; 3] = [0.0; 3];
        let mut grad1: [f64; 3] = [0.0; 3];
        let mut grad2: [f64; 3] = [0.0; 3];
        let sdf_type = *(std::ptr::addr_of!((*s).r#type) as *const i32);

        match sdf_type {
            1 => {  // INTERSECTION
                crate::engine::engine_util_blas::mju_mulMatVec3(y.as_mut_ptr(), (*s).relmat, x);
                crate::engine::engine_util_blas::mju_addTo3(y.as_mut_ptr(), (*s).relpos);
                let d0 = geomDistance(m, d, *(*s).plugin.add(0), *(*s).id.add(0), x, *(*s).geomtype.add(0));
                let d1 = geomDistance(m, d, *(*s).plugin.add(1), *(*s).id.add(1), y.as_ptr(), *(*s).geomtype.add(1));
                let i = if d0 > d1 { 0 } else { 1 };
                let point_i = if i == 0 { x } else { y.as_ptr() };
                geomGradient(gradient, m, d, *(*s).plugin.add(i), *(*s).id.add(i), point_i, *(*s).geomtype.add(i));
                if i == 1 {
                    crate::engine::engine_util_blas::mju_mulMatTVec3(gradient, (*s).relmat, gradient as *const f64);
                }
            }

            2 => {  // MIDSURFACE
                crate::engine::engine_util_blas::mju_mulMatVec3(y.as_mut_ptr(), (*s).relmat, x);
                crate::engine::engine_util_blas::mju_addTo3(y.as_mut_ptr(), (*s).relpos);
                geomGradient(grad1.as_mut_ptr(), m, d, *(*s).plugin.add(0), *(*s).id.add(0), x, *(*s).geomtype.add(0));
                crate::engine::engine_util_blas::mju_normalize3(grad1.as_mut_ptr());
                geomGradient(grad2.as_mut_ptr(), m, d, *(*s).plugin.add(1), *(*s).id.add(1), y.as_ptr(), *(*s).geomtype.add(1));
                crate::engine::engine_util_blas::mju_mulMatTVec3(grad2.as_mut_ptr(), (*s).relmat, grad2.as_ptr());
                crate::engine::engine_util_blas::mju_normalize3(grad2.as_mut_ptr());
                crate::engine::engine_util_blas::mju_sub3(gradient, grad1.as_ptr(), grad2.as_ptr());
                crate::engine::engine_util_blas::mju_normalize3(gradient);
            }

            3 => {  // COLLISION
                crate::engine::engine_util_blas::mju_mulMatVec3(y.as_mut_ptr(), (*s).relmat, x);
                crate::engine::engine_util_blas::mju_addTo3(y.as_mut_ptr(), (*s).relpos);
                let a = geomDistance(m, d, *(*s).plugin.add(0), *(*s).id.add(0), x, *(*s).geomtype.add(0));
                let b = geomDistance(m, d, *(*s).plugin.add(1), *(*s).id.add(1), y.as_ptr(), *(*s).geomtype.add(1));
                geomGradient(grad1.as_mut_ptr(), m, d, *(*s).plugin.add(0), *(*s).id.add(0), x, *(*s).geomtype.add(0));
                geomGradient(grad2.as_mut_ptr(), m, d, *(*s).plugin.add(1), *(*s).id.add(1), y.as_ptr(), *(*s).geomtype.add(1));
                crate::engine::engine_util_blas::mju_mulMatTVec3(grad2.as_mut_ptr(), (*s).relmat, grad2.as_ptr());
                *gradient.add(0) = grad1[0] + grad2[0];
                *gradient.add(1) = grad1[1] + grad2[1];
                *gradient.add(2) = grad1[2] + grad2[2];
                let max_ab = if a > b { a } else { b };
                let scl = if max_ab > 0.0 { 1.0 } else { -1.0 };
                let extra = if a > b { grad1.as_ptr() } else { grad2.as_ptr() };
                crate::engine::engine_util_blas::mju_addToScl3(gradient, extra, scl);
            }

            0 => {  // SINGLE
                geomGradient(gradient, m, d, *(*s).plugin.add(0), *(*s).id.add(0), x, *(*s).geomtype.add(0));
            }

            _ => {
                crate::engine::engine_util_errmem::mju_error(
                    b"SDF type not available\0".as_ptr() as *const i8);
            }
        }
    }
}

/// C: mjc_getSDF (engine/engine_collision_sdf.c:505)
/// Calls: cxx:_mjp_getPluginAtSlotUnsafe, cxx:_mjp_pluginCount, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mjc_get_sdf(m: *const mjModel, id: i32) -> *const mjpPlugin {
    todo!() // cxx:_mjc_getSDF
}

/// C: mapPose (engine/engine_collision_sdf.c:519)
/// Calls: cxx:_mju_mulPose, cxx:_mju_negPose, cxx:_mju_quat2Mat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn map_pose(xpos1: *const f64, xquat1: *const f64, xpos2: *const f64, xquat2: *const f64, pos12: *mut f64, mat12: *mut f64) {
    todo!() // cxx-internal:engine_collision_sdf.c.o:_mapPose
}

/// C: isknown (engine/engine_collision_sdf.c:532)
/// Calls: cxx:_mju_dist3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn isknown(points: *const f64, x: *const f64, cnt: i32) -> i32 {
    const MJ_MINVAL: f64 = 1E-15_f64;
    for i in 0..cnt {
        // SAFETY: points has at least cnt*3 f64, x has 3 f64 (caller contract)
        unsafe {
            if crate::engine::engine_util_blas::mju_dist3(x, points.add((3 * i) as usize)) < MJ_MINVAL {
                return 1;
            }
        }
    }
    0
}

/// C: addPreContact (engine/engine_collision_sdf.c:545)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_sdf.c:_isknown, cxx:_mjc_gradient, cxx:_mju_addTo3, cxx:_mju_copy3, cxx:_mju_normalize3, cxx:_mju_rotVecQuat, cxx:_mju_scl3, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_pre_contact(points: *mut f64, con: *mut mjPreContact, x: *const f64, pos2: *const f64, quat2: *const f64, dist: f64, cnt: i32, m: *const mjModel, s: *const mjSDF, d: *const mjData, flipNormal: i32) -> i32 {
    todo!() // cxx-internal:engine_collision_sdf.c.o:_addPreContact
}

/// C: stepFrankWolfe (engine/engine_collision_sdf.c:585)
/// Calls: cxx:_mjc_distance, cxx:_mjc_gradient, cxx:_mju_addToScl3, cxx:_mju_copy3, cxx:_mju_dot3, cxx:_mju_subFrom3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn step_frank_wolfe(x: *mut f64, corners: *const f64, ncorners: i32, m: *const mjModel, sdf: *const mjSDF, d: *const mjData) -> f64 {
    todo!() // cxx-internal:engine_collision_sdf.c.o:_stepFrankWolfe
}

/// C: stepGradient (engine/engine_collision_sdf.c:615)
/// Calls: cxx:_mjc_distance, cxx:_mjc_gradient, cxx:_mju_addScl3, cxx:_mju_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn step_gradient(x: *mut f64, m: *const mjModel, s: *const mjSDF, d: *const mjData, niter: i32) -> f64 {
    todo!() // cxx-internal:engine_collision_sdf.c.o:_stepGradient
}

/// C: triangleIntersect (engine/engine_collision_sdf.c:665)
/// Calls: cxx:_mjc_distance, cxx:_mju_addTo3, cxx:_mju_addToScl3, cxx:_mju_cross, cxx:_mju_dist3, cxx:_mju_dot3, cxx:_mju_max, cxx:_mju_norm3, cxx:_mju_normalize3, cxx:_mju_scl3, cxx:_mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn triangle_intersect(triangle: *const f64, m: *const mjModel, sdf: *const mjSDF, d: *const mjData) -> i32 {
    todo!() // cxx-internal:engine_collision_sdf.c.o:_triangleIntersect
}

/// C: boxIntersect (engine/engine_collision_sdf.c:737)
/// Calls: cxx:_mjc_distance, cxx:_mju_addTo3, cxx:_mju_mulMatVec3, cxx:_mju_norm3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn box_intersect(bvh: *const f64, offset: *const f64, rotation: *const f64, m: *const mjModel, s: *const mjSDF, d: *const mjData) -> i32 {
    todo!() // cxx-internal:engine_collision_sdf.c.o:_boxIntersect
}

/// C: selectFPS (engine/engine_collision_sdf.c:752)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn select_fps(candidate: *const f64, dist: *const f64, ncandidate: i32, selected_indices: *mut i32, max_select: i32) -> i32 {
    todo!() // cxx-internal:engine_collision_sdf.c.o:_selectFPS
}

/// C: processSdfCorners (engine/engine_collision_sdf.c:808)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_sdf.c:_stepFrankWolfe, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_sdf.c:_triangleIntersect, cxx:_mju_Halton, cxx:_mju_copy3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn process_sdf_corners(corners: *const f64, m: *const mjModel, d: *const mjData, sdf: *const mjSDF, nstartpts: i32, candidate: *mut f64, dist: *mut f64, ncandidate: *mut i32) {
    todo!() // cxx-internal:engine_collision_sdf.c.o:_processSdfCorners
}

/// C: processOneFace (engine/engine_collision_sdf.c:866)
/// Calls: cxx-internal:engine_collision_sdf.c.o:_processSdfCorners, cxx:_mju_addTo3, cxx:_mju_mulMatVec3
#[allow(unused_variables, non_snake_case)]
pub fn process_one_face(faceid: i32, bvh_active: *mut bool, node: i32, ctx: *mut MeshSDFContext) {
    todo!() // cxx-internal:engine_collision_sdf.c.o:_processOneFace
}

/// C: traverseBVH (engine/engine_collision_sdf.c:903)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_sdf.c:_boxIntersect, cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn traverse_bvh(bvh: *const f64, nodeid: *const i32, child: *const i32, bvh_active: *mut bool, offset: *const f64, rotation: *const f64, m: *const mjModel, d: *const mjData, sdf: *const mjSDF, callback: BVHLeafCallback, ctx: *mut ()) {
    todo!() // cxx-internal:engine_collision_sdf.c.o:_traverseBVH
}

/// C: meshFaceCallback (engine/engine_collision_sdf.c:943)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_sdf.c:_processOneFace
#[allow(unused_variables, non_snake_case)]
pub fn mesh_face_callback(face_id: i32, node: i32, ctx: *mut ()) -> i32 {
    todo!() // cxx-internal:engine_collision_sdf.c.o:_meshFaceCallback
}

/// C: mjc_HFieldSDF (engine/engine_collision_sdf.c:954)
/// Calls: cxx:_mju_warning
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_h_field_sdf(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    todo!() // cxx:_mjc_HFieldSDF
}

/// C: mjc_MeshSDF (engine/engine_collision_sdf.c:961)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_sdf.c:_mapPose, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_sdf.c:_selectFPS, cxx-internal:engine_collision_sdf.c.o:_addPreContact, cxx-internal:engine_collision_sdf.c.o:_traverseBVH, cxx:_mjc_getSDF, cxx:_mju_mat2Quat, cxx:_mju_max
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_mesh_sdf(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    todo!() // cxx:_mjc_MeshSDF
}

/// C: mjc_SDF (engine/engine_collision_sdf.c:1040)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_sdf.c:_mapPose, cxx-internal:engine_collision_sdf.c.o:_addPreContact, cxx-internal:engine_collision_sdf.c.o:_stepGradient, cxx:_mjc_getSDF, cxx:_mju_Halton, cxx:_mju_addTo3, cxx:_mju_mat2Quat, cxx:_mju_max, cxx:_mju_message, cxx:_mju_min, cxx:_mju_mulMatVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_sdf(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    todo!() // cxx:_mjc_SDF
}

/// C: flexElemCallback (engine/engine_collision_sdf.c:1198)
/// Calls: cxx-internal:engine_collision_sdf.c.o:_processSdfCorners, cxx:_mju_addTo3, cxx:_mju_copy3, cxx:_mju_mulMatVec3
#[allow(unused_variables, non_snake_case)]
pub fn flex_elem_callback(elem_idx: i32, node: i32, ctx: *mut ()) -> i32 {
    todo!() // cxx-internal:engine_collision_sdf.c.o:_flexElemCallback
}

/// C: mjc_FlexSDF (engine/engine_collision_sdf.c:1249)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_sdf.c:_mapPose, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_sdf.c:_selectFPS, cxx-internal:engine_collision_sdf.c.o:_addPreContact, cxx-internal:engine_collision_sdf.c.o:_processSdfCorners, cxx-internal:engine_collision_sdf.c.o:_traverseBVH, cxx:_mjc_getSDF, cxx:_mju_addTo3, cxx:_mju_copy3, cxx:_mju_mat2Quat, cxx:_mju_max, cxx:_mju_mulMatVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_flex_sdf(m: *const mjModel, d: *const mjData, con: *mut mjPreContact, elem: *mut i32, g: i32, f: i32, margin: f64) -> i32 {
    todo!() // cxx:_mjc_FlexSDF
}

