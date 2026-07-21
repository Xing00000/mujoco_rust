//! Port of: engine/engine_collision_sdf.c
//! IR hash: 73a9f665ec0ecfc0
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: boxProjection (engine/engine_collision_sdf.c:35)
/// Calls: mju_max
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn box_projection(point: *mut f64, r#box: *const f64) -> f64 {
    // SAFETY: point is a mutable array of 3 f64, box is an array of 6 f64 (caller contract)
    unsafe {
        let r: [f64; 3] = [
            *point.add(0) - *r#box.add(0),
            *point.add(1) - *r#box.add(1),
            *point.add(2) - *r#box.add(2),
        ];
        let q: [f64; 3] = [
            f64::abs(r[0]) - *r#box.add(3),
            f64::abs(r[1]) - *r#box.add(4),
            f64::abs(r[2]) - *r#box.add(5),
        ];
        let mut dist_sqr: f64 = 0.0;
        let eps: f64 = 1e-6;

        if q[0] <= 0.0 && q[1] <= 0.0 && q[2] <= 0.0 {
            let max_q12 = if q[1] > q[2] { q[1] } else { q[2] };
            return if q[0] > max_q12 { q[0] } else { max_q12 };
        }

        if q[0] >= 0.0 {
            dist_sqr += q[0] * q[0];
            *point.add(0) -= if r[0] > 0.0 { q[0] + eps } else { -(q[0] + eps) };
        }
        if q[1] >= 0.0 {
            dist_sqr += q[1] * q[1];
            *point.add(1) -= if r[1] > 0.0 { q[1] + eps } else { -(q[1] + eps) };
        }
        if q[2] >= 0.0 {
            dist_sqr += q[2] * q[2];
            *point.add(2) -= if r[2] > 0.0 { q[2] + eps } else { -(q[2] + eps) };
        }

        f64::sqrt(dist_sqr)
    }
}

/// C: findOct (engine/engine_collision_sdf.c:69)
/// Calls: mju_error
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn find_oct(w: *mut f64, dw: *mut [f64; 3], oct_aabb: *const f64, oct_child: *const i32, p: *const f64) -> i32 {
    // SAFETY: oct_aabb, oct_child are valid arrays from mjModel.mesh_octaabb/octchild,
    //         p is a valid f64[3] pointer, w and dw may be null (optional outputs)
    unsafe {
        let mut stack: i32 = 0;
        let eps: f64 = 1e-8;
        let mut niter: i32 = 100;

        while niter > 0 {
            niter -= 1;
            let node = stack;

            if node == -1 {
                crate::engine::engine_util_errmem::mju_error(
                    b"Invalid node number\0".as_ptr() as *const i8);
                return -1;
            }

            let mut vmin: [f64; 3] = [0.0; 3];
            let mut vmax: [f64; 3] = [0.0; 3];
            for j in 0..3 {
                vmin[j] = *oct_aabb.add(6 * node as usize + j) - *oct_aabb.add(6 * node as usize + 3 + j);
                vmax[j] = *oct_aabb.add(6 * node as usize + j) + *oct_aabb.add(6 * node as usize + 3 + j);
            }

            // check if point is inside aabb
            if *p.add(0) + eps < vmin[0] || *p.add(0) - eps > vmax[0] ||
               *p.add(1) + eps < vmin[1] || *p.add(1) - eps > vmax[1] ||
               *p.add(2) + eps < vmin[2] || *p.add(2) - eps > vmax[2] {
                continue;
            }

            let coord: [f64; 3] = [
                (*p.add(0) - vmin[0]) / (vmax[0] - vmin[0]),
                (*p.add(1) - vmin[1]) / (vmax[1] - vmin[1]),
                (*p.add(2) - vmin[2]) / (vmax[2] - vmin[2]),
            ];

            // check if leaf node (all children == -1)
            let mut is_leaf = true;
            for j in 0..8 {
                if *oct_child.add(8 * node as usize + j) != -1 {
                    is_leaf = false;
                    break;
                }
            }

            if is_leaf {
                for j in 0..8_usize {
                    let cx = if j & 1 != 0 { coord[0] } else { 1.0 - coord[0] };
                    let cy = if j & 2 != 0 { coord[1] } else { 1.0 - coord[1] };
                    let cz = if j & 4 != 0 { coord[2] } else { 1.0 - coord[2] };

                    if !w.is_null() {
                        *w.add(j) = cx * cy * cz;
                    }
                    if !dw.is_null() {
                        let dx = if j & 1 != 0 { 1.0 } else { -1.0 };
                        let dy = if j & 2 != 0 { 1.0 } else { -1.0 };
                        let dz = if j & 4 != 0 { 1.0 } else { -1.0 };
                        (*dw.add(j))[0] = dx * cy * cz;
                        (*dw.add(j))[1] = cx * dy * cz;
                        (*dw.add(j))[2] = cx * cy * dz;
                    }
                }
                return node;
            }

            // compute which child to visit
            let x = if coord[0] < 0.5 { 0 } else { 1 };
            let y = if coord[1] < 0.5 { 0 } else { 1 };
            let z = if coord[2] < 0.5 { 0 } else { 1 };
            stack = *oct_child.add(8 * node as usize + 4 * z + 2 * y + x);
        }

        crate::engine::engine_util_errmem::mju_error(
            b"Node not found\0".as_ptr() as *const i8);
        -1
    }
}

