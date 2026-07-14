//! Port of: engine/engine_collision_gjk.c
//! IR hash: 9614bd3d92e7766b
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: align8 (engine/engine_collision_gjk.c:49)
/// Calls: FilePath::size
#[allow(unused_variables, non_snake_case)]
pub fn align8(size: usize) -> usize {
    (size + 7) & !7
}

/// C: subdistance (engine/engine_collision_gjk.c:56)
/// Calls: S1D, S2D, S3D
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn subdistance(lambda: *mut f64, n: i32, simplex: *const Vertex) {
    // Vertex layout: { vert[3]: f64, vert1[3]: f64, vert2[3]: f64, index1: i32, index2: i32 }
    // Total size: 3*3*8 + 2*4 = 80 bytes. vert is at offset 0.
    const SIZEOF_VERTEX: usize = 80;

    // SAFETY: lambda has at least 4 elements. simplex has at least 4 Vertex elements.
    // Each Vertex's vert field is a f64[3] at offset 0.
    unsafe {
        std::ptr::write_bytes(lambda, 0, 4);

        let base = simplex as *const u8;
        let s1 = base as *const f64;
        let s2 = base.add(SIZEOF_VERTEX) as *const f64;
        let s3 = base.add(2 * SIZEOF_VERTEX) as *const f64;
        let s4 = base.add(3 * SIZEOF_VERTEX) as *const f64;

        match n {
            4 => s3d(lambda, s1, s2, s3, s4),
            3 => s2d(lambda, s1, s2, s3),
            2 => s1d(lambda, s1, s2),
            _ => *lambda.add(0) = 1.0,
        }
    }
}

/// C: S3D (engine/engine_collision_gjk.c:60)
/// Calls: S2D, det3, dot3, lincomb, sameSign2
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn s3d(lambda: *mut f64, s1: *const f64, s2: *const f64, s3: *const f64, s4: *const f64) {
    // SAFETY: caller guarantees lambda[4], s1[3], s2[3], s3[3], s4[3] are valid
    unsafe {
        // compute cofactors to find det(M)
        let C41: f64 = -det3(s2, s3, s4);
        let C42: f64 =  det3(s1, s3, s4);
        let C43: f64 = -det3(s1, s2, s4);
        let C44: f64 =  det3(s1, s2, s3);

        let m_det: f64 = C41 + C42 + C43 + C44;

        let comp1 = same_sign2(m_det, C41);
        let comp2 = same_sign2(m_det, C42);
        let comp3 = same_sign2(m_det, C43);
        let comp4 = same_sign2(m_det, C44);

        // if all signs are the same then the origin is inside the simplex
        if comp1 != 0 && comp2 != 0 && comp3 != 0 && comp4 != 0 {
            *lambda.add(0) = C41 / m_det;
            *lambda.add(1) = C42 / m_det;
            *lambda.add(2) = C43 / m_det;
            *lambda.add(3) = C44 / m_det;
            return;
        }

        // find the smallest distance, and use the corresponding barycentric coordinates
        let mut dmin: f64 = f64::MAX;

        if comp1 == 0 {
            let mut lambda_2d: [f64; 3] = [0.0; 3];
            let mut x: [f64; 3] = [0.0; 3];
            s2d(lambda_2d.as_mut_ptr(), s2, s3, s4);
            lincomb(x.as_mut_ptr(), lambda_2d.as_ptr(), 3, s2, s3, s4, std::ptr::null());
            let d = dot3(x.as_ptr(), x.as_ptr());
            *lambda.add(0) = 0.0;
            *lambda.add(1) = lambda_2d[0];
            *lambda.add(2) = lambda_2d[1];
            *lambda.add(3) = lambda_2d[2];
            dmin = d;
        }

        if comp2 == 0 {
            let mut lambda_2d: [f64; 3] = [0.0; 3];
            let mut x: [f64; 3] = [0.0; 3];
            s2d(lambda_2d.as_mut_ptr(), s1, s3, s4);
            lincomb(x.as_mut_ptr(), lambda_2d.as_ptr(), 3, s1, s3, s4, std::ptr::null());
            let d = dot3(x.as_ptr(), x.as_ptr());
            if d < dmin {
                *lambda.add(0) = lambda_2d[0];
                *lambda.add(1) = 0.0;
                *lambda.add(2) = lambda_2d[1];
                *lambda.add(3) = lambda_2d[2];
                dmin = d;
            }
        }

        if comp3 == 0 {
            let mut lambda_2d: [f64; 3] = [0.0; 3];
            let mut x: [f64; 3] = [0.0; 3];
            s2d(lambda_2d.as_mut_ptr(), s1, s2, s4);
            lincomb(x.as_mut_ptr(), lambda_2d.as_ptr(), 3, s1, s2, s4, std::ptr::null());
            let d = dot3(x.as_ptr(), x.as_ptr());
            if d < dmin {
                *lambda.add(0) = lambda_2d[0];
                *lambda.add(1) = lambda_2d[1];
                *lambda.add(2) = 0.0;
                *lambda.add(3) = lambda_2d[2];
                dmin = d;
            }
        }

        if comp4 == 0 {
            let mut lambda_2d: [f64; 3] = [0.0; 3];
            let mut x: [f64; 3] = [0.0; 3];
            s2d(lambda_2d.as_mut_ptr(), s1, s2, s3);
            lincomb(x.as_mut_ptr(), lambda_2d.as_ptr(), 3, s1, s2, s3, std::ptr::null());
            let d = dot3(x.as_ptr(), x.as_ptr());
            if d < dmin {
                *lambda.add(0) = lambda_2d[0];
                *lambda.add(1) = lambda_2d[1];
                *lambda.add(2) = lambda_2d[2];
                // lambda[3] stays unchanged (matches C behavior: no assignment for lambda[3])
            }
        }
    }
}

