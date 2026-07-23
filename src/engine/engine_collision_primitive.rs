//! Port of: engine/engine_collision_primitive.c
//! IR hash: 3fb6da908ad9d71c
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mjraw_PlaneSphere (engine/engine_collision_primitive.c:28)
/// Calls: mji_add3, mji_scl3, mji_zero3, mju_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjraw_plane_sphere(con: *mut mjPreContact, margin: f64, pos1: *const f64, mat1: *const f64, size1: *const f64, pos2: *const f64, mat2: *const f64, size2: *const f64) -> i32 {
    // SAFETY: all pointers are valid and arrays are properly sized (caller contract).
    unsafe {
        // set normal
        (*con).normal[0] = *mat1.add(2);
        (*con).normal[1] = *mat1.add(5);
        (*con).normal[2] = *mat1.add(8);

        // compute distance, return if too large
        let tmp: [f64; 3] = [
            *pos2.add(0) - *pos1.add(0),
            *pos2.add(1) - *pos1.add(1),
            *pos2.add(2) - *pos1.add(2),
        ];
        let cdist = crate::engine::engine_util_blas::mju_dot3(tmp.as_ptr(), (*con).normal.as_ptr());
        if cdist > margin + *size2.add(0) {
            return 0;
        }

        // depth and position
        (*con).dist = cdist - *size2.add(0);
        let mut scl_tmp: [f64; 3] = [0.0; 3];
        crate::engine::engine_inline::mji_scl3(
            scl_tmp.as_mut_ptr(), (*con).normal.as_ptr(), -(*con).dist / 2.0 - *size2.add(0));
        crate::engine::engine_inline::mji_add3(
            (*con).pos.as_mut_ptr(), pos2, scl_tmp.as_ptr());
        crate::engine::engine_inline::mji_zero3((*con).tangent.as_mut_ptr());
        1
    }
}

/// C: mjraw_SphereSphere (engine/engine_collision_primitive.c:262)
/// Calls: mji_addTo3, mji_cross, mji_scl3, mji_sub3, mji_zero3, mju_dot3, mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjraw_sphere_sphere(con: *mut mjPreContact, margin: f64, pos1: *const f64, mat1: *const f64, size1: *const f64, pos2: *const f64, mat2: *const f64, size2: *const f64) -> i32 {
    const MJ_MINVAL: f64 = 1E-15;
    // SAFETY: caller guarantees all pointers are valid and arrays are properly sized
    unsafe {
        // check bounding spheres
        let dif: [f64; 3] = [
            *pos1.add(0) - *pos2.add(0),
            *pos1.add(1) - *pos2.add(1),
            *pos1.add(2) - *pos2.add(2),
        ];
        let cdist_sqr = crate::engine::engine_util_blas::mju_dot3(dif.as_ptr(), dif.as_ptr());
        let min_dist = margin + *size1.add(0) + *size2.add(0);
        if cdist_sqr > min_dist * min_dist {
            return 0;
        }

        // depth and normal
        (*con).dist = cdist_sqr.sqrt() - *size1.add(0) - *size2.add(0);
        crate::engine::engine_inline::mji_sub3((*con).normal.as_mut_ptr(), pos2, pos1);
        let len = crate::engine::engine_util_blas::mju_normalize3((*con).normal.as_mut_ptr());

        // if centers are the same, norm = cross-product of z axes
        // if z axes are parallel, norm = [1;0;0]
        if len < MJ_MINVAL {
            let axis1: [f64; 3] = [*mat1.add(2), *mat1.add(5), *mat1.add(8)];
            let axis2: [f64; 3] = [*mat2.add(2), *mat2.add(5), *mat2.add(8)];
            crate::engine::engine_inline::mji_cross((*con).normal.as_mut_ptr(), axis1.as_ptr(), axis2.as_ptr());
            crate::engine::engine_util_blas::mju_normalize3((*con).normal.as_mut_ptr());
        }

        // position
        crate::engine::engine_inline::mji_scl3((*con).pos.as_mut_ptr(), (*con).normal.as_ptr(), *size1.add(0) + (*con).dist / 2.0);
        crate::engine::engine_inline::mji_add_to3((*con).pos.as_mut_ptr(), pos1);

        // axis
        crate::engine::engine_inline::mji_zero3((*con).tangent.as_mut_ptr());

        1
    }
}

/// C: areaSign (engine/engine_collision_primitive.c:534)
/// Calls: mju_sign
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn area_sign(p1: *const f64, p2: *const f64, p3: *const f64) -> f64 {
    // SAFETY: p1, p2, p3 each point to at least 2 f64 elements (caller contract)
    unsafe {
        let val = (*p1.add(0) - *p3.add(0)) * (*p2.add(1) - *p3.add(1))
                - (*p2.add(0) - *p3.add(0)) * (*p1.add(1) - *p3.add(1));
        if val > 0.0 { 1.0 } else if val < 0.0 { -1.0 } else { 0.0 }
    }
}

/// C: pointSegment (engine/engine_collision_primitive.c:540)
/// Calls: mju_addScl, mju_dot, mju_max
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn point_segment(res: *mut f64, p: *const f64, u: *const f64, v: *const f64) -> f64 {
    use crate::engine::engine_util_blas::{mju_dot, mju_add_scl};
    use crate::engine::engine_util_misc::mju_max;

    const MJ_MINVAL: f64 = 1E-15;

    // SAFETY: res, p, u, v each point to at least 2 f64 (caller contract)
    unsafe {
        // make u the origin
        let uv: [f64; 2] = [*v.add(0) - *u.add(0), *v.add(1) - *u.add(1)];
        let up: [f64; 2] = [*p.add(0) - *u.add(0), *p.add(1) - *u.add(1)];

        // project: find a s.t. uv is orthogonal to (up-a*uv)
        let a = mju_dot(uv.as_ptr(), up.as_ptr(), 2)
              / mju_max(MJ_MINVAL, mju_dot(uv.as_ptr(), uv.as_ptr(), 2));

        // find nearest point to p, clamp to u or v if a is not in (0,1)
        if a <= 0.0 {
            *res.add(0) = *u.add(0);
            *res.add(1) = *u.add(1);
        } else if a >= 1.0 {
            *res.add(0) = *v.add(0);
            *res.add(1) = *v.add(1);
        } else {
            mju_add_scl(res, u, uv.as_ptr(), a, 2);
        }

        // compute distance
        f64::sqrt((*res.add(0) - *p.add(0)) * (*res.add(0) - *p.add(0))
                + (*res.add(1) - *p.add(1)) * (*res.add(1) - *p.add(1)))
    }
}

/// C: mjraw_SphereCapsule (engine/engine_collision_primitive.h:28)
/// Calls: mji_addTo3, mji_scl3, mjraw_SphereSphere, mju_clip, mju_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjraw_sphere_capsule(con: *mut mjPreContact, margin: f64, pos1: *const f64, mat1: *const f64, size1: *const f64, pos2: *const f64, mat2: *const f64, size2: *const f64) -> i32 {
    // SAFETY: caller guarantees all pointers are valid and arrays properly sized
    unsafe {
        // get capsule length and axis
        let len = *size2.add(1);
        let axis: [f64; 3] = [*mat2.add(2), *mat2.add(5), *mat2.add(8)];

        // find projection, clip to segment
        let mut vec: [f64; 3] = [
            *pos1.add(0) - *pos2.add(0),
            *pos1.add(1) - *pos2.add(1),
            *pos1.add(2) - *pos2.add(2),
        ];
        let x = crate::engine::engine_util_misc::mju_clip(
            crate::engine::engine_util_blas::mju_dot3(axis.as_ptr(), vec.as_ptr()),
            -len, len);

        // find nearest point on segment, do sphere-sphere test
        crate::engine::engine_inline::mji_scl3(vec.as_mut_ptr(), axis.as_ptr(), x);
        crate::engine::engine_inline::mji_add_to3(vec.as_mut_ptr(), pos2);
        mjraw_sphere_sphere(con, margin, pos1, mat1, size1, vec.as_ptr(), mat2, size2)
    }
}