/// C: oct_distance (engine/engine_collision_sdf.c:138)
/// Calls: boxProjection, findOct, mju_message
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
        let box_dist = box_projection(point.as_mut_ptr(), oct_aabb);
        let node = find_oct(w.as_mut_ptr(), std::ptr::null_mut(), oct_aabb, oct_child, point.as_ptr());

        let mut sdf: f64 = 0.0;
        for i in 0..8 {
            sdf += w[i] * *oct_coeff.add(8 * node as usize + i);
        }

        if box_dist > 0.0 { sdf + box_dist } else { sdf }
    }
}

/// C: oct_gradient (engine/engine_collision_sdf.c:162)
/// Calls: boxProjection, findOct, mju_message, mju_zero3, oct_distance
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn oct_gradient(m: *const mjModel, grad: *mut f64, point: *const f64, meshid: i32) {
    todo!() // oct_gradient
}

/// C: radialField3d (engine/engine_collision_sdf.c:205)
/// Calls: mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn radial_field3d(field: *mut f64, a: *const f64, x: *const f64, size: *const f64) {
    // SAFETY: caller guarantees field, a, x, size point to at least 3 valid f64 elements
    unsafe {
        *field.add(0) = -*size.add(0) / *a.add(0);
        *field.add(1) = -*size.add(1) / *a.add(1);
        *field.add(2) = -*size.add(2) / *a.add(2);
        crate::engine::engine_util_blas::mju_normalize3(field);

        // flip sign if necessary
        if *x.add(0) < 0.0 {
            *field.add(0) = -*field.add(0);
        }
        if *x.add(1) < 0.0 {
            *field.add(1) = -*field.add(1);
        }
        if *x.add(2) < 0.0 {
            *field.add(2) = -*field.add(2);
        }
    }
}

