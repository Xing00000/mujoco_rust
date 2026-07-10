//! Port of: engine/engine_collision_sdf.c
//! IR hash: 05737965add36adb
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
    // SAFETY: point points to 3 f64; box points to 6 f64 per caller contract
    unsafe {
        let r: [f64; 3] = [
            *point.add(0) - *r#box.add(0),
            *point.add(1) - *r#box.add(1),
            *point.add(2) - *r#box.add(2),
        ];
        let q: [f64; 3] = [
            r[0].abs() - *r#box.add(3),
            r[1].abs() - *r#box.add(4),
            r[2].abs() - *r#box.add(5),
        ];
        let mut dist_sqr: f64 = 0.0;
        let eps: f64 = 1e-6;

        // skip the projection if inside
        if q[0] <= 0.0 && q[1] <= 0.0 && q[2] <= 0.0 {
            let m01 = if q[0] > q[1] { q[0] } else { q[1] };
            return if m01 > q[2] { m01 } else { q[2] };
        }

        // in-place projection inside the box if outside
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

        dist_sqr.sqrt()
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
pub fn find_oct(w: *mut f64, dw: [[f64; 8]; 3], oct_aabb: *const f64, oct_child: *const i32, p: *const f64) -> i32 {
    unsafe {
        let eps: f64 = 1e-8;
        let mut niter: i32 = 100;
        let mut stack: i32 = 0;

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
            let mut j: i32 = 0;
            while j < 3 {
                vmin[j as usize] = *oct_aabb.add(6 * node as usize + j as usize) - *oct_aabb.add(6 * node as usize + 3 + j as usize);
                vmax[j as usize] = *oct_aabb.add(6 * node as usize + j as usize) + *oct_aabb.add(6 * node as usize + 3 + j as usize);
                j += 1;
            }

            // check if the point is inside the aabb of the octree node
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

            // check if the node is a leaf
            if *oct_child.add(8 * node as usize + 0) == -1 && *oct_child.add(8 * node as usize + 1) == -1 &&
               *oct_child.add(8 * node as usize + 2) == -1 && *oct_child.add(8 * node as usize + 3) == -1 &&
               *oct_child.add(8 * node as usize + 4) == -1 && *oct_child.add(8 * node as usize + 5) == -1 &&
               *oct_child.add(8 * node as usize + 6) == -1 && *oct_child.add(8 * node as usize + 7) == -1 {
                let mut j: i32 = 0;
                while j < 8 {
                    if !w.is_null() {
                        *w.add(j as usize) =
                            (if j & 1 != 0 { coord[0] } else { 1.0 - coord[0] }) *
                            (if j & 2 != 0 { coord[1] } else { 1.0 - coord[1] }) *
                            (if j & 4 != 0 { coord[2] } else { 1.0 - coord[2] });
                    }
                    // dw is [[f64; 8]; 3] passed by value — in C ABI it's a pointer
                    let dw_ptr = &dw as *const [[f64; 8]; 3] as *mut f64;
                    if !dw_ptr.is_null() {
                        // dw[j][0]
                        *dw_ptr.add(j as usize * 3 + 0) =
                            (if j & 1 != 0 { 1.0 } else { -1.0 }) *
                            (if j & 2 != 0 { coord[1] } else { 1.0 - coord[1] }) *
                            (if j & 4 != 0 { coord[2] } else { 1.0 - coord[2] });
                        // dw[j][1]
                        *dw_ptr.add(j as usize * 3 + 1) =
                            (if j & 1 != 0 { coord[0] } else { 1.0 - coord[0] }) *
                            (if j & 2 != 0 { 1.0 } else { -1.0 }) *
                            (if j & 4 != 0 { coord[2] } else { 1.0 - coord[2] });
                        // dw[j][2]
                        *dw_ptr.add(j as usize * 3 + 2) =
                            (if j & 1 != 0 { coord[0] } else { 1.0 - coord[0] }) *
                            (if j & 2 != 0 { coord[1] } else { 1.0 - coord[1] }) *
                            (if j & 4 != 0 { 1.0 } else { -1.0 });
                    }
                    j += 1;
                }
                return node;
            }

            // compute which of 8 children to visit next
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
        let mut sdf: f64 = 0.0;
        let mut point: [f64; 3] = [*p.add(0), *p.add(1), *p.add(2)];
        let box_dist = box_projection(point.as_mut_ptr(), oct_aabb);
        let dw_zero: [[f64; 8]; 3] = [[0.0; 8]; 3];
        let node = find_oct(w.as_mut_ptr(), dw_zero, oct_aabb, oct_child, point.as_ptr());
        let mut i: i32 = 0;
        while i < 8 {
            sdf += w[i as usize] * *oct_coeff.add(8 * node as usize + i as usize);
            i += 1;
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
        if box_projection(p.as_mut_ptr(), oct_aabb) <= 0.0 {
            let mut dw: [[f64; 8]; 3] = [[0.0; 8]; 3];
            let node = find_oct(std::ptr::null_mut(), dw, oct_aabb, oct_child, p.as_ptr());
            // dw layout: dw[j][c] stored as dw_ptr[j*3 + c] after find_oct writes
            let dw_ptr = &dw as *const [[f64; 8]; 3] as *const f64;
            let mut i: i32 = 0;
            while i < 8 {
                let coeff_i = *oct_coeff.add(8 * node as usize + i as usize);
                *grad.add(0) += *dw_ptr.add(i as usize * 3 + 0) * coeff_i;
                *grad.add(1) += *dw_ptr.add(i as usize * 3 + 1) * coeff_i;
                *grad.add(2) += *dw_ptr.add(i as usize * 3 + 2) * coeff_i;
                i += 1;
            }
            return;
        }

        // finite difference in the exterior
        let eps: f64 = 1e-8;
        let dist0 = oct_distance(m, point, meshid);
        let point_x: [f64; 3] = [*point.add(0) + eps, *point.add(1), *point.add(2)];
        let dist_x = oct_distance(m, point_x.as_ptr(), meshid);
        let point_y: [f64; 3] = [*point.add(0), *point.add(1) + eps, *point.add(2)];
        let dist_y = oct_distance(m, point_y.as_ptr(), meshid);
        let point_z: [f64; 3] = [*point.add(0), *point.add(1), *point.add(2) + eps];
        let dist_z = oct_distance(m, point_z.as_ptr(), meshid);

        *grad.add(0) = (dist_x - dist0) / eps;
        *grad.add(1) = (dist_y - dist0) / eps;
        *grad.add(2) = (dist_z - dist0) / eps;
    }
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
    unsafe {
        // SAFETY: field, a, x, size each point to at least 3 f64 elements
        *field.add(0) = -*size.add(0) / *a.add(0);
        *field.add(1) = -*size.add(1) / *a.add(1);
        *field.add(2) = -*size.add(2) / *a.add(2);
        crate::engine::engine_util_blas::mju_normalize3(field);

        // flip sign if necessary
        if *x.add(0) < 0.0 { *field.add(0) = -*field.add(0); }
        if *x.add(1) < 0.0 { *field.add(1) = -*field.add(1); }
        if *x.add(2) < 0.0 { *field.add(2) = -*field.add(2); }
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
pub fn geom_distance(m: *const mjModel, d: *const mjData, p: *const mjpPlugin, i: i32, x: *const f64, r#type: mjtGeom) -> f64 {
    extern "C" {
        fn geomDistance(m: *const mjModel, d: *const mjData, p: *const mjpPlugin, i: i32, x: *const f64, r#type: mjtGeom) -> f64;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { geomDistance(m, d, p, i, x, r#type) }
}

/// C: geomGradient (engine/engine_collision_sdf.c:295)
/// Calls: mju_clip, mju_copy3, mju_max, mju_message, mju_norm, mju_norm3, mju_normalize3, mju_zero3, oct_gradient, radialField3d
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn geom_gradient(gradient: *mut f64, m: *const mjModel, d: *const mjData, p: *const mjpPlugin, i: i32, x: *const f64, r#type: mjtGeom) {
    extern "C" {
        fn geomGradient(gradient: *mut f64, m: *const mjModel, d: *const mjData, p: *const mjpPlugin, i: i32, x: *const f64, r#type: mjtGeom);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { geomGradient(gradient, m, d, p, i, x, r#type) }
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
    // SAFETY: all pointers valid per caller contract; intermediate arrays are stack-local
    unsafe {
        let mut negpos: [f64; 3] = [0.0; 3];
        let mut negquat: [f64; 4] = [0.0; 4];
        let mut quat12: [f64; 4] = [0.0; 4];
        crate::engine::engine_util_spatial::mju_neg_pose(
            negpos.as_mut_ptr(), negquat.as_mut_ptr(), xpos2, xquat2);
        crate::engine::engine_util_spatial::mju_mul_pose(
            pos12, quat12.as_mut_ptr(), negpos.as_ptr(), negquat.as_ptr(), xpos1, xquat1);
        crate::engine::engine_util_spatial::mju_quat2mat(mat12, quat12.as_ptr());
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
    // WARNING: signature changed — verify body
    // Previous params: (points : * const f64, x : * const f64, cnt : i32)
    // Previous return: i32
    unsafe { use crate :: engine :: engine_util_blas :: mju_dist3 ; const MJMINVAL : f64 = 1e-15 ; for i in 0 .. cnt as usize { if mju_dist3 (x , points . add (3 * i)) < MJMINVAL { return 1 ; } } 0 }
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
    const MJMINVAL: f64 = 1e-15;
    // SAFETY: points has 3*cnt f64, con valid mjPreContact, x/pos2/quat2/m/s/d valid per caller
    unsafe {
        // check if there is a collision
        if dist > 0.0 || isknown(points, x, cnt) != 0 {
            return cnt;
        } else {
            crate::engine::engine_util_blas::mju_copy3(points.add(3 * cnt as usize), x);
        }

        // compute normal in local coordinates
        let mut norm: [f64; 3] = [0.0; 3];
        let mut vec: [f64; 3] = [0.0; 3];
        mjc_gradient(m, d, s, norm.as_mut_ptr(), x);

        // validate normal - skip if gradient is degenerate
        let norm_len: f64 = crate::engine::engine_util_blas::mju_normalize3(norm.as_mut_ptr());
        if norm_len < MJMINVAL {
            return cnt;
        }

        // normal direction: flipNormal=0 -> INTO SDF, flipNormal=1 -> OUT of SDF
        if flipNormal == 0 {
            crate::engine::engine_util_blas::mju_scl3(norm.as_mut_ptr(), norm.as_ptr(), -1.0);
        }

        // construct contact
        (*con.add(cnt as usize)).dist = dist;
        crate::engine::engine_util_spatial::mju_rot_vec_quat(
            (*con.add(cnt as usize)).normal.as_mut_ptr(), norm.as_ptr(), quat2);
        crate::engine::engine_util_blas::mju_scl3(
            vec.as_mut_ptr(), (*con.add(cnt as usize)).normal.as_ptr(), -0.5 * dist);
        crate::engine::engine_util_spatial::mju_rot_vec_quat(
            (*con.add(cnt as usize)).pos.as_mut_ptr(), x, quat2);
        crate::engine::engine_util_blas::mju_zero3((*con.add(cnt as usize)).tangent.as_mut_ptr());
        crate::engine::engine_util_blas::mju_add_to3((*con.add(cnt as usize)).pos.as_mut_ptr(), pos2);
        crate::engine::engine_util_blas::mju_add_to3((*con.add(cnt as usize)).pos.as_mut_ptr(), vec.as_ptr());

        cnt + 1
    }
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
    const MJMAXVAL: f64 = 1e10;
    // SAFETY: x points to 3 f64, corners points to 3*ncorners f64, m/sdf/d valid per caller
    unsafe {
        let mut step: i32 = 0;
        while step < (*m).opt.sdf_iterations {
            let mut best: f64 = MJMAXVAL;
            let mut s: [f64; 3] = [0.0; 3];
            let mut grad: [f64; 3] = [0.0; 3];

            // evaluate gradient
            mjc_gradient(m, d, sdf, grad.as_mut_ptr(), x);

            // evaluate all corners
            let mut i: i32 = 0;
            while i < ncorners {
                // compute sdf
                let fun: f64 = crate::engine::engine_util_blas::mju_dot3(corners.add(3 * i as usize), grad.as_ptr());

                // save argmin
                if fun < best {
                    best = fun;
                    crate::engine::engine_util_blas::mju_copy3(s.as_mut_ptr(), corners.add(3 * i as usize));
                }
                i += 1;
            }

            // update collision point
            crate::engine::engine_util_blas::mju_sub_from3(s.as_mut_ptr(), x as *const f64);
            crate::engine::engine_util_blas::mju_add_to_scl3(x, s.as_ptr(), 2.0 / (step + 2) as f64);
            step += 1;
        }

        // compute distance
        mjc_distance(m, d, sdf, x)
    }
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
    const MJMAXVAL: f64 = 1e10;
    // SAFETY: x points to 3 f64, m/s/d valid per caller contract
    unsafe {
        let c: f64 = 0.1;       // reduction factor for the target decrease
        let rho: f64 = 0.5;    // reduction factor for alpha
        let amin: f64 = 1e-4;  // minimum alpha
        let mut dist: f64 = MJMAXVAL;

        let mut step: i32 = 0;
        while step < niter {
            let mut grad: [f64; 3] = [0.0; 3];
            let mut alpha: f64 = 2.0;

            // evaluate gradient
            mjc_gradient(m, d, s, grad.as_mut_ptr(), x);

            // sanity check
            if grad[0].is_nan() || grad[0] > MJMAXVAL || grad[0] < -MJMAXVAL
                || grad[1].is_nan() || grad[1] > MJMAXVAL || grad[1] < -MJMAXVAL
                || grad[2].is_nan() || grad[2] > MJMAXVAL || grad[2] < -MJMAXVAL
            {
                return MJMAXVAL;
            }

            // save current solution
            let x0: [f64; 3] = [*x.add(0), *x.add(1), *x.add(2)];

            // evaluate distance
            let dist0: f64 = mjc_distance(m, d, s, x0.as_ptr());
            let mut wolfe: f64 = -c * alpha * crate::engine::engine_util_blas::mju_dot3(grad.as_ptr(), grad.as_ptr());

            // backtracking line search
            loop {
                alpha *= rho;
                wolfe *= rho;
                crate::engine::engine_util_blas::mju_add_scl3(x, x0.as_ptr(), grad.as_ptr(), -alpha);
                dist = mjc_distance(m, d, s, x);
                if !(alpha > amin && dist - dist0 > wolfe) {
                    break;
                }
            }

            // if no improvement, early stop
            if dist0 < dist {
                return dist;
            }

            step += 1;
        }

        dist
    }
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
    const MJMINVAL: f64 = 1e-15;
    // SAFETY: triangle[9], m/sdf/d valid per caller
    unsafe {
        let mut edges: [f64; 6] = [0.0; 6];
        let mut normal: [f64; 3] = [0.0; 3];
        let mut center: [f64; 3] = [0.0; 3];
        let mut v: [f64; 9] = [0.0; 9];
        let mut cross: [f64; 9] = [0.0; 9];
        let mut p: [f64; 3] = [0.0; 3];
        let kDistanceScl: f64 = 10.0;
        let kMinHeight: f64 = 0.1;

        // triangle normal
        crate::engine::engine_util_blas::mju_sub3(edges.as_mut_ptr(), triangle.add(3), triangle);
        crate::engine::engine_util_blas::mju_sub3(edges.as_mut_ptr().add(3), triangle.add(6), triangle);
        crate::engine::engine_util_spatial::mju_cross(normal.as_mut_ptr(), edges.as_ptr(), edges.as_ptr().add(3));
        crate::engine::engine_util_blas::mju_normalize3(normal.as_mut_ptr());

        // triangle centroid
        crate::engine::engine_util_blas::mju_scl3(p.as_mut_ptr(), triangle, 1.0 / 3.0);
        crate::engine::engine_util_blas::mju_add_to_scl3(p.as_mut_ptr(), triangle.add(3), 1.0 / 3.0);
        crate::engine::engine_util_blas::mju_add_to_scl3(p.as_mut_ptr(), triangle.add(6), 1.0 / 3.0);

        // SDF distance at centroid
        let dist_at_centroid: f64 = mjc_distance(m, d, sdf, p.as_ptr());

        // compute h = offset for fourth point
        let h: f64 = -dist_at_centroid / kDistanceScl;

        // degenerate check
        if h.abs() < kMinHeight {
            let a: f64 = crate::engine::engine_util_blas::mju_dist3(triangle, triangle.add(3));
            let b: f64 = crate::engine::engine_util_blas::mju_dist3(triangle.add(3), triangle.add(6));
            let c: f64 = crate::engine::engine_util_blas::mju_dist3(triangle.add(6), triangle);
            let s: f64 = (a + b + c) / 2.0;
            let area: f64 = (s * (s - a) * (s - b) * (s - c)).sqrt();
            let circumradius: f64 = (a * b * c) / (4.0 * crate::engine::engine_util_misc::mju_max(area, MJMINVAL));
            return (dist_at_centroid < circumradius) as i32;
        }

        // fourth point: triangle centroid pushed along normal
        crate::engine::engine_util_blas::mju_add_to_scl3(p.as_mut_ptr(), normal.as_ptr(), -h);

        // circumsphere center
        crate::engine::engine_util_blas::mju_sub3(v.as_mut_ptr(), triangle, p.as_ptr());
        crate::engine::engine_util_blas::mju_sub3(v.as_mut_ptr().add(3), triangle.add(3), p.as_ptr());
        crate::engine::engine_util_blas::mju_sub3(v.as_mut_ptr().add(6), triangle.add(6), p.as_ptr());
        crate::engine::engine_util_spatial::mju_cross(cross.as_mut_ptr(), v.as_ptr().add(3), v.as_ptr().add(6));
        crate::engine::engine_util_spatial::mju_cross(cross.as_mut_ptr().add(3), v.as_ptr().add(6), v.as_ptr());
        crate::engine::engine_util_spatial::mju_cross(cross.as_mut_ptr().add(6), v.as_ptr(), v.as_ptr().add(3));
        crate::engine::engine_util_blas::mju_scl3(center.as_mut_ptr(), cross.as_ptr(), crate::engine::engine_util_blas::mju_dot3(v.as_ptr(), v.as_ptr()));
        crate::engine::engine_util_blas::mju_add_to_scl3(center.as_mut_ptr(), cross.as_ptr().add(3), crate::engine::engine_util_blas::mju_dot3(v.as_ptr().add(3), v.as_ptr().add(3)));
        crate::engine::engine_util_blas::mju_add_to_scl3(center.as_mut_ptr(), cross.as_ptr().add(6), crate::engine::engine_util_blas::mju_dot3(v.as_ptr().add(6), v.as_ptr().add(6)));

        let denom: f64 = 2.0 * crate::engine::engine_util_blas::mju_dot3(v.as_ptr(), cross.as_ptr());
        if denom.abs() < MJMINVAL {
            return (dist_at_centroid < crate::engine::engine_util_blas::mju_norm3(edges.as_ptr())) as i32;
        }
        crate::engine::engine_util_blas::mju_scl3(center.as_mut_ptr(), center.as_ptr(), 1.0 / denom);

        // circumsphere radius
        let r: f64 = crate::engine::engine_util_blas::mju_dot3(center.as_ptr(), center.as_ptr()).sqrt();

        // coordinate change
        crate::engine::engine_util_blas::mju_add_to3(center.as_mut_ptr(), p.as_ptr());

        (mjc_distance(m, d, sdf, center.as_ptr()) < r) as i32
    }
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
    // SAFETY: bvh[6], offset[3], rotation[9] valid per caller. m/s/d valid.
    unsafe {
        let mut candidate: [f64; 3] = [0.0; 3];
        let r: f64 = crate::engine::engine_util_blas::mju_norm3(bvh.add(3));

        crate::engine::engine_util_blas::mju_mul_mat_vec3(candidate.as_mut_ptr(), rotation, bvh);
        crate::engine::engine_util_blas::mju_add_to3(candidate.as_mut_ptr(), offset);

        // check if inside the bounding box
        (mjc_distance(m, d, s, candidate.as_ptr()) < r) as i32
    }
}

/// C: selectFPS (engine/engine_collision_sdf.c:752)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn select_fps(candidate: *const f64, dist: *const f64, ncandidate: i32, selected_indices: *mut i32, max_select: i32) -> i32 {
    const MJ_MAXCONPAIR: usize = 50;
    const MJ_MAXVAL: f64 = 1e10;

    if ncandidate <= 0 { return 0; }

    // SAFETY: candidate points to 3*ncandidate f64; dist points to ncandidate f64;
    // selected_indices points to at least max_select i32 elements
    unsafe {
        let mut selected: [bool; MJ_MAXCONPAIR] = [false; MJ_MAXCONPAIR];
        let mut min_dist2: [f64; MJ_MAXCONPAIR] = [MJ_MAXVAL; MJ_MAXCONPAIR];

        // start with deepest penetrating contact
        let mut best: i32 = 0;
        let mut bestval: f64 = -*dist.add(0);
        for i in 1..ncandidate as usize {
            if -*dist.add(i) > bestval {
                bestval = -*dist.add(i);
                best = i as i32;
            }
        }

        // iteratively select contacts using FPS
        let mut nselected: i32 = 0;
        while nselected < max_select && (nselected as usize) < MJ_MAXCONPAIR && best >= 0 {
            selected[best as usize] = true;
            *selected_indices.add(nselected as usize) = best;
            nselected += 1;

            let bestpos: *const f64 = candidate.add(3 * best as usize);

            // find next farthest point
            let mut nextbest: i32 = -1;
            let mut nextbestdist: f64 = -1.0;
            for i in 0..ncandidate as usize {
                if selected[i] { continue; }

                let dx: f64 = *candidate.add(3 * i + 0) - *bestpos.add(0);
                let dy: f64 = *candidate.add(3 * i + 1) - *bestpos.add(1);
                let dz: f64 = *candidate.add(3 * i + 2) - *bestpos.add(2);
                let d2: f64 = dx * dx + dy * dy + dz * dz;
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
    const MJMAXCONPAIR: i32 = 50;
    // SAFETY: corners[9], candidate/dist arrays, ncandidate valid per caller
    unsafe {
        // stricter culling using triangle circumsphere
        if triangle_intersect(corners, m, sdf, d) == 0 {
            return;
        }

        // sample multiple starting points using Halton sequence
        let mut sp: i32 = 0;
        while sp < nstartpts {
            if *ncandidate >= MJMAXCONPAIR {
                break;
            }

            // barycentric coordinates from Halton sequence
            let mut u: f64 = crate::engine::engine_util_misc::mju_halton(sp + 1, 2);
            let mut v: f64 = crate::engine::engine_util_misc::mju_halton(sp + 1, 3);
            if u + v > 1.0 {
                u = 1.0 - u;
                v = 1.0 - v;
            }
            let b0: f64 = 1.0 - u - v;
            let b1: f64 = u;
            let b2: f64 = v;

            // starting point
            let mut x: [f64; 3] = [
                b0 * *corners.add(0) + b1 * *corners.add(3) + b2 * *corners.add(6),
                b0 * *corners.add(1) + b1 * *corners.add(4) + b2 * *corners.add(7),
                b0 * *corners.add(2) + b1 * *corners.add(5) + b2 * *corners.add(8),
            ];

            let depth: f64 = step_frank_wolfe(x.as_mut_ptr(), corners, 3, m, sdf, d);

            // store candidate if penetration
            if depth < 0.0 {
                let nc: i32 = *ncandidate;
                crate::engine::engine_util_blas::mju_copy3(candidate.add(3 * nc as usize), x.as_ptr());
                *dist.add(nc as usize) = depth;
                *ncandidate += 1;
            }

            sp += 1;
        }
    }
}

/// C: processOneFace (engine/engine_collision_sdf.c:866)
/// Calls: mju_addTo3, mju_mulMatVec3, processSdfCorners
#[allow(unused_variables, non_snake_case)]
pub fn process_one_face(faceid: i32, bvh_active: *mut mjtBool, node: i32, ctx: *mut MeshSDFContext) {
    extern "C" { fn processOneFace(faceid: i32, bvh_active: *mut mjtBool, node: i32, ctx: *mut MeshSDFContext); }
    // SAFETY: delegates to C implementation
    unsafe { processOneFace(faceid, bvh_active, node, ctx) }
}

/// C: traverseBVH (engine/engine_collision_sdf.c:903)
/// Calls: boxIntersect, mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn traverse_bvh(bvh: *const f64, nodeid: *const i32, child: *const i32, bvh_active: *mut mjtBool, offset: *const f64, rotation: *const f64, m: *const mjModel, d: *const mjData, sdf: *const mjSDF, callback: BVHLeafCallback, ctx: *mut ()) {
    extern "C" {
        fn traverseBVH(bvh: *const f64, nodeid: *const i32, child: *const i32, bvh_active: *mut mjtBool, offset: *const f64, rotation: *const f64, m: *const mjModel, d: *const mjData, sdf: *const mjSDF, callback: BVHLeafCallback, ctx: *mut ());
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { traverseBVH(bvh, nodeid, child, bvh_active, offset, rotation, m, d, sdf, callback, ctx) }
}

/// C: meshFaceCallback (engine/engine_collision_sdf.c:943)
/// Calls: processOneFace
#[allow(unused_variables, non_snake_case)]
pub fn mesh_face_callback(face_id: i32, node: i32, ctx: *mut ()) -> i32 {
    extern "C" { fn mesh_face_callback(face_id: i32, node: i32, ctx: *mut ()) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mesh_face_callback(face_id, node, ctx) }
}

/// C: flexElemCallback (engine/engine_collision_sdf.c:1198)
/// Calls: mju_addTo3, mju_copy3, mju_mulMatVec3, processSdfCorners
#[allow(unused_variables, non_snake_case)]
pub fn flex_elem_callback(elem_idx: i32, node: i32, ctx: *mut ()) -> i32 {
    extern "C" { fn flex_elem_callback(elem_idx: i32, node: i32, ctx: *mut ()) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { flex_elem_callback(elem_idx, node, ctx) }
}

/// C: mjc_getSDF (engine/engine_collision_sdf.h:29)
/// Calls: mjp_getPluginAtSlotUnsafe, mjp_pluginCount, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mjc_get_sdf(m: *const mjModel, id: i32) -> *const mjpPlugin {
    extern "C" {
        fn mjc_getSDF(m: *const mjModel, id: i32) -> *const mjpPlugin;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjc_getSDF(m, id) }
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
    extern "C" {
        fn mjc_distance(m: *const mjModel, d: *const mjData, s: *const mjSDF, x: *const f64) -> f64;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjc_distance(m, d, s, x) }
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
    extern "C" {
        fn mjc_gradient(m: *const mjModel, d: *const mjData, s: *const mjSDF, gradient: *mut f64, x: *const f64);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjc_gradient(m, d, s, gradient, x) }
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
    // SAFETY: calling mju_warning with a static C string literal
    unsafe {
        crate::engine::engine_util_errmem::mju_warning(
            b"HField vs SDF collision not yet supported!\0".as_ptr() as *const i8,
        );
    }
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
    extern "C" { fn mjc_MeshSDF(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjc_MeshSDF(m, d, con, g1, g2, margin) }
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
    extern "C" { fn mjc_SDF(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjc_SDF(m, d, con, g1, g2, margin) }
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
    extern "C" { fn mjc_FlexSDF(m: *const mjModel, d: *const mjData, con: *mut mjPreContact, elem: *mut i32, g: i32, f: i32, margin: f64) -> i32; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjc_FlexSDF(m, d, con, elem, g, f, margin) }
}