/// C: mjraw_CapsuleCapsule (engine/engine_collision_primitive.h:31)
/// Calls: mji_add3, mji_addTo3, mji_scl3, mji_sub3, mjraw_SphereSphere, mju_clip, mju_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjraw_capsule_capsule(con: *mut mjPreContact, margin: f64, pos1: *const f64, mat1: *const f64, size1: *const f64, pos2: *const f64, mat2: *const f64, size2: *const f64) -> i32 {
    const MJ_MINVAL: f64 = 1E-15;
    // SAFETY: caller guarantees all pointers are valid and arrays properly sized
    unsafe {
        // get capsule axes (scaled) and center difference
        let axis1: [f64; 3] = [
            *mat1.add(2) * *size1.add(1),
            *mat1.add(5) * *size1.add(1),
            *mat1.add(8) * *size1.add(1),
        ];
        let axis2: [f64; 3] = [
            *mat2.add(2) * *size2.add(1),
            *mat2.add(5) * *size2.add(1),
            *mat2.add(8) * *size2.add(1),
        ];
        let dif: [f64; 3] = [
            *pos1.add(0) - *pos2.add(0),
            *pos1.add(1) - *pos2.add(1),
            *pos1.add(2) - *pos2.add(2),
        ];

        // compute matrix coefficients and determinant
        let ma = crate::engine::engine_util_blas::mju_dot3(axis1.as_ptr(), axis1.as_ptr());
        let mb = -crate::engine::engine_util_blas::mju_dot3(axis1.as_ptr(), axis2.as_ptr());
        let mc = crate::engine::engine_util_blas::mju_dot3(axis2.as_ptr(), axis2.as_ptr());
        let u = -crate::engine::engine_util_blas::mju_dot3(axis1.as_ptr(), dif.as_ptr());
        let v = crate::engine::engine_util_blas::mju_dot3(axis2.as_ptr(), dif.as_ptr());
        let det = ma * mc - mb * mb;

        // general configuration (non-parallel axes)
        if det.abs() >= MJ_MINVAL {
            let mut x1 = (mc * u - mb * v) / det;
            let mut x2 = (ma * v - mb * u) / det;

            if x1 > 1.0 {
                x1 = 1.0;
                x2 = (v - mb) / mc;
            } else if x1 < -1.0 {
                x1 = -1.0;
                x2 = (v + mb) / mc;
            }
            if x2 > 1.0 {
                x2 = 1.0;
                x1 = crate::engine::engine_util_misc::mju_clip((u - mb) / ma, -1.0, 1.0);
            } else if x2 < -1.0 {
                x2 = -1.0;
                x1 = crate::engine::engine_util_misc::mju_clip((u + mb) / ma, -1.0, 1.0);
            }

            // find nearest points, do sphere-sphere test
            let mut vec1: [f64; 3] = [0.0; 3];
            let mut vec2: [f64; 3] = [0.0; 3];
            crate::engine::engine_inline::mji_scl3(vec1.as_mut_ptr(), axis1.as_ptr(), x1);
            crate::engine::engine_inline::mji_add_to3(vec1.as_mut_ptr(), pos1);
            crate::engine::engine_inline::mji_scl3(vec2.as_mut_ptr(), axis2.as_ptr(), x2);
            crate::engine::engine_inline::mji_add_to3(vec2.as_mut_ptr(), pos2);

            return mjraw_sphere_sphere(con, margin, vec1.as_ptr(), mat1, size1, vec2.as_ptr(), mat2, size2);
        }

        // parallel axes
        // x1 = 1
        let mut vec1: [f64; 3] = [0.0; 3];
        crate::engine::engine_inline::mji_add3(vec1.as_mut_ptr(), pos1, axis1.as_ptr());
        let mut x2 = crate::engine::engine_util_misc::mju_clip((v - mb) / mc, -1.0, 1.0);
        let mut vec2: [f64; 3] = [0.0; 3];
        crate::engine::engine_inline::mji_scl3(vec2.as_mut_ptr(), axis2.as_ptr(), x2);
        crate::engine::engine_inline::mji_add_to3(vec2.as_mut_ptr(), pos2);
        let n1 = mjraw_sphere_sphere(con, margin, vec1.as_ptr(), mat1, size1, vec2.as_ptr(), mat2, size2);

        // x1 = -1
        crate::engine::engine_inline::mji_sub3(vec1.as_mut_ptr(), pos1, axis1.as_ptr());
        x2 = crate::engine::engine_util_misc::mju_clip((v + mb) / mc, -1.0, 1.0);
        crate::engine::engine_inline::mji_scl3(vec2.as_mut_ptr(), axis2.as_ptr(), x2);
        crate::engine::engine_inline::mji_add_to3(vec2.as_mut_ptr(), pos2);
        let n2 = mjraw_sphere_sphere(con.add(n1 as usize), margin, vec1.as_ptr(), mat1, size1, vec2.as_ptr(), mat2, size2);

        if n1 + n2 >= 2 {
            return n1 + n2;
        }

        // x2 = 1
        crate::engine::engine_inline::mji_add3(vec2.as_mut_ptr(), pos2, axis2.as_ptr());
        let mut x1 = crate::engine::engine_util_misc::mju_clip((u - mb) / ma, -1.0, 1.0);
        crate::engine::engine_inline::mji_scl3(vec1.as_mut_ptr(), axis1.as_ptr(), x1);
        crate::engine::engine_inline::mji_add_to3(vec1.as_mut_ptr(), pos1);
        let n3 = mjraw_sphere_sphere(con.add((n1 + n2) as usize), margin, vec1.as_ptr(), mat1, size1, vec2.as_ptr(), mat2, size2);

        if n1 + n2 + n3 >= 2 {
            return n1 + n2 + n3;
        }

        // x2 = -1
        crate::engine::engine_inline::mji_sub3(vec2.as_mut_ptr(), pos2, axis2.as_ptr());
        x1 = crate::engine::engine_util_misc::mju_clip((u + mb) / ma, -1.0, 1.0);
        crate::engine::engine_inline::mji_scl3(vec1.as_mut_ptr(), axis1.as_ptr(), x1);
        crate::engine::engine_inline::mji_add_to3(vec1.as_mut_ptr(), pos1);
        let n4 = mjraw_sphere_sphere(con.add((n1 + n2 + n3) as usize), margin, vec1.as_ptr(), mat1, size1, vec2.as_ptr(), mat2, size2);

        n1 + n2 + n3 + n4
    }
}