/// C: geomDistance (engine/engine_collision_sdf.c:218)
/// Calls: mju_clip, mju_max, mju_message, mju_min, mju_norm, mju_norm3, oct_distance, radialField3d
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn geom_distance(m: *const mjModel, d: *const mjData, p: *const mjpPlugin, i: i32, x: *const f64, r#type: u32) -> f64 {
    // SAFETY: m, d, x are valid pointers; i is valid geom index (caller contract)
    unsafe {
        let size = (*m).geom_size.add(3 * i as usize);

        match r#type {
            0 => {  // mjGEOM_PLANE
                *x.add(2)
            }

            2 => {  // mjGEOM_SPHERE
                crate::engine::engine_util_blas::mju_norm3(x) - *size.add(0)
            }

            6 => {  // mjGEOM_BOX
                let mut a: [f64; 3] = [0.0; 3];
                let mut b: [f64; 3] = [0.0; 3];
                a[0] = (*x.add(0)).abs() - *size.add(0);
                a[1] = (*x.add(1)).abs() - *size.add(1);
                a[2] = (*x.add(2)).abs() - *size.add(2);
                if a[0] >= 0.0 || a[1] >= 0.0 || a[2] >= 0.0 {
                    b[0] = if a[0] > 0.0 { a[0] } else { 0.0 };
                    b[1] = if a[1] > 0.0 { a[1] } else { 0.0 };
                    b[2] = if a[2] > 0.0 { a[2] } else { 0.0 };
                    let max_a = if a[0] > a[1] { if a[0] > a[2] { a[0] } else { a[2] } } else { if a[1] > a[2] { a[1] } else { a[2] } };
                    let min_part = if max_a < 0.0 { max_a } else { 0.0 };
                    return crate::engine::engine_util_blas::mju_norm3(b.as_ptr()) + min_part;
                }
                radial_field3d(b.as_mut_ptr(), a.as_ptr(), x, size);
                let mut t: [f64; 3] = [0.0; 3];
                t[0] = -a[0] / b[0].abs();
                t[1] = -a[1] / b[1].abs();
                t[2] = -a[2] / b[2].abs();
                let min_t = if t[0] < t[1] { if t[0] < t[2] { t[0] } else { t[2] } } else { if t[1] < t[2] { t[1] } else { t[2] } };
                -min_t * crate::engine::engine_util_blas::mju_norm3(b.as_ptr())
            }

            3 => {  // mjGEOM_CAPSULE
                let mut a: [f64; 3] = [0.0; 3];
                a[0] = *x.add(0);
                a[1] = *x.add(1);
                let clamped = crate::engine::engine_util_misc::mju_clip(*x.add(2), -*size.add(1), *size.add(1));
                a[2] = *x.add(2) - clamped;
                crate::engine::engine_util_blas::mju_norm3(a.as_ptr()) - *size.add(0)
            }

            4 => {  // mjGEOM_ELLIPSOID
                let mut a: [f64; 3] = [0.0; 3];
                let mut b: [f64; 3] = [0.0; 3];
                a[0] = *x.add(0) / *size.add(0);
                a[1] = *x.add(1) / *size.add(1);
                a[2] = *x.add(2) / *size.add(2);
                b[0] = a[0] / *size.add(0);
                b[1] = a[1] / *size.add(1);
                b[2] = a[2] / *size.add(2);
                let k0 = crate::engine::engine_util_blas::mju_norm3(a.as_ptr());
                let k1 = crate::engine::engine_util_blas::mju_norm3(b.as_ptr());
                k0 * (k0 - 1.0) / k1
            }

            5 => {  // mjGEOM_CYLINDER
                let mut a: [f64; 2] = [0.0; 2];
                let mut b: [f64; 2] = [0.0; 2];
                a[0] = ((*x.add(0)) * (*x.add(0)) + (*x.add(1)) * (*x.add(1))).sqrt() - *size.add(0);
                a[1] = (*x.add(2)).abs() - *size.add(1);
                b[0] = if a[0] > 0.0 { a[0] } else { 0.0 };
                b[1] = if a[1] > 0.0 { a[1] } else { 0.0 };
                let max_a = if a[0] > a[1] { a[0] } else { a[1] };
                let min_part = if max_a < 0.0 { max_a } else { 0.0 };
                min_part + crate::engine::engine_util_blas::mju_norm(b.as_ptr(), 2)
            }

            8 => {  // mjGEOM_SDF
                if !p.is_null() {
                    // SAFETY: p->sdf_distance is a valid function pointer
                    let sdf_dist: unsafe extern "C" fn(*const f64, *const mjData, i32) -> f64 =
                        std::mem::transmute((*p).sdf_distance);
                    sdf_dist(x, d, i)
                } else {
                    oct_distance(m, x, i)
                }
            }

            7 => {  // mjGEOM_MESH
                if *(*m).mesh_octadr.add(i as usize) == -1 {
                    crate::engine::engine_util_errmem::mju_error(
                        b"sdf queries require needsdf=\"true\" on mesh %d\0".as_ptr() as *const i8);
                    return 0.0;
                }
                oct_distance(m, x, i)
            }

            _ => {
                crate::engine::engine_util_errmem::mju_error(
                    b"sdf collisions not available for geom type %d\0".as_ptr() as *const i8);
                0.0
            }
        }
    }
}

