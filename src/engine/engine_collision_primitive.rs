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
#[allow(unused_variables, non_snake_case)]
pub fn mjraw_capsule_box(con: *mut mjPreContact, margin: f64, pos1: *const f64, mat1: *const f64, size1: *const f64, pos2: *const f64, mat2: *const f64, size2: *const f64) -> i32 {
    extern "C" {
        fn mjraw_CapsuleBox_impl(con: *mut mjPreContact, margin: f64, pos1: *const f64, mat1: *const f64, size1: *const f64, pos2: *const f64, mat2: *const f64, size2: *const f64) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjraw_CapsuleBox_impl(con, margin, pos1, mat1, size1, pos2, mat2, size2) }
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
    extern "C" {
        fn mjraw_BoxTriangle_impl(con: *mut mjPreContact, margin: f64, pos: *const f64, mat: *const f64, size: *const f64, t1: *const f64, t2: *const f64, t3: *const f64, rt: f64) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjraw_BoxTriangle_impl(con, margin, pos, mat, size, t1, t2, t3, rt) }
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
    extern "C" {
        fn mjraw_CapsuleTriangle_impl(con: *mut mjPreContact, margin: f64, pos: *const f64, mat: *const f64, size: *const f64, t1: *const f64, t2: *const f64, t3: *const f64, rt: f64) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjraw_CapsuleTriangle_impl(con, margin, pos, mat, size, t1, t2, t3, rt) }
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
    extern "C" { fn mjc_PlaneSphere_impl(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjc_PlaneSphere_impl(m, d, con, g1, g2, margin) }
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
    extern "C" { fn mjc_PlaneCapsule_impl(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjc_PlaneCapsule_impl(m, d, con, g1, g2, margin) }
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
    extern "C" { fn mjc_PlaneCylinder_impl(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjc_PlaneCylinder_impl(m, d, con, g1, g2, margin) }
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

    extern "C" { fn mjc_CapsuleBox_impl(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjc_CapsuleBox_impl(m, d, con, g1, g2, margin) }
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
    extern "C" { fn mjc_BoxBox_impl(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32; }
    // SAFETY: delegates to C implementation which handles complex box-box collision detection
    unsafe { mjc_BoxBox_impl(m, d, con, g1, g2, margin) }
}