/// C: mjraw_CapsuleBox (engine/engine_collision_primitive.h:34)
/// Calls: mji_addToScl3, mji_copy3, mji_mulMatTVec3, mji_scl3, mji_sub3, mji_subFrom3, mjraw_SphereBox, mju_addTo3, mju_copy3, mju_dot3, mju_mulMatVec3, mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjraw_capsule_box(con: *mut mjPreContact, margin: f64, pos1: *const f64, mat1: *const f64, size1: *const f64, pos2: *const f64, mat2: *const f64, size2: *const f64) -> i32 {
    const MJ_MINVAL: f64 = 1E-15;

    // SAFETY: all pointers are valid arrays provided by the collision system
    unsafe {
        let mut tmp1: [f64; 3] = [0.0; 3];
        let mut tmp2: [f64; 3] = [0.0; 3];
        let mut tmp3: [f64; 3] = [0.0; 3];
        let mut halfaxis: [f64; 3] = [0.0; 3];
        let mut axis: [f64; 3] = [0.0; 3];
        let mut dif: [f64; 3] = [0.0; 3];
        let mut pos: [f64; 3] = [0.0; 3];

        let halflength = *size1.add(1);
        let mut bestdist: f64;
        let bestdistmax: f64;
        let mut bestsegmentpos: f64;
        let mut secondpos: f64;
        let mut dist: f64;
        let mut bestboxpos: f64 = 0.0;
        let mut mul: f64 = 0.0;
        let mut e1: f64;
        let mut e2: f64;
        let mut dp: f64 = 0.0;
        let mut de: f64 = 0.0;

        let mut ma: f64;
        let mut mb: f64;
        let mut mc: f64;
        let mut x1: f64;
        let mut x2: f64;
        let mut idet: f64;

        let mut s1: i32;
        let mut s2: i32;
        let mut c1: i32;
        let mut c2: i32;
        let mut cltype: i32 = -4;
        let mut clface: i32 = 0;
        let mut clcorner: i32 = 0;
        let mut cledge: i32 = 0;
        let mut axisdir: i32;
        let n: i32;
        let mut ax: i32 = 0;
        let mut ax1: i32 = 0;
        let mut ax2: i32 = 0;

        secondpos = -4.0;

        crate::engine::engine_inline::mji_sub3(tmp1.as_mut_ptr(), pos1, pos2);
        crate::engine::engine_inline::mji_mul_mat_t_vec3(pos.as_mut_ptr(), mat2, tmp1.as_ptr());

        tmp1[0] = *mat1.add(2);
        tmp1[1] = *mat1.add(5);
        tmp1[2] = *mat1.add(8);

        crate::engine::engine_inline::mji_mul_mat_t_vec3(axis.as_mut_ptr(), mat2, tmp1.as_ptr());
        crate::engine::engine_inline::mji_scl3(halfaxis.as_mut_ptr(), axis.as_ptr(), halflength);

        axisdir = 0;
        if halfaxis[0] > 0.0 { axisdir += 1; }
        if halfaxis[1] > 0.0 { axisdir += 2; }
        if halfaxis[2] > 0.0 { axisdir += 4; }

        bestdistmax = margin + 2.0 * (*size1.add(0) + halflength + *size2.add(0) + *size2.add(1) + *size2.add(2));
        bestdist = bestdistmax;
        bestsegmentpos = 0.0;

        crate::engine::engine_util_blas::mju_zero3(tmp2.as_mut_ptr());

        // test face-closest
        for i_signed in [-1i32, 1i32] {
            let i = i_signed as f64;
            crate::engine::engine_inline::mji_copy3(tmp1.as_mut_ptr(), pos.as_ptr());
            crate::engine::engine_inline::mji_add_to_scl3(tmp1.as_mut_ptr(), halfaxis.as_ptr(), i);
            crate::engine::engine_inline::mji_copy3(tmp2.as_mut_ptr(), tmp1.as_ptr());

            c1 = 0;
            c2 = -1;
            for j in 0..3i32 {
                if tmp1[j as usize] < -*size2.add(j as usize) {
                    c1 += 1;
                    c2 = j;
                    tmp1[j as usize] = -*size2.add(j as usize);
                } else if tmp1[j as usize] > *size2.add(j as usize) {
                    c1 += 1;
                    c2 = j;
                    tmp1[j as usize] = *size2.add(j as usize);
                }
            }

            if c1 > 1 { continue; }

            crate::engine::engine_inline::mji_sub_from3(tmp1.as_mut_ptr(), tmp2.as_ptr());
            dist = crate::engine::engine_util_blas::mju_dot3(tmp1.as_ptr(), tmp1.as_ptr());

            if dist < bestdist {
                bestdist = dist;
                bestsegmentpos = i;
                cltype = -2 + i_signed;
                clface = c2;
            }
        }

        crate::engine::engine_util_blas::mju_zero3(tmp2.as_mut_ptr());

        // edge tests
        for j in 0..3i32 {
            for i in 0..8i32 {
                if (i & (1 << j)) == 0 {
                    tmp3[0] = (if (i & 1) != 0 { 1.0 } else { -1.0 }) * *size2.add(0);
                    tmp3[1] = (if (i & 2) != 0 { 1.0 } else { -1.0 }) * *size2.add(1);
                    tmp3[2] = (if (i & 4) != 0 { 1.0 } else { -1.0 }) * *size2.add(2);
                    tmp3[j as usize] = 0.0;

                    crate::engine::engine_inline::mji_sub3(dif.as_mut_ptr(), tmp3.as_ptr(), pos.as_ptr());

                    ma = *size2.add(j as usize) * *size2.add(j as usize);
                    mb = -*size2.add(j as usize) * halfaxis[j as usize];
                    mc = *size1.add(1) * *size1.add(1);

                    let u = -*size2.add(j as usize) * dif[j as usize];
                    let v = crate::engine::engine_util_blas::mju_dot3(halfaxis.as_ptr(), dif.as_ptr());

                    let det = ma * mc - mb * mb;
                    if det.abs() < MJ_MINVAL { continue; }
                    idet = 1.0 / det;

                    x1 = (mc * u - mb * v) * idet;
                    x2 = (ma * v - mb * u) * idet;

                    s1 = 1;
                    s2 = 1;

                    if x1 > 1.0 {
                        x1 = 1.0;
                        s1 = 2;
                        x2 = (v - mb) * (1.0 / mc);
                    } else if x1 < -1.0 {
                        x1 = -1.0;
                        s1 = 0;
                        x2 = (v + mb) * (1.0 / mc);
                    }

                    if x2 > 1.0 {
                        x2 = 1.0;
                        s2 = 2;
                        x1 = (u - mb) * (1.0 / ma);
                        if x1 > 1.0 { x1 = 1.0; s1 = 2; }
                        else if x1 < -1.0 { x1 = -1.0; s1 = 0; }
                    } else if x2 < -1.0 {
                        x2 = -1.0;
                        s2 = 0;
                        x1 = (u + mb) * (1.0 / ma);
                        if x1 > 1.0 { x1 = 1.0; s1 = 2; }
                        else if x1 < -1.0 { x1 = -1.0; s1 = 0; }
                    }

                    crate::engine::engine_inline::mji_sub3(dif.as_mut_ptr(), tmp3.as_ptr(), pos.as_ptr());
                    crate::engine::engine_inline::mji_add_to_scl3(dif.as_mut_ptr(), halfaxis.as_ptr(), -x2);
                    dif[j as usize] += *size2.add(j as usize) * x1;

                    tmp1[2] = crate::engine::engine_util_blas::mju_dot3(dif.as_ptr(), dif.as_ptr());
                    c1 = s1 * 3 + s2;

                    if tmp1[2] < bestdist - MJ_MINVAL {
                        bestdist = tmp1[2];
                        bestsegmentpos = x2;
                        bestboxpos = x1;
                        c2 = c1 / 6;
                        clcorner = i + (1 << j) * c2;
                        cledge = j;
                        cltype = c1;
                    }
                }
            }
        }

        // invalid type
        if cltype == -4 { return 0; }

        // determine second contact point (skip label equivalent)
        'skip: {
            if cltype >= 0 && cltype / 3 != 1 {
                // closest to corner
                c1 = axisdir ^ clcorner;
                if c1 == 0 || c1 == 7 { break 'skip; }

                if c1 == 1 || c1 == 2 || c1 == 4 {
                    mul = 1.0;
                    de = 1.0 - bestsegmentpos;
                    dp = 1.0 + bestsegmentpos;
                }
                if c1 == 3 || c1 == 5 || c1 == 6 {
                    mul = -1.0;
                    c1 = 7 - c1;
                    dp = 1.0 - bestsegmentpos;
                    de = 1.0 + bestsegmentpos;
                }

                if c1 == 1 { ax = 0; ax1 = 1; ax2 = 2; }
                if c1 == 2 { ax = 1; ax1 = 2; ax2 = 0; }
                if c1 == 4 { ax = 2; ax1 = 0; ax2 = 1; }

                if axis[ax as usize] * axis[ax as usize] > 0.5 {
                    secondpos = de;
                    e1 = 2.0 * *size2.add(ax as usize) / halfaxis[ax as usize].abs();
                    if e1 < secondpos { secondpos = e1; }
                    secondpos *= mul;
                } else {
                    secondpos = dp;
                    e1 = 2.0 * *size2.add(ax1 as usize) / halfaxis[ax1 as usize].abs();
                    if e1 < secondpos { secondpos = e1; }
                    e1 = 2.0 * *size2.add(ax2 as usize) / halfaxis[ax2 as usize].abs();
                    if e1 < secondpos { secondpos = e1; }
                    secondpos *= -mul;
                }
            } else if cltype >= 0 && cltype / 3 == 1 {
                // on box's edge
                c1 = axisdir ^ clcorner;
                c1 &= 7 - (1 << cledge);

                if c1 != 1 && c1 != 2 && c1 != 4 { break 'skip; }

                if cledge == 0 { ax1 = 1; ax2 = 2; }
                if cledge == 1 { ax1 = 2; ax2 = 0; }
                if cledge == 2 { ax1 = 0; ax2 = 1; }
                ax = cledge;

                if axis[ax1 as usize].abs() > axis[ax2 as usize].abs() {
                    ax1 = ax2;
                }
                ax2 = 3 - ax - ax1;

                if (c1 & (1 << ax2)) != 0 {
                    mul = 1.0;
                    secondpos = 1.0 - bestsegmentpos;
                } else {
                    mul = -1.0;
                    secondpos = 1.0 + bestsegmentpos;
                }

                e1 = 2.0 * *size2.add(ax2 as usize) / halfaxis[ax2 as usize].abs();
                if e1 < secondpos { secondpos = e1; }

                if ((axisdir & (1 << ax)) != 0) == ((c1 & (1 << ax2)) != 0) {
                    e2 = 1.0 - bestboxpos;
                } else {
                    e2 = 1.0 + bestboxpos;
                }
                e1 = *size2.add(ax as usize) * e2 / halfaxis[ax as usize].abs();
                if e1 < secondpos { secondpos = e1; }

                secondpos *= mul;
            } else if cltype < 0 {
                // face case
                if clface == -1 { break 'skip; }
                if cltype == -3 { mul = 1.0; } else { mul = -1.0; }
                secondpos = 2.0;

                crate::engine::engine_inline::mji_copy3(tmp1.as_mut_ptr(), pos.as_ptr());
                crate::engine::engine_inline::mji_add_to_scl3(tmp1.as_mut_ptr(), halfaxis.as_ptr(), -mul);

                for i in 0..3i32 {
                    if i != clface {
                        e1 = (*size2.add(i as usize) - tmp1[i as usize]) / halfaxis[i as usize] * mul;
                        if e1 > 0.0 {
                            if e1 < secondpos { secondpos = e1; }
                        }
                        e1 = (-*size2.add(i as usize) - tmp1[i as usize]) / halfaxis[i as usize] * mul;
                        if e1 > 0.0 {
                            if e1 < secondpos { secondpos = e1; }
                        }
                    }
                }
                secondpos *= mul;
            }
        } // end 'skip

        // create sphere at first contact point
        crate::engine::engine_util_blas::mju_copy3(tmp1.as_mut_ptr(), pos.as_ptr());
        crate::engine::engine_inline::mji_add_to_scl3(tmp1.as_mut_ptr(), halfaxis.as_ptr(), bestsegmentpos);
        crate::engine::engine_util_blas::mju_mul_mat_vec3(tmp2.as_mut_ptr(), mat2, tmp1.as_ptr());
        crate::engine::engine_util_blas::mju_add_to3(tmp2.as_mut_ptr(), pos2);

        n = crate::engine::engine_collision_box::mjraw_sphere_box(con, margin, tmp2.as_ptr(), mat1, size1, pos2, mat2, size2);

        if secondpos > -3.0 {
            crate::engine::engine_util_blas::mju_copy3(tmp1.as_mut_ptr(), pos.as_ptr());
            crate::engine::engine_inline::mji_add_to_scl3(tmp1.as_mut_ptr(), halfaxis.as_ptr(), secondpos + bestsegmentpos);
            crate::engine::engine_util_blas::mju_mul_mat_vec3(tmp2.as_mut_ptr(), mat2, tmp1.as_ptr());
            crate::engine::engine_util_blas::mju_add_to3(tmp2.as_mut_ptr(), pos2);
            n + crate::engine::engine_collision_box::mjraw_sphere_box(con.add(n as usize), margin, tmp2.as_ptr(), mat1, size1, pos2, mat2, size2)
        } else {
            n
        }
    }
}

/// C: mjraw_SphereTriangle (engine/engine_collision_primitive.h:37)
/// Calls: areaSign, mji_addScl3, mji_addToScl3, mji_copy3, mji_cross, mji_scl3, mju_dot3, mju_normalize3, mju_zero3, pointSegment
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjraw_sphere_triangle(con: *mut mjPreContact, margin: f64, s: *const f64, rs: f64, t1: *const f64, t2: *const f64, t3: *const f64, rt: f64) -> i32 {
    // SAFETY: caller guarantees all pointers are valid and arrays properly sized
    unsafe {
        let rbound = margin + rs + rt;
        let mut X: [f64; 3] = [0.0; 3];

        // make t1 the origin: triangle is (O,A,B); sphere center is S
        let S: [f64; 3] = [*s.add(0) - *t1.add(0), *s.add(1) - *t1.add(1), *s.add(2) - *t1.add(2)];
        let A: [f64; 3] = [*t2.add(0) - *t1.add(0), *t2.add(1) - *t1.add(1), *t2.add(2) - *t1.add(2)];
        let B: [f64; 3] = [*t3.add(0) - *t1.add(0), *t3.add(1) - *t1.add(1), *t3.add(2) - *t1.add(2)];

        // N is normal to triangle plane
        let mut N: [f64; 3] = [0.0; 3];
        crate::engine::engine_inline::mji_cross(N.as_mut_ptr(), A.as_ptr(), B.as_ptr());
        crate::engine::engine_util_blas::mju_normalize3(N.as_mut_ptr());

        // dstS is signed distance from S to plane; exit if too large
        let dstS = crate::engine::engine_util_blas::mju_dot3(N.as_ptr(), S.as_ptr());
        if dstS.abs() > rbound {
            return 0;
        }

        // P is projection of S in triangle plane
        let mut P: [f64; 3] = [0.0; 3];
        crate::engine::engine_inline::mji_add_scl3(P.as_mut_ptr(), S.as_ptr(), N.as_ptr(), -dstS);

        // construct orthogonal axes (V1~A, V2) of triangle plane
        let mut V1: [f64; 3] = [0.0; 3];
        let mut V2: [f64; 3] = [0.0; 3];
        crate::engine::engine_inline::mji_copy3(V1.as_mut_ptr(), A.as_ptr());
        let lenA = crate::engine::engine_util_blas::mju_normalize3(V1.as_mut_ptr());
        crate::engine::engine_inline::mji_cross(V2.as_mut_ptr(), N.as_ptr(), A.as_ptr());
        crate::engine::engine_util_blas::mju_normalize3(V2.as_mut_ptr());

        // triangle is (o,a,b), sphere center is p — in 2D
        let o: [f64; 2] = [0.0, 0.0];
        let a: [f64; 2] = [lenA, 0.0];
        let b: [f64; 2] = [
            crate::engine::engine_util_blas::mju_dot3(V1.as_ptr(), B.as_ptr()),
            crate::engine::engine_util_blas::mju_dot3(V2.as_ptr(), B.as_ptr()),
        ];
        let p: [f64; 2] = [
            crate::engine::engine_util_blas::mju_dot3(V1.as_ptr(), P.as_ptr()),
            crate::engine::engine_util_blas::mju_dot3(V2.as_ptr(), P.as_ptr()),
        ];

        // computed signs of areas
        let sign1 = area_sign(p.as_ptr(), o.as_ptr(), a.as_ptr());
        let sign2 = area_sign(p.as_ptr(), a.as_ptr(), b.as_ptr());
        let sign3 = area_sign(p.as_ptr(), b.as_ptr(), o.as_ptr());

        // p is inside triangle
        if sign1 == sign2 && sign2 == sign3 {
            crate::engine::engine_inline::mji_copy3(X.as_mut_ptr(), P.as_ptr());
        }
        // p is not inside triangle
        else {
            let mut x: [[f64; 2]; 3] = [[0.0; 2]; 3];
            let mut dstx: [f64; 3] = [0.0; 3];
            dstx[0] = point_segment(x[0].as_mut_ptr(), p.as_ptr(), o.as_ptr(), a.as_ptr());
            dstx[1] = point_segment(x[1].as_mut_ptr(), p.as_ptr(), a.as_ptr(), b.as_ptr());
            dstx[2] = point_segment(x[2].as_mut_ptr(), p.as_ptr(), b.as_ptr(), o.as_ptr());

            // select minimum
            let best: usize = if dstx[0] < dstx[1] && dstx[0] < dstx[2] {
                0
            } else if dstx[1] < dstx[2] {
                1
            } else {
                2
            };

            // convert x[best] to 3D
            crate::engine::engine_inline::mji_scl3(X.as_mut_ptr(), V1.as_ptr(), x[best][0]);
            crate::engine::engine_inline::mji_add_to_scl3(X.as_mut_ptr(), V2.as_ptr(), x[best][1]);
        }

        // compute contact normal and distance
        let mut nrm: [f64; 3] = [X[0] - S[0], X[1] - S[1], X[2] - S[2]];
        let dst = crate::engine::engine_util_blas::mju_normalize3(nrm.as_mut_ptr());

        // exit if too far
        if dst > rbound {
            return 0;
        }

        // construct contact
        (*con).dist = dst - rs - rt;
        crate::engine::engine_inline::mji_add_scl3((*con).pos.as_mut_ptr(), s, nrm.as_ptr(), rs + (*con).dist / 2.0);
        crate::engine::engine_inline::mji_copy3((*con).normal.as_mut_ptr(), nrm.as_ptr());
        crate::engine::engine_util_blas::mju_zero3((*con).tangent.as_mut_ptr());

        1
    }
}

/// C: mjraw_BoxTriangle (engine/engine_collision_primitive.h:39)
/// Calls: mji_addScl3, mjraw_SphereTriangle, mju_addTo3, mju_mulMatTVec3, mju_mulMatVec3, mju_sub3, mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjraw_box_triangle(con: *mut mjPreContact, margin: f64, pos: *const f64, mat: *const f64, size: *const f64, t1: *const f64, t2: *const f64, t3: *const f64, rt: f64) -> i32 {
    const MJ_MAXCONPAIR: i32 = 50;
    // SAFETY: caller guarantees all pointers valid and arrays properly sized
    unsafe {
        let mut cnt: i32 = 0;
        let vert: [*const f64; 3] = [t1, t2, t3];

        // check triangle vertices against box faces
        for i in 0..3usize {
            let mut diff: [f64; 3] = [0.0; 3];
            let mut local: [f64; 3] = [0.0; 3];
            crate::engine::engine_util_blas::mju_sub3(diff.as_mut_ptr(), vert[i], pos);
            crate::engine::engine_util_blas::mju_mul_mat_t_vec3(local.as_mut_ptr(), mat, diff.as_ptr());

            // find max penetration / closest face
            let mut maxaxis: usize = 0;
            let mut maxval = local[0].abs() - *size.add(0);
            for j in 1..3usize {
                let val = local[j].abs() - *size.add(j);
                if val > maxval {
                    maxval = val;
                    maxaxis = j;
                }
            }

            if maxval - rt > margin {
                continue;
            }

            // check if within other dimensions
            let mut inside = true;
            for j in 0..3usize {
                if local[j].abs() > *size.add(j) + margin + rt {
                    inside = false;
                    break;
                }
            }
            if !inside {
                continue;
            }

            // create contact
            if cnt < MJ_MAXCONPAIR {
                let mut nrm_local: [f64; 3] = [0.0, 0.0, 0.0];
                nrm_local[maxaxis] = if local[maxaxis] > 0.0 { 1.0 } else { -1.0 };

                crate::engine::engine_util_blas::mju_mul_mat_vec3(
                    (*con.add(cnt as usize)).normal.as_mut_ptr(), mat, nrm_local.as_ptr());

                (*con.add(cnt as usize)).dist = maxval - rt;

                let offset = rt + (*con.add(cnt as usize)).dist * 0.5;
                crate::engine::engine_inline::mji_add_scl3(
                    (*con.add(cnt as usize)).pos.as_mut_ptr(), vert[i],
                    (*con.add(cnt as usize)).normal.as_ptr(), -offset);

                crate::engine::engine_util_blas::mju_zero3((*con.add(cnt as usize)).tangent.as_mut_ptr());

                cnt += 1;
            }
        }

        // check box corners against triangle
        for i in 0..8usize {
            if cnt >= MJ_MAXCONPAIR {
                break;
            }

            let mut vec: [f64; 3] = [0.0; 3];
            vec[0] = if i & 1 != 0 { *size.add(0) } else { -*size.add(0) };
            vec[1] = if i & 2 != 0 { *size.add(1) } else { -*size.add(1) };
            vec[2] = if i & 4 != 0 { *size.add(2) } else { -*size.add(2) };

            let mut corner: [f64; 3] = [0.0; 3];
            crate::engine::engine_util_blas::mju_mul_mat_vec3(corner.as_mut_ptr(), mat, vec.as_ptr());
            crate::engine::engine_util_blas::mju_add_to3(corner.as_mut_ptr(), pos);

            if mjraw_sphere_triangle(con.add(cnt as usize), margin, corner.as_ptr(), 0.0, t1, t2, t3, rt) != 0 {
                cnt += 1;
            }
        }

        cnt
    }
}