/// C: geomGradient (engine/engine_collision_sdf.c:295)
/// Calls: mju_clip, mju_copy3, mju_max, mju_message, mju_norm, mju_norm3, mju_normalize3, mju_zero3, oct_gradient, radialField3d
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn geom_gradient(gradient: *mut f64, m: *const mjModel, d: *const mjData, p: *const mjpPlugin, i: i32, x: *const f64, r#type: u32) {
    todo!() // geomGradient
}

/// C: mapPose (engine/engine_collision_sdf.c:519)
/// Calls: mju_mulPose, mju_negPose, mju_quat2Mat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn map_pose(xpos1: *const f64, xquat1: *const f64, xpos2: *const f64, xquat2: *const f64, pos12: *mut f64, mat12: *mut f64) {
    use crate::engine::engine_util_spatial::{mju_neg_pose, mju_mul_pose, mju_quat2mat};

    // SAFETY: all pointers are valid arrays of appropriate size (caller contract)
    unsafe {
        let mut negpos: [f64; 3] = [0.0; 3];
        let mut negquat: [f64; 4] = [0.0; 4];
        let mut quat12: [f64; 4] = [0.0; 4];

        mju_neg_pose(negpos.as_mut_ptr(), negquat.as_mut_ptr(), xpos2, xquat2);
        mju_mul_pose(pos12, quat12.as_mut_ptr(), negpos.as_ptr(), negquat.as_ptr(), xpos1, xquat1);
        mju_quat2mat(mat12, quat12.as_ptr());
    }
}

/// C: isknown (engine/engine_collision_sdf.c:532)
/// Calls: mju_dist3
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
/// Calls: isknown, mjc_gradient, mju_addTo3, mju_copy3, mju_normalize3, mju_rotVecQuat, mju_scl3, mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_pre_contact(points: *mut f64, con: *mut mjPreContact, x: *const f64, pos2: *const f64, quat2: *const f64, dist: f64, cnt: i32, m: *const mjModel, s: *const mjSDF, d: *const mjData, flipNormal: i32) -> i32 {
    todo!() // addPreContact
}

/// C: stepFrankWolfe (engine/engine_collision_sdf.c:585)
/// Calls: mjc_distance, mjc_gradient, mju_addToScl3, mju_copy3, mju_dot3, mju_subFrom3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn step_frank_wolfe(x: *mut f64, corners: *const f64, ncorners: i32, m: *const mjModel, sdf: *const mjSDF, d: *const mjData) -> f64 {
    todo!() // stepFrankWolfe
}

/// C: stepGradient (engine/engine_collision_sdf.c:615)
/// Calls: mjc_distance, mjc_gradient, mju_addScl3, mju_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn step_gradient(x: *mut f64, m: *const mjModel, s: *const mjSDF, d: *const mjData, niter: i32) -> f64 {
    todo!() // stepGradient
}

/// C: triangleIntersect (engine/engine_collision_sdf.c:665)
/// Calls: mjc_distance, mju_addTo3, mju_addToScl3, mju_cross, mju_dist3, mju_dot3, mju_max, mju_norm3, mju_normalize3, mju_scl3, mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn triangle_intersect(triangle: *const f64, m: *const mjModel, sdf: *const mjSDF, d: *const mjData) -> i32 {
    todo!() // triangleIntersect
}

