//! Port of: engine/engine_collision_gjk.c
//! IR hash: 73393814548a07d1
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
    let mut dir: [f64; 3] = [0.0; 3];
    let mut dir_neg: [f64; 3] = [0.0; 3];

    // mjc_support requires a normalized direction
    scl3(dir_neg.as_mut_ptr(), x_k, 1.0 / x_norm);
    scl3(dir.as_mut_ptr(), dir_neg.as_ptr(), -1.0);
    support(v, obj1, obj2, dir.as_ptr(), dir_neg.as_ptr());
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
    // Polytope layout (104 bytes):
    //   Vertex* verts;     offset 0
    //   int nverts;        offset 8
    const VERTS_OFFSET: usize = 0;
    const NVERTS_OFFSET: usize = 8;
    const SIZEOF_VERTEX: usize = 80;

    // SAFETY: pt is a valid Polytope pointer, obj1/obj2 are valid mjCCDObj pointers,
    //         d is a valid f64[3] pointer
    unsafe {
        let pt_base = pt as *mut u8;

        let mut dir: [f64; 3] = [1.0, 0.0, 0.0];
        let mut dir_neg: [f64; 3] = [-1.0, 0.0, 0.0];

        // mjc_support assumes a normalized direction
        if dnorm > 1e-14 {
            dir[0] = *d.add(0) / dnorm;
            dir[1] = *d.add(1) / dnorm;
            dir[2] = *d.add(2) / dnorm;
            dir_neg[0] = -dir[0];
            dir_neg[1] = -dir[1];
            dir_neg[2] = -dir[2];
        }

        // n = pt->nverts++
        let nverts_ptr = pt_base.add(NVERTS_OFFSET) as *mut i32;
        let n = *nverts_ptr;
        *nverts_ptr = n + 1;

        // Vertex* v = pt->verts + n
        let verts_ptr = *(pt_base.add(VERTS_OFFSET) as *const *mut u8);
        let v = verts_ptr.add(n as usize * SIZEOF_VERTEX) as *mut Vertex;

        // support(v, obj1, obj2, dir, dir_neg)
        support(v, obj1, obj2, dir.as_ptr(), dir_neg.as_ptr());

        n
    }
}

/// C: insertVertex (engine/engine_collision_gjk.c:112)
#[allow(unused_variables, non_snake_case)]
pub fn insert_vertex(pt: *mut Polytope, v: *const Vertex) -> i32 {
    const VERTS_OFFSET: usize = 0;
    const NVERTS_OFFSET: usize = 8;
    const SIZEOF_VERTEX: usize = 80;

    // SAFETY: pt is a valid Polytope pointer; v points to a valid Vertex (80 bytes).
    // Polytope layout: Vertex* verts at offset 0, int nverts at offset 8.
    unsafe {
        let pt_base = pt as *mut u8;
        let nverts_ptr = pt_base.add(NVERTS_OFFSET) as *mut i32;
        let n = *nverts_ptr;
        *nverts_ptr = n + 1;
        let verts_ptr = *(pt_base.add(VERTS_OFFSET) as *const *mut u8);
        let dst = verts_ptr.add(n as usize * SIZEOF_VERTEX);
        std::ptr::copy_nonoverlapping(v as *const u8, dst, SIZEOF_VERTEX);
        n
    }
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
    // Polytope layout (C struct):
    //   Vertex* verts;    offset 0
    //   int nverts;       offset 8
    //   Face* faces;      offset 16 (ptr aligned)
    //   int nfaces;       offset 24
    //   int maxfaces;     offset 28
    //   mjtNum center[3]; offset 32
    //   Face** map;       offset 56
    //   int nmap;         offset 64
    //
    // Face layout (C struct, 56 bytes):
    //   int verts;        offset 0
    //   int adj[3];       offset 4
    //   mjtNum v[3];      offset 16 (aligned to 8)
    //   mjtNum dist2;     offset 40
    //   int index;        offset 48
    //
    // Vertex layout (80 bytes):
    //   mjtNum vert[3];   offset 0
    //   mjtNum vert1[3];  offset 24
    //   mjtNum vert2[3];  offset 48
    //   int index1;       offset 72
    //   int index2;       offset 76

    const SIZEOF_FACE: usize = 56;
    const SIZEOF_VERTEX: usize = 80;

    // SAFETY: pt is a valid Polytope pointer. All field accesses follow the known C struct layout.
    unsafe {
        let pt_base = pt as *mut u8;
        let verts_ptr = *(pt_base as *const *mut u8);                    // offset 0: Vertex*
        let faces_ptr = *(pt_base.add(16) as *const *mut u8);           // offset 16: Face*
        let nfaces_ptr = pt_base.add(24) as *mut i32;                    // offset 24: int nfaces
        let center_ptr = pt_base.add(32) as *const f64;                  // offset 32: mjtNum center[3]

        let nfaces = *nfaces_ptr;
        *nfaces_ptr = nfaces + 1;

        let face = faces_ptr.add(nfaces as usize * SIZEOF_FACE);

        // face->verts = v1 + (v2 << 10) + (v3 << 20)
        *(face as *mut i32) = v1 + (v2 << 10) + (v3 << 20);

        // face->adj[0..3]
        let adj_ptr = face.add(4) as *mut i32;
        *adj_ptr.add(0) = adj1;
        *adj_ptr.add(1) = adj2;
        *adj_ptr.add(2) = adj3;

        // compute witness point v: projectOriginPlane(face->v, verts[v3].vert, verts[v2].vert, verts[v1].vert)
        let face_v = face.add(16) as *mut f64;
        let v3_vert = verts_ptr.add(v3 as usize * SIZEOF_VERTEX) as *const f64;
        let v2_vert = verts_ptr.add(v2 as usize * SIZEOF_VERTEX) as *const f64;
        let v1_vert = verts_ptr.add(v1 as usize * SIZEOF_VERTEX) as *const f64;

        let ret = project_origin_plane(face_v, v3_vert, v2_vert, v1_vert);
        if ret != 0 {
            return 0.0;
        }

        // ensure projection points outward from the polytope
        let mut outward: [f64; 3] = [0.0; 3];
        sub3(outward.as_mut_ptr(), v1_vert, center_ptr);
        if dot3(face_v, outward.as_ptr()) < 0.0 {
            scl3(face_v, face_v, -1.0);
        }

        // face->dist2 = dot3(face->v, face->v)
        let dist2 = dot3(face_v, face_v);
        *(face.add(40) as *mut f64) = dist2;

        // face->index = -1
        *(face.add(48) as *mut i32) = -1;

        dist2
    }
}