/// C: mjraw_CapsuleTriangle (engine/engine_collision_primitive.h:42)
/// Calls: mji_add3, mji_addScl3, mji_addToScl3, mji_copy3, mjraw_SphereTriangle, mju_addScl3, mju_dot3, mju_normalize3, mju_scl3, mju_sub3, mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjraw_capsule_triangle(con: *mut mjPreContact, margin: f64, pos: *const f64, mat: *const f64, size: *const f64, t1: *const f64, t2: *const f64, t3: *const f64, rt: f64) -> i32 {
    const MJ_MAXCONPAIR: i32 = 50;
    const MJ_MINVAL: f64 = 1E-15;
    // SAFETY: caller guarantees all pointers valid and arrays properly sized
    unsafe {
        let mut cnt: i32 = 0;
        let radius = *size.add(0);
        let len = *size.add(1);
        let axis: [f64; 3] = [*mat.add(2), *mat.add(5), *mat.add(8)];
        let mut p1: [f64; 3] = [0.0; 3];
        let mut p2: [f64; 3] = [0.0; 3];

        // capsule endpoints
        crate::engine::engine_util_blas::mju_add_scl3(p1.as_mut_ptr(), pos, axis.as_ptr(), -len);
        crate::engine::engine_util_blas::mju_add_scl3(p2.as_mut_ptr(), pos, axis.as_ptr(), len);

        // Check endpoints against triangle
        cnt += mjraw_sphere_triangle(con.add(cnt as usize), margin, p1.as_ptr(), radius, t1, t2, t3, rt);
        if cnt >= MJ_MAXCONPAIR { return cnt; }
        cnt += mjraw_sphere_triangle(con.add(cnt as usize), margin, p2.as_ptr(), radius, t1, t2, t3, rt);
        if cnt >= MJ_MAXCONPAIR { return cnt; }

        // Check triangle vertices against capsule axis
        let vert: [*const f64; 3] = [t1, t2, t3];
        for i in 0..3usize {
            // point-segment distance
            let mut vec: [f64; 3] = [0.0; 3];
            let mut ab: [f64; 3] = [0.0; 3];
            crate::engine::engine_util_blas::mju_sub3(vec.as_mut_ptr(), vert[i], p1.as_ptr());
            crate::engine::engine_util_blas::mju_sub3(ab.as_mut_ptr(), p2.as_ptr(), p1.as_ptr());
            let t = crate::engine::engine_util_blas::mju_dot3(vec.as_ptr(), ab.as_ptr()) / (4.0 * len * len);

            // clamp t to [0, 1] segment (only process interior)
            if t <= MJ_MINVAL || t >= 1.0 - MJ_MINVAL {
                continue;
            }

            // closest point on segment
            let mut closest: [f64; 3] = [0.0; 3];
            crate::engine::engine_inline::mji_add_scl3(closest.as_mut_ptr(), p1.as_ptr(), ab.as_ptr(), t);

            // distance vector
            crate::engine::engine_util_blas::mju_sub3(vec.as_mut_ptr(), vert[i], closest.as_ptr());
            let dist = crate::engine::engine_util_blas::mju_normalize3(vec.as_mut_ptr());

            if dist > radius + rt + margin {
                continue;
            }

            // set contact
            (*con.add(cnt as usize)).dist = dist - radius - rt;
            crate::engine::engine_inline::mji_copy3((*con.add(cnt as usize)).normal.as_mut_ptr(), vec.as_ptr());
            crate::engine::engine_util_blas::mju_zero3((*con.add(cnt as usize)).tangent.as_mut_ptr());

            // position: midway between surfaces
            crate::engine::engine_inline::mji_add3((*con.add(cnt as usize)).pos.as_mut_ptr(), closest.as_ptr(), vert[i]);
            crate::engine::engine_inline::mji_add_to_scl3((*con.add(cnt as usize)).pos.as_mut_ptr(), vec.as_ptr(), radius - rt);
            crate::engine::engine_util_blas::mju_scl3(
                (*con.add(cnt as usize)).pos.as_mut_ptr(),
                (*con.add(cnt as usize)).pos.as_ptr(),
                0.5);

            cnt += 1;
            if cnt >= MJ_MAXCONPAIR { return cnt; }
        }

        cnt
    }
}