/// C: boxIntersect (engine/engine_collision_sdf.c:737)
/// Calls: mjc_distance, mju_addTo3, mju_mulMatVec3, mju_norm3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn box_intersect(bvh: *const f64, offset: *const f64, rotation: *const f64, m: *const mjModel, s: *const mjSDF, d: *const mjData) -> i32 {
    todo!() // boxIntersect
}

/// C: selectFPS (engine/engine_collision_sdf.c:752)
/// Calls: next
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn select_fps(candidate: *const f64, dist: *const f64, ncandidate: i32, selected_indices: *mut i32, max_select: i32) -> i32 {
    const MJ_MAXCONPAIR: i32 = 50;
    const MJ_MAXVAL: f64 = 1E10;
    // SAFETY: caller guarantees candidate[ncandidate*3], dist[ncandidate], selected_indices[max_select] valid
    unsafe {
        if ncandidate <= 0 {
            return 0;
        }

        let mut selected: [bool; 50] = [false; 50];
        let mut min_dist2: [f64; 50] = [MJ_MAXVAL; 50];

        // start with deepest penetrating contact
        let mut best: i32 = 0;
        let mut bestval = -*dist.add(0);
        for i in 1..ncandidate as usize {
            if -*dist.add(i) > bestval {
                bestval = -*dist.add(i);
                best = i as i32;
            }
        }

        // iteratively select contacts using FPS
        let mut nselected: i32 = 0;
        while nselected < max_select && nselected < MJ_MAXCONPAIR && best >= 0 {
            selected[best as usize] = true;
            *selected_indices.add(nselected as usize) = best;
            nselected += 1;

            let bestpos = candidate.add(3 * best as usize);

            // find next farthest point
            let mut nextbest: i32 = -1;
            let mut nextbestdist: f64 = -1.0;
            for i in 0..ncandidate as usize {
                if selected[i] {
                    continue;
                }

                let dx = *candidate.add(3 * i + 0) - *bestpos.add(0);
                let dy = *candidate.add(3 * i + 1) - *bestpos.add(1);
                let dz = *candidate.add(3 * i + 2) - *bestpos.add(2);
                let d2 = dx * dx + dy * dy + dz * dz;
                if d2 < min_dist2[i] {
                    min_dist2[i] = d2;
                }
                if min_dist2[i] > nextbestdist {
                    nextbestdist = min_dist2[i];
                    nextbest = i as i32;
                }
            }
            best = nextbest;
        }

        nselected
    }
}

/// C: processSdfCorners (engine/engine_collision_sdf.c:808)
/// Calls: mju_Halton, mju_copy3, stepFrankWolfe, triangleIntersect
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn process_sdf_corners(corners: *const f64, m: *const mjModel, d: *const mjData, sdf: *const mjSDF, nstartpts: i32, candidate: *mut f64, dist: *mut f64, ncandidate: *mut i32) {
    todo!() // processSdfCorners
}

/// C: processOneFace (engine/engine_collision_sdf.c:866)
/// Calls: mju_addTo3, mju_mulMatVec3, processSdfCorners
#[allow(unused_variables, non_snake_case)]
pub fn process_one_face(faceid: i32, bvh_active: *mut bool, node: i32, ctx: *mut MeshSDFContext) {
    todo!() // processOneFace
}

/// C: traverseBVH (engine/engine_collision_sdf.c:903)
/// Calls: boxIntersect, mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn traverse_bvh(bvh: *const f64, nodeid: *const i32, child: *const i32, bvh_active: *mut bool, offset: *const f64, rotation: *const f64, m: *const mjModel, d: *const mjData, sdf: *const mjSDF, callback: BVHLeafCallback, ctx: *mut ()) {
    todo!() // traverseBVH
}