/// C: S2D (engine/engine_collision_gjk.c:62)
/// Calls: S1D, dot3, lincomb, projectOriginPlane, sameSign2
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn s2d(lambda: *mut f64, s1: *const f64, s2: *const f64, s3: *const f64) {
    // SAFETY: caller guarantees lambda[3], s1[3], s2[3], s3[3] are valid
    unsafe {
        // project origin onto affine hull of the simplex
        let mut p_o: [f64; 3] = [0.0; 3];
        if project_origin_plane(p_o.as_mut_ptr(), s1, s2, s3) != 0 {
            s1d(lambda, s1, s2);
            *lambda.add(2) = 0.0;
            return;
        }

        // Minors M_i4 of the matrix M
        let M_14: f64 = *s2.add(1) * *s3.add(2) - *s2.add(2) * *s3.add(1)
                       - *s1.add(1) * *s3.add(2) + *s1.add(2) * *s3.add(1)
                       + *s1.add(1) * *s2.add(2) - *s1.add(2) * *s2.add(1);
        let M_24: f64 = *s2.add(0) * *s3.add(2) - *s2.add(2) * *s3.add(0)
                       - *s1.add(0) * *s3.add(2) + *s1.add(2) * *s3.add(0)
                       + *s1.add(0) * *s2.add(2) - *s1.add(2) * *s2.add(0);
        let M_34: f64 = *s2.add(0) * *s3.add(1) - *s2.add(1) * *s3.add(0)
                       - *s1.add(0) * *s3.add(1) + *s1.add(1) * *s3.add(0)
                       + *s1.add(0) * *s2.add(1) - *s1.add(1) * *s2.add(0);

        // exclude the axis with the largest projection of the simplex
        let mut M_max: f64 = 0.0;
        let mut s1_2D: [f64; 2] = [0.0; 2];
        let mut s2_2D: [f64; 2] = [0.0; 2];
        let mut s3_2D: [f64; 2] = [0.0; 2];
        let mut p_o_2D: [f64; 2] = [0.0; 2];
        let mu1: f64 = f64::abs(M_14);
        let mu2: f64 = f64::abs(M_24);
        let mu3: f64 = f64::abs(M_34);

        if mu1 >= mu2 && mu1 >= mu3 {
            M_max = M_14;
            s1_2D[0] = *s1.add(1);
            s1_2D[1] = *s1.add(2);
            s2_2D[0] = *s2.add(1);
            s2_2D[1] = *s2.add(2);
            s3_2D[0] = *s3.add(1);
            s3_2D[1] = *s3.add(2);
            p_o_2D[0] = p_o[1];
            p_o_2D[1] = p_o[2];
        } else if mu2 >= mu3 {
            M_max = M_24;
            s1_2D[0] = *s1.add(0);
            s1_2D[1] = *s1.add(2);
            s2_2D[0] = *s2.add(0);
            s2_2D[1] = *s2.add(2);
            s3_2D[0] = *s3.add(0);
            s3_2D[1] = *s3.add(2);
            p_o_2D[0] = p_o[0];
            p_o_2D[1] = p_o[2];
        } else {
            M_max = M_34;
            s1_2D[0] = *s1.add(0);
            s1_2D[1] = *s1.add(1);
            s2_2D[0] = *s2.add(0);
            s2_2D[1] = *s2.add(1);
            s3_2D[0] = *s3.add(0);
            s3_2D[1] = *s3.add(1);
            p_o_2D[0] = p_o[0];
            p_o_2D[1] = p_o[1];
        }

        // cofactors C3i
        // C31: signed area of (p_o_2D, s2_2D, s3_2D)
        let C31: f64 = p_o_2D[0] * s2_2D[1] + p_o_2D[1] * s3_2D[0] + s2_2D[0] * s3_2D[1]
                     - p_o_2D[0] * s3_2D[1] - p_o_2D[1] * s2_2D[0] - s3_2D[0] * s2_2D[1];

        // C32: signed area of (p_o_2D, s1_2D, s3_2D)
        let C32: f64 = p_o_2D[0] * s3_2D[1] + p_o_2D[1] * s1_2D[0] + s3_2D[0] * s1_2D[1]
                     - p_o_2D[0] * s1_2D[1] - p_o_2D[1] * s3_2D[0] - s1_2D[0] * s3_2D[1];

        // C33: signed area of (p_o_2D, s1_2D, s2_2D)
        let C33: f64 = p_o_2D[0] * s1_2D[1] + p_o_2D[1] * s2_2D[0] + s1_2D[0] * s2_2D[1]
                     - p_o_2D[0] * s2_2D[1] - p_o_2D[1] * s1_2D[0] - s2_2D[0] * s1_2D[1];

        let comp1: i32 = same_sign2(M_max, C31);
        let comp2: i32 = same_sign2(M_max, C32);
        let comp3: i32 = same_sign2(M_max, C33);

        // all the same sign, p_o is inside the 2-simplex
        if comp1 != 0 && comp2 != 0 && comp3 != 0 {
            *lambda.add(0) = C31 / M_max;
            *lambda.add(1) = C32 / M_max;
            *lambda.add(2) = C33 / M_max;
            return;
        }

        // find the smallest distance, and use the corresponding barycentric coordinates
        let mut dmin: f64 = f64::MAX;

        if comp1 == 0 {
            let mut lambda_1d: [f64; 2] = [0.0; 2];
            let mut x: [f64; 3] = [0.0; 3];
            s1d(lambda_1d.as_mut_ptr(), s2, s3);
            lincomb(x.as_mut_ptr(), lambda_1d.as_ptr(), 2, s2, s3, std::ptr::null(), std::ptr::null());
            let d: f64 = dot3(x.as_ptr(), x.as_ptr());
            *lambda.add(0) = 0.0;
            *lambda.add(1) = lambda_1d[0];
            *lambda.add(2) = lambda_1d[1];
            dmin = d;
        }

        if comp2 == 0 {
            let mut lambda_1d: [f64; 2] = [0.0; 2];
            let mut x: [f64; 3] = [0.0; 3];
            s1d(lambda_1d.as_mut_ptr(), s1, s3);
            lincomb(x.as_mut_ptr(), lambda_1d.as_ptr(), 2, s1, s3, std::ptr::null(), std::ptr::null());
            let d: f64 = dot3(x.as_ptr(), x.as_ptr());
            if d < dmin {
                *lambda.add(0) = lambda_1d[0];
                *lambda.add(1) = 0.0;
                *lambda.add(2) = lambda_1d[1];
                dmin = d;
            }
        }

        if comp3 == 0 {
            let mut lambda_1d: [f64; 2] = [0.0; 2];
            let mut x: [f64; 3] = [0.0; 3];
            s1d(lambda_1d.as_mut_ptr(), s1, s2);
            lincomb(x.as_mut_ptr(), lambda_1d.as_ptr(), 2, s1, s2, std::ptr::null(), std::ptr::null());
            let d: f64 = dot3(x.as_ptr(), x.as_ptr());
            if d < dmin {
                *lambda.add(0) = lambda_1d[0];
                *lambda.add(1) = lambda_1d[1];
                *lambda.add(2) = 0.0;
            }
        }
    }
}