/// C: mjc_PlaneSphere (engine/engine_collision_primitive.h:47)
/// Calls: mjraw_PlaneSphere
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_plane_sphere(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    // SAFETY: m, d, con are valid pointers (caller contract). g1, g2 are valid geom indices.
    unsafe {
        let pos1 = (*d).geom_xpos.add(3 * g1 as usize);
        let pos2 = (*d).geom_xpos.add(3 * g2 as usize);
        let mat1 = (*d).geom_xmat.add(9 * g1 as usize);
        let mat2 = (*d).geom_xmat.add(9 * g2 as usize);
        let size1 = (*m).geom_size.add(3 * g1 as usize);
        let size2 = (*m).geom_size.add(3 * g2 as usize);
        mjraw_plane_sphere(con, margin, pos1, mat1, size1, pos2, mat2, size2)
    }
}

/// C: mjc_PlaneCapsule (engine/engine_collision_primitive.h:49)
/// Calls: mji_copy3, mjraw_PlaneSphere, mju_add3, mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_plane_capsule(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    // SAFETY: m, d, con are valid pointers (caller contract). g1, g2 are valid geom indices.
    unsafe {
        let pos1 = (*d).geom_xpos.add(3 * g1 as usize);
        let pos2 = (*d).geom_xpos.add(3 * g2 as usize);
        let mat1 = (*d).geom_xmat.add(9 * g1 as usize);
        let mat2 = (*d).geom_xmat.add(9 * g2 as usize);
        let size1 = (*m).geom_size.add(3 * g1 as usize);
        let size2 = (*m).geom_size.add(3 * g2 as usize);

        // get capsule axis, segment = scaled axis
        let axis: [f64; 3] = [*mat2.add(2), *mat2.add(5), *mat2.add(8)];
        let segment: [f64; 3] = [
            *size2.add(1) * axis[0],
            *size2.add(1) * axis[1],
            *size2.add(1) * axis[2],
        ];

        // get point 1, do sphere-plane test
        let mut endpoint: [f64; 3] = [0.0; 3];
        crate::engine::engine_util_blas::mju_add3(endpoint.as_mut_ptr(), pos2, segment.as_ptr());
        let n1 = mjraw_plane_sphere(con, margin, pos1, mat1, size1, endpoint.as_ptr(), mat2, size2);

        // get point 2, do sphere-plane test
        crate::engine::engine_util_blas::mju_sub3(endpoint.as_mut_ptr(), pos2, segment.as_ptr());
        let n2 = mjraw_plane_sphere(con.add(n1 as usize), margin, pos1, mat1, size1, endpoint.as_ptr(), mat2, size2);

        // align contact frames with capsule axis
        if n1 != 0 {
            crate::engine::engine_inline::mji_copy3((*con).tangent.as_mut_ptr(), axis.as_ptr());
        }
        if n2 != 0 {
            crate::engine::engine_inline::mji_copy3((*con.add(n1 as usize)).tangent.as_mut_ptr(), axis.as_ptr());
        }

        n1 + n2
    }
}