/// C: gjkIntersect (engine/engine_collision_gjk.c:119)
/// Calls: dot3, gjkIntersectSupport, signedDistance
#[allow(unused_variables, non_snake_case)]
pub fn gjk_intersect(status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> i32 {
    // mjCCDStatus layout:
    //   dist:            f64,  offset 0
    //   x1[150]:         f64[], offset 8  (3*50 doubles)
    //   x2[150]:         f64[], offset 1208
    //   nx:              i32,  offset 2408
    //   max_iterations:  i32,  offset 2412
    //   tolerance:       f64,  offset 2416 (aligned 8)
    //   max_contacts:    i32,  offset 2424
    //   dist_cutoff:     f64,  offset 2432
    //   gjk_iterations:  i32,  offset 2440
    //   epa_iterations:  i32,  offset 2444
    //   epa_status:      i32,  offset 2448
    //   (pad 4)
    //   simplex[4]:      Vertex[4], offset 2456
    //   nsimplex:        i32,  offset 2776
    const STATUS_GJK_ITER_OFFSET: usize = 2440;
    const STATUS_MAX_ITER_OFFSET: usize = 2412;
    const STATUS_SIMPLEX_OFFSET: usize = 2456;
    const STATUS_NSIMPLEX_OFFSET: usize = 2776;
    const SIZEOF_VERTEX: usize = 80;

    // SAFETY: status, obj1, obj2 are valid pointers. Offset arithmetic follows C struct layout.
    // simplex is a stack-local copy of 4 vertices (4*80 = 320 bytes).
    unsafe {
        let status_base = status as *mut u8;

        // Vertex simplex[4] = {status->simplex[0..3]}  (local copy)
        let mut simplex_buf = [0u8; 4 * SIZEOF_VERTEX];
        std::ptr::copy_nonoverlapping(
            status_base.add(STATUS_SIMPLEX_OFFSET),
            simplex_buf.as_mut_ptr(),
            4 * SIZEOF_VERTEX,
        );
        let simplex = simplex_buf.as_mut_ptr(); // *mut u8, stride SIZEOF_VERTEX

        // int s[4] = {0, 1, 2, 3}  (permutation indices)
        let mut s: [i32; 4] = [0, 1, 2, 3];

        let k_start = *(status_base.add(STATUS_GJK_ITER_OFFSET) as *const i32);
        let kmax = *(status_base.add(STATUS_MAX_ITER_OFFSET) as *const i32);
        let mut k = k_start;

        while k < kmax {
            let mut dist: [f64; 4] = [0.0; 4];
            let mut normals = [0.0f64; 12];

            // vertices in the simplex, by permutation index
            let sv = |i: usize| -> *const Vertex {
                simplex.add(s[i] as usize * SIZEOF_VERTEX) as *const Vertex
            };

            dist[0] = signed_distance(normals.as_mut_ptr().add(0),  sv(2), sv(1), sv(3));
            dist[1] = signed_distance(normals.as_mut_ptr().add(3),  sv(0), sv(2), sv(3));
            dist[2] = signed_distance(normals.as_mut_ptr().add(6),  sv(1), sv(0), sv(3));
            dist[3] = signed_distance(normals.as_mut_ptr().add(9),  sv(0), sv(1), sv(2));

            // if origin is on any affine hull, convergence will fail
            if dist[3] == 0.0 || dist[2] == 0.0 || dist[1] == 0.0 || dist[0] == 0.0 {
                *(status_base.add(STATUS_GJK_ITER_OFFSET) as *mut i32) = k;
                return -1;
            }

            // find face with smallest distance
            let i = if dist[0] < dist[1] { 0 } else { 1 };
            let j = if dist[2] < dist[3] { 2 } else { 3 };
            let index = if dist[i] < dist[j] { i } else { j };

            // origin inside of simplex → run EPA for contact information
            if dist[index] > 0.0 {
                *(status_base.add(STATUS_NSIMPLEX_OFFSET) as *mut i32) = 4;
                // status->simplex[0..3] = simplex[s[0..3]]
                for si in 0..4usize {
                    std::ptr::copy_nonoverlapping(
                        simplex.add(s[si] as usize * SIZEOF_VERTEX),
                        status_base.add(STATUS_SIMPLEX_OFFSET + si * SIZEOF_VERTEX),
                        SIZEOF_VERTEX,
                    );
                }
                *(status_base.add(STATUS_GJK_ITER_OFFSET) as *mut i32) = k;
                return 1;
            }

            // replace worst vertex with new candidate
            let sv_index = simplex.add(s[index] as usize * SIZEOF_VERTEX) as *mut Vertex;
            gjk_intersect_support(sv_index, obj1, obj2, normals.as_ptr().add(3 * index));

            // found origin outside Minkowski difference → no collision
            let vert_ptr = (simplex.add(s[index] as usize * SIZEOF_VERTEX)) as *const f64;
            if dot3(normals.as_ptr().add(3 * index), vert_ptr) < 0.0 {
                *(status_base.add(STATUS_NSIMPLEX_OFFSET) as *mut i32) = 0;
                *(status_base.add(STATUS_GJK_ITER_OFFSET) as *mut i32) = k;
                return 0;
            }

            // swap vertices in simplex to retain orientation
            let si = (index + 1) & 3;
            let sj = (index + 2) & 3;
            s.swap(si, sj);

            k += 1;
        }

        *(status_base.add(STATUS_GJK_ITER_OFFSET) as *mut i32) = k;
        -1  // never found origin
    }
}

/// C: polytope2 (engine/engine_collision_gjk.c:122)
/// Calls: add3, attachFace, cross3, epaSupport, insertVertex, mju_mulMatVec3, norm3, polytope3, rayTriangle, replaceSimplex3, rotmat, scl3, sub3
#[allow(unused_variables, non_snake_case)]
pub fn polytope2(pt: *mut Polytope, status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> i32 {
    // EPA return codes
    const MJ_EPA_P2_INVALID_FACES: i32 = 1;  // mjEPA_P2_INVALID_FACES
    const MJ_EPA_P2_NONCONVEX: i32 = 2;       // mjEPA_P2_NONCONVEX
    const MJ_EPA_P2_ORIGIN_ON_FACE: i32 = 3;  // mjEPA_P2_ORIGIN_ON_FACE
    const MJ_MAX_LIMIT: f64 = f64::MAX;
    const MJ_MINDIST2: f64 = 1e-15_f64 * 1e-15_f64;

    // mjCCDStatus simplex layout
    const STATUS_SIMPLEX_OFFSET: usize = 2456;
    const SIZEOF_VERTEX: usize = 80;

    #[repr(C)]
    struct FaceRepr { verts: i32, adj: [i32; 3], v: [f64; 3], dist2: f64, index: i32, _pad: i32 }
    #[repr(C)]
    struct PolytopeRepr {
        verts: *mut u8, nverts: i32, _pad0: i32,
        faces: *mut u8, nfaces: i32, maxfaces: i32,
        center: [f64; 3], map: *mut *mut u8, nmap: i32, _pad1: i32,
    }

    // SAFETY: All pointers are valid (caller contract). Vertex/Polytope layout follows C ABI.
    unsafe {
        let sb = status as *mut u8;
        let p = pt as *mut PolytopeRepr;
        let simplex_base = sb.add(STATUS_SIMPLEX_OFFSET);

        let v1 = simplex_base as *mut f64;                    // simplex[0].vert
        let v2 = simplex_base.add(SIZEOF_VERTEX) as *mut f64; // simplex[1].vert

        // set polytope center = (v1 + v2) / 2
        add3((*p).center.as_mut_ptr(), v1 as *const f64, v2 as *const f64);
        scl3((*p).center.as_mut_ptr(), (*p).center.as_ptr(), 0.5);

        let mut diff = [0.0f64; 3];
        sub3(diff.as_mut_ptr(), v2 as *const f64, v1 as *const f64);

        // find component with smallest magnitude (for largest cross product)
        let mut value: f64 = MJ_MAX_LIMIT;
        let mut index: usize = 0;
        for i in 0..3usize {
            let abs_d = diff[i].abs();
            if abs_d < value { value = abs_d; index = i; }
        }

        // cross product with best coordinate axis
        let mut e = [0.0f64; 3];
        e[index] = 1.0;
        let mut d1 = [0.0f64; 3];
        cross3(d1.as_mut_ptr(), e.as_ptr(), diff.as_ptr());

        // rotate 120 degrees around the line segment
        let mut R = [0.0f64; 9];
        rotmat(R.as_mut_ptr(), diff.as_ptr());

        let mut d2 = [0.0f64; 3];
        let mut d3 = [0.0f64; 3];
        crate::engine::engine_util_blas::mju_mul_mat_vec3(d2.as_mut_ptr(), R.as_ptr(), d1.as_ptr());
        crate::engine::engine_util_blas::mju_mul_mat_vec3(d3.as_mut_ptr(), R.as_ptr(), d2.as_ptr());

        let v1i = insert_vertex(pt, simplex_base as *const Vertex);
        let v2i = insert_vertex(pt, simplex_base.add(SIZEOF_VERTEX) as *const Vertex);
        let v3i = epa_support(pt, obj1, obj2, d1.as_ptr(), norm3(d1.as_ptr()));
        let v4i = epa_support(pt, obj1, obj2, d2.as_ptr(), norm3(d2.as_ptr()));
        let v5i = epa_support(pt, obj1, obj2, d3.as_ptr(), norm3(d3.as_ptr()));

        let v3 = ((*p).verts as *const u8).add(v3i as usize * SIZEOF_VERTEX) as *const f64;
        let v4 = ((*p).verts as *const u8).add(v4i as usize * SIZEOF_VERTEX) as *const f64;
        let v5 = ((*p).verts as *const u8).add(v5i as usize * SIZEOF_VERTEX) as *const f64;

        // build hexahedron
        if attach_face(pt, v1i, v3i, v4i, 1, 3, 2) < MJ_MINDIST2 {
            replace_simplex3(pt, status, v1i, v3i, v4i);
            return polytope3(pt, status, obj1, obj2);
        }
        if attach_face(pt, v1i, v5i, v3i, 2, 4, 0) < MJ_MINDIST2 {
            replace_simplex3(pt, status, v1i, v5i, v3i);
            return polytope3(pt, status, obj1, obj2);
        }
        if attach_face(pt, v1i, v4i, v5i, 0, 5, 1) < MJ_MINDIST2 {
            replace_simplex3(pt, status, v1i, v4i, v5i);
            return polytope3(pt, status, obj1, obj2);
        }
        if attach_face(pt, v2i, v4i, v3i, 5, 0, 4) < MJ_MINDIST2 {
            replace_simplex3(pt, status, v2i, v4i, v3i);
            return polytope3(pt, status, obj1, obj2);
        }
        if attach_face(pt, v2i, v3i, v5i, 3, 1, 5) < MJ_MINDIST2 {
            replace_simplex3(pt, status, v2i, v3i, v5i);
            return polytope3(pt, status, obj1, obj2);
        }
        if attach_face(pt, v2i, v5i, v4i, 4, 2, 3) < MJ_MINDIST2 {
            replace_simplex3(pt, status, v2i, v5i, v4i);
            return polytope3(pt, status, obj1, obj2);
        }

        // check hexahedron is convex
        if ray_triangle(v1 as *const f64, v2 as *const f64, v3, v4, v5) == 0 {
            return MJ_EPA_P2_NONCONVEX;
        }

        // populate face map
        let faces_base = (*p).faces as *mut FaceRepr;
        for i in 0..6usize {
            *(*p).map.add(i) = faces_base.add(i) as *mut u8;
            (*faces_base.add(i)).index = i as i32;
        }
        (*p).nmap = 6;
        0  // mjEPA_SUCCESS
    }
}

/// C: polytope3 (engine/engine_collision_gjk.c:123)
/// Calls: add3, attachFace, cross3, epaSupport, insertVertex, norm3, scl3, sub3, testTetra, triPointIntersect
#[allow(unused_variables, non_snake_case)]
pub fn polytope3(pt: *mut Polytope, status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> i32 {
    // EPA return codes
    const MJ_EPA_P3_BAD_NORMAL: i32 = 4;    // mjEPA_P3_BAD_NORMAL
    const MJ_EPA_P3_INVALID_V4: i32 = 5;    // mjEPA_P3_INVALID_V4
    const MJ_EPA_P3_INVALID_V5: i32 = 6;    // mjEPA_P3_INVALID_V5
    const MJ_EPA_P3_MISSING_ORIGIN: i32 = 7;// mjEPA_P3_MISSING_ORIGIN
    const MJ_EPA_P3_ORIGIN_ON_FACE: i32 = 8;// mjEPA_P3_ORIGIN_ON_FACE
    const MJ_MINVAL: f64 = 1e-15_f64;
    const MJ_MINDIST3: f64 = MJ_MINVAL * MJ_MINVAL; // mjMINVAL2

    // mjCCDStatus layout
    const STATUS_DIST_OFFSET: usize = 0;
    const STATUS_SIMPLEX_OFFSET: usize = 2456;
    const SIZEOF_VERTEX: usize = 80;
    // Polytope layout:
    // verts(8), nverts(4), _pad0(4), faces(8), nfaces(4), maxfaces(4), center([f64;3]=24),
    // map(8), nmap(4), _pad1(4) = 72 bytes before horizon

    #[repr(C)]
    struct FaceRepr {
        verts: i32, adj: [i32; 3], v: [f64; 3], dist2: f64, index: i32, _pad: i32,
    }
    #[repr(C)]
    struct PolytopeRepr {
        verts: *mut u8, nverts: i32, _pad0: i32,
        faces: *mut u8, nfaces: i32, maxfaces: i32,
        center: [f64; 3],
        map: *mut *mut u8, nmap: i32, _pad1: i32,
    }

    // SAFETY: pt, status, obj1, obj2 are valid pointers. Vertex/Face layout follows C ABI.
    unsafe {
        let sb = status as *mut u8;
        let p = pt as *mut PolytopeRepr;

        let simplex_base = sb.add(STATUS_SIMPLEX_OFFSET);
        let v1 = simplex_base as *const f64;  // simplex[0].vert
        let v2 = simplex_base.add(SIZEOF_VERTEX) as *const f64; // simplex[1].vert
        let v3 = simplex_base.add(2 * SIZEOF_VERTEX) as *const f64; // simplex[2].vert

        // set polytope center = (v1 + v2 + v3) / 3
        add3((*p).center.as_mut_ptr(), v1, v2);
        add3((*p).center.as_mut_ptr(), (*p).center.as_ptr(), v3);
        scl3((*p).center.as_mut_ptr(), (*p).center.as_ptr(), 1.0 / 3.0);

        // get normals in both directions
        let mut diff1 = [0.0f64; 3];
        let mut diff2 = [0.0f64; 3];
        let mut n = [0.0f64; 3];
        sub3(diff1.as_mut_ptr(), v2, v1);
        sub3(diff2.as_mut_ptr(), v3, v1);
        cross3(n.as_mut_ptr(), diff1.as_ptr(), diff2.as_ptr());
        let n_norm = norm3(n.as_ptr());
        if n_norm < MJ_MINVAL {
            return MJ_EPA_P3_BAD_NORMAL;
        }

        let mut n_neg = [0.0f64; 3];
        scl3(n_neg.as_mut_ptr(), n.as_ptr(), -1.0);

        // save vertices and get indices
        let v1i = insert_vertex(pt, simplex_base as *const Vertex);
        let v2i = insert_vertex(pt, simplex_base.add(SIZEOF_VERTEX) as *const Vertex);
        let v3i = insert_vertex(pt, simplex_base.add(2 * SIZEOF_VERTEX) as *const Vertex);
        let v5i = epa_support(pt, obj1, obj2, n_neg.as_ptr(), n_norm);
        let v4i = epa_support(pt, obj1, obj2, n.as_ptr(), n_norm);
        let v4 = ((*p).verts as *const u8).add(v4i as usize * SIZEOF_VERTEX) as *const f64;
        let v5 = ((*p).verts as *const u8).add(v5i as usize * SIZEOF_VERTEX) as *const f64;

        // check v4 not contained in 2-simplex
        if tri_point_intersect(v1, v2, v3, v4) != 0 {
            return MJ_EPA_P3_INVALID_V4;
        }
        if tri_point_intersect(v1, v2, v3, v5) != 0 {
            return MJ_EPA_P3_INVALID_V5;
        }

        let dist = *(sb.add(STATUS_DIST_OFFSET) as *const f64);
        if dist > 10.0 * MJ_MINVAL && test_tetra(v1, v2, v3, v4) == 0 && test_tetra(v1, v2, v3, v5) == 0 {
            return MJ_EPA_P3_MISSING_ORIGIN;
        }

        // create hexahedron for EPA (6 faces)
        if attach_face(pt, v4i, v1i, v2i, 1, 3, 2) < MJ_MINDIST3 { return MJ_EPA_P3_ORIGIN_ON_FACE; }
        if attach_face(pt, v4i, v3i, v1i, 2, 4, 0) < MJ_MINDIST3 { return MJ_EPA_P3_ORIGIN_ON_FACE; }
        if attach_face(pt, v4i, v2i, v3i, 0, 5, 1) < MJ_MINDIST3 { return MJ_EPA_P3_ORIGIN_ON_FACE; }
        if attach_face(pt, v5i, v2i, v1i, 5, 0, 4) < MJ_MINDIST3 { return MJ_EPA_P3_ORIGIN_ON_FACE; }
        if attach_face(pt, v5i, v1i, v3i, 3, 1, 5) < MJ_MINDIST3 { return MJ_EPA_P3_ORIGIN_ON_FACE; }
        if attach_face(pt, v5i, v3i, v2i, 4, 2, 3) < MJ_MINDIST3 { return MJ_EPA_P3_ORIGIN_ON_FACE; }

        // populate face map
        let faces_base = (*p).faces as *mut FaceRepr;
        for i in 0..6usize {
            *(*p).map.add(i) = faces_base.add(i) as *mut u8;
            (*faces_base.add(i)).index = i as i32;
        }
        (*p).nmap = 6;
        0  // mjEPA_SUCCESS
    }
}

/// C: polytope4 (engine/engine_collision_gjk.c:124)
/// Calls: add3, attachFace, insertVertex, polytope3, replaceSimplex3, scl3, testTetra
#[allow(unused_variables, non_snake_case)]
pub fn polytope4(pt: *mut Polytope, status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> i32 {
    const MJ_EPA_P4_MISSING_ORIGIN: i32 = 9;  // mjEPA_P4_MISSING_ORIGIN
    const MJ_MINDIST4: f64 = 1e-15_f64 * 1e-15_f64; // mjMINVAL2

    const STATUS_SIMPLEX_OFFSET: usize = 2456;
    const SIZEOF_VERTEX: usize = 80;

    #[repr(C)]
    struct FaceRepr { verts: i32, adj: [i32; 3], v: [f64; 3], dist2: f64, index: i32, _pad: i32 }
    #[repr(C)]
    struct PolytopeRepr {
        verts: *mut u8, nverts: i32, _pad0: i32,
        faces: *mut u8, nfaces: i32, maxfaces: i32,
        center: [f64; 3], map: *mut *mut u8, nmap: i32, _pad1: i32,
    }

    // SAFETY: All pointers are valid (caller contract).
    unsafe {
        let sb = status as *mut u8;
        let p = pt as *mut PolytopeRepr;
        let simplex_base = sb.add(STATUS_SIMPLEX_OFFSET);

        let v1 = insert_vertex(pt, simplex_base as *const Vertex);
        let v2 = insert_vertex(pt, simplex_base.add(SIZEOF_VERTEX) as *const Vertex);
        let v3 = insert_vertex(pt, simplex_base.add(2 * SIZEOF_VERTEX) as *const Vertex);
        let v4 = insert_vertex(pt, simplex_base.add(3 * SIZEOF_VERTEX) as *const Vertex);

        // set polytope center = (verts[v1] + verts[v2] + verts[v3] + verts[v4]) / 4
        let vert_base = (*p).verts as *const u8;
        let p1 = vert_base.add(v1 as usize * SIZEOF_VERTEX) as *const f64;
        let p2 = vert_base.add(v2 as usize * SIZEOF_VERTEX) as *const f64;
        let p3 = vert_base.add(v3 as usize * SIZEOF_VERTEX) as *const f64;
        let p4 = vert_base.add(v4 as usize * SIZEOF_VERTEX) as *const f64;
        add3((*p).center.as_mut_ptr(), p1, p2);
        add3((*p).center.as_mut_ptr(), (*p).center.as_ptr(), p3);
        add3((*p).center.as_mut_ptr(), (*p).center.as_ptr(), p4);
        scl3((*p).center.as_mut_ptr(), (*p).center.as_ptr(), 0.25);

        // if origin is on a face, replace with 2-simplex
        if attach_face(pt, v1, v2, v3, 1, 3, 2) < MJ_MINDIST4 {
            replace_simplex3(pt, status, v1, v2, v3);
            return polytope3(pt, status, obj1, obj2);
        }
        if attach_face(pt, v1, v4, v2, 2, 3, 0) < MJ_MINDIST4 {
            replace_simplex3(pt, status, v1, v4, v2);
            return polytope3(pt, status, obj1, obj2);
        }
        if attach_face(pt, v1, v3, v4, 0, 3, 1) < MJ_MINDIST4 {
            replace_simplex3(pt, status, v1, v3, v4);
            return polytope3(pt, status, obj1, obj2);
        }
        if attach_face(pt, v4, v3, v2, 2, 0, 1) < MJ_MINDIST4 {
            replace_simplex3(pt, status, v4, v3, v2);
            return polytope3(pt, status, obj1, obj2);
        }

        // verify origin inside tetrahedron
        if test_tetra(p1, p2, p3, p4) == 0 {
            return MJ_EPA_P4_MISSING_ORIGIN;
        }

        // populate face map
        let faces_base = (*p).faces as *mut FaceRepr;
        for i in 0..4usize {
            *(*p).map.add(i) = faces_base.add(i) as *mut u8;
            (*faces_base.add(i)).index = i as i32;
        }
        (*p).nmap = 4;
        0  // mjEPA_SUCCESS
    }
}

/// C: epa (engine/engine_collision_gjk.c:128)
/// Calls: attachFace, discreteGeoms, dot3, epaSupport, epaWitness, horizon, maxFaces, mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn epa(status: *mut mjCCDStatus, pt: *mut Polytope, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> *mut Face {
    // mjCCDStatus layout (relevant fields):
    //   dist:            f64,  offset 0
    //   x1[150]:         f64[150], offset 8
    //   x2[150]:         f64[150], offset 1208
    //   nx:              i32,  offset 2408
    //   max_iterations:  i32,  offset 2412
    //   tolerance:       f64,  offset 2416
    //   epa_iterations:  i32,  offset 2444
    const STATUS_DIST_OFFSET: usize = 0;
    const STATUS_X1_OFFSET: usize = 8;
    const STATUS_X2_OFFSET: usize = 1208;
    const STATUS_NX_OFFSET: usize = 2408;
    const STATUS_MAX_ITER_OFFSET: usize = 2412;
    const STATUS_TOLERANCE_OFFSET: usize = 2416;
    const STATUS_EPA_ITER_OFFSET: usize = 2444;
    // Vertex layout: vert[3] at offset 0, vert1[3] at 24, vert2[3] at 48, index1 at 72, index2 at 76
    const SIZEOF_VERTEX: usize = 80;
    const VERTEX_INDEX1_OFFSET: usize = 72;
    const VERTEX_INDEX2_OFFSET: usize = 76;
    const MJ_MAX_LIMIT: f64 = f64::MAX;
    const MJ_MINVAL: f64 = 1e-15_f64;
    const MJ_MINEPATOK: f64 = MJ_MINVAL; // double precision

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
        // horizon follows
    }

    // SAFETY: status, pt, obj1, obj2 are valid pointers (caller contract).
    // All field accesses use verified C struct offsets.
    unsafe {
        let status_base = status as *mut u8;
        let p = pt as *mut PolytopeRepr;

        let mut upper: f64 = MJ_MAX_LIMIT;
        let mut upper2: f64 = MJ_MAX_LIMIT;
        let mut face: *mut FaceRepr = std::ptr::null_mut();
        let mut pface: *mut FaceRepr;

        let discrete = discrete_geoms(obj1, obj2);
        let tolerance = if discrete != 0 {
            MJ_MINEPATOK
        } else {
            *(status_base.add(STATUS_TOLERANCE_OFFSET) as *const f64)
        };

        let max_iter_raw = *(status_base.add(STATUS_MAX_ITER_OFFSET) as *const i32);
        let kmax = if max_iter_raw < 1000 { max_iter_raw } else { 1000 };

        let mut k = 0i32;
        while k < kmax {
            pface = face;

            // find the face closest to origin (lower bound for penetration depth)
            let mut lower2: f64 = MJ_MAX_LIMIT;
            for i in 0..(*p).nmap {
                let map_entry = *(*p).map.add(i as usize) as *mut FaceRepr;
                if (*map_entry).dist2 < lower2 {
                    face = map_entry;
                    lower2 = (*map_entry).dist2;
                }
            }

            // face not valid, return previous face
            if lower2 > upper2 || face.is_null() {
                face = pface;
                break;
            }

            // check if lower bound is 0
            if lower2 <= 0.0 {
                crate::engine::engine_util_errmem::mju_warning(
                    b"EPA: origin lies on affine hull of face\0".as_ptr() as *const i8);
                break;
            }

            // compute support point w from closest face's normal
            let lower = lower2.sqrt();
            let wi = epa_support(pt, obj1, obj2, (*face).v.as_ptr(), lower);
            let w = ((*p).verts as *const u8).add(wi as usize * SIZEOF_VERTEX);
            let w_vert = w as *const f64; // vert[3] at offset 0
            let upper_k = dot3((*face).v.as_ptr(), w_vert) / lower;
            if upper_k < upper {
                upper = upper_k;
                upper2 = upper * upper;
            }
            if upper - lower < tolerance {
                // terminate without contact when upper < lower on first iteration
                if k == 0 && upper < lower - 1e-10 {
                    face = std::ptr::null_mut();
                }
                break;
            }

            // check if vertex w is a repeated support point (discrete geoms)
            if discrete != 0 {
                let nverts = (*p).nverts - 1;
                let w_idx1 = *(w.add(VERTEX_INDEX1_OFFSET) as *const i32);
                let w_idx2 = *(w.add(VERTEX_INDEX2_OFFSET) as *const i32);
                let mut i = 0i32;
                while i < nverts {
                    let vi = ((*p).verts as *const u8).add(i as usize * SIZEOF_VERTEX);
                    let vi_idx1 = *(vi.add(VERTEX_INDEX1_OFFSET) as *const i32);
                    let vi_idx2 = *(vi.add(VERTEX_INDEX2_OFFSET) as *const i32);
                    if w_idx1 == vi_idx1 && w_idx2 == vi_idx2 {
                        break;
                    }
                    i += 1;
                }
                if i != nverts {
                    break;
                }
            }

            // set horizon.w = w->vert and expand polytope
            // horizon.w is at offset 96 in PolytopeRepr (after nmap+pad1, then 24 bytes for horizon)
            // Polytope: verts(8) + nverts(4)+pad(4) + faces(8) + nfaces(4) + maxfaces(4) + center(24)
            //           + map(8) + nmap(4)+pad(4) = 72 bytes before horizon
            // horizon: indices(8) + edges(8) + nedges(4)+pad(4) + w(8)
            // horizon.w is at offset 72 + 20 = 92... wait, let me recalculate
            // PolytopeRepr: verts(*u8)=8, nverts(i32)=4, _pad0=4, faces(*u8)=8, nfaces=4, maxfaces=4,
            //               center([f64;3])=24, map(**u8)=8, nmap=4, _pad1=4 = 72 total
            // then Horizon: indices=8, edges=8, nedges=4, _pad=4, w=8
            // horizon.w is at offset 72+8+8+4+4 = 96
            let hw_ptr = (pt as *mut u8).add(96) as *mut *const f64;
            *hw_ptr = w_vert;
            horizon(pt, face as *mut Face);

            // unrecoverable numerical issue
            let nedges_ptr = (pt as *mut u8).add(72 + 8 + 8) as *const i32; // horizon.nedges at 88
            if *nedges_ptr < 3 {
                face = std::ptr::null_mut();
                break;
            }

            let nfaces_saved = (*p).nfaces;
            let nedges = *nedges_ptr;

            // check if there's enough memory to store new faces
            if nedges > max_faces(pt) {
                crate::engine::engine_util_errmem::mju_warning(
                    b"EPA: out of memory for faces on expanding polytope\0".as_ptr() as *const i8);
                break;
            }

            // horizon.indices and horizon.edges pointers
            let hz_indices_ptr = *(pt as *const *const i32).add(0); // wrong, need correct offset
            // Actually: horizon is at offset 72, horizon.indices at 72, horizon.edges at 80, nedges at 88
            let hzn_base = (pt as *mut u8).add(72);
            let hz_indices = *(hzn_base as *const *mut i32);         // offset 72: indices ptr
            let hz_edges   = *(hzn_base.add(8) as *const *mut i32);  // offset 80: edges ptr
            // nedges is at offset 88 (already read above as *nedges_ptr)

            // attach first face
            let hzn_index_0 = *hz_indices.add(0);
            let hzn_edge_0  = *hz_edges.add(0);
            let hzn_face_0  = ((*p).faces as *mut FaceRepr).add(hzn_index_0 as usize);
            let hzn_verts_packed_0 = (*hzn_face_0).verts;
            let hzn_verts_0 = [
                hzn_verts_packed_0 & 0x3FF,
                (hzn_verts_packed_0 >> 10) & 0x3FF,
                (hzn_verts_packed_0 >> 20) & 0x3FF,
            ];
            let v1_0 = hzn_verts_0[hzn_edge_0 as usize];
            let v2_0 = hzn_verts_0[(hzn_edge_0 as usize + 1) % 3];
            (*hzn_face_0).adj[hzn_edge_0 as usize] = nfaces_saved;
            let dist2_0 = attach_face(pt, wi, v2_0, v1_0, nfaces_saved + nedges - 1, hzn_index_0, nfaces_saved + 1);

            // unrecoverable numerical issue
            if dist2_0 == 0.0 {
                face = std::ptr::null_mut();
                break;
            }

            // store face in map
            if dist2_0 >= lower2 && dist2_0 <= upper2 {
                let idx = (*p).nmap;
                *(*p).map.add(idx as usize) = ((*p).faces as *mut u8).add(((*p).nfaces - 1) as usize * 56);
                let new_face = *(*p).map.add(idx as usize) as *mut FaceRepr;
                (*new_face).index = idx;
                (*p).nmap = idx + 1;
            }

            // attach remaining faces
            let mut broken = false;
            let mut i = 1i32;
            while i < nedges {
                let cur = nfaces_saved + i;
                let next = nfaces_saved + (i + 1) % nedges;

                let hzn_index_i = *hz_indices.add(i as usize);
                let hzn_edge_i  = *hz_edges.add(i as usize);
                let hzn_face_i  = ((*p).faces as *mut FaceRepr).add(hzn_index_i as usize);
                let hzn_verts_packed_i = (*hzn_face_i).verts;
                let hzn_verts_i = [
                    hzn_verts_packed_i & 0x3FF,
                    (hzn_verts_packed_i >> 10) & 0x3FF,
                    (hzn_verts_packed_i >> 20) & 0x3FF,
                ];
                let v1_i = hzn_verts_i[hzn_edge_i as usize];
                let v2_i = hzn_verts_i[(hzn_edge_i as usize + 1) % 3];
                (*hzn_face_i).adj[hzn_edge_i as usize] = cur;
                let dist2_i = attach_face(pt, wi, v2_i, v1_i, cur - 1, hzn_index_i, next);

                // unrecoverable numerical issue
                if dist2_i == 0.0 {
                    face = std::ptr::null_mut();
                    broken = true;
                    break;
                }

                // store face in map
                if dist2_i >= lower2 && dist2_i <= upper2 {
                    let idx = (*p).nmap;
                    *(*p).map.add(idx as usize) = ((*p).faces as *mut u8).add(((*p).nfaces - 1) as usize * 56);
                    let new_face = *(*p).map.add(idx as usize) as *mut FaceRepr;
                    (*new_face).index = idx;
                    (*p).nmap = idx + 1;
                }
                i += 1;
            }

            // clear horizon
            *(hzn_base.add(16) as *mut i32) = 0;  // horizon.nedges = 0

            if broken { break; }

            // no face candidates left
            if (*p).nmap == 0 || face.is_null() {
                break;
            }

            k += 1;
        }

        *(status_base.add(STATUS_EPA_ITER_OFFSET) as *mut i32) = k;

        if !face.is_null() {
            let x1 = status_base.add(STATUS_X1_OFFSET) as *mut f64;
            let x2 = status_base.add(STATUS_X2_OFFSET) as *mut f64;
            let dist = epa_witness(pt as *const Polytope, face as *const Face, x1, x2);
            *(status_base.add(STATUS_DIST_OFFSET) as *mut f64) = dist;
            *(status_base.add(STATUS_NX_OFFSET) as *mut i32) = 1;
        } else {
            *(status_base.add(STATUS_NX_OFFSET) as *mut i32) = 0;
            *(status_base.add(STATUS_DIST_OFFSET) as *mut f64) = 0.0;
        }

        face as *mut Face
    }
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
    const MJ_GEOM_MESH: i32 = 7;
    const MJ_GEOM_BOX: i32 = 6;
    const MJ_GEOM_HFIELD: i32 = 1;

    // SAFETY: obj1 and obj2 are valid mjCCDObj pointers (caller contract)
    unsafe {
        // non-zero margin makes geoms smooth
        if (*obj1).margin != 0.0 || (*obj2).margin != 0.0 {
            return 0;
        }

        let g1 = (*obj1).geom_type;
        let g2 = (*obj2).geom_type;
        if (g1 == MJ_GEOM_MESH || g1 == MJ_GEOM_BOX || g1 == MJ_GEOM_HFIELD)
            && (g2 == MJ_GEOM_MESH || g2 == MJ_GEOM_BOX || g2 == MJ_GEOM_HFIELD)
        {
            1
        } else {
            0
        }
    }
}

/// C: gjk (engine/engine_collision_gjk.c:200)
/// Calls: copy3, discreteGeoms, dot3, equal3, gjkIntersect, gjkSupport, lincomb, sub3, subdistance
#[allow(unused_variables, non_snake_case)]
pub fn gjk(status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) {
    // mjCCDStatus field offsets (same as gjk_intersect / epa):
    const STATUS_DIST_OFFSET: usize = 0;
    const STATUS_X1_OFFSET: usize = 8;
    const STATUS_X2_OFFSET: usize = 1208;
    const STATUS_NX_OFFSET: usize = 2408;
    const STATUS_MAX_ITER_OFFSET: usize = 2412;
    const STATUS_TOLERANCE_OFFSET: usize = 2416;
    const STATUS_DIST_CUTOFF_OFFSET: usize = 2432;
    const STATUS_GJK_ITER_OFFSET: usize = 2440;
    const STATUS_SIMPLEX_OFFSET: usize = 2456;
    const STATUS_NSIMPLEX_OFFSET: usize = 2776;
    const SIZEOF_VERTEX: usize = 80;
    const MJ_MAX_LIMIT: f64 = f64::MAX;
    const MJ_MINVAL2: f64 = 1e-15_f64 * 1e-15_f64;

    // SAFETY: status, obj1, obj2 are valid pointers (caller contract).
    unsafe {
        let sb = status as *mut u8;

        let get_dist = *(sb.add(STATUS_DIST_CUTOFF_OFFSET) as *const f64) > 0.0;
        let mut backup_gjk = !get_dist;
        let simplex = sb.add(STATUS_SIMPLEX_OFFSET) as *mut Vertex;
        let mut n: i32 = 0;
        let mut k: i32 = 0;
        let kmax = *(sb.add(STATUS_MAX_ITER_OFFSET) as *const i32);
        let x1_k = sb.add(STATUS_X1_OFFSET) as *mut f64;
        let x2_k = sb.add(STATUS_X2_OFFSET) as *mut f64;
        let mut x_k: [f64; 3] = [0.0; 3];
        let mut lambda: [f64; 4] = [1.0, 0.0, 0.0, 0.0];
        let dist_cutoff = *(sb.add(STATUS_DIST_CUTOFF_OFFSET) as *const f64);
        let cutoff2 = dist_cutoff * dist_cutoff;
        let tolerance = *(sb.add(STATUS_TOLERANCE_OFFSET) as *const f64);
        let tol2 = tolerance * tolerance;

        // if both geoms are discrete, finite convergence is guaranteed; set tolerance to 0
        let epsilon = if discrete_geoms(obj1, obj2) != 0 { 0.0 } else { 0.5 * tol2 };
        let min_norm2 = if discrete_geoms(obj1, obj2) != 0 { MJ_MINVAL2 } else { tol2 };

        // set initial guess: x_k = x1_k - x2_k
        sub3(x_k.as_mut_ptr(), x1_k, x2_k);

        let mut x_norm: f64 = 0.0;

        'outer: loop {
            if k >= kmax { break; }

            x_norm = dot3(x_k.as_ptr(), x_k.as_ptr());
            if x_norm < min_norm2 {
                break;
            }
            x_norm = x_norm.sqrt();

            // compute the kth support point
            let sv_n = (simplex as *mut u8).add(n as usize * SIZEOF_VERTEX) as *mut Vertex;
            gjk_support(sv_n, obj1, obj2, x_k.as_ptr(), x_norm);
            let s_k = sv_n as *const f64; // vert field at offset 0

            // stopping criteria: Frank-Wolfe duality gap
            let mut diff: [f64; 3] = [0.0; 3];
            sub3(diff.as_mut_ptr(), x_k.as_ptr(), s_k);
            if dot3(x_k.as_ptr(), diff.as_ptr()) < epsilon {
                if k == 0 { n = 1; }
                break;
            }

            // hyperplane separation check
            if !get_dist {
                if dot3(x_k.as_ptr(), s_k) > 0.0 {
                    *(sb.add(STATUS_GJK_ITER_OFFSET) as *mut i32) = k;
                    *(sb.add(STATUS_NSIMPLEX_OFFSET) as *mut i32) = 0;
                    *(sb.add(STATUS_NX_OFFSET) as *mut i32) = 0;
                    *(sb.add(STATUS_DIST_OFFSET) as *mut f64) = MJ_MAX_LIMIT;
                    return;
                }
            } else if dist_cutoff < MJ_MAX_LIMIT {
                let vs = dot3(x_k.as_ptr(), s_k);
                let vv = dot3(x_k.as_ptr(), x_k.as_ptr());
                if vs > 0.0 && vs * vs / vv >= cutoff2 {
                    *(sb.add(STATUS_GJK_ITER_OFFSET) as *mut i32) = k;
                    *(sb.add(STATUS_NSIMPLEX_OFFSET) as *mut i32) = 0;
                    *(sb.add(STATUS_NX_OFFSET) as *mut i32) = 0;
                    *(sb.add(STATUS_DIST_OFFSET) as *mut f64) = MJ_MAX_LIMIT;
                    return;
                }
            }

            // tetrahedron: fallback to gjkIntersect
            if n == 3 && backup_gjk {
                *(sb.add(STATUS_GJK_ITER_OFFSET) as *mut i32) = k;
                let ret = gjk_intersect(status, obj1, obj2);
                if ret != -1 {
                    *(sb.add(STATUS_NX_OFFSET) as *mut i32) = 0;
                    *(sb.add(STATUS_DIST_OFFSET) as *mut f64) = if ret > 0 { 0.0 } else { MJ_MAX_LIMIT };
                    return;
                }
                k = *(sb.add(STATUS_GJK_ITER_OFFSET) as *const i32);
                backup_gjk = false;
            }

            // run subdistance algorithm
            subdistance(lambda.as_mut_ptr(), n + 1, simplex as *const Vertex);

            // compact simplex: remove vertices where lambda == 0
            let mut n_new: i32 = 0;
            for i in 0..4i32 {
                if lambda[i as usize] == 0.0 { continue; }
                // simplex[n_new] = simplex[i]
                std::ptr::copy_nonoverlapping(
                    (simplex as *mut u8).add(i as usize * SIZEOF_VERTEX),
                    (simplex as *mut u8).add(n_new as usize * SIZEOF_VERTEX),
                    SIZEOF_VERTEX,
                );
                lambda[n_new as usize] = lambda[i as usize];
                n_new += 1;
            }
            n = n_new;

            // SHOULD NOT OCCUR
            if n < 1 {
                *(sb.add(STATUS_GJK_ITER_OFFSET) as *mut i32) = k;
                *(sb.add(STATUS_NSIMPLEX_OFFSET) as *mut i32) = 0;
                *(sb.add(STATUS_NX_OFFSET) as *mut i32) = 0;
                *(sb.add(STATUS_DIST_OFFSET) as *mut f64) = MJ_MAX_LIMIT;
                return;
            }

            // get next x_k
            let mut x_next: [f64; 3] = [0.0; 3];
            let sv0 = (simplex as *const u8).add(0) as *const f64;
            let sv1 = (simplex as *const u8).add(SIZEOF_VERTEX) as *const f64;
            let sv2 = (simplex as *const u8).add(2 * SIZEOF_VERTEX) as *const f64;
            let sv3 = (simplex as *const u8).add(3 * SIZEOF_VERTEX) as *const f64;
            lincomb(x_next.as_mut_ptr(), lambda.as_ptr(), n, sv0, sv1, sv2, sv3);

            // x_k converged
            if equal3(x_next.as_ptr(), x_k.as_ptr()) != 0 {
                break;
            }
            copy3(x_k.as_mut_ptr(), x_next.as_ptr());

            // tetrahedron containing origin
            if n == 4 {
                x_norm = 0.0;
                break;
            }

            k += 1;
        }

        // compute approximate witness points
        let v1_vert1 = (simplex as *const u8).add(24) as *const f64;   // vert1 at offset 24
        let v1_vert2 = (simplex as *const u8).add(48) as *const f64;   // vert2 at offset 48
        let v2_vert1 = (simplex as *const u8).add(SIZEOF_VERTEX + 24) as *const f64;
        let v2_vert2 = (simplex as *const u8).add(SIZEOF_VERTEX + 48) as *const f64;
        let v3_vert1 = (simplex as *const u8).add(2 * SIZEOF_VERTEX + 24) as *const f64;
        let v3_vert2 = (simplex as *const u8).add(2 * SIZEOF_VERTEX + 48) as *const f64;
        let v4_vert1 = (simplex as *const u8).add(3 * SIZEOF_VERTEX + 24) as *const f64;
        let v4_vert2 = (simplex as *const u8).add(3 * SIZEOF_VERTEX + 48) as *const f64;
        lincomb(x1_k, lambda.as_ptr(), n, v1_vert1, v2_vert1, v3_vert1, v4_vert1);
        lincomb(x2_k, lambda.as_ptr(), n, v1_vert2, v2_vert2, v3_vert2, v4_vert2);

        *(sb.add(STATUS_NX_OFFSET) as *mut i32) = 1;
        *(sb.add(STATUS_GJK_ITER_OFFSET) as *mut i32) = k;
        *(sb.add(STATUS_NSIMPLEX_OFFSET) as *mut i32) = n;
        *(sb.add(STATUS_DIST_OFFSET) as *mut f64) = x_norm;
    }
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
    // Vertex layout (80 bytes):
    //   mjtNum vert[3];   offset 0
    //   mjtNum vert1[3];  offset 24
    //   mjtNum vert2[3];  offset 48
    //   int index1;       offset 72
    //   int index2;       offset 76

    const VERT_OFFSET: usize = 0;
    const VERT1_OFFSET: usize = 24;
    const VERT2_OFFSET: usize = 48;
    const INDEX1_OFFSET: usize = 72;
    const INDEX2_OFFSET: usize = 76;

    type SupportFn = unsafe extern "C" fn(*mut f64, *mut mjCCDObj, *const f64);

    // SAFETY: v is a valid Vertex pointer, obj1/obj2 are valid mjCCDObj pointers,
    //         dir and dir_neg are valid f64[3] pointers. Function pointers are valid.
    unsafe {
        let v_base = v as *mut u8;
        let vert1 = v_base.add(VERT1_OFFSET) as *mut f64;
        let vert2 = v_base.add(VERT2_OFFSET) as *mut f64;
        let vert = v_base.add(VERT_OFFSET) as *mut f64;

        // obj1->support(v->vert1, obj1, dir)
        let support1: SupportFn = *(&(*obj1).support.unwrap() as *const _ as *const SupportFn);
        support1(vert1, obj1, dir);
        if (*obj1).margin > 0.0 && (*obj1).geom >= 0 {
            let margin = 0.5 * (*obj1).margin;
            *vert1.add(0) += *dir.add(0) * margin;
            *vert1.add(1) += *dir.add(1) * margin;
            *vert1.add(2) += *dir.add(2) * margin;
        }

        // obj2->support(v->vert2, obj2, dir_neg)
        let support2: SupportFn = *(&(*obj2).support.unwrap() as *const _ as *const SupportFn);
        support2(vert2, obj2, dir_neg);
        if (*obj2).margin > 0.0 && (*obj2).geom >= 0 {
            let margin = 0.5 * (*obj2).margin;
            *vert2.add(0) += *dir_neg.add(0) * margin;
            *vert2.add(1) += *dir_neg.add(1) * margin;
            *vert2.add(2) += *dir_neg.add(2) * margin;
        }

        // v->vert = v->vert1 - v->vert2
        sub3(vert, vert1, vert2);

        // v->index1 = obj1->vertindex
        *(v_base.add(INDEX1_OFFSET) as *mut i32) = (*obj1).vertindex;
        // v->index2 = obj2->vertindex
        *(v_base.add(INDEX2_OFFSET) as *mut i32) = (*obj2).vertindex;
    }
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
    // SAFETY: dir is a valid f64[3] pointer (caller contract)
    let dir_neg: [f64; 3] = unsafe {[
        -*dir.add(0),
        -*dir.add(1),
        -*dir.add(2),
    ]};
    support(v, obj1, obj2, dir, dir_neg.as_ptr());
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
    // Vertex layout: vert[3] at offset 0
    const MJ_MINVAL2: f64 = 1E-15_f64 * 1E-15_f64;
    const MJ_MAXVAL2: f64 = 1E+10_f64 * 1E+10_f64;
    const MJ_MAX_LIMIT: f64 = f64::MAX;

    let v1_vert = v1 as *const f64;
    let v2_vert = v2 as *const f64;
    let v3_vert = v3 as *const f64;

    let mut diff1: [f64; 3] = [0.0; 3];
    let mut diff2: [f64; 3] = [0.0; 3];
    sub3(diff1.as_mut_ptr(), v3_vert, v1_vert);
    sub3(diff2.as_mut_ptr(), v2_vert, v1_vert);
    cross3(normal, diff1.as_ptr(), diff2.as_ptr());
    let norm2 = dot3(normal, normal);
    if norm2 > MJ_MINVAL2 && norm2 < MJ_MAXVAL2 {
        scl3(normal, normal, 1.0 / norm2.sqrt());
        return dot3(normal, v1_vert);
    }
    MJ_MAX_LIMIT  // cannot recover normal (ignore face)
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
    // Polytope layout:
    //   Vertex* verts;   offset 0
    //   int nverts;      offset 8
    //   ...
    //   int nfaces;      offset 24
    //   int nmap;        offset 64
    //
    // mjCCDStatus layout:
    //   ...
    //   gjk_iterations:  i32,  offset 2440
    //   epa_iterations:  i32,  offset 2444
    //   epa_status:      i32,  offset 2448
    //   (pad 4)
    //   simplex[4]:      Vertex[4], offset 2456
    //   nsimplex:        i32,  offset 2776
    //
    // Vertex: 80 bytes

    const SIZEOF_VERTEX: usize = 80;
    const STATUS_SIMPLEX_OFFSET: usize = 2456;
    const STATUS_NSIMPLEX_OFFSET: usize = 2776;

    // SAFETY: pt is a valid Polytope pointer, status is a valid mjCCDStatus pointer.
    //         v1, v2, v3 are valid vertex indices in pt->verts.
    unsafe {
        let pt_base = pt as *mut u8;
        let status_base = status as *mut u8;

        let verts_ptr = *(pt_base as *const *const u8);  // Vertex* verts
        let simplex_ptr = status_base.add(STATUS_SIMPLEX_OFFSET);

        // status->nsimplex = 3
        *(status_base.add(STATUS_NSIMPLEX_OFFSET) as *mut i32) = 3;

        // status->simplex[0] = pt->verts[v1]
        std::ptr::copy_nonoverlapping(
            verts_ptr.add(v1 as usize * SIZEOF_VERTEX),
            simplex_ptr as *mut u8,
            SIZEOF_VERTEX,
        );
        // status->simplex[1] = pt->verts[v2]
        std::ptr::copy_nonoverlapping(
            verts_ptr.add(v2 as usize * SIZEOF_VERTEX),
            simplex_ptr.add(SIZEOF_VERTEX) as *mut u8,
            SIZEOF_VERTEX,
        );
        // status->simplex[2] = pt->verts[v3]
        std::ptr::copy_nonoverlapping(
            verts_ptr.add(v3 as usize * SIZEOF_VERTEX),
            simplex_ptr.add(2 * SIZEOF_VERTEX) as *mut u8,
            SIZEOF_VERTEX,
        );

        // reset polytope
        *(pt_base.add(24) as *mut i32) = 0;  // pt->nfaces = 0
        *(pt_base.add(8) as *mut i32) = 0;   // pt->nverts = 0
        *(pt_base.add(64) as *mut i32) = 0;  // pt->nmap = 0
    }
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
    let mut diff1: [f64; 3] = [0.0; 3];
    let mut diff2: [f64; 3] = [0.0; 3];
    let mut diff3: [f64; 3] = [0.0; 3];
    let mut diff4: [f64; 3] = [0.0; 3];
    let mut n: [f64; 3] = [0.0; 3];

    sub3(diff1.as_mut_ptr(), p1, p0);
    sub3(diff2.as_mut_ptr(), p2, p0);
    cross3(n.as_mut_ptr(), diff1.as_ptr(), diff2.as_ptr());

    sub3(diff3.as_mut_ptr(), p3, p0);
    let dot1 = dot3(n.as_ptr(), diff3.as_ptr());

    scl3(diff4.as_mut_ptr(), p0, -1.0);
    let dot2 = dot3(n.as_ptr(), diff4.as_ptr());

    if dot1 > 0.0 && dot2 > 0.0 { return 1; }
    if dot1 < 0.0 && dot2 < 0.0 { return 1; }
    0
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
    // SAFETY: R is a valid f64[9] output pointer, axis is a valid f64[3] pointer.
    unsafe {
        let n = norm3(axis);
        let u1 = *axis.add(0) / n;
        let u2 = *axis.add(1) / n;
        let u3 = *axis.add(2) / n;
        let sin: f64 = 0.86602540378;  // sin(120 deg)
        let cos: f64 = -0.5;           // cos(120 deg)
        *R.add(0) = cos + u1 * u1 * (1.0 - cos);
        *R.add(1) = u1 * u2 * (1.0 - cos) - u3 * sin;
        *R.add(2) = u1 * u3 * (1.0 - cos) + u2 * sin;
        *R.add(3) = u2 * u1 * (1.0 - cos) + u3 * sin;
        *R.add(4) = cos + u2 * u2 * (1.0 - cos);
        *R.add(5) = u2 * u3 * (1.0 - cos) - u1 * sin;
        *R.add(6) = u1 * u3 * (1.0 - cos) - u2 * sin;
        *R.add(7) = u2 * u3 * (1.0 - cos) + u1 * sin;
        *R.add(8) = cos + u3 * u3 * (1.0 - cos);
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
    // SAFETY: v1..v5 are valid pointers to [f64;3] arrays (caller contract)
    unsafe {
        let mut diff12: [f64; 3] = [0.0; 3];
        let mut diff13: [f64; 3] = [0.0; 3];
        let mut diff14: [f64; 3] = [0.0; 3];
        let mut diff15: [f64; 3] = [0.0; 3];
        sub3(diff12.as_mut_ptr(), v2, v1);
        sub3(diff13.as_mut_ptr(), v3, v1);
        sub3(diff14.as_mut_ptr(), v4, v1);
        sub3(diff15.as_mut_ptr(), v5, v1);

        let vol1 = det3(diff13.as_ptr(), diff14.as_ptr(), diff12.as_ptr());
        let vol2 = det3(diff14.as_ptr(), diff15.as_ptr(), diff12.as_ptr());
        let vol3 = det3(diff15.as_ptr(), diff13.as_ptr(), diff12.as_ptr());

        if vol1 >= 0.0 && vol2 >= 0.0 && vol3 >= 0.0 { return 1; }
        if vol1 <= 0.0 && vol2 <= 0.0 && vol3 <= 0.0 { return -1; }
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
    // SAFETY: lambda is a valid f64[3] output, v1/v2/v3/p are valid f64[3] pointers.
    unsafe {
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
        let mu1: f64 = f64::abs(M_14);
        let mu2: f64 = f64::abs(M_24);
        let mu3: f64 = f64::abs(M_34);

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

        // C31: signed area of (p, v2, v3)
        let C31: f64 = *p.add(x) * *v2.add(y) + *p.add(y) * *v3.add(x) + *v2.add(x) * *v3.add(y)
                     - *p.add(x) * *v3.add(y) - *p.add(y) * *v2.add(x) - *v3.add(x) * *v2.add(y);

        // C32: signed area of (p, v1, v3)
        let C32: f64 = *p.add(x) * *v3.add(y) + *p.add(y) * *v1.add(x) + *v3.add(x) * *v1.add(y)
                     - *p.add(x) * *v1.add(y) - *p.add(y) * *v3.add(x) - *v1.add(x) * *v3.add(y);

        // C33: signed area of (p, v1, v2)
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
    // Polytope layout:
    //   Face** map;  offset 56
    //   int nmap;    offset 64
    //
    // Face layout (56 bytes):
    //   int verts;   offset 0
    //   int adj[3];  offset 4
    //   mjtNum v[3]; offset 16
    //   mjtNum dist2; offset 40
    //   int index;   offset 48

    // SAFETY: pt is a valid Polytope pointer, face is a valid Face in pt->faces (EPA invariant)
    unsafe {
        let pt_base = pt as *mut u8;
        let map_ptr = *(pt_base.add(56) as *const *mut *mut u8);         // Face** map
        let nmap_ptr = pt_base.add(64) as *mut i32;                       // int nmap

        let face_base = face as *mut u8;
        let face_index = *(face_base.add(48) as *const i32);             // face->index

        if face_index >= 0 {
            // pt->map[face->index] = pt->map[--pt->nmap]
            let nmap = *nmap_ptr - 1;
            *nmap_ptr = nmap;
            let last_face = *map_ptr.add(nmap as usize);
            *map_ptr.add(face_index as usize) = last_face;
            // pt->map[face->index]->index = face->index
            *(last_face.add(48) as *mut i32) = face_index;
        }
        // face->index = -2
        *(face_base.add(48) as *mut i32) = -2;
    }
}

/// C: maxFaces (engine/engine_collision_gjk.c:1226)
#[allow(unused_variables, non_snake_case)]
pub fn max_faces(pt: *mut Polytope) -> i32 {
    // Polytope layout:
    //   int nfaces;   offset 24
    //   int maxfaces; offset 28

    // SAFETY: pt is a valid Polytope pointer (caller contract from EPA)
    unsafe {
        let pt_base = pt as *mut u8;
        let maxfaces = *(pt_base.add(28) as *const i32);
        let nfaces = *(pt_base.add(24) as *const i32);
        maxfaces - nfaces
    }
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
    // Vertex: { vert[3]: f64, vert1[3]: f64, vert2[3]: f64, index1: i32, index2: i32 } = 80 bytes
    const SIZEOF_VERTEX: usize = 80;

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

    // SAFETY: pt is a valid Polytope, face is a valid Face. Vertex layout is 80 bytes with
    // vert at offset 0, vert1 at offset 24, vert2 at offset 48.
    unsafe {
        let p = pt as *const PolytopeRepr;
        let f = face as *const FaceRepr;

        let verts_packed = (*f).verts;
        let verts = [
            (verts_packed & 0x3FF) as usize,
            ((verts_packed >> 10) & 0x3FF) as usize,
            ((verts_packed >> 20) & 0x3FF) as usize,
        ];

        let verts_base = (*p).verts;
        // vert field at offset 0
        let v1_vert = verts_base.add(verts[0] * SIZEOF_VERTEX) as *const f64;
        let v2_vert = verts_base.add(verts[1] * SIZEOF_VERTEX) as *const f64;
        let v3_vert = verts_base.add(verts[2] * SIZEOF_VERTEX) as *const f64;

        // vert1 field at offset 24 (3 * sizeof(f64))
        let v1_vert1 = verts_base.add(verts[0] * SIZEOF_VERTEX + 24) as *const f64;
        let v2_vert1 = verts_base.add(verts[1] * SIZEOF_VERTEX + 24) as *const f64;
        let v3_vert1 = verts_base.add(verts[2] * SIZEOF_VERTEX + 24) as *const f64;

        // vert2 field at offset 48 (6 * sizeof(f64))
        let v1_vert2 = verts_base.add(verts[0] * SIZEOF_VERTEX + 48) as *const f64;
        let v2_vert2 = verts_base.add(verts[1] * SIZEOF_VERTEX + 48) as *const f64;
        let v3_vert2 = verts_base.add(verts[2] * SIZEOF_VERTEX + 48) as *const f64;

        // compute affine coordinates for witness points on plane defined by face
        let mut lambda: [f64; 3] = [0.0; 3];
        tri_affine_coord(lambda.as_mut_ptr(), v1_vert, v2_vert, v3_vert, (*f).v.as_ptr());

        // witness point on geom 1
        lincomb(x1, lambda.as_ptr(), 3, v1_vert1, v2_vert1, v3_vert1, std::ptr::null());

        // witness point on geom 2
        lincomb(x2, lambda.as_ptr(), 3, v1_vert2, v2_vert2, v3_vert2, std::ptr::null());

        -(*f).dist2.sqrt()
    }
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
    // SAFETY: caller guarantees polygon points to nvert*3 f64 array, curr is within bounds
    unsafe {
        if curr == polygon.add(3 * (nvert as usize - 1)) {
            polygon
        } else {
            curr.add(3)
        }
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
pub fn polygon_quad(res: *mut *mut f64, polygon: *mut f64, nvert: i32) {
    // SAFETY: caller guarantees polygon[nvert*3], res[4] valid, nvert >= 4
    unsafe {
        let mut a = polygon;
        let mut b = polygon.add(3);
        let mut c = polygon.add(6);
        let mut d = polygon.add(9);
        *res.add(0) = a;
        *res.add(1) = b;
        *res.add(2) = c;
        *res.add(3) = d;
        let mut m = area4(a, b, c, d);
        let end = polygon.add(3 * nvert as usize);

        while a < end {
            loop {
                let m_next = area4(a, b, c, next(polygon, nvert, d));
                if m_next <= m {
                    break;
                }
                m = m_next;
                d = next(polygon, nvert, d);
                *res.add(0) = a;
                *res.add(1) = b;
                *res.add(2) = c;
                *res.add(3) = d;
                loop {
                    let m_next2 = area4(a, b, next(polygon, nvert, c), d);
                    if m_next2 <= m {
                        break;
                    }
                    m = m_next2;
                    c = next(polygon, nvert, c);
                    *res.add(0) = a;
                    *res.add(1) = b;
                    *res.add(2) = c;
                    *res.add(3) = d;
                }
                loop {
                    let m_next3 = area4(a, next(polygon, nvert, b), c, d);
                    if m_next3 <= m {
                        break;
                    }
                    m = m_next3;
                    b = next(polygon, nvert, b);
                    *res.add(0) = a;
                    *res.add(1) = b;
                    *res.add(2) = c;
                    *res.add(3) = d;
                }
            }
            if b == a {
                b = next(polygon, nvert, b);
                if c == b {
                    c = next(polygon, nvert, c);
                    if d == c {
                        d = next(polygon, nvert, d);
                    }
                }
            }
            a = a.add(3);
        }
    }
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
    // SAFETY: res[3], v1[3], v2[3], n[3] are valid pointers from caller
    unsafe {
        let mut v3: [f64; 3] = [0.0; 3];
        let mut diff1: [f64; 3] = [0.0; 3];
        let mut diff2: [f64; 3] = [0.0; 3];
        add3(v3.as_mut_ptr(), v1, n);
        sub3(diff1.as_mut_ptr(), v2, v1);
        sub3(diff2.as_mut_ptr(), v3.as_ptr(), v1);
        cross3(res, diff1.as_ptr(), diff2.as_ptr());

        // normalize isn't needed (cancelled out), but done to avoid asymmetric rounding later on
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
    const MJ_MINVAL: f64 = 1E-15_f64;
    // SAFETY: a[3], n[3], p[3] are valid pointers from caller
    unsafe {
        let diff: [f64; 3] = [
            *p.add(0) - *a.add(0),
            *p.add(1) - *a.add(1),
            *p.add(2) - *a.add(2),
        ];
        if dot3(diff.as_ptr(), n) > -MJ_MINVAL { 1 } else { 0 }
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
    const MJ_MAX_LIMIT: f64 = f64::MAX;
    // SAFETY: res[3], pn[3], a[3], b[3] are valid pointers from caller
    unsafe {
        let mut ab: [f64; 3] = [0.0; 3];
        sub3(ab.as_mut_ptr(), b, a);
        let temp = dot3(pn, ab.as_ptr());
        if temp == 0.0 {
            return MJ_MAX_LIMIT;  // parallel; no intersection
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
    const MJ_MAX_POLYVERT: usize = 150;
    const MJ_MAX_CONPAIR: i32 = 50;
    const STATUS_X1_OFFSET: usize = 8;
    const STATUS_X2_OFFSET: usize = 1208;
    const STATUS_NX_OFFSET: usize = 2408;
    const STATUS_MAX_CONTACTS_OFFSET: usize = 2424;

    // SAFETY: All pointer args are valid (caller contract). Stack arrays bounded by MJ_MAX_POLYVERT.
    unsafe {
        if nface1 < 3 { return; }

        let sb = status as *mut u8;
        let mut pn = [0.0f64; 3 * MJ_MAX_POLYVERT];
        let mut pd = [0.0f64; MJ_MAX_POLYVERT];

        for i in 0..(nface1 as usize - 1) {
            pd[i] = plane_normal(pn.as_mut_ptr().add(3 * i), face1.add(3 * i), face1.add(3 * i + 3), n);
        }
        let last = (nface1 as usize) - 1;
        pd[last] = plane_normal(pn.as_mut_ptr().add(3 * last), face1.add(3 * last), face1, n);

        let mut polygon1 = [0.0f64; 6 * MJ_MAX_POLYVERT];
        let mut polygon2 = [0.0f64; 6 * MJ_MAX_POLYVERT];
        let mut npolygon = nface2 as usize;
        let mut use_poly1 = true;

        for i in 0..nface2 as usize {
            polygon1[3 * i] = *face2.add(3 * i);
            polygon1[3 * i + 1] = *face2.add(3 * i + 1);
            polygon1[3 * i + 2] = *face2.add(3 * i + 2);
        }

        let mut e = 0usize;
        while e < (3 * nface1 as usize) {
            let mut nclipped: usize = 0;
            let (polygon, clipped) = if use_poly1 {
                (polygon1.as_mut_ptr(), polygon2.as_mut_ptr())
            } else {
                (polygon2.as_mut_ptr(), polygon1.as_mut_ptr())
            };

            for i in 0..npolygon {
                let p_ptr = polygon.add(3 * i);
                let q_ptr = if i < npolygon - 1 { polygon.add(3 * (i + 1)) } else { polygon };
                let inside1 = halfspace(face1.add(e), pn.as_ptr().add(e), p_ptr);
                let inside2 = halfspace(face1.add(e), pn.as_ptr().add(e), q_ptr);
                if inside1 == 0 && inside2 == 0 { continue; }
                if inside1 != 0 && inside2 != 0 {
                    copy3(clipped.add(3 * nclipped), q_ptr);
                    nclipped += 1;
                    continue;
                }
                let t = plane_intersect(clipped.add(3 * nclipped), pn.as_ptr().add(e), pd[e / 3], p_ptr, q_ptr);
                if (0.0..=1.0).contains(&t) {
                    nclipped += 1;
                }
                if inside2 != 0 {
                    copy3(clipped.add(3 * nclipped), q_ptr);
                    nclipped += 1;
                }
            }
            use_poly1 = !use_poly1;
            npolygon = nclipped;
            e += 3;
        }

        if npolygon < 1 { return; }

        let polygon = if !use_poly1 { polygon1.as_mut_ptr() } else { polygon2.as_mut_ptr() };
        let x1_k = sb.add(STATUS_X1_OFFSET) as *mut f64;
        let x2_k = sb.add(STATUS_X2_OFFSET) as *mut f64;
        let max_contacts = *(sb.add(STATUS_MAX_CONTACTS_OFFSET) as *const i32);

        if max_contacts < 5 && npolygon > 4 {
            *(sb.add(STATUS_NX_OFFSET) as *mut i32) = 4;
            let mut rect: [*mut f64; 4] = [std::ptr::null_mut(); 4];
            polygon_quad(rect.as_mut_ptr(), polygon, npolygon as i32);
            for i in 0..4 {
                copy3(x2_k.add(3 * i), rect[i]);
                sub3(x1_k.add(3 * i), x2_k.add(3 * i), dir);
            }
            return;
        }

        if npolygon > MJ_MAX_CONPAIR as usize {
            *(sb.add(STATUS_NX_OFFSET) as *mut i32) = MJ_MAX_CONPAIR;
            let mut i = 0usize;
            while i < (3 * MJ_MAX_CONPAIR as usize) {
                copy3(x2_k.add(i), polygon.add(i));
                sub3(x1_k.add(i), x2_k.add(i), dir);
                i += 3;
            }
            return;
        }

        if nface2 == 2 && npolygon > 2 {
            let mut best1 = 0usize;
            let mut best2 = 1usize;
            let mut d = 0.0f64;
            for i in 0..npolygon {
                for j in (i + 1)..npolygon {
                    let mut diff: [f64; 3] = [0.0; 3];
                    sub3(diff.as_mut_ptr(), polygon.add(3 * j), polygon.add(3 * i));
                    let d2 = dot3(diff.as_ptr(), diff.as_ptr());
                    if d2 > d { d = d2; best1 = i; best2 = j; }
                }
            }
            copy3(x2_k, polygon.add(3 * best1));
            sub3(x1_k, x2_k, dir);
            copy3(x2_k.add(3), polygon.add(3 * best2));
            sub3(x1_k.add(3), x2_k.add(3), dir);
            *(sb.add(STATUS_NX_OFFSET) as *mut i32) = 2;
            return;
        }

        let mut i = 0usize;
        while i < (3 * npolygon) {
            copy3(x2_k.add(i), polygon.add(i));
            sub3(x1_k.add(i), x2_k.add(i), dir);
            i += 3;
        }
        *(sb.add(STATUS_NX_OFFSET) as *mut i32) = npolygon as i32;
    }
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
    // SAFETY: caller guarantees arr1[n], arr2[m], res[2] are valid
    unsafe {
        let mut count: i32 = 0;
        for i in 0..n as usize {
            for j in 0..m as usize {
                if *arr1.add(i) == *arr2.add(j) {
                    *res.add(count as usize) = *arr1.add(i);
                    count += 1;
                    if count == 2 {
                        return 2;
                    }
                }
            }
        }
        count
    }
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
    const MJ_MAX_POLYVERT: i32 = 150;

    #[repr(C)]
    struct MeshData {
        nvert: i32,
        mesh_polynum: i32,
        vert: *const f32,
        mpolymapadr: *const i32,
        mpolymapnum: *const i32,
        polymap: *const i32,
        polyvertadr: *const i32,
        polyvertnum: *const i32,
        polyvert: *const i32,
        polynormal: *const f64,
        graph: *const i32,
    }

    // SAFETY: res[3*n], resind[n], obj are valid pointers from caller contract
    unsafe {
        let mesh_ptr = &(*obj).data as *const _ as *const MeshData;
        let polymap = (*mesh_ptr).polymap;
        let polynormal = (*mesh_ptr).polynormal;
        let mat = (*obj).mat.as_ptr();

        if dim == 3 {
            let v1_adr = *(*mesh_ptr).mpolymapadr.add(v1 as usize);
            let v1_num = *(*mesh_ptr).mpolymapnum.add(v1 as usize);

            let v2_adr = *(*mesh_ptr).mpolymapadr.add(v2 as usize);
            let v2_num = *(*mesh_ptr).mpolymapnum.add(v2 as usize);

            let v3_adr = *(*mesh_ptr).mpolymapadr.add(v3 as usize);
            let v3_num = *(*mesh_ptr).mpolymapnum.add(v3 as usize);

            let mut edgeset = [0i32; 2];
            let mut faceset = [0i32; 2];
            let n = intersect(edgeset.as_mut_ptr(), polymap.add(v1_adr as usize),
                              polymap.add(v2_adr as usize), v1_num, v2_num);
            if n == 0 { return 0; }
            let n = intersect(faceset.as_mut_ptr(), edgeset.as_ptr(),
                              polymap.add(v3_adr as usize), n, v3_num);
            if n == 0 { return 0; }

            // three vertices on mesh define a unique face
            let normal = polynormal.add(3 * faceset[0] as usize);
            globalcoord(res, mat, std::ptr::null(), *normal.add(0), *normal.add(1), *normal.add(2));
            *resind.add(0) = faceset[0];
            return 1;
        }

        if dim == 2 {
            let v1_adr = *(*mesh_ptr).mpolymapadr.add(v1 as usize);
            let v1_num = *(*mesh_ptr).mpolymapnum.add(v1 as usize);

            let v2_adr = *(*mesh_ptr).mpolymapadr.add(v2 as usize);
            let v2_num = *(*mesh_ptr).mpolymapnum.add(v2 as usize);

            // up to two faces as vertices on mesh define an edge
            let mut edgeset = [0i32; 2];
            let n = intersect(edgeset.as_mut_ptr(), polymap.add(v1_adr as usize),
                              polymap.add(v2_adr as usize), v1_num, v2_num);
            if n == 0 { return 0; }
            for i in 0..n as usize {
                let normal = polynormal.add(3 * edgeset[i] as usize);
                globalcoord(res.add(3 * i), mat, std::ptr::null(),
                            *normal.add(0), *normal.add(1), *normal.add(2));
                *resind.add(i) = edgeset[i];
            }
            return n;
        }

        if dim == 1 {
            let v1_adr = *(*mesh_ptr).mpolymapadr.add(v1 as usize);
            let mut v1_num = *(*mesh_ptr).mpolymapnum.add(v1 as usize);

            // cap number of possible faces intersecting at a vertex
            if v1_num > MJ_MAX_POLYVERT { v1_num = MJ_MAX_POLYVERT; }
            for i in 0..v1_num as usize {
                let index = *polymap.add(v1_adr as usize + i);
                let normal = polynormal.add(3 * index as usize);
                globalcoord(res.add(3 * i), mat, std::ptr::null(),
                            *normal.add(0), *normal.add(1), *normal.add(2));
                *resind.add(i) = index;
            }
            return v1_num;
        }

        0
    }
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
    const MJ_MAX_POLYVERT: i32 = 150;

    #[repr(C)]
    struct MeshData {
        nvert: i32,
        mesh_polynum: i32,
        vert: *const f32,
        mpolymapadr: *const i32,
        mpolymapnum: *const i32,
        polymap: *const i32,
        polyvertadr: *const i32,
        polyvertnum: *const i32,
        polyvert: *const i32,
        polynormal: *const f64,
        graph: *const i32,
    }

    // SAFETY: res, endverts, obj, v1[3], v2[3] are valid pointers from caller
    unsafe {
        let mat = (*obj).mat.as_ptr();
        let pos = (*obj).pos.as_ptr();
        let mesh_ptr = &(*obj).data as *const _ as *const MeshData;
        let vert = (*mesh_ptr).vert;
        let polyvert = (*mesh_ptr).polyvert;

        // only one edge
        if dim == 2 {
            copy3(endverts, v2);
            sub3(res, v2, v1);
            crate::engine::engine_util_blas::mju_normalize3(res);
            return 1;
        }

        if dim == 1 {
            let v1_adr = *(*mesh_ptr).mpolymapadr.add(v1i as usize);
            let mut v1_num = *(*mesh_ptr).mpolymapnum.add(v1i as usize);
            if v1_num > MJ_MAX_POLYVERT {
                v1_num = MJ_MAX_POLYVERT;
            }

            // loop through all faces with vertex v1
            for i in 0..v1_num {
                let idx = *(*mesh_ptr).polymap.add((v1_adr + i) as usize);
                let adr = *(*mesh_ptr).polyvertadr.add(idx as usize);
                let nvert = *(*mesh_ptr).polyvertnum.add(idx as usize);
                // find previous vertex in polygon to form edge
                for j in 0..nvert {
                    if *polyvert.add((adr + j) as usize) == v1i {
                        let k = if j == 0 { nvert - 1 } else { j - 1 };
                        let v = vert.add(3 * *polyvert.add((adr + k) as usize) as usize);
                        globalcoord(endverts.add(3 * i as usize), mat, pos,
                                    *v.add(0) as f64, *v.add(1) as f64, *v.add(2) as f64);
                        sub3(res.add(3 * i as usize), endverts.add(3 * i as usize), v1);
                        crate::engine::engine_util_blas::mju_normalize3(res.add(3 * i as usize));
                        break;
                    }
                }
            }
            return v1_num;
        }
        0
    }
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
    const MJ_FACE_TOL: f64 = 0.99999872;
    // SAFETY: res[9], resind[3], mat[9], n[3] are valid pointers from caller
    unsafe {
        // list of box face normals
        let normals: [f64; 18] = [
            1.0, 0.0, 0.0,   -1.0,  0.0,  0.0,
            0.0, 1.0, 0.0,    0.0, -1.0,  0.0,
            0.0, 0.0, 1.0,    0.0,  0.0, -1.0,
        ];

        // get local coordinates of the normal (mat^T * n)
        let mut local_n: [f64; 3] = [0.0; 3];
        local_n[0] = *mat.add(0) * *n.add(0) + *mat.add(3) * *n.add(1) + *mat.add(6) * *n.add(2);
        local_n[1] = *mat.add(1) * *n.add(0) + *mat.add(4) * *n.add(1) + *mat.add(7) * *n.add(2);
        local_n[2] = *mat.add(2) * *n.add(0) + *mat.add(5) * *n.add(1) + *mat.add(8) * *n.add(2);
        let len = dot3(local_n.as_ptr(), local_n.as_ptr()).sqrt();
        scl3(local_n.as_mut_ptr(), local_n.as_ptr(), 1.0 / len);

        // determine if there is a side close to the normal
        for i in 0..6_i32 {
            if dot3(local_n.as_ptr(), normals.as_ptr().add(3 * i as usize)) > MJ_FACE_TOL {
                globalcoord(res, mat, std::ptr::null(), normals[3 * i as usize], normals[3 * i as usize + 1], normals[3 * i as usize + 2]);
                *resind.add(0) = i;
                return 1;
            }
        }
        0
    }
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
    // SAFETY: res[9], resind[3], obj, dir are valid pointers from caller
    unsafe {
        let mat = (*obj).mat.as_ptr();
        if dim == 3 {
            let mut c: usize = 0;
            let x = ((v1 & 1 != 0) && (v2 & 1 != 0) && (v3 & 1 != 0)) as i32
                  - ((v1 & 1 == 0) && (v2 & 1 == 0) && (v3 & 1 == 0)) as i32;
            let y = ((v1 & 2 != 0) && (v2 & 2 != 0) && (v3 & 2 != 0)) as i32
                  - ((v1 & 2 == 0) && (v2 & 2 == 0) && (v3 & 2 == 0)) as i32;
            let z = ((v1 & 4 != 0) && (v2 & 4 != 0) && (v3 & 4 != 0)) as i32
                  - ((v1 & 4 == 0) && (v2 & 4 == 0) && (v3 & 4 == 0)) as i32;
            globalcoord(res, mat, std::ptr::null(), x as f64, y as f64, z as f64);
            let sgn = x + y + z;
            if x != 0 { *resind.add(c) = 0; c += 1; }
            if y != 0 { *resind.add(c) = 2; c += 1; }
            if z != 0 { *resind.add(c) = 4; c += 1; }
            if sgn == -1 { *resind.add(0) += 1; }
            return if c == 1 { 1 } else { box_normals2(res, resind, mat, dir) };
        }

        if dim == 2 {
            let mut c: usize = 0;
            let x = ((v1 & 1 != 0) && (v2 & 1 != 0)) as i32
                  - ((v1 & 1 == 0) && (v2 & 1 == 0)) as i32;
            let y = ((v1 & 2 != 0) && (v2 & 2 != 0)) as i32
                  - ((v1 & 2 == 0) && (v2 & 2 == 0)) as i32;
            let z = ((v1 & 4 != 0) && (v2 & 4 != 0)) as i32
                  - ((v1 & 4 == 0) && (v2 & 4 == 0)) as i32;
            if x != 0 {
                globalcoord(res, mat, std::ptr::null(), x as f64, 0.0, 0.0);
                *resind.add(c) = if x > 0 { 0 } else { 1 };
                c += 1;
            }
            if y != 0 {
                globalcoord(res.add(3 * c), mat, std::ptr::null(), 0.0, y as f64, 0.0);
                *resind.add(c) = if y > 0 { 2 } else { 3 };
                c += 1;
            }
            if z != 0 {
                globalcoord(res.add(3), mat, std::ptr::null(), 0.0, 0.0, z as f64);
                *resind.add(c) = if z > 0 { 4 } else { 5 };
                c += 1;
            }
            return if c == 2 { 2 } else { box_normals2(res, resind, mat, dir) };
        }

        if dim == 1 {
            let x: f64 = if v1 & 1 != 0 { 1.0 } else { -1.0 };
            let y: f64 = if v1 & 2 != 0 { 1.0 } else { -1.0 };
            let z: f64 = if v1 & 4 != 0 { 1.0 } else { -1.0 };
            globalcoord(res.add(0), mat, std::ptr::null(), x, 0.0, 0.0);
            globalcoord(res.add(3), mat, std::ptr::null(), 0.0, y, 0.0);
            globalcoord(res.add(6), mat, std::ptr::null(), 0.0, 0.0, z);
            *resind.add(0) = if x > 0.0 { 0 } else { 1 };
            *resind.add(1) = if y > 0.0 { 2 } else { 3 };
            *resind.add(2) = if z > 0.0 { 4 } else { 5 };
            return 3;
        }
        0
    }
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
    // SAFETY: res[9], endverts[9], obj, v1[3], v2[3] are valid pointers from caller
    unsafe {
        let mat = (*obj).mat.as_ptr();
        let pos = (*obj).pos.as_ptr();
        let size = (*obj).size.as_ptr();

        if dim == 2 {
            copy3(endverts, v2);
            sub3(res, v2, v1);
            crate::engine::engine_util_blas::mju_normalize3(res);
            return 1;
        }

        // return 3 adjacent vertices
        if dim == 1 {
            let x: f64 = if v1i & 1 != 0 { *size.add(0) } else { -*size.add(0) };
            let y: f64 = if v1i & 2 != 0 { *size.add(1) } else { -*size.add(1) };
            let z: f64 = if v1i & 4 != 0 { *size.add(2) } else { -*size.add(2) };

            globalcoord(endverts, mat, pos, -x, y, z);
            sub3(res, endverts, v1);
            crate::engine::engine_util_blas::mju_normalize3(res);

            globalcoord(endverts.add(3), mat, pos, x, -y, z);
            sub3(res.add(3), endverts.add(3), v1);
            crate::engine::engine_util_blas::mju_normalize3(res.add(3));

            globalcoord(endverts.add(6), mat, pos, x, y, -z);
            sub3(res.add(6), endverts.add(6), v1);
            crate::engine::engine_util_blas::mju_normalize3(res.add(6));
            return 3;
        }
        0
    }
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
    // SAFETY: res[12], obj are valid pointers from caller
    unsafe {
        let mat = (*obj).mat.as_ptr();
        let pos = (*obj).pos.as_ptr();
        let size = (*obj).size.as_ptr();

        // compute global coordinates of the box face and face normal
        match idx {
            0 => {  // right
                globalcoord(res.add(0), mat, pos,  *size.add(0),  *size.add(1),  *size.add(2));
                globalcoord(res.add(3), mat, pos,  *size.add(0),  *size.add(1), -*size.add(2));
                globalcoord(res.add(6), mat, pos,  *size.add(0), -*size.add(1), -*size.add(2));
                globalcoord(res.add(9), mat, pos,  *size.add(0), -*size.add(1),  *size.add(2));
                4
            }
            1 => {  // left
                globalcoord(res.add(0), mat, pos, -*size.add(0),  *size.add(1), -*size.add(2));
                globalcoord(res.add(3), mat, pos, -*size.add(0),  *size.add(1),  *size.add(2));
                globalcoord(res.add(6), mat, pos, -*size.add(0), -*size.add(1),  *size.add(2));
                globalcoord(res.add(9), mat, pos, -*size.add(0), -*size.add(1), -*size.add(2));
                4
            }
            2 => {  // top
                globalcoord(res.add(0), mat, pos, -*size.add(0),  *size.add(1), -*size.add(2));
                globalcoord(res.add(3), mat, pos,  *size.add(0),  *size.add(1), -*size.add(2));
                globalcoord(res.add(6), mat, pos,  *size.add(0),  *size.add(1),  *size.add(2));
                globalcoord(res.add(9), mat, pos, -*size.add(0),  *size.add(1),  *size.add(2));
                4
            }
            3 => {  // bottom
                globalcoord(res.add(0), mat, pos, -*size.add(0), -*size.add(1),  *size.add(2));
                globalcoord(res.add(3), mat, pos,  *size.add(0), -*size.add(1),  *size.add(2));
                globalcoord(res.add(6), mat, pos,  *size.add(0), -*size.add(1), -*size.add(2));
                globalcoord(res.add(9), mat, pos, -*size.add(0), -*size.add(1), -*size.add(2));
                4
            }
            4 => {  // front
                globalcoord(res.add(0), mat, pos, -*size.add(0),  *size.add(1),  *size.add(2));
                globalcoord(res.add(3), mat, pos,  *size.add(0),  *size.add(1),  *size.add(2));
                globalcoord(res.add(6), mat, pos,  *size.add(0), -*size.add(1),  *size.add(2));
                globalcoord(res.add(9), mat, pos, -*size.add(0), -*size.add(1),  *size.add(2));
                4
            }
            5 => {  // back
                globalcoord(res.add(0), mat, pos,  *size.add(0),  *size.add(1), -*size.add(2));
                globalcoord(res.add(3), mat, pos, -*size.add(0),  *size.add(1), -*size.add(2));
                globalcoord(res.add(6), mat, pos, -*size.add(0), -*size.add(1), -*size.add(2));
                globalcoord(res.add(9), mat, pos,  *size.add(0), -*size.add(1), -*size.add(2));
                4
            }
            _ => 0,
        }
    }
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
    const MJ_MAX_POLYVERT: i32 = 150;

    #[repr(C)]
    struct MeshData {
        nvert: i32,
        mesh_polynum: i32,
        vert: *const f32,
        mpolymapadr: *const i32,
        mpolymapnum: *const i32,
        polymap: *const i32,
        polyvertadr: *const i32,
        polyvertnum: *const i32,
        polyvert: *const i32,
        polynormal: *const f64,
        graph: *const i32,
    }

    // SAFETY: res points to enough space, obj is a valid mjCCDObj with mesh data
    unsafe {
        let mat = (*obj).mat.as_ptr();
        let pos = (*obj).pos.as_ptr();

        let mesh_ptr = &(*obj).data as *const _ as *const MeshData;
        let adr = *(*mesh_ptr).polyvertadr.add(idx as usize);
        let mut nvert = *(*mesh_ptr).polyvertnum.add(idx as usize);
        if nvert > MJ_MAX_POLYVERT {
            nvert = MJ_MAX_POLYVERT;
        }
        let vert = (*mesh_ptr).vert;
        let polyvert = (*mesh_ptr).polyvert.add(adr as usize);

        let mut j: i32 = 0;
        for i in (0..nvert).rev() {
            let v = vert.add(3 * *polyvert.add(i as usize) as usize);
            globalcoord(res.add(3 * j as usize), mat, pos, *v.add(0) as f64, *v.add(1) as f64, *v.add(2) as f64);
            j += 1;
        }
        nvert
    }
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