/// C: S1D (engine/engine_collision_gjk.c:63)
/// Calls: projectOriginLine, sameSign2
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn s1d(lambda: *mut f64, s1: *const f64, s2: *const f64) {
    // SAFETY: caller guarantees lambda[2], s1[3], s2[3] are valid
    unsafe {
        let mut p_o: [f64; 3] = [0.0; 3];
        project_origin_line(p_o.as_mut_ptr(), s1, s2);

        let mut mu: f64 = *s1.add(0) - *s2.add(0);
        let mut mu_max: f64 = mu;
        let mut index: usize = 0;

        mu = *s1.add(1) - *s2.add(1);
        if f64::abs(mu) >= f64::abs(mu_max) {
            mu_max = mu;
            index = 1;
        }

        mu = *s1.add(2) - *s2.add(2);
        if f64::abs(mu) >= f64::abs(mu_max) {
            mu_max = mu;
            index = 2;
        }

        let c1: f64 = p_o[index] - *s2.add(index);
        let c2: f64 = *s1.add(index) - p_o[index];

        let same: bool = same_sign2(mu_max, c1) != 0 && same_sign2(mu_max, c2) != 0;

        *lambda.add(0) = if same { c1 / mu_max } else { 0.0 };
        *lambda.add(1) = if same { c2 / mu_max } else { 1.0 };
    }
}

/// C: gjkSupport (engine/engine_collision_gjk.c:66)
/// Calls: scl3, support
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn gjk_support(v: *mut Vertex, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj, x_k: *const f64, x_norm: f64) {
    todo!() // gjkSupport
}

/// C: lincomb (engine/engine_collision_gjk.c:70)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn lincomb(res: *mut f64, coef: *const f64, n: i32, v1: *const f64, v2: *const f64, v3: *const f64, v4: *const f64) {
    // SAFETY: raw pointer arithmetic matching C array access; caller guarantees valid pointers and sizes
    unsafe {
        match n {
            1 => {
                *res.add(0) = *coef.add(0) * *v1.add(0);
                *res.add(1) = *coef.add(0) * *v1.add(1);
                *res.add(2) = *coef.add(0) * *v1.add(2);
            }
            2 => {
                *res.add(0) = *coef.add(0) * *v1.add(0) + *coef.add(1) * *v2.add(0);
                *res.add(1) = *coef.add(0) * *v1.add(1) + *coef.add(1) * *v2.add(1);
                *res.add(2) = *coef.add(0) * *v1.add(2) + *coef.add(1) * *v2.add(2);
            }
            3 => {
                *res.add(0) = *coef.add(0) * *v1.add(0) + *coef.add(1) * *v2.add(0) + *coef.add(2) * *v3.add(0);
                *res.add(1) = *coef.add(0) * *v1.add(1) + *coef.add(1) * *v2.add(1) + *coef.add(2) * *v3.add(1);
                *res.add(2) = *coef.add(0) * *v1.add(2) + *coef.add(1) * *v2.add(2) + *coef.add(2) * *v3.add(2);
            }
            4 => {
                *res.add(0) = *coef.add(0) * *v1.add(0) + *coef.add(1) * *v2.add(0) + *coef.add(2) * *v3.add(0) + *coef.add(3) * *v4.add(0);
                *res.add(1) = *coef.add(0) * *v1.add(1) + *coef.add(1) * *v2.add(1) + *coef.add(2) * *v3.add(1) + *coef.add(3) * *v4.add(1);
                *res.add(2) = *coef.add(0) * *v1.add(2) + *coef.add(1) * *v2.add(2) + *coef.add(2) * *v3.add(2) + *coef.add(3) * *v4.add(2);
            }
            _ => {}
        }
    }
}

/// C: epaSupport (engine/engine_collision_gjk.c:108)
/// Calls: scl3, support
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn epa_support(pt: *mut Polytope, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj, d: *const f64, dnorm: f64) -> i32 {
    todo!() // epaSupport
}

/// C: insertVertex (engine/engine_collision_gjk.c:112)
#[allow(unused_variables, non_snake_case)]
pub fn insert_vertex(pt: *mut Polytope, v: *const Vertex) -> i32 {
    todo!() // insertVertex
}

/// C: attachFace (engine/engine_collision_gjk.c:115)
/// Calls: dot3, projectOriginPlane, scl3, sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn attach_face(pt: *mut Polytope, v1: i32, v2: i32, v3: i32, adj1: i32, adj2: i32, adj3: i32) -> f64 {
    todo!() // attachFace
}

/// C: gjkIntersect (engine/engine_collision_gjk.c:119)
/// Calls: dot3, gjkIntersectSupport, signedDistance
#[allow(unused_variables, non_snake_case)]
pub fn gjk_intersect(status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> i32 {
    todo!() // gjkIntersect
}

/// C: polytope2 (engine/engine_collision_gjk.c:122)
/// Calls: add3, attachFace, cross3, epaSupport, insertVertex, mju_mulMatVec3, norm3, polytope3, rayTriangle, replaceSimplex3, rotmat, scl3, sub3
#[allow(unused_variables, non_snake_case)]
pub fn polytope2(pt: *mut Polytope, status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> i32 {
    todo!() // polytope2
}

/// C: polytope3 (engine/engine_collision_gjk.c:123)
/// Calls: add3, attachFace, cross3, epaSupport, insertVertex, norm3, scl3, sub3, testTetra, triPointIntersect
#[allow(unused_variables, non_snake_case)]
pub fn polytope3(pt: *mut Polytope, status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> i32 {
    todo!() // polytope3
}

/// C: polytope4 (engine/engine_collision_gjk.c:124)
/// Calls: add3, attachFace, insertVertex, polytope3, replaceSimplex3, scl3, testTetra
#[allow(unused_variables, non_snake_case)]
pub fn polytope4(pt: *mut Polytope, status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> i32 {
    todo!() // polytope4
}

/// C: epa (engine/engine_collision_gjk.c:128)
/// Calls: attachFace, discreteGeoms, dot3, epaSupport, epaWitness, horizon, maxFaces, mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn epa(status: *mut mjCCDStatus, pt: *mut Polytope, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> *mut Face {
    todo!() // epa
}

/// C: equal3 (engine/engine_collision_gjk.c:133)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn equal3(v1: *const f64, v2: *const f64) -> i32 {
    // SAFETY: raw pointer arithmetic matching C array access; caller guarantees valid pointers
    unsafe {
        if (*v1.add(0) - *v2.add(0)).abs() < 1e-15
            && (*v1.add(1) - *v2.add(1)).abs() < 1e-15
            && (*v1.add(2) - *v2.add(2)).abs() < 1e-15
        {
            1
        } else {
            0
        }
    }
}

/// C: add3 (engine/engine_collision_gjk.c:140)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add3(res: *mut f64, v1: *const f64, v2: *const f64) {
    // SAFETY: raw pointer arithmetic matching C array access; caller guarantees valid pointers
    unsafe {
        *res.add(0) = *v1.add(0) + *v2.add(0);
        *res.add(1) = *v1.add(1) + *v2.add(1);
        *res.add(2) = *v1.add(2) + *v2.add(2);
    }
}

/// C: sub3 (engine/engine_collision_gjk.c:145)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn sub3(res: *mut f64, v1: *const f64, v2: *const f64) {
    // SAFETY: raw pointer arithmetic matching C array access; caller guarantees valid pointers
    unsafe {
        *res.add(0) = *v1.add(0) - *v2.add(0);
        *res.add(1) = *v1.add(1) - *v2.add(1);
        *res.add(2) = *v1.add(2) - *v2.add(2);
    }
}