/// C: mjc_PlaneCylinder (engine/engine_collision_primitive.h:51)
/// Calls: mji_add3, mji_addTo3, mji_addToScl3, mji_copy3, mji_cross, mji_sub3, mji_subFrom3, mji_zero3, mju_dot3, mju_normalize3, mju_scl3, mju_subFrom3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_plane_cylinder(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    const MJMINVAL: f64 = 1e-15;

    // SAFETY: m, d, con are valid pointers (caller contract). g1, g2 are valid geom indices.
    unsafe {
        let pos1 = (*d).geom_xpos.add(3 * g1 as usize);
        let pos2 = (*d).geom_xpos.add(3 * g2 as usize);
        let mat1 = (*d).geom_xmat.add(9 * g1 as usize);
        let mat2 = (*d).geom_xmat.add(9 * g2 as usize);
        let size2 = (*m).geom_size.add(3 * g2 as usize);

        let normal: [f64; 3] = [*mat1.add(2), *mat1.add(5), *mat1.add(8)];
        let mut axis: [f64; 3] = [*mat2.add(2), *mat2.add(5), *mat2.add(8)];

        // project, make sure axis points towards plane
        let mut prjaxis = crate::engine::engine_util_blas::mju_dot3(normal.as_ptr(), axis.as_ptr());
        if prjaxis > 0.0 {
            crate::engine::engine_util_blas::mju_scl3(axis.as_mut_ptr(), axis.as_ptr(), -1.0);
            prjaxis = -prjaxis;
        }

        // compute normal distance to cylinder center
        let mut vec: [f64; 3] = [
            *pos2.add(0) - *pos1.add(0),
            *pos2.add(1) - *pos1.add(1),
            *pos2.add(2) - *pos1.add(2),
        ];
        let dist0 = crate::engine::engine_util_blas::mju_dot3(vec.as_ptr(), normal.as_ptr());

        // remove component of -normal along axis, compute length
        crate::engine::engine_inline::mji_scl3(vec.as_mut_ptr(), axis.as_ptr(), prjaxis);
        crate::engine::engine_inline::mji_sub_from3(vec.as_mut_ptr(), normal.as_ptr());
        let len_sqr = crate::engine::engine_util_blas::mju_dot3(vec.as_ptr(), vec.as_ptr());

        // general configuration: normalize vector, scale by radius
        if len_sqr >= MJMINVAL * MJMINVAL {
            let scl = *size2.add(0) / f64::sqrt(len_sqr);
            vec[0] *= scl;
            vec[1] *= scl;
            vec[2] *= scl;
        }
        // disk parallel to plane: pick x-axis of cylinder, scale by radius
        else {
            vec[0] = *mat2.add(0) * *size2.add(0);
            vec[1] = *mat2.add(3) * *size2.add(0);
            vec[2] = *mat2.add(6) * *size2.add(0);
        }

        // project vector on normal
        let prjvec = crate::engine::engine_util_blas::mju_dot3(vec.as_ptr(), normal.as_ptr());

        // scale axis by half-length
        crate::engine::engine_util_blas::mju_scl3(axis.as_mut_ptr(), axis.as_ptr(), *size2.add(1));
        let prjaxis_scaled = prjaxis * *size2.add(1);

        // check first point, construct contact
        let mut cnt: i32 = 0;
        if dist0 + prjaxis_scaled + prjvec <= margin {
            (*con.add(cnt as usize)).dist = dist0 + prjaxis_scaled + prjvec;
            crate::engine::engine_inline::mji_add3(
                (*con.add(cnt as usize)).pos.as_mut_ptr(), pos2, vec.as_ptr());
            crate::engine::engine_inline::mji_add_to3(
                (*con.add(cnt as usize)).pos.as_mut_ptr(), axis.as_ptr());
            crate::engine::engine_inline::mji_add_to_scl3(
                (*con.add(cnt as usize)).pos.as_mut_ptr(), normal.as_ptr(),
                -(*con.add(cnt as usize)).dist * 0.5);
            crate::engine::engine_inline::mji_copy3(
                (*con.add(cnt as usize)).normal.as_mut_ptr(), normal.as_ptr());
            crate::engine::engine_inline::mji_zero3(
                (*con.add(cnt as usize)).tangent.as_mut_ptr());
            cnt += 1;
        } else {
            return 0; // nearest point is above margin: no contacts
        }

        // check second point, construct contact
        if dist0 - prjaxis_scaled + prjvec <= margin {
            (*con.add(cnt as usize)).dist = dist0 - prjaxis_scaled + prjvec;
            crate::engine::engine_inline::mji_add3(
                (*con.add(cnt as usize)).pos.as_mut_ptr(), pos2, vec.as_ptr());
            crate::engine::engine_inline::mji_sub_from3(
                (*con.add(cnt as usize)).pos.as_mut_ptr(), axis.as_ptr());
            crate::engine::engine_inline::mji_add_to_scl3(
                (*con.add(cnt as usize)).pos.as_mut_ptr(), normal.as_ptr(),
                -(*con.add(cnt as usize)).dist * 0.5);
            crate::engine::engine_inline::mji_copy3(
                (*con.add(cnt as usize)).normal.as_mut_ptr(), normal.as_ptr());
            crate::engine::engine_inline::mji_zero3(
                (*con.add(cnt as usize)).tangent.as_mut_ptr());
            cnt += 1;
        }

        // try to add triangle points on side closer to plane
        let prjvec1 = -prjvec * 0.5;
        if dist0 + prjaxis_scaled + prjvec1 <= margin {
            // compute sideways vector: vec1
            let mut vec1: [f64; 3] = [0.0; 3];
            crate::engine::engine_inline::mji_cross(vec1.as_mut_ptr(), vec.as_ptr(), axis.as_ptr());
            crate::engine::engine_util_blas::mju_normalize3(vec1.as_mut_ptr());
            crate::engine::engine_util_blas::mju_scl3(
                vec1.as_mut_ptr(), vec1.as_ptr(), *size2.add(0) * f64::sqrt(3.0) / 2.0);

            // add point A
            (*con.add(cnt as usize)).dist = dist0 + prjaxis_scaled + prjvec1;
            crate::engine::engine_inline::mji_add3(
                (*con.add(cnt as usize)).pos.as_mut_ptr(), pos2, vec1.as_ptr());
            crate::engine::engine_inline::mji_add_to3(
                (*con.add(cnt as usize)).pos.as_mut_ptr(), axis.as_ptr());
            crate::engine::engine_inline::mji_add_to_scl3(
                (*con.add(cnt as usize)).pos.as_mut_ptr(), vec.as_ptr(), -0.5);
            crate::engine::engine_inline::mji_add_to_scl3(
                (*con.add(cnt as usize)).pos.as_mut_ptr(), normal.as_ptr(),
                -(*con.add(cnt as usize)).dist * 0.5);
            crate::engine::engine_inline::mji_copy3(
                (*con.add(cnt as usize)).normal.as_mut_ptr(), normal.as_ptr());
            crate::engine::engine_inline::mji_zero3(
                (*con.add(cnt as usize)).tangent.as_mut_ptr());
            cnt += 1;

            // add point B
            (*con.add(cnt as usize)).dist = dist0 + prjaxis_scaled + prjvec1;
            crate::engine::engine_inline::mji_sub3(
                (*con.add(cnt as usize)).pos.as_mut_ptr(), pos2, vec1.as_ptr());
            crate::engine::engine_inline::mji_add_to3(
                (*con.add(cnt as usize)).pos.as_mut_ptr(), axis.as_ptr());
            crate::engine::engine_inline::mji_add_to_scl3(
                (*con.add(cnt as usize)).pos.as_mut_ptr(), vec.as_ptr(), -0.5);
            crate::engine::engine_inline::mji_add_to_scl3(
                (*con.add(cnt as usize)).pos.as_mut_ptr(), normal.as_ptr(),
                -(*con.add(cnt as usize)).dist * 0.5);
            crate::engine::engine_inline::mji_copy3(
                (*con.add(cnt as usize)).normal.as_mut_ptr(), normal.as_ptr());
            crate::engine::engine_inline::mji_zero3(
                (*con.add(cnt as usize)).tangent.as_mut_ptr());
            cnt += 1;
        }

        cnt
    }
}

