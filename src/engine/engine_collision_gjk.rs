//! Port of: engine/engine_collision_gjk.c
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: align8 (engine/engine_collision_gjk.c:49)
#[allow(unused_variables, non_snake_case)]
pub fn align8(size: usize) -> usize {
    ((size + 7) / 8) * 8
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
    if lambda.is_null() || simplex.is_null() {
        return;
    }
    extern "C" { fn subdistance(lambda: *mut f64, n: i32, simplex: *const Vertex); }
    // SAFETY: lambda, simplex verified non-null; delegates to C implementation
    unsafe { subdistance(lambda, n, simplex) }
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
    // SAFETY: lambda points to 4 f64; s1-s4 each point to 3 f64
    unsafe {
        // compute cofactors to find det(M)
        let C41: f64 = -crate::engine::engine_collision_gjk::det3(s2, s3, s4);
        let C42: f64 =  crate::engine::engine_collision_gjk::det3(s1, s3, s4);
        let C43: f64 = -crate::engine::engine_collision_gjk::det3(s1, s2, s4);
        let C44: f64 =  crate::engine::engine_collision_gjk::det3(s1, s2, s3);

        let m_det: f64 = C41 + C42 + C43 + C44;

        let comp1: i32 = crate::engine::engine_collision_gjk::same_sign2(m_det, C41);
        let comp2: i32 = crate::engine::engine_collision_gjk::same_sign2(m_det, C42);
        let comp3: i32 = crate::engine::engine_collision_gjk::same_sign2(m_det, C43);
        let comp4: i32 = crate::engine::engine_collision_gjk::same_sign2(m_det, C44);

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
            crate::engine::engine_collision_gjk::s2d(lambda_2d.as_mut_ptr(), s2, s3, s4);
            crate::engine::engine_collision_gjk::lincomb(
                x.as_mut_ptr(), lambda_2d.as_ptr(), 3, s2, s3, s4, std::ptr::null());
            let d: f64 = crate::engine::engine_collision_gjk::dot3(x.as_ptr(), x.as_ptr());
            *lambda.add(0) = 0.0;
            *lambda.add(1) = lambda_2d[0];
            *lambda.add(2) = lambda_2d[1];
            *lambda.add(3) = lambda_2d[2];
            dmin = d;
        }

        if comp2 == 0 {
            let mut lambda_2d: [f64; 3] = [0.0; 3];
            let mut x: [f64; 3] = [0.0; 3];
            crate::engine::engine_collision_gjk::s2d(lambda_2d.as_mut_ptr(), s1, s3, s4);
            crate::engine::engine_collision_gjk::lincomb(
                x.as_mut_ptr(), lambda_2d.as_ptr(), 3, s1, s3, s4, std::ptr::null());
            let d: f64 = crate::engine::engine_collision_gjk::dot3(x.as_ptr(), x.as_ptr());
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
            crate::engine::engine_collision_gjk::s2d(lambda_2d.as_mut_ptr(), s1, s2, s4);
            crate::engine::engine_collision_gjk::lincomb(
                x.as_mut_ptr(), lambda_2d.as_ptr(), 3, s1, s2, s4, std::ptr::null());
            let d: f64 = crate::engine::engine_collision_gjk::dot3(x.as_ptr(), x.as_ptr());
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
            crate::engine::engine_collision_gjk::s2d(lambda_2d.as_mut_ptr(), s1, s2, s3);
            crate::engine::engine_collision_gjk::lincomb(
                x.as_mut_ptr(), lambda_2d.as_ptr(), 3, s1, s2, s3, std::ptr::null());
            let d: f64 = crate::engine::engine_collision_gjk::dot3(x.as_ptr(), x.as_ptr());
            if d < dmin {
                *lambda.add(0) = lambda_2d[0];
                *lambda.add(1) = lambda_2d[1];
                *lambda.add(2) = lambda_2d[2];
                // lambda[3] not assigned in C (stays from last write or init)
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
    // SAFETY: lambda points to 3 f64; s1-s3 each point to 3 f64
    unsafe {
        // project origin onto affine hull of the simplex
        let mut p_o: [f64; 3] = [0.0; 3];
        if crate::engine::engine_collision_gjk::project_origin_plane(
            p_o.as_mut_ptr(), s1, s2, s3) != 0 {
            crate::engine::engine_collision_gjk::s1d(lambda, s1, s2);
            *lambda.add(2) = 0.0;
            return;
        }

        // Minors M_i4
        let M_14: f64 = *s2.add(1) * *s3.add(2) - *s2.add(2) * *s3.add(1)
                       - *s1.add(1) * *s3.add(2) + *s1.add(2) * *s3.add(1)
                       + *s1.add(1) * *s2.add(2) - *s1.add(2) * *s2.add(1);
        let M_24: f64 = *s2.add(0) * *s3.add(2) - *s2.add(2) * *s3.add(0)
                       - *s1.add(0) * *s3.add(2) + *s1.add(2) * *s3.add(0)
                       + *s1.add(0) * *s2.add(2) - *s1.add(2) * *s2.add(0);
        let M_34: f64 = *s2.add(0) * *s3.add(1) - *s2.add(1) * *s3.add(0)
                       - *s1.add(0) * *s3.add(1) + *s1.add(1) * *s3.add(0)
                       + *s1.add(0) * *s2.add(1) - *s1.add(1) * *s2.add(0);

        // exclude the axis with the largest projection
        let M_max: f64;
        let mut s1_2D: [f64; 2] = [0.0; 2];
        let mut s2_2D: [f64; 2] = [0.0; 2];
        let mut s3_2D: [f64; 2] = [0.0; 2];
        let mut p_o_2D: [f64; 2] = [0.0; 2];
        let mu1: f64 = M_14.abs();
        let mu2: f64 = M_24.abs();
        let mu3: f64 = M_34.abs();

        if mu1 >= mu2 && mu1 >= mu3 {
            M_max = M_14;
            s1_2D[0] = *s1.add(1); s1_2D[1] = *s1.add(2);
            s2_2D[0] = *s2.add(1); s2_2D[1] = *s2.add(2);
            s3_2D[0] = *s3.add(1); s3_2D[1] = *s3.add(2);
            p_o_2D[0] = p_o[1]; p_o_2D[1] = p_o[2];
        } else if mu2 >= mu3 {
            M_max = M_24;
            s1_2D[0] = *s1.add(0); s1_2D[1] = *s1.add(2);
            s2_2D[0] = *s2.add(0); s2_2D[1] = *s2.add(2);
            s3_2D[0] = *s3.add(0); s3_2D[1] = *s3.add(2);
            p_o_2D[0] = p_o[0]; p_o_2D[1] = p_o[2];
        } else {
            M_max = M_34;
            s1_2D[0] = *s1.add(0); s1_2D[1] = *s1.add(1);
            s2_2D[0] = *s2.add(0); s2_2D[1] = *s2.add(1);
            s3_2D[0] = *s3.add(0); s3_2D[1] = *s3.add(1);
            p_o_2D[0] = p_o[0]; p_o_2D[1] = p_o[1];
        }

        // compute the cofactors C3i
        // C31 corresponds to the signed area of 2-simplex: (p_o_2D, s2_2D, s3_2D)
        let C31: f64 = p_o_2D[0] * s2_2D[1] + p_o_2D[1] * s3_2D[0] + s2_2D[0] * s3_2D[1]
                     - p_o_2D[0] * s3_2D[1] - p_o_2D[1] * s2_2D[0] - s3_2D[0] * s2_2D[1];

        // C32 corresponds to the signed area of 2-simplex: (p_o_2D, s1_2D, s3_2D)
        let C32: f64 = p_o_2D[0] * s3_2D[1] + p_o_2D[1] * s1_2D[0] + s3_2D[0] * s1_2D[1]
                     - p_o_2D[0] * s1_2D[1] - p_o_2D[1] * s3_2D[0] - s1_2D[0] * s3_2D[1];

        // C33 corresponds to the signed area of 2-simplex: (p_o_2D, s1_2D, s2_2D)
        let C33: f64 = p_o_2D[0] * s1_2D[1] + p_o_2D[1] * s2_2D[0] + s1_2D[0] * s2_2D[1]
                     - p_o_2D[0] * s2_2D[1] - p_o_2D[1] * s1_2D[0] - s2_2D[0] * s1_2D[1];

        let comp1: i32 = crate::engine::engine_collision_gjk::same_sign2(M_max, C31);
        let comp2: i32 = crate::engine::engine_collision_gjk::same_sign2(M_max, C32);
        let comp3: i32 = crate::engine::engine_collision_gjk::same_sign2(M_max, C33);

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
            crate::engine::engine_collision_gjk::s1d(lambda_1d.as_mut_ptr(), s2, s3);
            crate::engine::engine_collision_gjk::lincomb(
                x.as_mut_ptr(), lambda_1d.as_ptr(), 2, s2, s3, std::ptr::null(), std::ptr::null());
            let d: f64 = crate::engine::engine_collision_gjk::dot3(x.as_ptr(), x.as_ptr());
            *lambda.add(0) = 0.0;
            *lambda.add(1) = lambda_1d[0];
            *lambda.add(2) = lambda_1d[1];
            dmin = d;
        }

        if comp2 == 0 {
            let mut lambda_1d: [f64; 2] = [0.0; 2];
            let mut x: [f64; 3] = [0.0; 3];
            crate::engine::engine_collision_gjk::s1d(lambda_1d.as_mut_ptr(), s1, s3);
            crate::engine::engine_collision_gjk::lincomb(
                x.as_mut_ptr(), lambda_1d.as_ptr(), 2, s1, s3, std::ptr::null(), std::ptr::null());
            let d: f64 = crate::engine::engine_collision_gjk::dot3(x.as_ptr(), x.as_ptr());
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
            crate::engine::engine_collision_gjk::s1d(lambda_1d.as_mut_ptr(), s1, s2);
            crate::engine::engine_collision_gjk::lincomb(
                x.as_mut_ptr(), lambda_1d.as_ptr(), 2, s1, s2, std::ptr::null(), std::ptr::null());
            let d: f64 = crate::engine::engine_collision_gjk::dot3(x.as_ptr(), x.as_ptr());
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
    unsafe {
        // SAFETY: lambda points to at least 2 f64; s1, s2 point to 3 f64 each
        let mut p_o: [f64; 3] = [0.0; 3];
        project_origin_line(p_o.as_mut_ptr(), s1, s2);

        let mut mu: f64 = *s1.add(0) - *s2.add(0);
        let mut mu_max: f64 = mu;
        let mut index: usize = 0;

        mu = *s1.add(1) - *s2.add(1);
        if mu.abs() >= mu_max.abs() {
            mu_max = mu;
            index = 1;
        }

        mu = *s1.add(2) - *s2.add(2);
        if mu.abs() >= mu_max.abs() {
            mu_max = mu;
            index = 2;
        }

        let c1: f64 = p_o[index] - *s2.add(index);
        let c2: f64 = *s1.add(index) - p_o[index];
        let same = same_sign2(mu_max, c1) != 0 && same_sign2(mu_max, c2) != 0;

        if same {
            *lambda.add(0) = c1 / mu_max;
            *lambda.add(1) = c2 / mu_max;
        } else {
            *lambda.add(0) = 0.0;
            *lambda.add(1) = 1.0;
        }
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
    if v.is_null() { return; }
    extern "C" { fn gjkSupport(v: *mut Vertex, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj, x_k: *const f64, x_norm: f64); }
    // SAFETY: v verified non-null; delegates to C implementation
    unsafe { gjkSupport(v, obj1, obj2, x_k, x_norm) }
}

/// C: lincomb (engine/engine_collision_gjk.c:70)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn lincomb(res: *mut f64, coef: *const f64, n: i32, v1: *const f64, v2: *const f64, v3: *const f64, v4: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, coef : * const f64, n : i32, v1 : * const f64, v2 : * const f64, v3 : * const f64, v4 : * const f64)
    // Previous return: ()
    unsafe {
        // SAFETY: raw pointer arithmetic matching C lincomb switch(n) cases
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
pub fn epa_support(pt: *mut Polytope, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj, d: *const f64, dnorm: f64) -> i32  {
    if pt.is_null() { return 0; }
    extern "C" { fn epaSupport(pt: *mut Polytope, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj, d: *const f64, dnorm: f64) -> i32; }
    // SAFETY: pt verified non-null; delegates to C implementation
    unsafe { epaSupport(pt, obj1, obj2, d, dnorm) }
}

/// C: insertVertex (engine/engine_collision_gjk.c:112)
#[allow(unused_variables, non_snake_case)]
pub fn insert_vertex(pt: *mut Polytope, v: *const Vertex) -> i32  {
    if pt.is_null() || v.is_null() {
        return -1;
    }
    extern "C" { fn insertVertex(pt: *mut Polytope, v: *const Vertex) -> i32; }
    // SAFETY: pt, v verified non-null; delegates to C implementation
    unsafe { insertVertex(pt, v) }
}

/// C: attachFace (engine/engine_collision_gjk.c:115)
/// Calls: dot3, projectOriginPlane, scl3, sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn attach_face(pt: *mut Polytope, v1: i32, v2: i32, v3: i32, adj1: i32, adj2: i32, adj3: i32) -> f64  {
    if pt.is_null() {
        return 0.0;
    }
    extern "C" { fn attachFace(pt: *mut Polytope, v1: i32, v2: i32, v3: i32, adj1: i32, adj2: i32, adj3: i32) -> f64; }
    // SAFETY: pt verified non-null; delegates to C implementation
    unsafe { attachFace(pt, v1, v2, v3, adj1, adj2, adj3) }
}

/// C: gjkIntersect (engine/engine_collision_gjk.c:119)
/// Calls: dot3, gjkIntersectSupport, signedDistance
#[allow(unused_variables, non_snake_case)]
pub fn gjk_intersect(status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> i32  {
    extern "C" { fn gjkIntersect(status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { gjkIntersect(status, obj1, obj2) }
}

/// C: polytope2 (engine/engine_collision_gjk.c:122)
/// Calls: add3, attachFace, cross3, epaSupport, insertVertex, mju_mulMatVec3, norm3, polytope3, rayTriangle, replaceSimplex3, rotmat, scl3, sub3
#[allow(unused_variables, non_snake_case)]
pub fn polytope2(pt: *mut Polytope, status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> i32 {
    extern "C" {
        fn polytope2(pt: *mut Polytope, status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { polytope2(pt, status, obj1, obj2) }
}

/// C: polytope3 (engine/engine_collision_gjk.c:123)
/// Calls: add3, attachFace, cross3, epaSupport, insertVertex, norm3, scl3, sub3, testTetra, triPointIntersect
#[allow(unused_variables, non_snake_case)]
pub fn polytope3(pt: *mut Polytope, status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> i32 {
    extern "C" {
        fn polytope3(pt: *mut Polytope, status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { polytope3(pt, status, obj1, obj2) }
}

/// C: polytope4 (engine/engine_collision_gjk.c:124)
/// Calls: add3, attachFace, insertVertex, polytope3, replaceSimplex3, scl3, testTetra
#[allow(unused_variables, non_snake_case)]
pub fn polytope4(pt: *mut Polytope, status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> i32 {
    extern "C" {
        fn polytope4(pt: *mut Polytope, status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { polytope4(pt, status, obj1, obj2) }
}

/// C: epa (engine/engine_collision_gjk.c:128)
/// Calls: attachFace, discreteGeoms, dot3, epaSupport, epaWitness, horizon, maxFaces, mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn epa(status: *mut mjCCDStatus, pt: *mut Polytope, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> *mut Face {
    extern "C" {
        fn epa(status: *mut mjCCDStatus, pt: *mut Polytope, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> *mut Face;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { epa(status, pt, obj1, obj2) }
}

/// C: equal3 (engine/engine_collision_gjk.c:133)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn equal3(v1: *const f64, v2: *const f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (v1 : * const f64, v2 : * const f64)
    // Previous return: i32
    unsafe {
        // SAFETY: raw pointer arithmetic matching C equal3
        const MJ_MINVAL: f64 = 1e-15;
        if (*v1.add(0) - *v2.add(0)).abs() < MJ_MINVAL
            && (*v1.add(1) - *v2.add(1)).abs() < MJ_MINVAL
            && (*v1.add(2) - *v2.add(2)).abs() < MJ_MINVAL
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
    unsafe {
        // SAFETY: raw pointer arithmetic matching C add3
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
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, v1 : * const f64, v2 : * const f64)
    // Previous return: ()
    unsafe { * res . add (0) = * v1 . add (0) - * v2 . add (0) ; * res . add (1) = * v1 . add (1) - * v2 . add (1) ; * res . add (2) = * v1 . add (2) - * v2 . add (2) ; }
}

/// C: dot3 (engine/engine_collision_gjk.c:150)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn dot3(v1: *const f64, v2: *const f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (v1 : * const f64, v2 : * const f64)
    // Previous return: f64
    unsafe { * v1 . add (0) * * v2 . add (0) + * v1 . add (1) * * v2 . add (1) + * v1 . add (2) * * v2 . add (2) }
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
    // delegates to dot3 which performs raw pointer reads internally
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
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, v : * const f64)
    // Previous return: ()
    unsafe {
        // SAFETY: raw pointer arithmetic matching C copy3
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
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, v : * const f64, s : f64)
    // Previous return: ()
    unsafe { * res . add (0) = s * * v . add (0) ; * res . add (1) = s * * v . add (1) ; * res . add (2) = s * * v . add (2) ; }
}

/// C: cross3 (engine/engine_collision_gjk.c:170)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn cross3(res: *mut f64, v1: *const f64, v2: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, v1 : * const f64, v2 : * const f64)
    // Previous return: ()
    unsafe { * res . add (0) = * v1 . add (1) * * v2 . add (2) - * v1 . add (2) * * v2 . add (1) ; * res . add (1) = * v1 . add (2) * * v2 . add (0) - * v1 . add (0) * * v2 . add (2) ; * res . add (2) = * v1 . add (0) * * v2 . add (1) - * v1 . add (1) * * v2 . add (0) ; }
}

/// C: det3 (engine/engine_collision_gjk.c:177)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn det3(v1: *const f64, v2: *const f64, v3: *const f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (v1 : * const f64, v2 : * const f64, v3 : * const f64)
    // Previous return: f64
    unsafe {
        // SAFETY: raw pointer arithmetic matching C det3 (scalar triple product)
        *v1.add(0) * (*v2.add(1) * *v3.add(2) - *v2.add(2) * *v3.add(1))
        + *v1.add(1) * (*v2.add(2) * *v3.add(0) - *v2.add(0) * *v3.add(2))
        + *v1.add(2) * (*v2.add(0) * *v3.add(1) - *v2.add(1) * *v3.add(0))
    }
}

/// C: discreteGeoms (engine/engine_collision_gjk.c:188)
#[allow(unused_variables, non_snake_case)]
pub fn discrete_geoms(obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> i32 {
    unsafe {
        // SAFETY: obj1, obj2 are valid mjCCDObj pointers with typed fields
        const MJ_GEOM_MESH: i32 = 7;
        const MJ_GEOM_BOX: i32 = 6;
        const MJ_GEOM_HFIELD: i32 = 1;

        // non-zero margin makes geoms smooth
        if (*obj1).margin != 0.0 || (*obj2).margin != 0.0 {
            return 0;
        }

        let g1: i32 = (*obj1).geom_type;
        let g2: i32 = (*obj2).geom_type;
        ((g1 == MJ_GEOM_MESH || g1 == MJ_GEOM_BOX || g1 == MJ_GEOM_HFIELD)
            && (g2 == MJ_GEOM_MESH || g2 == MJ_GEOM_BOX || g2 == MJ_GEOM_HFIELD)) as i32
    }
}

/// C: gjk (engine/engine_collision_gjk.c:200)
/// Calls: copy3, discreteGeoms, dot3, equal3, gjkIntersect, gjkSupport, lincomb, sub3, subdistance
#[allow(unused_variables, non_snake_case)]
pub fn gjk(status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) {
    extern "C" { fn gjk(status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj); }
    // SAFETY: delegates to C implementation
    unsafe { gjk(status, obj1, obj2) }
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
    if v.is_null() || obj1.is_null() || obj2.is_null() {
        return;
    }
    extern "C" { fn support(v: *mut Vertex, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj, dir: *const f64, dir_neg: *const f64); }
    // SAFETY: v, obj1, obj2 verified non-null; delegates to C implementation
    unsafe { support(v, obj1, obj2, dir, dir_neg) }
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
    if v.is_null() { return; }
    extern "C" { fn gjkIntersectSupport(v: *mut Vertex, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj, dir: *const f64); }
    // SAFETY: v verified non-null; delegates to C implementation
    unsafe { gjkIntersectSupport(v, obj1, obj2, dir) }
}

/// C: signedDistance (engine/engine_collision_gjk.c:404)
/// Calls: cross3, dot3, scl3, sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn signed_distance(normal: *mut f64, v1: *const Vertex, v2: *const Vertex, v3: *const Vertex) -> f64  {
    if v1.is_null() || v2.is_null() || v3.is_null() {
        return 0.0;
    }
    extern "C" { fn signedDistance(normal: *mut f64, v1: *const Vertex, v2: *const Vertex, v3: *const Vertex) -> f64; }
    // SAFETY: v1, v2, v3 verified non-null; delegates to C implementation
    unsafe { signedDistance(normal, v1, v2, v3) }
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
    const MJ_MINVAL: f64 = 1e-15;
    unsafe {
        // SAFETY: res, v1, v2, v3 are f64[3] arrays from caller
        let mut diff21: [f64; 3] = [0.0; 3];
        let mut diff31: [f64; 3] = [0.0; 3];
        let mut diff32: [f64; 3] = [0.0; 3];
        let mut n: [f64; 3] = [0.0; 3];

        sub3(diff21.as_mut_ptr(), v2, v1);
        sub3(diff31.as_mut_ptr(), v3, v1);
        sub3(diff32.as_mut_ptr(), v3, v2);

        // n = (v3 - v2) x (v2 - v1)
        cross3(n.as_mut_ptr(), diff32.as_ptr(), diff21.as_ptr());
        let nv: f64 = dot3(n.as_ptr(), v2);
        let nn: f64 = dot3(n.as_ptr(), n.as_ptr());
        if nn == 0.0 {
            return 1;
        }
        if nv != 0.0 && nn > MJ_MINVAL {
            scl3(res, n.as_ptr(), nv / nn);
            return 0;
        }

        // n = (v2 - v1) x (v3 - v1)
        cross3(n.as_mut_ptr(), diff21.as_ptr(), diff31.as_ptr());
        let nv: f64 = dot3(n.as_ptr(), v1);
        let nn: f64 = dot3(n.as_ptr(), n.as_ptr());
        if nn == 0.0 {
            return 1;
        }
        if nv != 0.0 && nn > MJ_MINVAL {
            scl3(res, n.as_ptr(), nv / nn);
            return 0;
        }

        // n = (v3 - v1) x (v3 - v2)
        cross3(n.as_mut_ptr(), diff31.as_ptr(), diff32.as_ptr());
        let nv: f64 = dot3(n.as_ptr(), v3);
        let nn: f64 = dot3(n.as_ptr(), n.as_ptr());
        scl3(res, n.as_ptr(), nv / nn);
        0
    }
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
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, v1 : * const f64, v2 : * const f64)
    // Previous return: ()
    unsafe {
        // SAFETY: raw pointer arithmetic matching C projectOriginLine
        let mut diff: [f64; 3] = [0.0; 3];
        sub3(diff.as_mut_ptr(), v2, v1);
        let scl = -(dot3(v2, diff.as_ptr()) / dot3(diff.as_ptr(), diff.as_ptr()));
        *res.add(0) = *v2.add(0) + scl * *diff.as_ptr().add(0);
        *res.add(1) = *v2.add(1) + scl * *diff.as_ptr().add(1);
        *res.add(2) = *v2.add(2) + scl * *diff.as_ptr().add(2);
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
    // WARNING: signature changed — verify body
    // Previous params: (a : f64, b : f64)
    // Previous return: i32
    // SAFETY: no pointer ops, pure arithmetic matching C sameSign2
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
    if pt.is_null() || status.is_null() {
        return;
    }
    extern "C" { fn replaceSimplex3(pt: *mut Polytope, status: *mut mjCCDStatus, v1: i32, v2: i32, v3: i32); }
    // SAFETY: pt, status verified non-null; delegates to C implementation
    unsafe { replaceSimplex3(pt, status, v1, v2, v3) }
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
    unsafe {
        // SAFETY: p0..p3 each point to 3 f64 elements
        let mut diff1: [f64; 3] = [0.0; 3];
        let mut diff2: [f64; 3] = [0.0; 3];
        let mut diff3: [f64; 3] = [0.0; 3];
        let mut diff4: [f64; 3] = [0.0; 3];
        let mut n: [f64; 3] = [0.0; 3];

        sub3(diff1.as_mut_ptr(), p1, p0);
        sub3(diff2.as_mut_ptr(), p2, p0);
        cross3(n.as_mut_ptr(), diff1.as_ptr(), diff2.as_ptr());

        sub3(diff3.as_mut_ptr(), p3, p0);
        let dot1: f64 = dot3(n.as_ptr(), diff3.as_ptr());

        scl3(diff4.as_mut_ptr(), p0, -1.0);
        let dot2: f64 = dot3(n.as_ptr(), diff4.as_ptr());

        if dot1 > 0.0 && dot2 > 0.0 {
            return 1;
        }
        if dot1 < 0.0 && dot2 < 0.0 {
            return 1;
        }
        0
    }
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
    // SAFETY: p0..p3 each point to 3 f64 elements; delegates to same_side
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
    unsafe {
        // SAFETY: R points to 9 f64 elements, axis points to 3 f64 elements
        const MJ_MINVAL: f64 = 1e-15;
        let n = norm3(axis);
        let mut c: f64 = 1.0;
        let mut s: f64 = 0.0;
        let mut t: f64 = 0.0;
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        let mut z: f64 = 1.0;
        if n > MJ_MINVAL {
            c = n.cos();
            s = n.sin();
            t = 1.0 - c;
            x = *axis.add(0) / n;
            y = *axis.add(1) / n;
            z = *axis.add(2) / n;
        }
        *R.add(0) = t * x * x + c;
        *R.add(1) = t * x * y - s * z;
        *R.add(2) = t * x * z + s * y;
        *R.add(3) = t * x * y + s * z;
        *R.add(4) = t * y * y + c;
        *R.add(5) = t * y * z - s * x;
        *R.add(6) = t * x * z - s * y;
        *R.add(7) = t * y * z + s * x;
        *R.add(8) = t * z * z + c;
    }
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
    // SAFETY: caller guarantees v1..v5 are valid pointers to f64[3] arrays
    unsafe {
        let mut diff12: [f64; 3] = [0.0; 3];
        let mut diff13: [f64; 3] = [0.0; 3];
        let mut diff14: [f64; 3] = [0.0; 3];
        let mut diff15: [f64; 3] = [0.0; 3];

        sub3(diff12.as_mut_ptr(), v2, v1);
        sub3(diff13.as_mut_ptr(), v3, v1);
        sub3(diff14.as_mut_ptr(), v4, v1);
        sub3(diff15.as_mut_ptr(), v5, v1);

        let vol1: f64 = det3(diff13.as_ptr(), diff14.as_ptr(), diff12.as_ptr());
        let vol2: f64 = det3(diff14.as_ptr(), diff15.as_ptr(), diff12.as_ptr());
        let vol3: f64 = det3(diff15.as_ptr(), diff13.as_ptr(), diff12.as_ptr());

        if vol1 >= 0.0 && vol2 >= 0.0 && vol3 >= 0.0 {
            return 1;
        }
        if vol1 <= 0.0 && vol2 <= 0.0 && vol3 <= 0.0 {
            return -1;
        }
        0
    }
}

/// C: triAffineCoord (engine/engine_collision_gjk.c:1016)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn tri_affine_coord(lambda: *mut f64, v1: *const f64, v2: *const f64, v3: *const f64, p: *const f64) {
    unsafe {
        // SAFETY: lambda is f64[3], v1/v2/v3/p are f64[3] arrays from caller

        // compute minors as in S2D
        let M_14: f64 = *v2.add(1) * *v3.add(2) - *v2.add(2) * *v3.add(1)
                       - *v1.add(1) * *v3.add(2) + *v1.add(2) * *v3.add(1)
                       + *v1.add(1) * *v2.add(2) - *v1.add(2) * *v2.add(1);
        let M_24: f64 = *v2.add(0) * *v3.add(2) - *v2.add(2) * *v3.add(0)
                       - *v1.add(0) * *v3.add(2) + *v1.add(2) * *v3.add(0)
                       + *v1.add(0) * *v2.add(2) - *v1.add(2) * *v2.add(0);
        let M_34: f64 = *v2.add(0) * *v3.add(1) - *v2.add(1) * *v3.add(0)
                       - *v1.add(0) * *v3.add(1) + *v1.add(1) * *v3.add(0)
                       + *v1.add(0) * *v2.add(1) - *v1.add(1) * *v2.add(0);

        // exclude one of the axes with the largest projection
        let mut M_max: f64 = 0.0;
        let x: usize;
        let y: usize;
        let mu1: f64 = M_14.abs();
        let mu2: f64 = M_24.abs();
        let mu3: f64 = M_34.abs();
        if mu1 >= mu2 && mu1 >= mu3 {
            M_max = M_14;
            x = 1;
            y = 2;
        } else if mu2 >= mu3 {
            M_max = M_24;
            x = 0;
            y = 2;
        } else {
            M_max = M_34;
            x = 0;
            y = 1;
        }

        // C31 corresponds to signed area of 2-simplex: (p, v2, v3)
        let C31: f64 = *p.add(x) * *v2.add(y) + *p.add(y) * *v3.add(x) + *v2.add(x) * *v3.add(y)
                     - *p.add(x) * *v3.add(y) - *p.add(y) * *v2.add(x) - *v3.add(x) * *v2.add(y);

        // C32 corresponds to signed area of 2-simplex: (p, v1, v3)
        let C32: f64 = *p.add(x) * *v3.add(y) + *p.add(y) * *v1.add(x) + *v3.add(x) * *v1.add(y)
                     - *p.add(x) * *v1.add(y) - *p.add(y) * *v3.add(x) - *v1.add(x) * *v3.add(y);

        // C33 corresponds to signed area of 2-simplex: (p, v1, v2)
        let C33: f64 = *p.add(x) * *v1.add(y) + *p.add(y) * *v2.add(x) + *v1.add(x) * *v2.add(y)
                     - *p.add(x) * *v2.add(y) - *p.add(y) * *v1.add(x) - *v2.add(x) * *v1.add(y);

        // compute affine coordinates
        *lambda.add(0) = C31 / M_max;
        *lambda.add(1) = C32 / M_max;
        *lambda.add(2) = C33 / M_max;
    }
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
    unsafe {
        // SAFETY: v1, v2, v3, p each point to 3 f64 elements
        const MJ_MINVAL: f64 = 1e-15;
        let mut lambda: [f64; 3] = [0.0; 3];
        tri_affine_coord(lambda.as_mut_ptr(), v1, v2, v3, p);
        if lambda[0] < 0.0 || lambda[1] < 0.0 || lambda[2] < 0.0 {
            return 0;
        }
        let mut pr: [f64; 3] = [0.0; 3];
        let mut diff: [f64; 3] = [0.0; 3];
        pr[0] = *v1.add(0) * lambda[0] + *v2.add(0) * lambda[1] + *v3.add(0) * lambda[2];
        pr[1] = *v1.add(1) * lambda[0] + *v2.add(1) * lambda[1] + *v3.add(1) * lambda[2];
        pr[2] = *v1.add(2) * lambda[0] + *v2.add(2) * lambda[1] + *v3.add(2) * lambda[2];
        sub3(diff.as_mut_ptr(), pr.as_ptr(), p);
        (norm3(diff.as_ptr()) < MJ_MINVAL) as i32
    }
}

/// C: deleteFace (engine/engine_collision_gjk.c:1216)
#[allow(unused_variables, non_snake_case)]
pub fn delete_face(pt: *mut Polytope, face: *mut Face) {
    if pt.is_null() || face.is_null() {
        return;
    }
    extern "C" { fn deleteFace(pt: *mut Polytope, face: *mut Face); }
    // SAFETY: pt, face verified non-null; delegates to C implementation
    unsafe { deleteFace(pt, face) }
}

/// C: maxFaces (engine/engine_collision_gjk.c:1226)
#[allow(unused_variables, non_snake_case)]
pub fn max_faces(pt: *mut Polytope) -> i32  {
    if pt.is_null() {
        return 0;
    }
    extern "C" { fn maxFaces(pt: *mut Polytope) -> i32; }
    // SAFETY: pt verified non-null; delegates to C implementation
    unsafe { maxFaces(pt) }
}

/// C: addEdge (engine/engine_collision_gjk.c:1263)
#[allow(unused_variables, non_snake_case)]
pub fn add_edge(pt: *mut Polytope, index: i32, edge: i32) {
    if pt.is_null() {
        return;
    }
    extern "C" { fn addEdge(pt: *mut Polytope, index: i32, edge: i32); }
    // SAFETY: pt verified non-null; delegates to C implementation
    unsafe { addEdge(pt, index, edge) }
}

/// C: getEdge (engine/engine_collision_gjk.c:1270)
#[allow(unused_variables, non_snake_case)]
pub fn get_edge(face: *mut Face, vertex: i32) -> i32  {
    if face.is_null() {
        return -1;
    }
    extern "C" { fn getEdge(face: *mut Face, vertex: i32) -> i32; }
    // SAFETY: face verified non-null; delegates to C implementation
    unsafe { getEdge(face, vertex) }
}

/// C: horizonRec (engine/engine_collision_gjk.c:1279)
/// Calls: addEdge, deleteFace, dot3, getEdge
#[allow(unused_variables, non_snake_case)]
pub fn horizon_rec(pt: *mut Polytope, face: *mut Face, e: i32) -> i32  {
    if pt.is_null() { return 0; }
    extern "C" { fn horizonRec(pt: *mut Polytope, face: *mut Face, e: i32) -> i32; }
    // SAFETY: pt verified non-null; delegates to C implementation
    unsafe { horizonRec(pt, face, e) }
}

/// C: horizon (engine/engine_collision_gjk.c:1303)
/// Calls: addEdge, deleteFace, getEdge, horizonRec
#[allow(unused_variables, non_snake_case)]
pub fn horizon(pt: *mut Polytope, face: *mut Face) {
    extern "C" { fn horizon(pt: *mut Polytope, face: *mut Face); }
    // SAFETY: delegates to C implementation
    unsafe { horizon(pt, face) }
}

/// C: epaWitness (engine/engine_collision_gjk.c:1331)
/// Calls: lincomb, triAffineCoord
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn epa_witness(pt: *const Polytope, face: *const Face, x1: *mut f64, x2: *mut f64) -> f64  {
    if pt.is_null() || face.is_null() {
        return 0.0;
    }
    extern "C" { fn epaWitness(pt: *const Polytope, face: *const Face, x1: *mut f64, x2: *mut f64) -> f64; }
    // SAFETY: pt, face verified non-null; delegates to C implementation
    unsafe { epaWitness(pt, face, x1, x2) }
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
    unsafe {
        // SAFETY: a, b, c, d each point to 3 f64 elements
        let mut cross1: [f64; 3] = [0.0; 3];
        let mut cross2: [f64; 3] = [0.0; 3];
        let mut combined: [f64; 3] = [0.0; 3];
        cross3(cross1.as_mut_ptr(), a, b);
        cross3(cross2.as_mut_ptr(), c, d);
        add3(combined.as_mut_ptr(), cross1.as_ptr(), cross2.as_ptr());
        norm3(combined.as_ptr())
    }
}

/// C: next (engine/engine_collision_gjk.c:1520)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn next(polygon: *mut f64, nvert: i32, curr: *mut f64) -> *mut f64 {
    unsafe {
        // SAFETY: raw pointer arithmetic matching C next
        if curr == polygon.add(3 * (nvert as usize - 1)) {
            return polygon;
        }
        curr.add(3)
    }
}

/// C: polygonQuad (engine/engine_collision_gjk.c:1529)
/// Calls: area4, next
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn polygon_quad(res: [*mut f64; 4], polygon: *mut f64, nvert: i32) {
    if polygon.is_null() {
        return;
    }
    extern "C" { fn polygonQuad(res: [*mut f64; 4], polygon: *mut f64, nvert: i32); }
    // SAFETY: polygon verified non-null; delegates to C implementation
    unsafe { polygonQuad(res, polygon, nvert) }
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
    unsafe {
        // SAFETY: res, v1, v2, n each point to 3 f64 elements
        let mut combined: [f64; 3] = [0.0; 3];
        add3(combined.as_mut_ptr(), v1, v2);
        cross3(res, combined.as_ptr(), n);
        crate::engine::engine_util_blas::mju_normalize3(res);
        dot3(res, v1)
    }
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
    unsafe {
        // SAFETY: raw pointer arithmetic matching C halfspace
        const MJ_MINVAL: f64 = 1e-15;
        let mut diff = [0.0f64; 3];
        diff[0] = *p.add(0) - *a.add(0);
        diff[1] = *p.add(1) - *a.add(1);
        diff[2] = *p.add(2) - *a.add(2);
        (dot3(diff.as_ptr(), n) > -MJ_MINVAL) as i32
    }
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
    unsafe {
        // SAFETY: raw pointer arithmetic matching C planeIntersect
        const MJ_MAX_LIMIT: f64 = f64::MAX;
        let mut ab = [0.0f64; 3];
        sub3(ab.as_mut_ptr(), b, a);
        let temp = dot3(pn, ab.as_ptr());
        if temp == 0.0 {
            return MJ_MAX_LIMIT;
        }
        let t = (pd - dot3(pn, a)) / temp;
        if t >= 0.0 && t <= 1.0 {
            *res.add(0) = *a.add(0) + t * ab[0];
            *res.add(1) = *a.add(1) + t * ab[1];
            *res.add(2) = *a.add(2) + t * ab[2];
        }
        t
    }
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
    if status.is_null() { return; }
    extern "C" { fn polygonClip(status: *mut mjCCDStatus, face1: *const f64, nface1: i32, face2: *const f64, nface2: i32, n: *const f64, dir: *const f64); }
    // SAFETY: status verified non-null; delegates to C implementation
    unsafe { polygonClip(status, face1, nface1, face2, nface2, n, dir) }
}

/// C: globalcoord (engine/engine_collision_gjk.c:1744)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn globalcoord(res: *mut f64, mat: *const f64, pos: *const f64, l1: f64, l2: f64, l3: f64) {
    unsafe {
        // SAFETY: raw pointer arithmetic matching C globalcoord
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
#[allow(unused_variables, non_snake_case)]
pub fn intersect(res: [i32; 2], arr1: *const i32, arr2: *const i32, n: i32, m: i32) -> i32  {
    if arr1.is_null() || arr2.is_null() {
        return 0;
    }
    extern "C" { fn intersect(res: [i32; 2], arr1: *const i32, arr2: *const i32, n: i32, m: i32) -> i32; }
    // SAFETY: arr1, arr2 verified non-null; delegates to C implementation
    unsafe { intersect(res, arr1, arr2, n, m) }
}

/// C: meshNormals (engine/engine_collision_gjk.c:1774)
/// Calls: globalcoord, intersect
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mesh_normals(res: *mut f64, resind: [i32; 3], dim: i32, obj: *mut mjCCDObj, v1: i32, v2: i32, v3: i32) -> i32  {
    if res.is_null() { return 0; }
    extern "C" { fn meshNormals(res: *mut f64, resind: [i32; 3], dim: i32, obj: *mut mjCCDObj, v1: i32, v2: i32, v3: i32) -> i32; }
    // SAFETY: res verified non-null; delegates to C implementation
    unsafe { meshNormals(res, resind, dim, obj, v1, v2, v3) }
}

/// C: meshEdgeNormals (engine/engine_collision_gjk.c:1840)
/// Calls: copy3, globalcoord, mju_normalize3, sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mesh_edge_normals(res: *mut f64, endverts: *mut f64, dim: i32, obj: *mut mjCCDObj, v1: *const f64, v2: *const f64, v1i: i32, v2i: i32) -> i32  {
    if obj.is_null() || v1.is_null() || v2.is_null() {
        return 0;
    }
    extern "C" { fn meshEdgeNormals(res: *mut f64, endverts: *mut f64, dim: i32, obj: *mut mjCCDObj, v1: *const f64, v2: *const f64, v1i: i32, v2i: i32) -> i32; }
    // SAFETY: obj, v1, v2 verified non-null; delegates to C implementation
    unsafe { meshEdgeNormals(res, endverts, dim, obj, v1, v2, v1i, v2i) }
}

/// C: boxNormals2 (engine/engine_collision_gjk.c:1885)
/// Calls: dot3, globalcoord, scl3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case, invalid_reference_casting)]
pub fn box_normals2(res: *mut f64, resind: [i32; 3], mat: *const f64, n: *const f64) -> i32  {
    if mat.is_null() || n.is_null() {
        return 0;
    }
    extern "C" { fn boxNormals2(res: *mut f64, resind: [i32; 3], mat: *const f64, n: *const f64) -> i32; }
    // SAFETY: mat, n verified non-null; delegates to C implementation
    unsafe { boxNormals2(res, resind, mat, n) }
}

/// C: boxNormals (engine/engine_collision_gjk.c:1911)
/// Calls: boxNormals2, globalcoord
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn box_normals(res: *mut f64, resind: [i32; 3], dim: i32, obj: *mut mjCCDObj, v1: i32, v2: i32, v3: i32, dir: *const f64) -> i32  {
    if res.is_null() { return 0; }
    extern "C" { fn boxNormals(res: *mut f64, resind: [i32; 3], dim: i32, obj: *mut mjCCDObj, v1: i32, v2: i32, v3: i32, dir: *const f64) -> i32; }
    // SAFETY: res verified non-null; delegates to C implementation
    unsafe { boxNormals(res, resind, dim, obj, v1, v2, v3, dir) }
}

/// C: boxEdgeNormals (engine/engine_collision_gjk.c:1965)
/// Calls: copy3, globalcoord, mju_normalize3, sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn box_edge_normals(res: *mut f64, endverts: *mut f64, dim: i32, obj: *mut mjCCDObj, v1: *const f64, v2: *const f64, v1i: i32, v2i: i32) -> i32  {
    if obj.is_null() || v1.is_null() || v2.is_null() {
        return 0;
    }
    extern "C" { fn boxEdgeNormals(res: *mut f64, endverts: *mut f64, dim: i32, obj: *mut mjCCDObj, v1: *const f64, v2: *const f64, v1i: i32, v2i: i32) -> i32; }
    // SAFETY: obj, v1, v2 verified non-null; delegates to C implementation
    unsafe { boxEdgeNormals(res, endverts, dim, obj, v1, v2, v1i, v2i) }
}

/// C: boxFace (engine/engine_collision_gjk.c:2002)
/// Calls: globalcoord
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn box_face(res: *mut f64, obj: *mut mjCCDObj, idx: i32) -> i32  {
    if obj.is_null() {
        return 0;
    }
    extern "C" { fn boxFace(res: *mut f64, obj: *mut mjCCDObj, idx: i32) -> i32; }
    // SAFETY: obj verified non-null; delegates to C implementation
    unsafe { boxFace(res, obj, idx) }
}

/// C: meshFace (engine/engine_collision_gjk.c:2052)
/// Calls: globalcoord
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mesh_face(res: *mut f64, obj: *mut mjCCDObj, idx: i32) -> i32  {
    if obj.is_null() {
        return 0;
    }
    extern "C" { fn meshFace(res: *mut f64, obj: *mut mjCCDObj, idx: i32) -> i32; }
    // SAFETY: obj verified non-null; delegates to C implementation
    unsafe { meshFace(res, obj, idx) }
}

/// C: alignedFaces (engine/engine_collision_gjk.c:2072)
/// Calls: dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn aligned_faces(mut res: [i32; 2], v: *const f64, nv: i32, w: *const f64, nw: i32) -> i32  {
    if v.is_null() || w.is_null() {
        return 0;
    }
    extern "C" { fn alignedFaces(res: [i32; 2], v: *const f64, nv: i32, w: *const f64, nw: i32) -> i32; }
    // SAFETY: v, w verified non-null; delegates to C implementation
    unsafe { alignedFaces(res, v, nv, w, nw) }
}

/// C: alignedFaceEdge (engine/engine_collision_gjk.c:2088)
/// Calls: dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case, invalid_reference_casting)]
pub fn aligned_face_edge(res: [i32; 2], edge: *const f64, nedge: i32, face: *const f64, nface: i32) -> i32  {
    if edge.is_null() || face.is_null() {
        return 0;
    }
    extern "C" { fn alignedFaceEdge(res: [i32; 2], edge: *const f64, nedge: i32, face: *const f64, nface: i32) -> i32; }
    // SAFETY: edge, face verified non-null; delegates to C implementation
    unsafe { alignedFaceEdge(res, edge, nedge, face, nface) }
}

/// C: simplexDim (engine/engine_collision_gjk.c:2104)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn simplex_dim(v1i: *mut i32, v2i: *mut i32, v3i: *mut i32, v1: *mut *mut f64, v2: *mut *mut f64, v3: *mut *mut f64) -> i32  {
    if v1i.is_null() || v2i.is_null() || v3i.is_null() {
        return 0;
    }
    extern "C" { fn simplexDim(v1i: *mut i32, v2i: *mut i32, v3i: *mut i32, v1: *mut *mut f64, v2: *mut *mut f64, v3: *mut *mut f64) -> i32; }
    // SAFETY: v1i, v2i, v3i verified non-null; delegates to C implementation
    unsafe { simplexDim(v1i, v2i, v3i, v1, v2, v3) }
}

/// C: multicontact (engine/engine_collision_gjk.c:2122)
/// Calls: alignedFaceEdge, alignedFaces, boxEdgeNormals, boxFace, boxNormals, copy3, meshEdgeNormals, meshFace, meshNormals, norm3, polygonClip, scl3, simplexDim, sub3
#[allow(unused_variables, non_snake_case)]
pub fn multicontact(pt: *mut Polytope, face: *mut Face, status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) {
    extern "C" {
        fn multicontact(pt: *mut Polytope, face: *mut Face, status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { multicontact(pt, face, status, obj1, obj2) }
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
    if status.is_null() {
        return;
    }
    extern "C" { fn inflate(status: *mut mjCCDStatus, margin1: f64, margin2: f64); }
    // SAFETY: status verified non-null; delegates to C implementation
    unsafe { inflate(status, margin1, margin2) }
}

/// C: mjc_ccdSize (engine/engine_collision_gjk.h:105)
/// Calls: align8
#[allow(unused_variables, non_snake_case)]
pub fn mjc_ccd_size(iterations: i32) -> usize {
    // sizeof(Vertex) = 80, sizeof(Face) = 56 on 64-bit (verified from C struct layout)
    const SIZEOF_VERTEX: usize = 80;
    const SIZEOF_FACE: usize = 56;
    const SIZEOF_FACE_PTR: usize = core::mem::size_of::<*const u8>();
    const SIZEOF_INT: usize = core::mem::size_of::<i32>();

    let iter = iterations as usize;
    align8(SIZEOF_VERTEX * (5 + iter))        // vertices in polytope
        + align8(SIZEOF_FACE * 6 * iter)      // faces in polytope
        + align8(SIZEOF_FACE_PTR * 6 * iter)  // map in polytope
        + align8(SIZEOF_INT * 24)             // horizon indices
        + align8(SIZEOF_INT * 24)             // horizon edges
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
    extern "C" {
        fn mjc_ccd(config: *const mjCCDConfig, status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> f64;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjc_ccd(config, status, obj1, obj2) }
}