/// C: dot3 (engine/engine_collision_gjk.c:150)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn dot3(v1: *const f64, v2: *const f64) -> f64 {
    // SAFETY: raw pointer arithmetic matching C array access; caller guarantees valid pointers
    unsafe {
        *v1.add(0) * *v2.add(0) + *v1.add(1) * *v2.add(1) + *v1.add(2) * *v2.add(2)
    }
}

/// C: norm3 (engine/engine_collision_gjk.c:155)
/// Calls: dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn norm3(v: *const f64) -> f64 {
    dot3(v, v).sqrt()
}

/// C: copy3 (engine/engine_collision_gjk.c:160)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn copy3(res: *mut f64, v: *const f64) {
    // SAFETY: raw pointer arithmetic matching C array access; caller guarantees valid pointers
    unsafe {
        *res.add(0) = *v.add(0);
        *res.add(1) = *v.add(1);
        *res.add(2) = *v.add(2);
    }
}

/// C: scl3 (engine/engine_collision_gjk.c:165)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn scl3(res: *mut f64, v: *const f64, s: f64) {
    // SAFETY: raw pointer arithmetic matching C array access; caller guarantees valid pointers
    unsafe {
        *res.add(0) = s * *v.add(0);
        *res.add(1) = s * *v.add(1);
        *res.add(2) = s * *v.add(2);
    }
}

/// C: cross3 (engine/engine_collision_gjk.c:170)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn cross3(res: *mut f64, v1: *const f64, v2: *const f64) {
    // SAFETY: raw pointer arithmetic matching C array access; caller guarantees valid pointers
    unsafe {
        *res.add(0) = *v1.add(1) * *v2.add(2) - *v1.add(2) * *v2.add(1);
        *res.add(1) = *v1.add(2) * *v2.add(0) - *v1.add(0) * *v2.add(2);
        *res.add(2) = *v1.add(0) * *v2.add(1) - *v1.add(1) * *v2.add(0);
    }
}

/// C: det3 (engine/engine_collision_gjk.c:177)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn det3(v1: *const f64, v2: *const f64, v3: *const f64) -> f64 {
    // SAFETY: raw pointer arithmetic matching C array access; caller guarantees valid pointers
    unsafe {
        *v1.add(0) * (*v2.add(1) * *v3.add(2) - *v2.add(2) * *v3.add(1))
            + *v1.add(1) * (*v2.add(2) * *v3.add(0) - *v2.add(0) * *v3.add(2))
            + *v1.add(2) * (*v2.add(0) * *v3.add(1) - *v2.add(1) * *v3.add(0))
    }
}

/// C: discreteGeoms (engine/engine_collision_gjk.c:188)
#[allow(unused_variables, non_snake_case)]
pub fn discrete_geoms(obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> i32 {
    todo!() // discreteGeoms
}

/// C: gjk (engine/engine_collision_gjk.c:200)
/// Calls: copy3, discreteGeoms, dot3, equal3, gjkIntersect, gjkSupport, lincomb, sub3, subdistance
#[allow(unused_variables, non_snake_case)]
pub fn gjk(status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) {
    todo!() // gjk
}

/// C: support (engine/engine_collision_gjk.c:334)
/// Calls: sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn support(v: *mut Vertex, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj, dir: *const f64, dir_neg: *const f64) {
    todo!() // support
}

/// C: gjkIntersectSupport (engine/engine_collision_gjk.c:396)
/// Calls: support
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn gjk_intersect_support(v: *mut Vertex, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj, dir: *const f64) {
    todo!() // gjkIntersectSupport
}

/// C: signedDistance (engine/engine_collision_gjk.c:404)
/// Calls: cross3, dot3, scl3, sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn signed_distance(normal: *mut f64, v1: *const Vertex, v2: *const Vertex, v3: *const Vertex) -> f64 {
    todo!() // signedDistance
}

/// C: projectOriginPlane (engine/engine_collision_gjk.c:507)
/// Calls: cross3, dot3, scl3, sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn project_origin_plane(res: *mut f64, v1: *const f64, v2: *const f64, v3: *const f64) -> i32 {
    const MJ_MINVAL: f64 = 1E-15_f64;
    let mut diff21: [f64; 3] = [0.0; 3];
    let mut diff31: [f64; 3] = [0.0; 3];
    let mut diff32: [f64; 3] = [0.0; 3];
    let mut n: [f64; 3] = [0.0; 3];

    sub3(diff21.as_mut_ptr(), v2, v1);
    sub3(diff31.as_mut_ptr(), v3, v1);
    sub3(diff32.as_mut_ptr(), v3, v2);

    // n = (v1 - v2) x (v3 - v2)
    cross3(n.as_mut_ptr(), diff32.as_ptr(), diff21.as_ptr());
    let mut nv = dot3(n.as_ptr(), v2);
    let mut nn = dot3(n.as_ptr(), n.as_ptr());
    if nn == 0.0 {
        return 1;
    }
    if nv != 0.0 && nn > MJ_MINVAL {
        scl3(res, n.as_ptr(), nv / nn);
        return 0;
    }

    // n = (v2 - v1) x (v3 - v1)
    cross3(n.as_mut_ptr(), diff21.as_ptr(), diff31.as_ptr());
    nv = dot3(n.as_ptr(), v1);
    nn = dot3(n.as_ptr(), n.as_ptr());
    if nn == 0.0 {
        return 1;
    }
    if nv != 0.0 && nn > MJ_MINVAL {
        scl3(res, n.as_ptr(), nv / nn);
        return 0;
    }

    // n = (v1 - v3) x (v2 - v3)
    cross3(n.as_mut_ptr(), diff31.as_ptr(), diff32.as_ptr());
    nv = dot3(n.as_ptr(), v3);
    nn = dot3(n.as_ptr(), n.as_ptr());
    scl3(res, n.as_ptr(), nv / nn);
    0
}