/// C: mjc_PlaneBox (engine/engine_collision_primitive.h:53)
/// Calls: mji_add3, mji_addTo3, mji_copy3, mji_scl3, mji_zero3, mju_dot3, mju_mulMatVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_plane_box(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    // SAFETY: m, d, con are valid pointers (caller contract). g1, g2 are valid geom indices.
    unsafe {
        let pos1 = (*d).geom_xpos.add(3 * g1 as usize);
        let pos2 = (*d).geom_xpos.add(3 * g2 as usize);
        let mat1 = (*d).geom_xmat.add(9 * g1 as usize);
        let mat2 = (*d).geom_xmat.add(9 * g2 as usize);
        let size2 = (*m).geom_size.add(3 * g2 as usize);

        // get normal, difference between centers, normal distance
        let norm: [f64; 3] = [*mat1.add(2), *mat1.add(5), *mat1.add(8)];
        let dif: [f64; 3] = [
            *pos2.add(0) - *pos1.add(0),
            *pos2.add(1) - *pos1.add(1),
            *pos2.add(2) - *pos1.add(2),
        ];
        let dist = crate::engine::engine_util_blas::mju_dot3(dif.as_ptr(), norm.as_ptr());

        // test all corners, pick bottom 4
        let mut cnt: i32 = 0;
        for i in 0..8i32 {
            // get corner in local coordinates
            let mut vec: [f64; 3] = [0.0; 3];
            vec[0] = if (i & 1) != 0 { *size2.add(0) } else { -*size2.add(0) };
            vec[1] = if (i & 2) != 0 { *size2.add(1) } else { -*size2.add(1) };
            vec[2] = if (i & 4) != 0 { *size2.add(2) } else { -*size2.add(2) };

            // get corner in global coordinates relative to box center
            let mut corner: [f64; 3] = [0.0; 3];
            crate::engine::engine_util_blas::mju_mul_mat_vec3(
                corner.as_mut_ptr(), mat2, vec.as_ptr());

            // compute distance to plane, skip if too far or pointing up
            let ldist = crate::engine::engine_util_blas::mju_dot3(norm.as_ptr(), corner.as_ptr());
            if dist + ldist > margin || ldist > 0.0 {
                continue;
            }

            // construct contact
            (*con.add(cnt as usize)).dist = dist + ldist;
            crate::engine::engine_inline::mji_copy3(
                (*con.add(cnt as usize)).normal.as_mut_ptr(), norm.as_ptr());
            crate::engine::engine_inline::mji_add_to3(corner.as_mut_ptr(), pos2);
            crate::engine::engine_inline::mji_scl3(
                vec.as_mut_ptr(), norm.as_ptr(), -(*con.add(cnt as usize)).dist / 2.0);
            crate::engine::engine_inline::mji_add3(
                (*con.add(cnt as usize)).pos.as_mut_ptr(), corner.as_ptr(), vec.as_ptr());
            crate::engine::engine_inline::mji_zero3(
                (*con.add(cnt as usize)).tangent.as_mut_ptr());

            // count; max is 4
            cnt += 1;
            if cnt >= 4 {
                return 4;
            }
        }

        cnt
    }
}

/// C: mjc_SphereSphere (engine/engine_collision_primitive.h:57)
/// Calls: mjraw_SphereSphere
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_sphere_sphere(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    // SAFETY: m, d, con are valid pointers (caller contract). g1, g2 are valid geom indices.
    unsafe {
        let pos1 = (*d).geom_xpos.add(3 * g1 as usize);
        let mat1 = (*d).geom_xmat.add(9 * g1 as usize);
        let size1 = (*m).geom_size.add(3 * g1 as usize);
        let pos2 = (*d).geom_xpos.add(3 * g2 as usize);
        let mat2 = (*d).geom_xmat.add(9 * g2 as usize);
        let size2 = (*m).geom_size.add(3 * g2 as usize);
        mjraw_sphere_sphere(con, margin, pos1, mat1, size1, pos2, mat2, size2)
    }
}