/// C: meshFaceCallback (engine/engine_collision_sdf.c:943)
/// Calls: processOneFace
#[allow(unused_variables, non_snake_case)]
pub fn mesh_face_callback(face_id: i32, node: i32, ctx: *mut ()) -> i32 {
    todo!() // meshFaceCallback
}

/// C: flexElemCallback (engine/engine_collision_sdf.c:1198)
/// Calls: mju_addTo3, mju_copy3, mju_mulMatVec3, processSdfCorners
#[allow(unused_variables, non_snake_case)]
pub fn flex_elem_callback(elem_idx: i32, node: i32, ctx: *mut ()) -> i32 {
    todo!() // flexElemCallback
}

/// C: mjc_getSDF (engine/engine_collision_sdf.h:29)
/// Calls: mjp_getPluginAtSlotUnsafe, mjp_pluginCount, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mjc_get_sdf(m: *const mjModel, id: i32) -> *const mjpPlugin {
    // SAFETY: m is a valid mjModel pointer, id is a valid geom index
    unsafe {
        let instance = *(*m).geom_plugin.add(id as usize);
        let nslot = crate::engine::engine_plugin::mjp_plugin_count();
        let slot = *(*m).plugin.add(instance as usize);
        let sdf = crate::engine::engine_plugin::mjp_get_plugin_at_slot_unsafe(slot, nslot);
        if sdf.is_null() {
            crate::engine::engine_util_errmem::mju_error(
                b"invalid plugin slot: %d\0".as_ptr() as *const i8);
        }
        if (*sdf).capabilityflags & (1 << 3) == 0 {  // mjPLUGIN_SDF = 1<<3
            crate::engine::engine_util_errmem::mju_error(
                b"Plugin is not a signed distance field at slot %d\0".as_ptr() as *const i8);
        }
        sdf
    }
}

/// C: mjc_distance (engine/engine_collision_sdf.h:32)
/// Calls: geomDistance, mju_addTo3, mju_max, mju_message, mju_mulMatVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_distance(m: *const mjModel, d: *const mjData, s: *const mjSDF, x: *const f64) -> f64 {
    todo!() // mjc_distance
}

/// C: mjc_gradient (engine/engine_collision_sdf.h:35)
/// Calls: geomDistance, geomGradient, mju_addTo3, mju_addToScl3, mju_max, mju_message, mju_mulMatTVec3, mju_mulMatVec3, mju_normalize3, mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_gradient(m: *const mjModel, d: *const mjData, s: *const mjSDF, gradient: *mut f64, x: *const f64) {
    todo!() // mjc_gradient
}

/// C: mjc_HFieldSDF (engine/engine_collision_sdf.h:39)
/// Calls: mju_warning
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_h_field_sdf(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    crate::engine::engine_util_errmem::mju_warning(
        b"HField vs SDF collision not yet supported!\0".as_ptr() as *const i8);
    0
}

/// C: mjc_MeshSDF (engine/engine_collision_sdf.h:42)
/// Calls: addPreContact, mapPose, mjc_getSDF, mju_mat2Quat, mju_max, selectFPS, traverseBVH
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_mesh_sdf(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    todo!() // mjc_MeshSDF
}

/// C: mjc_SDF (engine/engine_collision_sdf.h:45)
/// Calls: addPreContact, mapPose, mjc_getSDF, mju_Halton, mju_addTo3, mju_mat2Quat, mju_max, mju_message, mju_min, mju_mulMatVec3, stepGradient
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_sdf(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    todo!() // mjc_SDF
}

/// C: mjc_FlexSDF (engine/engine_collision_sdf.h:48)
/// Calls: addPreContact, mapPose, mjc_getSDF, mju_addTo3, mju_copy3, mju_mat2Quat, mju_max, mju_mulMatVec3, processSdfCorners, selectFPS, traverseBVH
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_flex_sdf(m: *const mjModel, d: *const mjData, con: *mut mjPreContact, elem: *mut i32, g: i32, f: i32, margin: f64) -> i32 {
    todo!() // mjc_FlexSDF
}