/// C: projectOriginLine (engine/engine_collision_gjk.c:544)
/// Calls: dot3, sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn project_origin_line(res: *mut f64, v1: *const f64, v2: *const f64) {
    let mut diff: [f64; 3] = [0.0; 3];
    sub3(diff.as_mut_ptr(), v2, v1);
    let scl = -(dot3(v2, diff.as_ptr()) / dot3(diff.as_ptr(), diff.as_ptr()));
    // SAFETY: raw pointer arithmetic matching C array access; caller guarantees valid pointers
    unsafe {
        *res.add(0) = *v2.add(0) + scl * *diff.get_unchecked(0);
        *res.add(1) = *v2.add(1) + scl * *diff.get_unchecked(1);
        *res.add(2) = *v2.add(2) + scl * *diff.get_unchecked(2);
    }
}

/// C: sameSign2 (engine/engine_collision_gjk.c:556)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn same_sign2(a: f64, b: f64) -> i32 {
    if a > 0.0 && b > 0.0 {
        return 1;
    }
    if a < 0.0 && b < 0.0 {
        return -1;
    }
    0
}

/// C: replaceSimplex3 (engine/engine_collision_gjk.c:849)
#[allow(unused_variables, non_snake_case)]
pub fn replace_simplex3(pt: *mut Polytope, status: *mut mjCCDStatus, v1: i32, v2: i32, v3: i32) {
    todo!() // replaceSimplex3
}

/// C: sameSide (engine/engine_collision_gjk.c:864)
/// Calls: cross3, dot3, scl3, sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn same_side(p0: *const f64, p1: *const f64, p2: *const f64, p3: *const f64) -> i32 {
    todo!() // sameSide
}

/// C: testTetra (engine/engine_collision_gjk.c:883)
/// Calls: sameSide
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn test_tetra(p0: *const f64, p1: *const f64, p2: *const f64, p3: *const f64) -> i32 {
    if same_side(p0, p1, p2, p3) != 0
        && same_side(p1, p2, p3, p0) != 0
        && same_side(p2, p3, p0, p1) != 0
        && same_side(p3, p0, p1, p2) != 0
    {
        1
    } else {
        0
    }
}

/// C: rotmat (engine/engine_collision_gjk.c:893)
/// Calls: norm3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn rotmat(R: *mut f64, axis: *const f64) {
    todo!() // rotmat
}

/// C: rayTriangle (engine/engine_collision_gjk.c:911)
/// Calls: det3, sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_triangle(v1: *const f64, v2: *const f64, v3: *const f64, v4: *const f64, v5: *const f64) -> i32 {
    todo!() // rayTriangle
}

/// C: triAffineCoord (engine/engine_collision_gjk.c:1016)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn tri_affine_coord(lambda: *mut f64, v1: *const f64, v2: *const f64, v3: *const f64, p: *const f64) {
    todo!() // triAffineCoord
}

/// C: triPointIntersect (engine/engine_collision_gjk.c:1061)
/// Calls: norm3, sub3, triAffineCoord
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn tri_point_intersect(v1: *const f64, v2: *const f64, v3: *const f64, p: *const f64) -> i32 {
    const MJ_MINVAL: f64 = 1E-15_f64;
    let mut lambda: [f64; 3] = [0.0; 3];
    tri_affine_coord(lambda.as_mut_ptr(), v1, v2, v3, p);
    if lambda[0] < 0.0 || lambda[1] < 0.0 || lambda[2] < 0.0 {
        return 0;
    }
    // SAFETY: v1, v2, v3, p are valid f64[3] pointers (caller contract from EPA/GJK)
    unsafe {
        let mut pr: [f64; 3] = [0.0; 3];
        pr[0] = *v1.add(0) * lambda[0] + *v2.add(0) * lambda[1] + *v3.add(0) * lambda[2];
        pr[1] = *v1.add(1) * lambda[0] + *v2.add(1) * lambda[1] + *v3.add(1) * lambda[2];
        pr[2] = *v1.add(2) * lambda[0] + *v2.add(2) * lambda[1] + *v3.add(2) * lambda[2];
        let mut diff: [f64; 3] = [0.0; 3];
        sub3(diff.as_mut_ptr(), pr.as_ptr(), p);
        if norm3(diff.as_ptr()) < MJ_MINVAL { 1 } else { 0 }
    }
}

/// C: deleteFace (engine/engine_collision_gjk.c:1216)
#[allow(unused_variables, non_snake_case)]
pub fn delete_face(pt: *mut Polytope, face: *mut Face) {
    todo!() // deleteFace
}

/// C: maxFaces (engine/engine_collision_gjk.c:1226)
#[allow(unused_variables, non_snake_case)]
pub fn max_faces(pt: *mut Polytope) -> i32 {
    todo!() // maxFaces
}

/// C: addEdge (engine/engine_collision_gjk.c:1263)
#[allow(unused_variables, non_snake_case)]
pub fn add_edge(pt: *mut Polytope, index: i32, edge: i32) {
    // SAFETY: pt is a valid Polytope pointer with horizon sub-struct (caller contract from EPA)
    unsafe {
        // Polytope.horizon is at a fixed offset; access fields via raw pointer arithmetic
        // horizon.edges[horizon.nedges] = edge
        // horizon.indices[horizon.nedges++] = index
        //
        // We cast to the C-layout Polytope and access horizon directly.
        // The Polytope struct layout from C:
        //   verts: *mut Vertex     (8 bytes)
        //   nverts: i32            (4 bytes) + pad(4)
        //   faces: *mut Face       (8 bytes)
        //   nfaces: i32            (4 bytes)
        //   maxfaces: i32          (4 bytes)
        //   center: [f64; 3]       (24 bytes)
        //   map: **Face            (8 bytes)
        //   nmap: i32              (4 bytes) + pad(4)
        //   horizon.indices: *mut i32 (8 bytes)
        //   horizon.edges: *mut i32   (8 bytes)
        //   horizon.nedges: i32       (4 bytes) + pad(4)
        //   horizon.w: *const f64     (8 bytes)

        #[repr(C)]
        struct Horizon {
            indices: *mut i32,
            edges: *mut i32,
            nedges: i32,
            _pad: i32,
            w: *const f64,
        }

        #[repr(C)]
        struct PolytopeRepr {
            verts: *mut u8,
            nverts: i32,
            _pad0: i32,
            faces: *mut u8,
            nfaces: i32,
            maxfaces: i32,
            center: [f64; 3],
            map: *mut *mut u8,
            nmap: i32,
            _pad1: i32,
            horizon: Horizon,
        }

        let p = pt as *mut PolytopeRepr;
        let nedges = (*p).horizon.nedges;
        *(*p).horizon.edges.add(nedges as usize) = edge;
        *(*p).horizon.indices.add(nedges as usize) = index;
        (*p).horizon.nedges = nedges + 1;
    }
}