/// C: mjc_SphereCapsule (engine/engine_collision_primitive.h:59)
/// Calls: mjraw_SphereCapsule
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_sphere_capsule(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    // SAFETY: m, d, con are valid pointers (caller contract). g1, g2 are valid geom indices.
    unsafe {
        let pos1 = (*d).geom_xpos.add(3 * g1 as usize);
        let pos2 = (*d).geom_xpos.add(3 * g2 as usize);
        let mat1 = (*d).geom_xmat.add(9 * g1 as usize);
        let mat2 = (*d).geom_xmat.add(9 * g2 as usize);
        let size1 = (*m).geom_size.add(3 * g1 as usize);
        let size2 = (*m).geom_size.add(3 * g2 as usize);
        mjraw_sphere_capsule(con, margin, pos1, mat1, size1, pos2, mat2, size2)
    }
}

/// C: mjc_SphereCylinder (engine/engine_collision_primitive.h:61)
/// Calls: mji_addTo3, mji_scl3, mji_sub3, mjraw_PlaneSphere, mjraw_SphereSphere, mju_addScl3, mju_dot3, mju_scl3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_sphere_cylinder(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    // SAFETY: m, d, con are valid pointers (caller contract). g1, g2 are valid geom indices.
    unsafe {
        let pos1 = (*d).geom_xpos.add(3 * g1 as usize);
        let pos2 = (*d).geom_xpos.add(3 * g2 as usize);
        let mat1 = (*d).geom_xmat.add(9 * g1 as usize);
        let mat2 = (*d).geom_xmat.add(9 * g2 as usize);
        let size1 = (*m).geom_size.add(3 * g1 as usize);
        let size2 = (*m).geom_size.add(3 * g2 as usize);

        // get cylinder sizes and axis
        let radius = *size2.add(0);
        let height = *size2.add(1);
        let axis: [f64; 3] = [*mat2.add(2), *mat2.add(5), *mat2.add(8)];

        // find sphere projection onto cylinder axis and plane
        let vec: [f64; 3] = [
            *pos1.add(0) - *pos2.add(0),
            *pos1.add(1) - *pos2.add(1),
            *pos1.add(2) - *pos2.add(2),
        ];
        let x = crate::engine::engine_util_blas::mju_dot3(axis.as_ptr(), vec.as_ptr());
        let mut a_proj: [f64; 3] = [0.0; 3];
        let mut p_proj: [f64; 3] = [0.0; 3];
        crate::engine::engine_inline::mji_scl3(a_proj.as_mut_ptr(), axis.as_ptr(), x);
        crate::engine::engine_inline::mji_sub3(p_proj.as_mut_ptr(), vec.as_ptr(), a_proj.as_ptr());
        let p_proj_sqr = crate::engine::engine_util_blas::mju_dot3(p_proj.as_ptr(), p_proj.as_ptr());

        // get collision type
        let mut collide_side: i32 = if x.abs() < height { 1 } else { 0 };
        let mut collide_cap: i32 = if p_proj_sqr < radius * radius { 1 } else { 0 };
        if collide_side != 0 && collide_cap != 0 {
            // deep penetration
            let dist_cap = height - x.abs();
            let dist_radius = radius - f64::sqrt(p_proj_sqr);
            if dist_cap < dist_radius {
                collide_side = 0;
            } else {
                collide_cap = 0;
            }
        }

        // side collision: use sphere-sphere
        if collide_side != 0 {
            crate::engine::engine_inline::mji_add_to3(a_proj.as_mut_ptr(), pos2);
            return mjraw_sphere_sphere(con, margin, pos1, mat1, size1, a_proj.as_ptr(), mat2, size2);
        }

        // cap collision: use plane-sphere
        if collide_cap != 0 {
            let flipmat: [f64; 9] = [
                -*mat2.add(0), *mat2.add(1), -*mat2.add(2),
                -*mat2.add(3), *mat2.add(4), -*mat2.add(5),
                -*mat2.add(6), *mat2.add(7), -*mat2.add(8),
            ];
            let mat_cap: *const f64;
            let mut pos_cap: [f64; 3] = [0.0; 3];
            if x > 0.0 {
                // top cap
                crate::engine::engine_util_blas::mju_add_scl3(
                    pos_cap.as_mut_ptr(), pos2, axis.as_ptr(), height);
                mat_cap = mat2;
            } else {
                // bottom cap
                crate::engine::engine_util_blas::mju_add_scl3(
                    pos_cap.as_mut_ptr(), pos2, axis.as_ptr(), -height);
                mat_cap = flipmat.as_ptr();
            }
            let ncon = mjraw_plane_sphere(
                con, margin, pos_cap.as_ptr(), mat_cap, size2, pos1, mat1, size1);
            if ncon != 0 {
                // flip direction normal
                crate::engine::engine_util_blas::mju_scl3(
                    (*con).normal.as_mut_ptr(), (*con).normal.as_ptr(), -1.0);
            }
            return ncon;
        }

        // otherwise corner collision: use sphere-sphere
        crate::engine::engine_util_blas::mju_scl3(
            p_proj.as_mut_ptr(), p_proj.as_ptr(),
            *size2.add(0) / f64::sqrt(p_proj_sqr));
        let mut corner: [f64; 3] = [0.0; 3];
        crate::engine::engine_inline::mji_scl3(
            corner.as_mut_ptr(), axis.as_ptr(), if x > 0.0 { height } else { -height });
        crate::engine::engine_inline::mji_add_to3(corner.as_mut_ptr(), p_proj.as_ptr());
        crate::engine::engine_inline::mji_add_to3(corner.as_mut_ptr(), pos2);

        // sphere-sphere with point sphere at the corner
        let size_zero: [f64; 1] = [0.0];
        mjraw_sphere_sphere(con, margin, pos1, mat1, size1, corner.as_ptr(), mat2, size_zero.as_ptr())
    }
}

/// C: mjc_CapsuleCapsule (engine/engine_collision_primitive.h:63)
/// Calls: mjraw_CapsuleCapsule
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_capsule_capsule(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    // SAFETY: m, d, con are valid pointers (caller contract). g1, g2 are valid geom indices.
    unsafe {
        let pos1 = (*d).geom_xpos.add(3 * g1 as usize);
        let pos2 = (*d).geom_xpos.add(3 * g2 as usize);
        let mat1 = (*d).geom_xmat.add(9 * g1 as usize);
        let mat2 = (*d).geom_xmat.add(9 * g2 as usize);
        let size1 = (*m).geom_size.add(3 * g1 as usize);
        let size2 = (*m).geom_size.add(3 * g2 as usize);
        mjraw_capsule_capsule(con, margin, pos1, mat1, size1, pos2, mat2, size2)
    }
}

/// C: mjc_CapsuleBox (engine/engine_collision_primitive.h:67)
/// Calls: mjraw_CapsuleBox
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_capsule_box(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    // SAFETY: m, d, con are valid pointers (caller contract). g1, g2 are valid geom indices.
    unsafe {
        let pos1 = (*d).geom_xpos.add(3 * g1 as usize);
        let pos2 = (*d).geom_xpos.add(3 * g2 as usize);
        let mat1 = (*d).geom_xmat.add(9 * g1 as usize);
        let mat2 = (*d).geom_xmat.add(9 * g2 as usize);
        let size1 = (*m).geom_size.add(3 * g1 as usize);
        let size2 = (*m).geom_size.add(3 * g2 as usize);
        mjraw_capsule_box(con, margin, pos1, mat1, size1, pos2, mat2, size2)
    }
}

/// C: mjc_SphereBox (engine/engine_collision_primitive.h:69)
/// Calls: mjraw_SphereBox
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_sphere_box(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    // SAFETY: m, d, con are valid pointers (caller contract). g1, g2 are valid geom indices.
    unsafe {
        let pos1 = (*d).geom_xpos.add(3 * g1 as usize);
        let mat1 = (*d).geom_xmat.add(9 * g1 as usize);
        let size1 = (*m).geom_size.add(3 * g1 as usize);
        let pos2 = (*d).geom_xpos.add(3 * g2 as usize);
        let mat2 = (*d).geom_xmat.add(9 * g2 as usize);
        let size2 = (*m).geom_size.add(3 * g2 as usize);
        crate::engine::engine_collision_box::mjraw_sphere_box(
            con, margin, pos1, mat1, size1, pos2, mat2, size2)
    }
}

/// C: mjc_BoxBox (engine/engine_collision_primitive.h:71)
/// Calls: _boxbox, mju_outsideBox
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_box_box(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    todo!() // mjc_BoxBox
}

