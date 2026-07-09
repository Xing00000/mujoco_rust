//! Port of: engine/engine_collision_primitive.c
//! IR hash: 05737965add36adb
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
    // SAFETY: caller guarantees all pointers are valid; con points to at least 1 mjPreContact,
    // pos/size are f64[3], mat is f64[9]
    unsafe {
        // con[0].normal = third column of mat1 (mat1[2], mat1[5], mat1[8])
        (*con.add(0)).normal[0] = *mat1.add(2);
        (*con.add(0)).normal[1] = *mat1.add(5);
        (*con.add(0)).normal[2] = *mat1.add(8);

        // tmp = pos2 - pos1
        let tmp: [f64; 3] = [
            *pos2.add(0) - *pos1.add(0),
            *pos2.add(1) - *pos1.add(1),
            *pos2.add(2) - *pos1.add(2),
        ];

        // cdist = dot(tmp, normal)
        let cdist: f64 = crate::engine::engine_util_blas::mju_dot3(
            tmp.as_ptr(),
            (*con.add(0)).normal.as_ptr(),
        );

        // early out
        if cdist > margin + *size2.add(0) {
            return 0;
        }

        // con[0].dist = cdist - size2[0]
        (*con.add(0)).dist = cdist - *size2.add(0);

        // tmp2 = normal * (-dist/2 - size2[0])
        let scl: f64 = -(*con.add(0)).dist / 2.0 - *size2.add(0);
        let mut tmp2: [f64; 3] = [0.0; 3];
        crate::engine::engine_inline::mji_scl3(
            tmp2.as_mut_ptr(),
            (*con.add(0)).normal.as_ptr(),
            scl,
        );

        // con[0].pos = pos2 + tmp2
        crate::engine::engine_inline::mji_add3(
            (*con.add(0)).pos.as_mut_ptr(),
            pos2,
            tmp2.as_ptr(),
        );

        // con[0].tangent = {0, 0, 0}
        crate::engine::engine_inline::mji_zero3((*con.add(0)).tangent.as_mut_ptr());

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
    // SAFETY: all pointers are valid; pos/size are f64[3], mat is f64[9], con is mjPreContact array
    unsafe {
        const MJ_MINVAL: f64 = 1e-15;

        // check bounding spheres (this is called from other functions)
        let dif: [f64; 3] = [
            *pos1.add(0) - *pos2.add(0),
            *pos1.add(1) - *pos2.add(1),
            *pos1.add(2) - *pos2.add(2),
        ];
        let cdist_sqr: f64 = crate::engine::engine_util_blas::mju_dot3(dif.as_ptr(), dif.as_ptr());
        let min_dist: f64 = margin + *size1.add(0) + *size2.add(0);
        if cdist_sqr > min_dist * min_dist {
            return 0;
        }

        // depth and normal
        (*con.add(0)).dist = cdist_sqr.sqrt() - *size1.add(0) - *size2.add(0);
        crate::engine::engine_inline::mji_sub3((*con.add(0)).normal.as_mut_ptr(), pos2, pos1);
        let len: f64 = crate::engine::engine_util_blas::mju_normalize3((*con.add(0)).normal.as_mut_ptr());

        // if centers are the same, norm = cross-product of z axes
        //  if z axes are parallel, norm = [1;0;0]
        if len < MJ_MINVAL {
            let axis1: [f64; 3] = [*mat1.add(2), *mat1.add(5), *mat1.add(8)];
            let axis2: [f64; 3] = [*mat2.add(2), *mat2.add(5), *mat2.add(8)];
            crate::engine::engine_inline::mji_cross((*con.add(0)).normal.as_mut_ptr(), axis1.as_ptr(), axis2.as_ptr());
            crate::engine::engine_util_blas::mju_normalize3((*con.add(0)).normal.as_mut_ptr());
        }

        // position
        crate::engine::engine_inline::mji_scl3(
            (*con.add(0)).pos.as_mut_ptr(),
            (*con.add(0)).normal.as_ptr(),
            *size1.add(0) + (*con.add(0)).dist / 2.0,
        );
        crate::engine::engine_inline::mji_add_to3((*con.add(0)).pos.as_mut_ptr(), pos1);

        // axis
        crate::engine::engine_inline::mji_zero3((*con.add(0)).tangent.as_mut_ptr());

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
    unsafe {
        // SAFETY: p1, p2, p3 each point to at least 2 f64 elements
        crate::engine::engine_util_misc::mju_sign(
            (*p1.add(0) - *p3.add(0)) * (*p2.add(1) - *p3.add(1))
            - (*p2.add(0) - *p3.add(0)) * (*p1.add(1) - *p3.add(1))
        )
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
    unsafe {
        // SAFETY: res, p, u, v each point to at least 2 f64 elements
        const MJ_MINVAL: f64 = 1e-15;

        // make u the origin
        let uv: [f64; 2] = [*v.add(0) - *u.add(0), *v.add(1) - *u.add(1)];
        let up: [f64; 2] = [*p.add(0) - *u.add(0), *p.add(1) - *u.add(1)];

        // project: find a s.t. uv is orthogonal to (up-a*uv)
        let a = crate::engine::engine_util_blas::mju_dot(uv.as_ptr(), up.as_ptr(), 2)
            / crate::engine::engine_util_misc::mju_max(
                MJ_MINVAL,
                crate::engine::engine_util_blas::mju_dot(uv.as_ptr(), uv.as_ptr(), 2),
            );

        // find nearest point to p, clamp to u or v if a is not in (0,1)
        if a <= 0.0 {
            *res.add(0) = *u.add(0);
            *res.add(1) = *u.add(1);
        } else if a >= 1.0 {
            *res.add(0) = *v.add(0);
            *res.add(1) = *v.add(1);
        } else {
            crate::engine::engine_util_blas::mju_add_scl(res, u, uv.as_ptr(), a, 2);
        }

        // compute distance
        ((*res.add(0) - *p.add(0)) * (*res.add(0) - *p.add(0))
            + (*res.add(1) - *p.add(1)) * (*res.add(1) - *p.add(1)))
        .sqrt()
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
    // SAFETY: all pointers are valid and properly aligned per caller contract.
    // pos1, pos2, mat2, size2 each point to sufficient f64 elements.
    unsafe {
        // get capsule length and axis
        let len = *size2.add(1);
        let mut axis: [f64; 3] = [*mat2.add(2), *mat2.add(5), *mat2.add(8)];

        // find projection, clip to segment
        let mut vec: [f64; 3] = [
            *pos1.add(0) - *pos2.add(0),
            *pos1.add(1) - *pos2.add(1),
            *pos1.add(2) - *pos2.add(2),
        ];
        let x = crate::engine::engine_util_misc::mju_clip(
            crate::engine::engine_util_blas::mju_dot3(axis.as_ptr(), vec.as_ptr()),
            -len,
            len,
        );

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
    // SAFETY: all pointers valid and properly aligned per caller contract.
    // con points to buffer with space for at least 4 mjPreContact entries.
    unsafe {
        const MJMINVAL: f64 = 1e-15;

        // get capsule axes (scaled) and center difference
        let mut axis1: [f64; 3] = [
            *mat1.add(2) * *size1.add(1),
            *mat1.add(5) * *size1.add(1),
            *mat1.add(8) * *size1.add(1),
        ];
        let mut axis2: [f64; 3] = [
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
        if det.abs() >= MJMINVAL {
            // find projections, clip to segments
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

        // return if two contacts already found
        if n1 + n2 >= 2 {
            return n1 + n2;
        }

        // x2 = 1
        crate::engine::engine_inline::mji_add3(vec2.as_mut_ptr(), pos2, axis2.as_ptr());
        let mut x1 = crate::engine::engine_util_misc::mju_clip((u - mb) / ma, -1.0, 1.0);
        crate::engine::engine_inline::mji_scl3(vec1.as_mut_ptr(), axis1.as_ptr(), x1);
        crate::engine::engine_inline::mji_add_to3(vec1.as_mut_ptr(), pos1);
        let n3 = mjraw_sphere_sphere(con.add((n1 + n2) as usize), margin, vec1.as_ptr(), mat1, size1, vec2.as_ptr(), mat2, size2);

        // return if two contacts already found
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
#[allow(unused_variables, unused_assignments, non_snake_case)]
pub fn mjraw_capsule_box(con: *mut mjPreContact, margin: f64, pos1: *const f64, mat1: *const f64, size1: *const f64, pos2: *const f64, mat2: *const f64, size2: *const f64) -> i32 {
    use crate::engine::engine_inline::{mji_sub3, mji_mul_mat_t_vec3, mji_copy3, mji_add_to_scl3, mji_scl3, mji_sub_from3};
    use crate::engine::engine_util_blas::{mju_zero3, mju_dot3, mju_copy3, mju_add_to3, mju_mul_mat_vec3};
    use crate::engine::engine_collision_box::mjraw_sphere_box;

    const mjMINVAL: f64 = 1e-15;

    // SAFETY: caller guarantees all pointers are valid; con points to sufficient mjPreContact slots,
    // pos/size are f64[3], mat is f64[9]. All pointer arithmetic within bounds per C contract.
    unsafe {
        let mut tmp1: [f64; 3] = [0.0; 3];
        let mut tmp2: [f64; 3] = [0.0; 3];
        let mut tmp3: [f64; 3] = [0.0; 3];
        let mut halfaxis: [f64; 3] = [0.0; 3];
        let mut axis: [f64; 3] = [0.0; 3];
        let mut dif: [f64; 3] = [0.0; 3];
        let mut pos: [f64; 3] = [0.0; 3];

        let halflength: f64;
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
        let mut u: f64;
        let mut v: f64;
        let mut det: f64;
        let mut x1: f64;
        let mut x2: f64;
        let mut idet: f64;

        let mut s1: i32;
        let mut s2: i32;
        let mut i: i32;
        let mut j: i32;
        let mut c1: i32;
        let mut c2: i32;
        let mut cltype: i32 = -4;
        let mut clface: i32 = 0;
        let mut clcorner: i32 = 0;
        let mut cledge: i32 = 0;
        let mut axisdir: i32;
        let n: i32;
        let mut ax1: i32 = 0;
        let mut ax2: i32 = 0;
        let mut ax: i32 = 0;

        halflength = *size1.add(1);
        secondpos = -4.0;

        mji_sub3(tmp1.as_mut_ptr(), pos1, pos2);
        mji_mul_mat_t_vec3(pos.as_mut_ptr(), mat2, tmp1.as_ptr());

        tmp1[0] = *mat1.add(2);
        tmp1[1] = *mat1.add(5);
        tmp1[2] = *mat1.add(8);

        mji_mul_mat_t_vec3(axis.as_mut_ptr(), mat2, tmp1.as_ptr());
        mji_scl3(halfaxis.as_mut_ptr(), axis.as_ptr(), halflength);

        axisdir = 0;
        if halfaxis[0] > 0.0 {
            axisdir += 1;
        }
        if halfaxis[1] > 0.0 {
            axisdir += 2;
        }
        if halfaxis[2] > 0.0 {
            axisdir += 4;
        }

        bestdistmax = margin + 2.0 * (*size1.add(0) + halflength + *size2.add(0) + *size2.add(1) +
                                      *size2.add(2));
        bestdist = bestdistmax;
        bestsegmentpos = 0.0;

        mju_zero3(tmp2.as_mut_ptr());

        // test to see if maybe a face of the box is closest to the capsule
        i = -1;
        while i <= 1 {
            mji_copy3(tmp1.as_mut_ptr(), pos.as_ptr());
            mji_add_to_scl3(tmp1.as_mut_ptr(), halfaxis.as_ptr(), i as f64);
            mji_copy3(tmp2.as_mut_ptr(), tmp1.as_ptr());

            c1 = 0;
            j = 0;
            c2 = -1;
            while j < 3 {
                if tmp1[j as usize] < -*size2.add(j as usize) {
                    c1 += 1;
                    c2 = j;
                    tmp1[j as usize] = -*size2.add(j as usize);
                } else if tmp1[j as usize] > *size2.add(j as usize) {
                    c1 += 1;
                    c2 = j;
                    tmp1[j as usize] = *size2.add(j as usize);
                }
                j += 1;
            }

            if c1 > 1 {
                i += 2;
                continue;
            }

            mji_sub_from3(tmp1.as_mut_ptr(), tmp2.as_ptr());
            dist = mju_dot3(tmp1.as_ptr(), tmp1.as_ptr());

            if dist < bestdist {
                bestdist = dist;
                bestsegmentpos = i as f64;
                cltype = -2 + i;
                clface = c2;
            }

            i += 2;
        }

        mju_zero3(tmp2.as_mut_ptr());

        j = 0;
        while j < 3 {
            i = 0;
            while i < 8 {
                if (i & (1 << j)) == 0 {
                    // trick to get a corner
                    tmp3[0] = (if (i & 1) != 0 { 1.0 } else { -1.0 }) * *size2.add(0);
                    tmp3[1] = (if (i & 2) != 0 { 1.0 } else { -1.0 }) * *size2.add(1);
                    tmp3[2] = (if (i & 4) != 0 { 1.0 } else { -1.0 }) * *size2.add(2);
                    tmp3[j as usize] = 0.0;

                    mji_sub3(dif.as_mut_ptr(), tmp3.as_ptr(), pos.as_ptr());

                    ma = *size2.add(j as usize) * *size2.add(j as usize);
                    mb = -*size2.add(j as usize) * halfaxis[j as usize];
                    mc = *size1.add(1) * *size1.add(1);

                    u = -*size2.add(j as usize) * dif[j as usize];
                    v = mju_dot3(halfaxis.as_ptr(), dif.as_ptr());

                    det = ma * mc - mb * mb;
                    if det.abs() < mjMINVAL {
                        i += 1;
                        continue;
                    }
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
                        if x1 > 1.0 {
                            x1 = 1.0;
                            s1 = 2;
                        } else if x1 < -1.0 {
                            x1 = -1.0;
                            s1 = 0;
                        }
                    } else if x2 < -1.0 {
                        x2 = -1.0;
                        s2 = 0;
                        x1 = (u + mb) * (1.0 / ma);
                        if x1 > 1.0 {
                            x1 = 1.0;
                            s1 = 2;
                        } else if x1 < -1.0 {
                            x1 = -1.0;
                            s1 = 0;
                        }
                    }

                    mji_sub3(dif.as_mut_ptr(), tmp3.as_ptr(), pos.as_ptr());

                    mji_add_to_scl3(dif.as_mut_ptr(), halfaxis.as_ptr(), -x2);
                    dif[j as usize] += *size2.add(j as usize) * x1;

                    tmp1[2] = mju_dot3(dif.as_ptr(), dif.as_ptr());

                    c1 = s1 * 3 + s2;

                    if tmp1[2] < bestdist - mjMINVAL {
                        bestdist = tmp1[2];
                        bestsegmentpos = x2;
                        bestboxpos = x1;

                        c2 = c1 / 6;

                        clcorner = i + (1 << j) * c2;
                        cledge = j;
                        cltype = c1;
                    }
                }
                i += 1;
            }
            j += 1;
        }

        // dead code in C: loop with j=0..3 that only executes body at j==2,
        // computing unused locals. We replicate the structure for bit-exactness
        // but the results are never used, so we skip it entirely (no side effects).

        // invalid type
        if cltype == -4 {
            return 0;
        }

        if cltype >= 0 && cltype / 3 != 1 {
            // closest to a corner of the box
            c1 = axisdir ^ clcorner;

            if c1 == 0 || c1 == 7 {
                // case 1: no chance of additional contact — goto skip
            } else {
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

                if c1 == 1 {
                    ax = 0; ax1 = 1; ax2 = 2;
                }
                if c1 == 2 {
                    ax = 1; ax1 = 2; ax2 = 0;
                }
                if c1 == 4 {
                    ax = 2; ax1 = 0; ax2 = 1;
                }

                if axis[ax as usize] * axis[ax as usize] > 0.5 {
                    // second point along the edge of the box
                    secondpos = de;
                    e1 = 2.0 * *size2.add(ax as usize) / halfaxis[ax as usize].abs();

                    if e1 < secondpos {
                        secondpos = e1;
                    }
                    secondpos *= mul;
                } else {
                    // second point along a face of the box
                    secondpos = dp;

                    e1 = 2.0 * *size2.add(ax1 as usize) / halfaxis[ax1 as usize].abs();
                    if e1 < secondpos {
                        secondpos = e1;
                    }

                    e1 = 2.0 * *size2.add(ax2 as usize) / halfaxis[ax2 as usize].abs();
                    if e1 < secondpos {
                        secondpos = e1;
                    }

                    secondpos *= -mul;
                }
            }
        } else if cltype >= 0 && cltype / 3 == 1 {
            // we are on box's edge
            c1 = axisdir ^ clcorner;
            c1 &= 7 - (1 << cledge);

            if c1 != 1 && c1 != 2 && c1 != 4 {
                // goto skip — no additional contact
            } else {
                if cledge == 0 {
                    ax1 = 1; ax2 = 2;
                }
                if cledge == 1 {
                    ax1 = 2; ax2 = 0;
                }
                if cledge == 2 {
                    ax1 = 0; ax2 = 1;
                }
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
                if e1 < secondpos {
                    secondpos = e1;
                }

                if ((axisdir & (1 << ax)) != 0) == ((c1 & (1 << ax2)) != 0) {
                    e2 = 1.0 - bestboxpos;
                } else {
                    e2 = 1.0 + bestboxpos;
                }

                e1 = *size2.add(ax as usize) * e2 / halfaxis[ax as usize].abs();

                if e1 < secondpos {
                    secondpos = e1;
                }

                secondpos *= mul;
            }
        } else if cltype < 0 {
            // one capsule's end is closest to a face of the box
            if clface == -1 {
                // goto skip — closest point is inside the box
            } else {
                if cltype == -3 {
                    mul = 1.0;
                } else {
                    mul = -1.0;
                }

                secondpos = 2.0;

                mji_copy3(tmp1.as_mut_ptr(), pos.as_ptr());
                mji_add_to_scl3(tmp1.as_mut_ptr(), halfaxis.as_ptr(), -mul);

                i = 0;
                while i < 3 {
                    if i != clface {
                        e1 = (*size2.add(i as usize) - tmp1[i as usize]) / halfaxis[i as usize] * mul;
                        if e1 > 0.0 {
                            if e1 < secondpos {
                                secondpos = e1;
                            }
                        }

                        e1 = (-*size2.add(i as usize) - tmp1[i as usize]) / halfaxis[i as usize] * mul;
                        if e1 > 0.0 {
                            if e1 < secondpos {
                                secondpos = e1;
                            }
                        }
                    }
                    i += 1;
                }
                secondpos *= mul;
            }
        }

        // skip: create sphere in original orientation at first contact point
        mju_copy3(tmp1.as_mut_ptr(), pos.as_ptr());
        mji_add_to_scl3(tmp1.as_mut_ptr(), halfaxis.as_ptr(), bestsegmentpos);
        mju_mul_mat_vec3(tmp2.as_mut_ptr(), mat2, tmp1.as_ptr());
        mju_add_to3(tmp2.as_mut_ptr(), pos2);

        // collide with box
        n = mjraw_sphere_box(con, margin, tmp2.as_ptr(), mat1, size1, pos2, mat2, size2);

        if secondpos > -3.0 {
            // secondpos was modified
            mju_copy3(tmp1.as_mut_ptr(), pos.as_ptr());
            mji_add_to_scl3(tmp1.as_mut_ptr(), halfaxis.as_ptr(), secondpos + bestsegmentpos);
            mju_mul_mat_vec3(tmp2.as_mut_ptr(), mat2, tmp1.as_ptr());
            mju_add_to3(tmp2.as_mut_ptr(), pos2);
            n + mjraw_sphere_box(con.add(n as usize), margin, tmp2.as_ptr(), mat1, size1, pos2, mat2, size2)
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
    // SAFETY: all pointers valid. s has 3 elements, t1/t2/t3 have 3 elements each.
    // con points to at least 1 mjPreContact.
    unsafe {
        let rbound = margin + rs + rt;

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

        // triangle is (o,a,b), sphere center is p (2D coordinates)
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

        // compute signs of areas of (p,o,a), (p,a,b), (p,b,o)
        let sign1 = area_sign(p.as_ptr(), o.as_ptr(), a.as_ptr());
        let sign2 = area_sign(p.as_ptr(), a.as_ptr(), b.as_ptr());
        let sign3 = area_sign(p.as_ptr(), b.as_ptr(), o.as_ptr());

        let mut X: [f64; 3] = [0.0; 3];

        // p is inside triangle
        if sign1 == sign2 && sign2 == sign3 {
            crate::engine::engine_inline::mji_copy3(X.as_mut_ptr(), P.as_ptr());
        }
        // p is not inside triangle
        else {
            // find nearest point to p on triangle edges (o,a), (a,b), (b,o)
            let mut x: [[f64; 2]; 3] = [[0.0; 2]; 3];
            let mut dstx: [f64; 3] = [0.0; 3];
            dstx[0] = point_segment(x[0].as_mut_ptr(), p.as_ptr(), o.as_ptr(), a.as_ptr());
            dstx[1] = point_segment(x[1].as_mut_ptr(), p.as_ptr(), a.as_ptr(), b.as_ptr());
            dstx[2] = point_segment(x[2].as_mut_ptr(), p.as_ptr(), b.as_ptr(), o.as_ptr());

            // select minimum
            let best = if dstx[0] < dstx[1] && dstx[0] < dstx[2] {
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

        // X is now the nearest point to S within the 3D triangle (O,A,B)
        // compute contact normal and distance
        let mut nrm: [f64; 3] = [X[0] - S[0], X[1] - S[1], X[2] - S[2]];
        let dst = crate::engine::engine_util_blas::mju_normalize3(nrm.as_mut_ptr());

        // exit if too far
        if dst > rbound {
            return 0;
        }

        // construct contact
        (*con).dist = dst - rs - rt;
        crate::engine::engine_inline::mji_add_scl3(
            (*con).pos.as_mut_ptr(), s, nrm.as_ptr(), rs + (*con).dist / 2.0,
        );
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
    // SAFETY: all pointers valid per caller contract. con has room for mjMAXCONPAIR entries.
    // pos/size are f64[3], mat is f64[9], t1/t2/t3 are f64[3].
    unsafe {
        const MJ_MAXCONPAIR: i32 = 50;

        let mut cnt: i32 = 0;
        let vert: [*const f64; 3] = [t1, t2, t3];

        // check triangle vertices against box faces
        let mut i: i32 = 0;
        while i < 3 {
            // map vertex to box local frame
            let mut diff: [f64; 3] = [0.0; 3];
            crate::engine::engine_util_blas::mju_sub3(diff.as_mut_ptr(), vert[i as usize], pos);
            let mut local: [f64; 3] = [0.0; 3];
            crate::engine::engine_util_blas::mju_mul_mat_t_vec3(local.as_mut_ptr(), mat, diff.as_ptr());

            // find max penetration / closest face
            let mut maxaxis: i32 = 0;
            let mut maxval: f64 = local[0].abs() - *size.add(0);
            let mut j: i32 = 1;
            while j < 3 {
                let val: f64 = local[j as usize].abs() - *size.add(j as usize);
                if val > maxval {
                    maxval = val;
                    maxaxis = j;
                }
                j += 1;
            }

            // contact distance: dist = maxval - rt
            if maxval - rt > margin {
                i += 1;
                continue;
            }

            // check if within other dimensions (with margin/radius)
            let mut inside: i32 = 1;
            j = 0;
            while j < 3 {
                if local[j as usize].abs() > *size.add(j as usize) + margin + rt {
                    inside = 0;
                    break;
                }
                j += 1;
            }
            if inside == 0 {
                i += 1;
                continue;
            }

            // create contact
            if cnt < MJ_MAXCONPAIR {
                // normal in local frame
                let mut nrm_local: [f64; 3] = [0.0, 0.0, 0.0];
                nrm_local[maxaxis as usize] = if local[maxaxis as usize] > 0.0 { 1.0 } else { -1.0 };

                // normal in global frame (from Box to Triangle)
                crate::engine::engine_util_blas::mju_mul_mat_vec3(
                    (*con.add(cnt as usize)).normal.as_mut_ptr(), mat, nrm_local.as_ptr(),
                );

                // distance
                (*con.add(cnt as usize)).dist = maxval - rt;

                // position: v - nrm * (rt + dist/2)
                let offset: f64 = rt + (*con.add(cnt as usize)).dist * 0.5;
                crate::engine::engine_inline::mji_add_scl3(
                    (*con.add(cnt as usize)).pos.as_mut_ptr(),
                    vert[i as usize],
                    (*con.add(cnt as usize)).normal.as_ptr(),
                    -offset,
                );

                // frame details
                crate::engine::engine_util_blas::mju_zero3((*con.add(cnt as usize)).tangent.as_mut_ptr());

                cnt += 1;
            }

            i += 1;
        }

        // check box corners against triangle
        i = 0;
        while i < 8 {
            if cnt >= MJ_MAXCONPAIR {
                break;
            }

            // get corner in local coordinates
            let mut vec: [f64; 3] = [0.0; 3];
            vec[0] = if i & 1 != 0 { *size.add(0) } else { -*size.add(0) };
            vec[1] = if i & 2 != 0 { *size.add(1) } else { -*size.add(1) };
            vec[2] = if i & 4 != 0 { *size.add(2) } else { -*size.add(2) };

            // get corner in global coordinates relative to box center
            let mut corner: [f64; 3] = [0.0; 3];
            crate::engine::engine_util_blas::mju_mul_mat_vec3(corner.as_mut_ptr(), mat, vec.as_ptr());
            crate::engine::engine_util_blas::mju_add_to3(corner.as_mut_ptr(), pos);

            // check collision with triangle (radius 0 for corner)
            if mjraw_sphere_triangle(con.add(cnt as usize), margin, corner.as_ptr(), 0.0, t1, t2, t3, rt) != 0 {
                cnt += 1;
            }

            i += 1;
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
    // SAFETY: all pointers valid per caller contract. con has room for mjMAXCONPAIR entries.
    // pos/size are f64[3], mat is f64[9], t1/t2/t3 are f64[3].
    unsafe {
        const MJ_MAXCONPAIR: i32 = 50;
        const MJMINVAL: f64 = 1e-15;

        let mut cnt: i32 = 0;
        let radius: f64 = *size.add(0);
        let len: f64 = *size.add(1);
        let axis: [f64; 3] = [*mat.add(2), *mat.add(5), *mat.add(8)];
        let mut p1: [f64; 3] = [0.0; 3];
        let mut p2: [f64; 3] = [0.0; 3];

        // capsule endpoints
        crate::engine::engine_util_blas::mju_add_scl3(p1.as_mut_ptr(), pos, axis.as_ptr(), -len);
        crate::engine::engine_util_blas::mju_add_scl3(p2.as_mut_ptr(), pos, axis.as_ptr(), len);

        // Check endpoints against triangle
        cnt += mjraw_sphere_triangle(con.add(cnt as usize), margin, p1.as_ptr(), radius, t1, t2, t3, rt);
        if cnt >= MJ_MAXCONPAIR {
            return cnt;
        }
        cnt += mjraw_sphere_triangle(con.add(cnt as usize), margin, p2.as_ptr(), radius, t1, t2, t3, rt);
        if cnt >= MJ_MAXCONPAIR {
            return cnt;
        }

        // Check triangle vertices against capsule axis (Point-Segment)
        let vert: [*const f64; 3] = [t1, t2, t3];
        let mut i: i32 = 0;
        while i < 3 {
            // point-segment distance
            let mut vec: [f64; 3] = [0.0; 3];
            let mut ab: [f64; 3] = [0.0; 3];
            crate::engine::engine_util_blas::mju_sub3(vec.as_mut_ptr(), vert[i as usize], p1.as_ptr());
            crate::engine::engine_util_blas::mju_sub3(ab.as_mut_ptr(), p2.as_ptr(), p1.as_ptr());
            let t: f64 = crate::engine::engine_util_blas::mju_dot3(vec.as_ptr(), ab.as_ptr())
                / (4.0 * len * len); // ab length is 2*len

            // clamp t to [0, 1] segment (only process interior)
            if t <= MJMINVAL || t >= 1.0 - MJMINVAL {
                i += 1;
                continue;
            }

            // closest point on segment
            let mut closest: [f64; 3] = [0.0; 3];
            crate::engine::engine_inline::mji_add_scl3(closest.as_mut_ptr(), p1.as_ptr(), ab.as_ptr(), t);

            // distance vector
            crate::engine::engine_util_blas::mju_sub3(vec.as_mut_ptr(), vert[i as usize], closest.as_ptr());
            let dist: f64 = crate::engine::engine_util_blas::mju_normalize3(vec.as_mut_ptr());

            if dist > radius + rt + margin {
                i += 1;
                continue;
            }

            // con->dist
            (*con.add(cnt as usize)).dist = dist - radius - rt;

            // Frame: normal from Capsule to Triangle. 'vec' points Closest->Vert.
            crate::engine::engine_inline::mji_copy3((*con.add(cnt as usize)).normal.as_mut_ptr(), vec.as_ptr());
            crate::engine::engine_util_blas::mju_zero3((*con.add(cnt as usize)).tangent.as_mut_ptr());

            // Position: midway between surfaces
            crate::engine::engine_inline::mji_add3(
                (*con.add(cnt as usize)).pos.as_mut_ptr(), closest.as_ptr(), vert[i as usize],
            );
            crate::engine::engine_inline::mji_add_to_scl3(
                (*con.add(cnt as usize)).pos.as_mut_ptr(), vec.as_ptr(), radius - rt,
            );
            crate::engine::engine_util_blas::mju_scl3(
                (*con.add(cnt as usize)).pos.as_mut_ptr(),
                (*con.add(cnt as usize)).pos.as_ptr(),
                0.5,
            );

            cnt += 1;
            if cnt >= MJ_MAXCONPAIR {
                return cnt;
            }

            i += 1;
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
    // SAFETY: m, d are valid model/data pointers; pointer arithmetic stays within allocated arrays
    unsafe {
        let pos1: *const f64 = (*d).geom_xpos.add(3 * g1 as usize);
        let pos2: *const f64 = (*d).geom_xpos.add(3 * g2 as usize);
        let mat1: *const f64 = (*d).geom_xmat.add(9 * g1 as usize);
        let mat2: *const f64 = (*d).geom_xmat.add(9 * g2 as usize);
        let size1: *const f64 = (*m).geom_size.add(3 * g1 as usize);
        let size2: *const f64 = (*m).geom_size.add(3 * g2 as usize);
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
    // SAFETY: m, d are valid model/data pointers; pointer arithmetic stays within allocated arrays.
    // con has room for at least 2 mjPreContact entries.
    unsafe {
        let pos1: *const f64 = (*d).geom_xpos.add(3 * g1 as usize);
        let pos2: *const f64 = (*d).geom_xpos.add(3 * g2 as usize);
        let mat1: *const f64 = (*d).geom_xmat.add(9 * g1 as usize);
        let mat2: *const f64 = (*d).geom_xmat.add(9 * g2 as usize);
        let size1: *const f64 = (*m).geom_size.add(3 * g1 as usize);
        let size2: *const f64 = (*m).geom_size.add(3 * g2 as usize);

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
        let n1: i32 = mjraw_plane_sphere(con, margin, pos1, mat1, size1, endpoint.as_ptr(), mat2, size2);

        // get point 2, do sphere-plane test
        crate::engine::engine_util_blas::mju_sub3(endpoint.as_mut_ptr(), pos2, segment.as_ptr());
        let n2: i32 = mjraw_plane_sphere(con.add(n1 as usize), margin, pos1, mat1, size1, endpoint.as_ptr(), mat2, size2);

        // align contact frames with capsule axis
        if n1 != 0 {
            crate::engine::engine_inline::mji_copy3((*con.add(0)).tangent.as_mut_ptr(), axis.as_ptr());
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
    // SAFETY: m, d are valid model/data pointers; pointer arithmetic stays within allocated arrays.
    // con has room for at least 4 mjPreContact entries.
    unsafe {
        const MJMINVAL: f64 = 1e-15;

        let pos1: *const f64 = (*d).geom_xpos.add(3 * g1 as usize);
        let pos2: *const f64 = (*d).geom_xpos.add(3 * g2 as usize);
        let mat1: *const f64 = (*d).geom_xmat.add(9 * g1 as usize);
        let mat2: *const f64 = (*d).geom_xmat.add(9 * g2 as usize);
        let size2: *const f64 = (*m).geom_size.add(3 * g2 as usize);

        let normal: [f64; 3] = [*mat1.add(2), *mat1.add(5), *mat1.add(8)];
        let mut axis: [f64; 3] = [*mat2.add(2), *mat2.add(5), *mat2.add(8)];

        // project, make sure axis points towards plane
        let mut prjaxis: f64 = crate::engine::engine_util_blas::mju_dot3(normal.as_ptr(), axis.as_ptr());
        if prjaxis > 0.0 {
            crate::engine::engine_util_blas::mju_scl3(axis.as_mut_ptr(), axis.as_ptr(), -1.0);
            prjaxis = -prjaxis;
        }

        // compute normal distance to cylinder center
        let vec_tmp: [f64; 3] = [
            *pos2.add(0) - *pos1.add(0),
            *pos2.add(1) - *pos1.add(1),
            *pos2.add(2) - *pos1.add(2),
        ];
        let dist0: f64 = crate::engine::engine_util_blas::mju_dot3(vec_tmp.as_ptr(), normal.as_ptr());

        // remove component of -normal along axis, compute length
        let mut vec: [f64; 3] = [0.0; 3];
        crate::engine::engine_util_blas::mju_scl3(vec.as_mut_ptr(), axis.as_ptr(), prjaxis);
        crate::engine::engine_util_blas::mju_sub_from3(vec.as_mut_ptr(), normal.as_ptr());
        let len_sqr: f64 = crate::engine::engine_util_blas::mju_dot3(vec.as_ptr(), vec.as_ptr());

        // general configuration: normalize vector, scale by radius
        if len_sqr >= MJMINVAL * MJMINVAL {
            let scl: f64 = *size2.add(0) / len_sqr.sqrt();
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
        let prjvec: f64 = crate::engine::engine_util_blas::mju_dot3(vec.as_ptr(), normal.as_ptr());

        // scale axis by half-length
        crate::engine::engine_util_blas::mju_scl3(axis.as_mut_ptr(), axis.as_ptr(), *size2.add(1));
        let prjaxis_scaled: f64 = prjaxis * *size2.add(1);

        // check first point, construct contact
        let mut cnt: i32 = 0;
        if dist0 + prjaxis_scaled + prjvec <= margin {
            (*con.add(cnt as usize)).dist = dist0 + prjaxis_scaled + prjvec;
            crate::engine::engine_inline::mji_add3((*con.add(cnt as usize)).pos.as_mut_ptr(), pos2, vec.as_ptr());
            crate::engine::engine_inline::mji_add_to3((*con.add(cnt as usize)).pos.as_mut_ptr(), axis.as_ptr());
            crate::engine::engine_inline::mji_add_to_scl3((*con.add(cnt as usize)).pos.as_mut_ptr(), normal.as_ptr(), -(*con.add(cnt as usize)).dist * 0.5);
            crate::engine::engine_inline::mji_copy3((*con.add(cnt as usize)).normal.as_mut_ptr(), normal.as_ptr());
            crate::engine::engine_inline::mji_zero3((*con.add(cnt as usize)).tangent.as_mut_ptr());
            cnt += 1;
        } else {
            return 0; // nearest point is above margin: no contacts
        }

        // check second point, construct contact
        if dist0 - prjaxis_scaled + prjvec <= margin {
            (*con.add(cnt as usize)).dist = dist0 - prjaxis_scaled + prjvec;
            crate::engine::engine_inline::mji_add3((*con.add(cnt as usize)).pos.as_mut_ptr(), pos2, vec.as_ptr());
            crate::engine::engine_inline::mji_sub_from3((*con.add(cnt as usize)).pos.as_mut_ptr(), axis.as_ptr());
            crate::engine::engine_inline::mji_add_to_scl3((*con.add(cnt as usize)).pos.as_mut_ptr(), normal.as_ptr(), -(*con.add(cnt as usize)).dist * 0.5);
            crate::engine::engine_inline::mji_copy3((*con.add(cnt as usize)).normal.as_mut_ptr(), normal.as_ptr());
            crate::engine::engine_inline::mji_zero3((*con.add(cnt as usize)).tangent.as_mut_ptr());
            cnt += 1;
        }

        // try to add triangle points on side closer to plane
        let prjvec1: f64 = -prjvec * 0.5;
        if dist0 + prjaxis_scaled + prjvec1 <= margin {
            // compute sideways vector: vec1
            let mut vec1: [f64; 3] = [0.0; 3];
            crate::engine::engine_inline::mji_cross(vec1.as_mut_ptr(), vec.as_ptr(), axis.as_ptr());
            crate::engine::engine_util_blas::mju_normalize3(vec1.as_mut_ptr());
            crate::engine::engine_util_blas::mju_scl3(vec1.as_mut_ptr(), vec1.as_ptr(), *size2.add(0) * (3.0_f64).sqrt() / 2.0);

            // add point A
            (*con.add(cnt as usize)).dist = dist0 + prjaxis_scaled + prjvec1;
            crate::engine::engine_inline::mji_add3((*con.add(cnt as usize)).pos.as_mut_ptr(), pos2, vec1.as_ptr());
            crate::engine::engine_inline::mji_add_to3((*con.add(cnt as usize)).pos.as_mut_ptr(), axis.as_ptr());
            crate::engine::engine_inline::mji_add_to_scl3((*con.add(cnt as usize)).pos.as_mut_ptr(), vec.as_ptr(), -0.5);
            crate::engine::engine_inline::mji_add_to_scl3((*con.add(cnt as usize)).pos.as_mut_ptr(), normal.as_ptr(), -(*con.add(cnt as usize)).dist * 0.5);
            crate::engine::engine_inline::mji_copy3((*con.add(cnt as usize)).normal.as_mut_ptr(), normal.as_ptr());
            crate::engine::engine_inline::mji_zero3((*con.add(cnt as usize)).tangent.as_mut_ptr());
            cnt += 1;

            // add point B
            (*con.add(cnt as usize)).dist = dist0 + prjaxis_scaled + prjvec1;
            crate::engine::engine_inline::mji_sub3((*con.add(cnt as usize)).pos.as_mut_ptr(), pos2, vec1.as_ptr());
            crate::engine::engine_inline::mji_add_to3((*con.add(cnt as usize)).pos.as_mut_ptr(), axis.as_ptr());
            crate::engine::engine_inline::mji_add_to_scl3((*con.add(cnt as usize)).pos.as_mut_ptr(), vec.as_ptr(), -0.5);
            crate::engine::engine_inline::mji_add_to_scl3((*con.add(cnt as usize)).pos.as_mut_ptr(), normal.as_ptr(), -(*con.add(cnt as usize)).dist * 0.5);
            crate::engine::engine_inline::mji_copy3((*con.add(cnt as usize)).normal.as_mut_ptr(), normal.as_ptr());
            crate::engine::engine_inline::mji_zero3((*con.add(cnt as usize)).tangent.as_mut_ptr());
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
    // SAFETY: m, d, con are valid pointers; con has room for at least 4 mjPreContact entries.
    // All pointer arithmetic stays within allocated model/data arrays.
    unsafe {
        let pos1: *const f64 = (*d).geom_xpos.add(3 * g1 as usize);
        let pos2: *const f64 = (*d).geom_xpos.add(3 * g2 as usize);
        let mat1: *const f64 = (*d).geom_xmat.add(9 * g1 as usize);
        let mat2: *const f64 = (*d).geom_xmat.add(9 * g2 as usize);
        let size2: *const f64 = (*m).geom_size.add(3 * g2 as usize);

        // get normal, difference between centers, normal distance
        let norm: [f64; 3] = [*mat1.add(2), *mat1.add(5), *mat1.add(8)];
        let dif: [f64; 3] = [
            *pos2.add(0) - *pos1.add(0),
            *pos2.add(1) - *pos1.add(1),
            *pos2.add(2) - *pos1.add(2),
        ];
        let dist: f64 = crate::engine::engine_util_blas::mju_dot3(dif.as_ptr(), norm.as_ptr());

        // test all corners, pick bottom 4
        let mut cnt: i32 = 0;
        let mut i: i32 = 0;
        while i < 8 {
            // get corner in local coordinates
            let mut vec: [f64; 3] = [0.0; 3];
            vec[0] = if i & 1 != 0 { *size2.add(0) } else { -*size2.add(0) };
            vec[1] = if i & 2 != 0 { *size2.add(1) } else { -*size2.add(1) };
            vec[2] = if i & 4 != 0 { *size2.add(2) } else { -*size2.add(2) };

            // get corner in global coordinates relative to box center
            let mut corner: [f64; 3] = [0.0; 3];
            crate::engine::engine_util_blas::mju_mul_mat_vec3(corner.as_mut_ptr(), mat2, vec.as_ptr());

            // compute distance to plane, skip if too far or pointing up
            let ldist: f64 = crate::engine::engine_util_blas::mju_dot3(norm.as_ptr(), corner.as_ptr());
            if dist + ldist > margin || ldist > 0.0 {
                i += 1;
                continue;
            }

            // construct contact
            (*con.add(cnt as usize)).dist = dist + ldist;
            crate::engine::engine_inline::mji_copy3((*con.add(cnt as usize)).normal.as_mut_ptr(), norm.as_ptr());
            crate::engine::engine_inline::mji_add_to3(corner.as_mut_ptr(), pos2);
            crate::engine::engine_inline::mji_scl3(vec.as_mut_ptr(), norm.as_ptr(), -(*con.add(cnt as usize)).dist / 2.0);
            crate::engine::engine_inline::mji_add3((*con.add(cnt as usize)).pos.as_mut_ptr(), corner.as_ptr(), vec.as_ptr());
            crate::engine::engine_inline::mji_zero3((*con.add(cnt as usize)).tangent.as_mut_ptr());

            // count; max is 4
            cnt += 1;
            if cnt >= 4 {
                return 4;
            }

            i += 1;
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
    // SAFETY: m, d are valid model/data pointers; pointer arithmetic stays within allocated arrays
    unsafe {
        let pos1: *const f64 = (*d).geom_xpos.add(3 * g1 as usize);
        let mat1: *const f64 = (*d).geom_xmat.add(9 * g1 as usize);
        let size1: *const f64 = (*m).geom_size.add(3 * g1 as usize);
        let pos2: *const f64 = (*d).geom_xpos.add(3 * g2 as usize);
        let mat2: *const f64 = (*d).geom_xmat.add(9 * g2 as usize);
        let size2: *const f64 = (*m).geom_size.add(3 * g2 as usize);
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
    // SAFETY: m, d are valid model/data pointers; pointer arithmetic stays within allocated arrays
    unsafe {
        let pos1: *const f64 = (*d).geom_xpos.add(3 * g1 as usize);
        let pos2: *const f64 = (*d).geom_xpos.add(3 * g2 as usize);
        let mat1: *const f64 = (*d).geom_xmat.add(9 * g1 as usize);
        let mat2: *const f64 = (*d).geom_xmat.add(9 * g2 as usize);
        let size1: *const f64 = (*m).geom_size.add(3 * g1 as usize);
        let size2: *const f64 = (*m).geom_size.add(3 * g2 as usize);
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
    // SAFETY: m, d, con are valid pointers; all pointer arithmetic stays within allocated arrays.
    // This function implements sphere-cylinder collision with three code paths: side, cap, corner.
    unsafe {
        let pos1: *const f64 = (*d).geom_xpos.add(3 * g1 as usize);
        let pos2: *const f64 = (*d).geom_xpos.add(3 * g2 as usize);
        let mat1: *const f64 = (*d).geom_xmat.add(9 * g1 as usize);
        let mat2: *const f64 = (*d).geom_xmat.add(9 * g2 as usize);
        let size1: *const f64 = (*m).geom_size.add(3 * g1 as usize);
        let size2: *const f64 = (*m).geom_size.add(3 * g2 as usize);

        // get cylinder sizes and axis
        let radius: f64 = *size2.add(0);
        let height: f64 = *size2.add(1);
        let axis: [f64; 3] = [*mat2.add(2), *mat2.add(5), *mat2.add(8)];

        // find sphere projection onto cylinder axis and plane
        let vec: [f64; 3] = [
            *pos1.add(0) - *pos2.add(0),
            *pos1.add(1) - *pos2.add(1),
            *pos1.add(2) - *pos2.add(2),
        ];
        let x: f64 = crate::engine::engine_util_blas::mju_dot3(axis.as_ptr(), vec.as_ptr());
        let mut a_proj: [f64; 3] = [0.0; 3];
        let mut p_proj: [f64; 3] = [0.0; 3];
        crate::engine::engine_inline::mji_scl3(a_proj.as_mut_ptr(), axis.as_ptr(), x);
        crate::engine::engine_inline::mji_sub3(p_proj.as_mut_ptr(), vec.as_ptr(), a_proj.as_ptr());
        let p_proj_sqr: f64 = crate::engine::engine_util_blas::mju_dot3(p_proj.as_ptr(), p_proj.as_ptr());

        // get collision type
        let mut collide_side: i32 = (x.abs() < height) as i32;
        let mut collide_cap: i32 = (p_proj_sqr < radius * radius) as i32;
        if collide_side != 0 && collide_cap != 0 {
            // deep penetration (sphere origin inside cylinder)
            let dist_cap: f64 = height - x.abs();
            let dist_radius: f64 = radius - p_proj_sqr.sqrt();
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
                crate::engine::engine_util_blas::mju_add_scl3(pos_cap.as_mut_ptr(), pos2, axis.as_ptr(), height);
                mat_cap = mat2;
            } else {
                // bottom cap
                crate::engine::engine_util_blas::mju_add_scl3(pos_cap.as_mut_ptr(), pos2, axis.as_ptr(), -height);
                mat_cap = flipmat.as_ptr();
            }
            let ncon: i32 = mjraw_plane_sphere(con, margin, pos_cap.as_ptr(), mat_cap, size2, pos1, mat1, size1);

            if ncon != 0 {
                // flip direction normal (because mjGEOM_PLANE < mjGEOM_SPHERE < mjGEOM_CYLINDER)
                crate::engine::engine_util_blas::mju_scl3((*con.add(0)).normal.as_mut_ptr(), (*con.add(0)).normal.as_ptr(), -1.0);
            }
            return ncon;
        }

        // otherwise corner collision: use sphere-sphere
        let mut p_proj_mut: [f64; 3] = p_proj;
        crate::engine::engine_util_blas::mju_scl3(
            p_proj_mut.as_mut_ptr(), p_proj_mut.as_ptr(),
            *size2.add(0) / p_proj_sqr.sqrt(),
        );
        let mut corner: [f64; 3] = [0.0; 3];
        crate::engine::engine_inline::mji_scl3(
            corner.as_mut_ptr(), axis.as_ptr(),
            if x > 0.0 { height } else { -height },
        );
        crate::engine::engine_inline::mji_add_to3(corner.as_mut_ptr(), p_proj_mut.as_ptr());
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
    // SAFETY: m, d are valid model/data pointers; pointer arithmetic stays within allocated arrays
    unsafe {
        let pos1: *const f64 = (*d).geom_xpos.add(3 * g1 as usize);
        let pos2: *const f64 = (*d).geom_xpos.add(3 * g2 as usize);
        let mat1: *const f64 = (*d).geom_xmat.add(9 * g1 as usize);
        let mat2: *const f64 = (*d).geom_xmat.add(9 * g2 as usize);
        let size1: *const f64 = (*m).geom_size.add(3 * g1 as usize);
        let size2: *const f64 = (*m).geom_size.add(3 * g2 as usize);
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
    // SAFETY: m, d, con valid. g1, g2 are valid geom indices.
    unsafe {
        let pos1: *const f64 = (*d).geom_xpos.add(3 * g1 as usize);
        let pos2: *const f64 = (*d).geom_xpos.add(3 * g2 as usize);
        let mat1: *const f64 = (*d).geom_xmat.add(9 * g1 as usize);
        let mat2: *const f64 = (*d).geom_xmat.add(9 * g2 as usize);
        let size1: *const f64 = (*m).geom_size.add(3 * g1 as usize);
        let size2: *const f64 = (*m).geom_size.add(3 * g2 as usize);
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
    // SAFETY: m, d, con valid. g1, g2 are valid geom indices.
    unsafe {
        let pos1: *const f64 = (*d).geom_xpos.add(3 * g1 as usize);
        let mat1: *const f64 = (*d).geom_xmat.add(9 * g1 as usize);
        let size1: *const f64 = (*m).geom_size.add(3 * g1 as usize);
        let pos2: *const f64 = (*d).geom_xpos.add(3 * g2 as usize);
        let mat2: *const f64 = (*d).geom_xmat.add(9 * g2 as usize);
        let size2: *const f64 = (*m).geom_size.add(3 * g2 as usize);
        crate::engine::engine_collision_box::mjraw_sphere_box(con, margin, pos1, mat1, size1, pos2, mat2, size2)
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
    extern "C" { fn mjc_BoxBox(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32; }
    // SAFETY: delegates to C implementation which handles complex box-box collision detection
    unsafe { mjc_BoxBox(m, d, con, g1, g2, margin) }
}