/// C: getEdge (engine/engine_collision_gjk.c:1270)
#[allow(unused_variables, non_snake_case)]
pub fn get_edge(face: *mut Face, vertex: i32) -> i32 {
    #[repr(C)]
    struct FaceRepr {
        verts: i32,
        adj: [i32; 3],
        v: [f64; 3],
        dist2: f64,
        index: i32,
        _pad: i32,
    }

    // SAFETY: face is a valid Face pointer in the polytope (EPA invariant)
    unsafe {
        let f = face as *mut FaceRepr;
        let verts_packed = (*f).verts;
        let verts = [
            verts_packed & 0x3FF,
            (verts_packed >> 10) & 0x3FF,
            (verts_packed >> 20) & 0x3FF,
        ];
        if verts[0] == vertex { return 0; }
        if verts[1] == vertex { return 1; }
        2
    }
}

/// C: horizonRec (engine/engine_collision_gjk.c:1279)
/// Calls: addEdge, deleteFace, dot3, getEdge
#[allow(unused_variables, non_snake_case)]
pub fn horizon_rec(pt: *mut Polytope, face: *mut Face, e: i32) -> i32 {
    // Face layout (C struct):
    //   int verts;        offset 0
    //   int adj[3];       offset 4
    //   mjtNum v[3];      offset 16 (aligned to 8)
    //   mjtNum dist2;     offset 40
    //   int index;        offset 48
    // Total with padding: 56 bytes

    #[repr(C)]
    struct FaceRepr {
        verts: i32,
        adj: [i32; 3],
        v: [f64; 3],
        dist2: f64,
        index: i32,
        _pad: i32,
    }

    #[repr(C)]
    struct Horizon {
        indices: *mut i32,
        edges: *mut i32,
        nedges: i32,
        _pad: i32,
        w: *const f64,
    }

    #[repr(C)]
    struct PolytopeRepr {
        verts: *mut u8,
        nverts: i32,
        _pad0: i32,
        faces: *mut u8,
        nfaces: i32,
        maxfaces: i32,
        center: [f64; 3],
        map: *mut *mut u8,
        nmap: i32,
        _pad1: i32,
        horizon: Horizon,
    }

    // SAFETY: pt is a valid Polytope, face is a valid Face in pt->faces (EPA invariant)
    unsafe {
        let p = pt as *mut PolytopeRepr;
        let f = face as *mut FaceRepr;
        let w = (*p).horizon.w;

        // dot3(face->v, pt->horizon.w) - face->dist2 > mjMINVAL
        let dot = (*f).v[0] * *w.add(0) + (*f).v[1] * *w.add(1) + (*f).v[2] * *w.add(2);
        if dot - (*f).dist2 > 1e-15_f64 {
            let verts_packed = (*f).verts;
            let verts = [
                verts_packed & 0x3FF,
                (verts_packed >> 10) & 0x3FF,
                (verts_packed >> 20) & 0x3FF,
            ];

            delete_face(pt, face);

            // recursively search the adjacent faces on the next two edges
            for k in 1..3_i32 {
                let i = ((e + k) % 3) as usize;
                let adj_idx = (*f).adj[i];
                let faces_base = (*p).faces as *mut FaceRepr;
                let adj_face_ptr = faces_base.add(adj_idx as usize) as *mut Face;
                let adj_face_repr = faces_base.add(adj_idx as usize);

                if (*adj_face_repr).index > -2 {
                    let adj_edge = get_edge(adj_face_ptr, verts[(i + 1) % 3]);
                    if horizon_rec(pt, adj_face_ptr, adj_edge) == 0 {
                        add_edge(pt, adj_idx, adj_edge);
                    }
                }
            }
            return 1;
        }
        0
    }
}

/// C: horizon (engine/engine_collision_gjk.c:1303)
/// Calls: addEdge, deleteFace, getEdge, horizonRec
#[allow(unused_variables, non_snake_case)]
pub fn horizon(pt: *mut Polytope, face: *mut Face) {
    #[repr(C)]
    struct FaceRepr {
        verts: i32,
        adj: [i32; 3],
        v: [f64; 3],
        dist2: f64,
        index: i32,
        _pad: i32,
    }

    #[repr(C)]
    struct PolytopeRepr {
        verts: *mut u8,
        nverts: i32,
        _pad0: i32,
        faces: *mut u8,
        nfaces: i32,
        maxfaces: i32,
        center: [f64; 3],
        map: *mut *mut u8,
        nmap: i32,
        _pad1: i32,
    }

    // SAFETY: pt is a valid Polytope, face is a valid Face in pt->faces (EPA invariant)
    unsafe {
        let p = pt as *mut PolytopeRepr;
        let f = face as *mut FaceRepr;

        delete_face(pt, face);

        let verts_packed = (*f).verts;
        let verts = [
            verts_packed & 0x3FF,
            (verts_packed >> 10) & 0x3FF,
            (verts_packed >> 20) & 0x3FF,
        ];

        let faces_base = (*p).faces as *mut FaceRepr;

        // first edge
        let adj_face = faces_base.add((*f).adj[0] as usize) as *mut Face;
        let adj_edge = get_edge(adj_face, verts[1]);
        if horizon_rec(pt, adj_face, adj_edge) == 0 {
            add_edge(pt, (*f).adj[0], adj_edge);
        }

        // second edge
        let adj_face = faces_base.add((*f).adj[1] as usize) as *mut Face;
        let adj_face_repr = faces_base.add((*f).adj[1] as usize);
        let adj_edge = get_edge(adj_face, verts[2]);
        if (*adj_face_repr).index > -2 && horizon_rec(pt, adj_face, adj_edge) == 0 {
            add_edge(pt, (*f).adj[1], adj_edge);
        }

        // third edge
        let adj_face = faces_base.add((*f).adj[2] as usize) as *mut Face;
        let adj_face_repr = faces_base.add((*f).adj[2] as usize);
        let adj_edge = get_edge(adj_face, verts[0]);
        if (*adj_face_repr).index > -2 && horizon_rec(pt, adj_face, adj_edge) == 0 {
            add_edge(pt, (*f).adj[2], adj_edge);
        }
    }
}

/// C: epaWitness (engine/engine_collision_gjk.c:1331)
/// Calls: lincomb, triAffineCoord
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn epa_witness(pt: *const Polytope, face: *const Face, x1: *mut f64, x2: *mut f64) -> f64 {
    todo!() // epaWitness
}

/// C: area4 (engine/engine_collision_gjk.c:1505)
/// Calls: add3, cross3, norm3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn area4(a: *const f64, b: *const f64, c: *const f64, d: *const f64) -> f64 {
    // SAFETY: a, b, c, d each point to [3] arrays
    unsafe {
        let ad: [f64; 3] = [
            *d.add(0) - *a.add(0),
            *d.add(1) - *a.add(1),
            *d.add(2) - *a.add(2),
        ];
        let db: [f64; 3] = [
            *b.add(0) - *d.add(0),
            *b.add(1) - *d.add(1),
            *b.add(2) - *d.add(2),
        ];
        let bc: [f64; 3] = [
            *c.add(0) - *b.add(0),
            *c.add(1) - *b.add(1),
            *c.add(2) - *b.add(2),
        ];
        let ca: [f64; 3] = [
            *a.add(0) - *c.add(0),
            *a.add(1) - *c.add(1),
            *a.add(2) - *c.add(2),
        ];
        let mut e: [f64; 3] = [0.0; 3];
        let mut f: [f64; 3] = [0.0; 3];
        let mut g: [f64; 3] = [0.0; 3];
        cross3(e.as_mut_ptr(), ad.as_ptr(), db.as_ptr());
        cross3(f.as_mut_ptr(), bc.as_ptr(), ca.as_ptr());
        add3(g.as_mut_ptr(), e.as_ptr(), f.as_ptr());
        0.5 * norm3(g.as_ptr())
    }
}

/// C: next (engine/engine_collision_gjk.c:1520)
/// Calls: mjCMesh::nvert
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn next(polygon: *mut f64, nvert: i32, curr: *mut f64) -> *mut f64 {
    todo!() // next
}

/// C: polygonQuad (engine/engine_collision_gjk.c:1529)
/// Calls: area4, next
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn polygon_quad(res: *mut *mut f64, polygon: *mut f64, nvert: i32) {
    todo!() // polygonQuad
}

/// C: planeNormal (engine/engine_collision_gjk.c:1577)
/// Calls: add3, cross3, dot3, mju_normalize3, sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn plane_normal(res: *mut f64, v1: *const f64, v2: *const f64, n: *const f64) -> f64 {
    todo!() // planeNormal
}

/// C: halfspace (engine/engine_collision_gjk.c:1592)
/// Calls: dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn halfspace(a: *const f64, n: *const f64, p: *const f64) -> i32 {
    todo!() // halfspace
}

/// C: planeIntersect (engine/engine_collision_gjk.c:1599)
/// Calls: dot3, sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn plane_intersect(res: *mut f64, pn: *const f64, pd: f64, a: *const f64, b: *const f64) -> f64 {
    todo!() // planeIntersect
}

/// C: polygonClip (engine/engine_collision_gjk.c:1616)
/// Calls: copy3, dot3, halfspace, planeIntersect, planeNormal, polygonQuad, sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn polygon_clip(status: *mut mjCCDStatus, face1: *const f64, nface1: i32, face2: *const f64, nface2: i32, n: *const f64, dir: *const f64) {
    todo!() // polygonClip
}

/// C: globalcoord (engine/engine_collision_gjk.c:1744)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn globalcoord(res: *mut f64, mat: *const f64, pos: *const f64, l1: f64, l2: f64, l3: f64) {
    // SAFETY: res points to [3], mat points to [9], pos (if non-null) points to [3]
    unsafe {
        // perform mat * (l1, l2, l3) + pos
        *res.add(0) = *mat.add(0) * l1 + *mat.add(1) * l2 + *mat.add(2) * l3;
        *res.add(1) = *mat.add(3) * l1 + *mat.add(4) * l2 + *mat.add(5) * l3;
        *res.add(2) = *mat.add(6) * l1 + *mat.add(7) * l2 + *mat.add(8) * l3;
        if !pos.is_null() {
            *res.add(0) += *pos.add(0);
            *res.add(1) += *pos.add(1);
            *res.add(2) += *pos.add(2);
        }
    }
}

/// C: intersect (engine/engine_collision_gjk.c:1759)
/// Calls: GlobalTable::count
#[allow(unused_variables, non_snake_case)]
pub fn intersect(res: *mut i32, arr1: *const i32, arr2: *const i32, n: i32, m: i32) -> i32 {
    todo!() // intersect
}

/// C: meshNormals (engine/engine_collision_gjk.c:1774)
/// Calls: globalcoord, intersect
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mesh_normals(res: *mut f64, resind: *mut i32, dim: i32, obj: *mut mjCCDObj, v1: i32, v2: i32, v3: i32) -> i32 {
    todo!() // meshNormals
}

/// C: meshEdgeNormals (engine/engine_collision_gjk.c:1840)
/// Calls: copy3, globalcoord, mju_normalize3, sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mesh_edge_normals(res: *mut f64, endverts: *mut f64, dim: i32, obj: *mut mjCCDObj, v1: *const f64, v2: *const f64, v1i: i32, v2i: i32) -> i32 {
    todo!() // meshEdgeNormals
}

/// C: boxNormals2 (engine/engine_collision_gjk.c:1885)
/// Calls: dot3, globalcoord, scl3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn box_normals2(res: *mut f64, resind: *mut i32, mat: *const f64, n: *const f64) -> i32 {
    todo!() // boxNormals2
}

/// C: boxNormals (engine/engine_collision_gjk.c:1911)
/// Calls: boxNormals2, globalcoord
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn box_normals(res: *mut f64, resind: *mut i32, dim: i32, obj: *mut mjCCDObj, v1: i32, v2: i32, v3: i32, dir: *const f64) -> i32 {
    todo!() // boxNormals
}

/// C: boxEdgeNormals (engine/engine_collision_gjk.c:1965)
/// Calls: copy3, globalcoord, mju_normalize3, sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn box_edge_normals(res: *mut f64, endverts: *mut f64, dim: i32, obj: *mut mjCCDObj, v1: *const f64, v2: *const f64, v1i: i32, v2i: i32) -> i32 {
    todo!() // boxEdgeNormals
}

/// C: boxFace (engine/engine_collision_gjk.c:2002)
/// Calls: globalcoord
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn box_face(res: *mut f64, obj: *mut mjCCDObj, idx: i32) -> i32 {
    todo!() // boxFace
}

/// C: meshFace (engine/engine_collision_gjk.c:2052)
/// Calls: globalcoord
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mesh_face(res: *mut f64, obj: *mut mjCCDObj, idx: i32) -> i32 {
    todo!() // meshFace
}

/// C: alignedFaces (engine/engine_collision_gjk.c:2072)
/// Calls: dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn aligned_faces(res: *mut i32, v: *const f64, nv: i32, w: *const f64, nw: i32) -> i32 {
    // SAFETY: caller guarantees res points to [2], v to [nv*3], w to [nw*3]
    unsafe {
        for i in 0..nv as usize {
            for j in 0..nw as usize {
                if dot3(v.add(3 * i), w.add(3 * j)) < -0.99999872 {
                    *res.add(0) = i as i32;
                    *res.add(1) = j as i32;
                    return 1;
                }
            }
        }
        0
    }
}

/// C: alignedFaceEdge (engine/engine_collision_gjk.c:2088)
/// Calls: dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn aligned_face_edge(res: *mut i32, edge: *const f64, nedge: i32, face: *const f64, nface: i32) -> i32 {
    // SAFETY: caller guarantees res points to [2], edge to [nedge*3], face to [nface*3]
    unsafe {
        for i in 0..nface as usize {
            for j in 0..nedge as usize {
                if dot3(edge.add(3 * j), face.add(3 * i)).abs() < 0.00159999931 {
                    *res.add(0) = j as i32;
                    *res.add(1) = i as i32;
                    return 1;
                }
            }
        }
        0
    }
}

/// C: simplexDim (engine/engine_collision_gjk.c:2104)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn simplex_dim(v1i: *mut i32, v2i: *mut i32, v3i: *mut i32, v1: *mut *mut f64, v2: *mut *mut f64, v3: *mut *mut f64) -> i32 {
    // SAFETY: all pointers are valid and dereferenceable.
    unsafe {
        let val1 = *v1i;
        let val2 = *v2i;
        let val3 = *v3i;

        if val1 != val2 {
            return if val3 == val1 || val3 == val2 { 2 } else { 3 };
        }
        if val1 != val3 {
            *v2i = *v3i;
            *v2 = *v3;
            return 2;
        }
        1
    }
}

/// C: multicontact (engine/engine_collision_gjk.c:2122)
/// Calls: alignedFaceEdge, alignedFaces, boxEdgeNormals, boxFace, boxNormals, copy3, meshEdgeNormals, meshFace, meshNormals, norm3, polygonClip, scl3, simplexDim, sub3
#[allow(unused_variables, non_snake_case)]
pub fn multicontact(pt: *mut Polytope, face: *mut Face, status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) {
    todo!() // multicontact
}

/// C: inflate (engine/engine_collision_gjk.c:2264)
/// Calls: mju_normalize3, sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn inflate(status: *mut mjCCDStatus, margin1: f64, margin2: f64) {
    use crate::engine::engine_util_blas::mju_normalize3;

    // mjCCDStatus layout:
    //   offset 0: dist (f64)
    //   offset 8: x1[3*50] (f64 array)
    //   offset 1208: x2[3*50] (f64 array)
    const OFFSET_DIST: usize = 0;
    const OFFSET_X1: usize = 8;
    const OFFSET_X2: usize = 8 + 3 * 50 * 8; // 1208

    // SAFETY: status points to valid mjCCDStatus with the layout above.
    unsafe {
        let base = status as *mut u8;
        let dist_ptr = base.add(OFFSET_DIST) as *mut f64;
        let x1 = base.add(OFFSET_X1) as *mut f64;
        let x2 = base.add(OFFSET_X2) as *mut f64;

        let mut n: [f64; 3] = [0.0; 3];
        sub3(n.as_mut_ptr(), x2 as *const f64, x1 as *const f64);
        mju_normalize3(n.as_mut_ptr());

        if margin1 != 0.0 {
            *x1.add(0) += margin1 * n[0];
            *x1.add(1) += margin1 * n[1];
            *x1.add(2) += margin1 * n[2];
        }
        if margin2 != 0.0 {
            *x2.add(0) -= margin2 * n[0];
            *x2.add(1) -= margin2 * n[1];
            *x2.add(2) -= margin2 * n[2];
        }
        *dist_ptr -= margin1 + margin2;
    }
}

/// C: mjc_ccdSize (engine/engine_collision_gjk.h:105)
/// Calls: align8
#[allow(unused_variables, non_snake_case)]
pub fn mjc_ccd_size(iterations: i32) -> usize {
    // C struct sizes (double precision, 64-bit):
    //   Vertex: 3*mjtNum[3] + 2*int = 72 + 8 = 80 bytes
    //   Face: int + int[3] + mjtNum[3] + mjtNum + int + padding = 56 bytes
    const SIZEOF_VERTEX: usize = 80;
    const SIZEOF_FACE: usize = 56;
    const SIZEOF_FACE_PTR: usize = 8;  // pointer size on 64-bit
    const SIZEOF_INT: usize = 4;

    let n = iterations as usize;
    align8(SIZEOF_VERTEX * (5 + n))       // vertices in polytope
        + align8(SIZEOF_FACE * 6 * n)     // faces in polytope
        + align8(SIZEOF_FACE_PTR * 6 * n) // map in polytope
        + align8(SIZEOF_INT * 24)         // horizon indices
        + align8(SIZEOF_INT * 24)         // horizon edges
}

/// C: mjc_ccd (engine/engine_collision_gjk.h:108)
/// Calls: align8, epa, gjk, inflate, multicontact, polytope2, polytope3, polytope4
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_ccd(config: *const mjCCDConfig, status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> f64 {
    todo!() // mjc_ccd
}

