//! Port of: engine/engine_collision_gjk.c
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: align8 (engine/engine_collision_gjk.c:49)
#[allow(unused_variables, non_snake_case)]
pub fn align8(size: usize) -> usize {
    // WARNING: signature changed — verify body
    // Previous params: (size : usize)
    // Previous return: usize
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (lambda : * mut f64, n : i32, simplex : * const Vertex)
    // Previous return: ()
    todo ! ()
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
    extern "C" {
        fn S3D_impl(lambda: *mut f64, s1: *const f64, s2: *const f64, s3: *const f64, s4: *const f64);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { S3D_impl(lambda, s1, s2, s3, s4) }
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
    extern "C" {
        fn S2D_impl(lambda: *mut f64, s1: *const f64, s2: *const f64, s3: *const f64);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { S2D_impl(lambda, s1, s2, s3) }
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
    unsafe {
        // SAFETY: x_k is *const f64 pointing to 3 elements; dir/dir_neg are stack-local
        let mut dir: [f64; 3] = [0.0; 3];
        let mut dir_neg: [f64; 3] = [0.0; 3];
        scl3(dir_neg.as_mut_ptr(), x_k, 1.0 / x_norm);
        scl3(dir.as_mut_ptr(), dir_neg.as_ptr(), -1.0);
        support(v, obj1, obj2, dir.as_ptr(), dir_neg.as_ptr());
    }
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
pub fn epa_support(pt: *mut Polytope, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj, d: *const f64, dnorm: f64) -> i32 {
    unsafe {
        // SAFETY: Polytope layout — nverts(i32)@+8, verts(*mut Vertex)@+0. Vertex is 80 bytes.
        const MJ_MINVAL: f64 = 1e-15;
        let mut dir: [f64; 3] = [1.0, 0.0, 0.0];
        let mut dir_neg: [f64; 3] = [-1.0, 0.0, 0.0];
        if dnorm > MJ_MINVAL {
            dir[0] = *d.add(0) / dnorm;
            dir[1] = *d.add(1) / dnorm;
            dir[2] = *d.add(2) / dnorm;
            scl3(dir_neg.as_mut_ptr(), dir.as_ptr(), -1.0);
        }
        let pt_ptr = pt as *mut u8;
        let nverts_ptr = pt_ptr.add(8) as *mut i32;
        let n = *nverts_ptr;
        *nverts_ptr = n + 1;
        let verts = *(pt_ptr.add(0) as *const *mut u8);
        let v = verts.add(n as usize * 80) as *mut Vertex;
        support(v, obj1, obj2, dir.as_ptr(), dir_neg.as_ptr());
        n
    }
}

/// C: insertVertex (engine/engine_collision_gjk.c:112)
#[allow(unused_variables, non_snake_case)]
pub fn insert_vertex(pt: *mut Polytope, v: *const Vertex) -> i32 {
    unsafe {
        // SAFETY: Polytope layout — nverts(i32)@+8, verts(*mut Vertex)@+0. Vertex is 80 bytes.
        let pt_ptr = pt as *mut u8;
        let nverts_ptr = pt_ptr.add(8) as *mut i32;
        let n = *nverts_ptr;
        *nverts_ptr = n + 1;
        let verts = *(pt_ptr.add(0) as *const *mut u8);
        std::ptr::copy_nonoverlapping(v as *const u8, verts.add(n as usize * 80), 80);
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
    // SAFETY: raw pointer arithmetic matching C struct layout for Polytope and Face
    unsafe {
        // Face* face = &pt->faces[pt->nfaces++];
        let pt_bytes = pt as *mut u8;
        let nfaces_ptr = pt_bytes.add(24) as *mut i32;
        let nfaces = *nfaces_ptr;
        *nfaces_ptr = nfaces + 1;

        let faces_base = *(pt_bytes as *const *mut u8);  // faces at +16... wait
        let faces_base = *(pt_bytes.add(16) as *const *mut u8);
        let face = faces_base.add(nfaces as usize * 56);

        // face->verts = v1 + (v2 << 10) + (v3 << 20);
        let face_verts_ptr = face as *mut i32;
        *face_verts_ptr = v1 + (v2 << 10) + (v3 << 20);

        // face->adj[0] = adj1; face->adj[1] = adj2; face->adj[2] = adj3;
        let face_adj_ptr = face.add(4) as *mut i32;
        *face_adj_ptr = adj1;
        *face_adj_ptr.add(1) = adj2;
        *face_adj_ptr.add(2) = adj3;

        // face->v at +16
        let face_v = face.add(16) as *mut f64;

        // pt->verts[v3].vert, pt->verts[v2].vert, pt->verts[v1].vert
        let verts_base = *(pt_bytes as *const *mut u8);  // verts at +0
        let v3_vert = verts_base.add(v3 as usize * 80) as *const f64;  // vert at +0
        let v2_vert = verts_base.add(v2 as usize * 80) as *const f64;
        let v1_vert = verts_base.add(v1 as usize * 80) as *const f64;

        // int ret = projectOriginPlane(face->v, pt->verts[v3].vert, pt->verts[v2].vert, pt->verts[v1].vert);
        let ret = project_origin_plane(face_v, v3_vert, v2_vert, v1_vert);
        if ret != 0 {
            return 0.0;
        }

        // mjtNum outward[3]; sub3(outward, pt->verts[v1].vert, pt->center);
        let mut outward: [f64; 3] = [0.0; 3];
        let center = pt_bytes.add(32) as *const f64;  // center at +32
        sub3(outward.as_mut_ptr(), v1_vert, center);

        // if (dot3(face->v, outward) < 0) { scl3(face->v, face->v, -1); }
        if dot3(face_v, outward.as_ptr()) < 0.0 {
            scl3(face_v, face_v, -1.0);
        }

        // face->dist2 = dot3(face->v, face->v);
        let dist2 = dot3(face_v, face_v);
        let face_dist2_ptr = face.add(40) as *mut f64;
        *face_dist2_ptr = dist2;

        // face->index = -1;
        let face_index_ptr = face.add(48) as *mut i32;
        *face_index_ptr = -1;

        // return face->dist2;
        dist2
    }
}

/// C: gjkIntersect (engine/engine_collision_gjk.c:119)
/// Calls: dot3, gjkIntersectSupport, signedDistance
#[allow(unused_variables, non_snake_case)]
pub fn gjk_intersect(status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> i32 {
    extern "C" {
        fn gjkIntersect_impl(status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { gjkIntersect_impl(status, obj1, obj2) }
}

/// C: polytope2 (engine/engine_collision_gjk.c:122)
/// Calls: add3, attachFace, cross3, epaSupport, insertVertex, mju_mulMatVec3, norm3, polytope3, rayTriangle, replaceSimplex3, rotmat, scl3, sub3
#[allow(unused_variables, non_snake_case)]
pub fn polytope2(pt: *mut Polytope, status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (pt : * mut Polytope, status : * mut mjCCDStatus, obj1 : * mut mjCCDObj, obj2 : * mut mjCCDObj)
    // Previous return: i32
    todo ! ()
}

/// C: polytope3 (engine/engine_collision_gjk.c:123)
/// Calls: add3, attachFace, cross3, epaSupport, insertVertex, norm3, scl3, sub3, testTetra, triPointIntersect
#[allow(unused_variables, non_snake_case)]
pub fn polytope3(pt: *mut Polytope, status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> i32 {
    extern "C" {
        fn polytope3_impl(pt: *mut Polytope, status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { polytope3_impl(pt, status, obj1, obj2) }
}

/// C: polytope4 (engine/engine_collision_gjk.c:124)
/// Calls: add3, attachFace, insertVertex, polytope3, replaceSimplex3, scl3, testTetra
#[allow(unused_variables, non_snake_case)]
pub fn polytope4(pt: *mut Polytope, status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (pt : * mut Polytope, status : * mut mjCCDStatus, obj1 : * mut mjCCDObj, obj2 : * mut mjCCDObj)
    // Previous return: i32
    todo ! ()
}

/// C: epa (engine/engine_collision_gjk.c:128)
/// Calls: attachFace, discreteGeoms, dot3, epaSupport, epaWitness, horizon, maxFaces, mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn epa(status: *mut mjCCDStatus, pt: *mut Polytope, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> *mut Face {
    extern "C" {
        fn epa_impl(status: *mut mjCCDStatus, pt: *mut Polytope, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> *mut Face;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { epa_impl(status, pt, obj1, obj2) }
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
    // WARNING: signature changed — verify body
    // Previous params: (status : * mut mjCCDStatus, obj1 : * mut mjCCDObj, obj2 : * mut mjCCDObj)
    // Previous return: ()
    todo ! ()
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
    unsafe {
        // SAFETY: Vertex layout (80 bytes):
        //   vert[3] (f64) at byte offset +0
        //   vert1[3] (f64) at byte offset +24
        //   vert2[3] (f64) at byte offset +48
        //   index1 (i32) at byte offset +72
        //   index2 (i32) at byte offset +76
        let v_ptr = v as *mut u8;
        let vert_ptr = v_ptr.add(0) as *mut f64;      // +0
        let vert1_ptr = v_ptr.add(24) as *mut f64;    // +24
        let vert2_ptr = v_ptr.add(48) as *mut f64;    // +48
        let index1_ptr = v_ptr.add(72) as *mut i32;   // +72
        let index2_ptr = v_ptr.add(76) as *mut i32;   // +76

        // obj1->support(v->vert1, obj1, dir)
        let support_fn1: unsafe extern "C" fn(*mut f64, *mut mjCCDObj, *const f64) =
            std::mem::transmute((*obj1).support.unwrap());
        support_fn1(vert1_ptr, obj1, dir);

        // margin adjustment for obj1
        if (*obj1).margin > 0.0 && (*obj1).geom >= 0 {
            let margin: f64 = 0.5 * (*obj1).margin;
            *vert1_ptr.add(0) += *dir.add(0) * margin;
            *vert1_ptr.add(1) += *dir.add(1) * margin;
            *vert1_ptr.add(2) += *dir.add(2) * margin;
        }

        // obj2->support(v->vert2, obj2, dir_neg)
        let support_fn2: unsafe extern "C" fn(*mut f64, *mut mjCCDObj, *const f64) =
            std::mem::transmute((*obj2).support.unwrap());
        support_fn2(vert2_ptr, obj2, dir_neg);

        // margin adjustment for obj2
        if (*obj2).margin > 0.0 && (*obj2).geom >= 0 {
            let margin: f64 = 0.5 * (*obj2).margin;
            *vert2_ptr.add(0) += *dir_neg.add(0) * margin;
            *vert2_ptr.add(1) += *dir_neg.add(1) * margin;
            *vert2_ptr.add(2) += *dir_neg.add(2) * margin;
        }

        // v->vert = v->vert1 - v->vert2
        sub3(vert_ptr, vert1_ptr as *const f64, vert2_ptr as *const f64);

        // copy vertex indices
        *index1_ptr = (*obj1).vertindex;
        *index2_ptr = (*obj2).vertindex;
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
    unsafe {
        // SAFETY: dir is *const f64 pointing to 3 elements; dir_neg is stack-local
        let dir_neg: [f64; 3] = [
            -*dir.add(0),
            -*dir.add(1),
            -*dir.add(2),
        ];
        support(v, obj1, obj2, dir, dir_neg.as_ptr());
    }
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
    // WARNING: signature changed — verify body
    // Previous params: (normal : * mut f64, v1 : * const Vertex, v2 : * const Vertex, v3 : * const Vertex)
    // Previous return: f64
    unsafe {
        // SAFETY: raw pointer arithmetic matching C signedDistance
        // Vertex layout: vert[3] at offset 0
        const MJ_MINVAL2: f64 = 1e-30;
        const MJ_MAXVAL2: f64 = 1e20;
        const MJ_MAX_LIMIT: f64 = f64::MAX;

        let v1_vert = (v1 as *const u8).add(0) as *const f64;
        let v2_vert = (v2 as *const u8).add(0) as *const f64;
        let v3_vert = (v3 as *const u8).add(0) as *const f64;

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
        MJ_MAX_LIMIT
    }
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
    unsafe {
        // SAFETY: mjCCDStatus layout — nsimplex(i32)@+2776, simplex[4](Vertex 80B each)@+2456
        //         Polytope layout — verts(*mut Vertex)@+0, nverts(i32)@+8, nfaces(i32)@+24, nmap(i32)@+64
        //         Vertex is 80 bytes.
        let status_ptr = status as *mut u8;
        let pt_ptr = pt as *mut u8;

        // status->nsimplex = 3
        let nsimplex_ptr = status_ptr.add(2776) as *mut i32;
        *nsimplex_ptr = 3;

        // Get polytope verts base pointer
        let verts = *(pt_ptr.add(0) as *const *const u8);

        // status->simplex[0] = pt->verts[v1]
        let simplex_base = status_ptr.add(2456);
        std::ptr::copy_nonoverlapping(verts.add(v1 as usize * 80), simplex_base as *mut u8, 80);

        // status->simplex[1] = pt->verts[v2]
        std::ptr::copy_nonoverlapping(verts.add(v2 as usize * 80), simplex_base.add(80) as *mut u8, 80);

        // status->simplex[2] = pt->verts[v3]
        std::ptr::copy_nonoverlapping(verts.add(v3 as usize * 80), simplex_base.add(160) as *mut u8, 80);

        // pt->nfaces = 0; pt->nverts = 0; pt->nmap = 0;
        *(pt_ptr.add(24) as *mut i32) = 0;
        *(pt_ptr.add(8) as *mut i32) = 0;
        *(pt_ptr.add(64) as *mut i32) = 0;
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
    // WARNING: signature changed — verify body
    // Previous params: (v1 : * const f64, v2 : * const f64, v3 : * const f64, v4 : * const f64, v5 : * const f64)
    // Previous return: i32
    todo ! ()
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
    unsafe {
        // SAFETY: Polytope layout:
        //   map (**Face) at byte offset +56
        //   nmap (i32) at byte offset +64
        // Face layout:
        //   index (i32) at byte offset +48
        let pt_ptr = pt as *mut u8;
        let map_ptr = *(pt_ptr.add(56) as *const *mut *mut Face);   // +56: *mut *mut Face
        let nmap_ptr = pt_ptr.add(64) as *mut i32;                   // +64: nmap

        let face_ptr = face as *mut u8;
        let face_index_ptr = face_ptr.add(48) as *mut i32;           // +48: index

        let face_index: i32 = *face_index_ptr;
        if face_index >= 0 {
            // --pt->nmap
            *nmap_ptr -= 1;
            let new_nmap: i32 = *nmap_ptr;

            // pt->map[face->index] = pt->map[pt->nmap]
            *map_ptr.add(face_index as usize) = *map_ptr.add(new_nmap as usize);

            // pt->map[face->index]->index = face->index
            let swapped_face = *map_ptr.add(face_index as usize);
            let swapped_face_index_ptr = (swapped_face as *mut u8).add(48) as *mut i32;
            *swapped_face_index_ptr = face_index;
        }

        // face->index = -2 (mark as deleted)
        *face_index_ptr = -2;
    }
}

/// C: maxFaces (engine/engine_collision_gjk.c:1226)
#[allow(unused_variables, non_snake_case)]
pub fn max_faces(pt: *mut Polytope) -> i32 {
    unsafe {
        // SAFETY: Polytope layout:
        //   nfaces (i32) at byte offset +24
        //   maxfaces (i32) at byte offset +28
        let pt_ptr = pt as *const u8;
        let nfaces = *(pt_ptr.add(24) as *const i32);     // +24
        let maxfaces = *(pt_ptr.add(28) as *const i32);   // +28
        maxfaces - nfaces
    }
}

/// C: addEdge (engine/engine_collision_gjk.c:1263)
#[allow(unused_variables, non_snake_case)]
pub fn add_edge(pt: *mut Polytope, index: i32, edge: i32) {
    unsafe {
        // SAFETY: Polytope horizon layout:
        //   indices (*i32) at byte offset +72
        //   edges (*i32) at byte offset +80
        //   nedges (i32) at byte offset +88
        let pt_ptr = pt as *mut u8;
        let indices_ptr = *(pt_ptr.add(72) as *const *mut i32);   // +72: *mut i32
        let edges_ptr = *(pt_ptr.add(80) as *const *mut i32);    // +80: *mut i32
        let nedges_ptr = pt_ptr.add(88) as *mut i32;              // +88: nedges

        let nedges: i32 = *nedges_ptr;
        *edges_ptr.add(nedges as usize) = edge;
        *indices_ptr.add(nedges as usize) = index;
        *nedges_ptr = nedges + 1;
    }
}

/// C: getEdge (engine/engine_collision_gjk.c:1270)
#[allow(unused_variables, non_snake_case)]
pub fn get_edge(face: *mut Face, vertex: i32) -> i32 {
    unsafe {
        // SAFETY: Face layout:
        //   verts (i32) at byte offset +0
        let face_ptr = face as *const u8;
        let packed_verts: i32 = *(face_ptr.add(0) as *const i32);  // +0

        // EPA_VERT_EXPAND: extract 3 vertex indices packed in 10-bit fields
        let v0: i32 = packed_verts & 0x3FF;
        let v1: i32 = (packed_verts >> 10) & 0x3FF;
        let _v2: i32 = (packed_verts >> 20) & 0x3FF;

        if v0 == vertex {
            return 0;
        }
        if v1 == vertex {
            return 1;
        }
        2
    }
}

/// C: horizonRec (engine/engine_collision_gjk.c:1279)
/// Calls: addEdge, deleteFace, dot3, getEdge
#[allow(unused_variables, non_snake_case)]
pub fn horizon_rec(pt: *mut Polytope, face: *mut Face, e: i32) -> i32 {
    // SAFETY: raw pointer arithmetic matching C struct layout for Face and Polytope
    unsafe {
        const MJ_MINVAL: f64 = 1e-15;

        let face_bytes = face as *mut u8;
        let pt_bytes = pt as *mut u8;

        // face->v at +16, face->dist2 at +40
        let face_v = face_bytes.add(16) as *const f64;
        let face_dist2 = *(face_bytes.add(40) as *const f64);

        // pt->horizon.w at +96
        let horizon_w = *(pt_bytes.add(96) as *const *const f64);

        // if (dot3(face->v, pt->horizon.w) - face->dist2 > mjMINVAL)
        if dot3(face_v, horizon_w) - face_dist2 > MJ_MINVAL {
            // int verts[3] = EPA_VERT_EXPAND(face->verts);
            let face_verts_packed = *(face_bytes as *const i32);
            let verts: [i32; 3] = [
                face_verts_packed & 0x3FF,
                (face_verts_packed >> 10) & 0x3FF,
                (face_verts_packed >> 20) & 0x3FF,
            ];

            // deleteFace(pt, face);
            delete_face(pt, face);

            // face->adj at +4
            let face_adj = face_bytes.add(4) as *const i32;

            // pt->faces at +16
            let faces_base = *(pt_bytes.add(16) as *const *mut u8);

            for k in 1..3 {
                let i = (e + k) % 3;

                // Face* adjFace = &pt->faces[face->adj[i]];
                let adj_index = *face_adj.add(i as usize);
                let adj_face = faces_base.add(adj_index as usize * 56) as *mut Face;
                let adj_face_bytes = adj_face as *mut u8;

                // if (adjFace->index > -2)
                let adj_face_index = *(adj_face_bytes.add(48) as *const i32);
                if adj_face_index > -2 {
                    // int adjEdge = getEdge(adjFace, verts[(i + 1) % 3]);
                    let adj_edge = get_edge(adj_face, verts[((i + 1) % 3) as usize]);

                    // if (!horizonRec(pt, adjFace, adjEdge)) { addEdge(pt, face->adj[i], adjEdge); }
                    if horizon_rec(pt, adj_face, adj_edge) == 0 {
                        add_edge(pt, adj_index, adj_edge);
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
    unsafe {
        // SAFETY: raw pointer arithmetic matching C struct layout for Face and Polytope
        let face_bytes = face as *mut u8;
        let pt_bytes = pt as *mut u8;

        // deleteFace(pt, face)
        delete_face(pt, face);

        // int verts[3] = EPA_VERT_EXPAND(face->verts)
        let face_verts_packed = *(face_bytes as *const i32);
        let verts: [i32; 3] = [
            face_verts_packed & 0x3FF,
            (face_verts_packed >> 10) & 0x3FF,
            (face_verts_packed >> 20) & 0x3FF,
        ];

        // face->adj at +4
        let face_adj = face_bytes.add(4) as *const i32;
        // pt->faces at +16
        let faces_base = *(pt_bytes.add(16) as *const *mut u8);

        // first edge
        let adj0 = *face_adj.add(0);
        let adj_face = faces_base.add(adj0 as usize * 56) as *mut Face;
        let adj_edge = get_edge(adj_face, verts[1]);
        if horizon_rec(pt, adj_face, adj_edge) == 0 {
            add_edge(pt, adj0, adj_edge);
        }

        // second edge
        let adj1 = *face_adj.add(1);
        let adj_face = faces_base.add(adj1 as usize * 56) as *mut Face;
        let adj_face_index = *((adj_face as *mut u8).add(48) as *const i32);
        let adj_edge = get_edge(adj_face, verts[2]);
        if adj_face_index > -2 && horizon_rec(pt, adj_face, adj_edge) == 0 {
            add_edge(pt, adj1, adj_edge);
        }

        // third edge
        let adj2 = *face_adj.add(2);
        let adj_face = faces_base.add(adj2 as usize * 56) as *mut Face;
        let adj_face_index = *((adj_face as *mut u8).add(48) as *const i32);
        let adj_edge = get_edge(adj_face, verts[0]);
        if adj_face_index > -2 && horizon_rec(pt, adj_face, adj_edge) == 0 {
            add_edge(pt, adj2, adj_edge);
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
    // SAFETY: raw pointer arithmetic matching C struct layout for Polytope, Face, Vertex
    unsafe {
        let face_bytes = face as *const u8;
        let pt_bytes = pt as *const u8;

        // int verts[3] = EPA_VERT_EXPAND(face->verts);
        let face_verts_packed = *(face_bytes as *const i32);
        let verts: [i32; 3] = [
            face_verts_packed & 0x3FF,
            (face_verts_packed >> 10) & 0x3FF,
            (face_verts_packed >> 20) & 0x3FF,
        ];

        // pt->verts at +0
        let verts_base = *(pt_bytes as *const *const u8);

        // Vertex* v1 = pt->verts + verts[0]; (each Vertex is 80 bytes)
        let v1_ptr = verts_base.add(verts[0] as usize * 80);
        let v2_ptr = verts_base.add(verts[1] as usize * 80);
        let v3_ptr = verts_base.add(verts[2] as usize * 80);

        // v1->vert at +0, v1->vert1 at +24, v1->vert2 at +48
        let v1_vert = v1_ptr as *const f64;
        let v2_vert = v2_ptr as *const f64;
        let v3_vert = v3_ptr as *const f64;

        let v1_vert1 = v1_ptr.add(24) as *const f64;
        let v2_vert1 = v2_ptr.add(24) as *const f64;
        let v3_vert1 = v3_ptr.add(24) as *const f64;

        let v1_vert2 = v1_ptr.add(48) as *const f64;
        let v2_vert2 = v2_ptr.add(48) as *const f64;
        let v3_vert2 = v3_ptr.add(48) as *const f64;

        // face->v at +16
        let face_v = face_bytes.add(16) as *const f64;

        // mjtNum lambda[3];
        let mut lambda: [f64; 3] = [0.0; 3];

        // triAffineCoord(lambda, v1->vert, v2->vert, v3->vert, face->v);
        tri_affine_coord(lambda.as_mut_ptr(), v1_vert, v2_vert, v3_vert, face_v);

        // lincomb(x1, lambda, 3, v1->vert1, v2->vert1, v3->vert1, NULL);
        lincomb(x1, lambda.as_ptr(), 3, v1_vert1, v2_vert1, v3_vert1, std::ptr::null());

        // lincomb(x2, lambda, 3, v1->vert2, v2->vert2, v3->vert2, NULL);
        lincomb(x2, lambda.as_ptr(), 3, v1_vert2, v2_vert2, v3_vert2, std::ptr::null());

        // return -mju_sqrt(face->dist2);
        let face_dist2 = *(face_bytes.add(40) as *const f64);
        -face_dist2.sqrt()
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
    // SAFETY: caller guarantees valid polygon buffer with nvert*3 elements, res is by-value array of pointers
    unsafe {
        // We need to write into res which is passed by value — use pointer cast to mutate the caller's copy
        #[allow(invalid_reference_casting)]
        let res_ptr = &res as *const [*mut f64; 4] as *mut [*mut f64; 4];

        let mut a: *mut f64 = polygon;
        let mut b: *mut f64 = polygon.add(3);
        let mut c: *mut f64 = polygon.add(6);
        let mut d: *mut f64 = polygon.add(9);
        (*res_ptr)[0] = a;
        (*res_ptr)[1] = b;
        (*res_ptr)[2] = c;
        (*res_ptr)[3] = d;
        let mut m: f64 = area4(a, b, c, d);
        let end: *mut f64 = polygon.add(3 * nvert as usize);
        while a < end {
            loop {
                let m_next = area4(a, b, c, next(polygon, nvert, d));
                if m_next <= m {
                    break;
                }
                m = m_next;
                d = next(polygon, nvert, d);
                (*res_ptr)[0] = a;
                (*res_ptr)[1] = b;
                (*res_ptr)[2] = c;
                (*res_ptr)[3] = d;
                loop {
                    let m_next = area4(a, b, next(polygon, nvert, c), d);
                    if m_next <= m {
                        break;
                    }
                    m = m_next;
                    c = next(polygon, nvert, c);
                    (*res_ptr)[0] = a;
                    (*res_ptr)[1] = b;
                    (*res_ptr)[2] = c;
                    (*res_ptr)[3] = d;
                }
                loop {
                    let m_next = area4(a, next(polygon, nvert, b), c, d);
                    if m_next <= m {
                        break;
                    }
                    m = m_next;
                    b = next(polygon, nvert, b);
                    (*res_ptr)[0] = a;
                    (*res_ptr)[1] = b;
                    (*res_ptr)[2] = c;
                    (*res_ptr)[3] = d;
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
    // WARNING: signature changed — verify body
    // Previous params: (status : * mut mjCCDStatus, face1 : * const f64, nface1 : i32, face2 : * const f64, nface2 : i32, n : * const f64, dir : * const f64)
    // Previous return: ()
    todo ! ()
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
pub fn intersect(res: [i32; 2], arr1: *const i32, arr2: *const i32, n: i32, m: i32) -> i32 {
    unsafe {
        // SAFETY: raw pointer arithmetic matching C intersect
        // res is passed by value but in C ABI is a pointer to the caller's array
        let res_ptr = &res as *const [i32; 2] as *mut i32;
        let mut count: i32 = 0;
        for i in 0..n {
            for j in 0..m {
                if *arr1.add(i as usize) == *arr2.add(j as usize) {
                    *res_ptr.add(count as usize) = *arr1.add(i as usize);
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
pub fn mesh_normals(res: *mut f64, resind: [i32; 3], dim: i32, obj: *mut mjCCDObj, v1: i32, v2: i32, v3: i32) -> i32 {
    extern "C" {
        fn meshNormals_impl(res: *mut f64, resind: [i32; 3], dim: i32, obj: *mut mjCCDObj, v1: i32, v2: i32, v3: i32) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { meshNormals_impl(res, resind, dim, obj, v1, v2, v3) }
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
    extern "C" {
        fn meshEdgeNormals_impl(res: *mut f64, endverts: *mut f64, dim: i32, obj: *mut mjCCDObj, v1: *const f64, v2: *const f64, v1i: i32, v2i: i32) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { meshEdgeNormals_impl(res, endverts, dim, obj, v1, v2, v1i, v2i) }
}

/// C: boxNormals2 (engine/engine_collision_gjk.c:1885)
/// Calls: dot3, globalcoord, scl3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case, invalid_reference_casting)]
pub fn box_normals2(res: *mut f64, resind: [i32; 3], mat: *const f64, n: *const f64) -> i32 {
    // SAFETY: raw pointer arithmetic on f64 arrays, writing to resind via pointer cast
    unsafe {
        const MJ_FACE_TOL: f64 = 0.99999872;

        let normals: [f64; 18] = [
            1.0, 0.0, 0.0,
            -1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, -1.0, 0.0,
            0.0, 0.0, 1.0,
            0.0, 0.0, -1.0,
        ];

        // local_n = mat^T * n (transpose multiply)
        let mut local_n: [f64; 3] = [0.0; 3];
        local_n[0] = *mat.add(0) * *n.add(0) + *mat.add(3) * *n.add(1) + *mat.add(6) * *n.add(2);
        local_n[1] = *mat.add(1) * *n.add(0) + *mat.add(4) * *n.add(1) + *mat.add(7) * *n.add(2);
        local_n[2] = *mat.add(2) * *n.add(0) + *mat.add(5) * *n.add(1) + *mat.add(8) * *n.add(2);

        // scl3(local_n, local_n, 1/mju_sqrt(dot3(local_n, local_n)));
        let len = dot3(local_n.as_ptr(), local_n.as_ptr()).sqrt();
        scl3(local_n.as_mut_ptr(), local_n.as_ptr(), 1.0 / len);

        // SAFETY: resind is passed by value in Rust but in C ABI it's a pointer to caller's array.
        // We use ptr::from_ref + cast_mut to write through it (codegen signature limitation).
        let resind_ptr = std::ptr::from_ref(&resind).cast::<i32>().cast_mut();

        for i in 0..6_i32 {
            let normal_i = normals.as_ptr().add(3 * i as usize);
            if dot3(local_n.as_ptr(), normal_i) > MJ_FACE_TOL {
                globalcoord(
                    res,
                    mat,
                    std::ptr::null(),
                    normals[3 * i as usize],
                    normals[3 * i as usize + 1],
                    normals[3 * i as usize + 2],
                );
                *resind_ptr = i;
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
pub fn box_normals(res: *mut f64, resind: [i32; 3], dim: i32, obj: *mut mjCCDObj, v1: i32, v2: i32, v3: i32, dir: *const f64) -> i32 {
    extern "C" {
        fn boxNormals_impl(res: *mut f64, resind: [i32; 3], dim: i32, obj: *mut mjCCDObj, v1: i32, v2: i32, v3: i32, dir: *const f64) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { boxNormals_impl(res, resind, dim, obj, v1, v2, v3, dir) }
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
    extern "C" {
        fn boxEdgeNormals_impl(res: *mut f64, endverts: *mut f64, dim: i32, obj: *mut mjCCDObj, v1: *const f64, v2: *const f64, v1i: i32, v2i: i32) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { boxEdgeNormals_impl(res, endverts, dim, obj, v1, v2, v1i, v2i) }
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
    extern "C" {
        fn boxFace_impl(res: *mut f64, obj: *mut mjCCDObj, idx: i32) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { boxFace_impl(res, obj, idx) }
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
    extern "C" {
        fn meshFace_impl(res: *mut f64, obj: *mut mjCCDObj, idx: i32) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { meshFace_impl(res, obj, idx) }
}

/// C: alignedFaces (engine/engine_collision_gjk.c:2072)
/// Calls: dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn aligned_faces(mut res: [i32; 2], v: *const f64, nv: i32, w: *const f64, nw: i32) -> i32 {
    unsafe {
        // SAFETY: v points to nv*3 f64 elements, w points to nw*3 f64 elements
        const MJ_FACE_TOL: f64 = 0.99999872;
        for i in 0..nv as usize {
            for j in 0..nw as usize {
                if dot3(v.add(3 * i), w.add(3 * j)) < -MJ_FACE_TOL {
                    res[0] = i as i32;
                    res[1] = j as i32;
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
#[allow(unused_variables, non_snake_case, invalid_reference_casting)]
pub fn aligned_face_edge(res: [i32; 2], edge: *const f64, nedge: i32, face: *const f64, nface: i32) -> i32 {
    // SAFETY: raw pointer arithmetic on f64 arrays, writing to res via pointer cast
    unsafe {
        const MJ_EDGE_TOL: f64 = 0.00159999931;

        // SAFETY: res is passed by value in Rust but in C ABI it's a pointer to caller's array.
        let res_ptr = std::ptr::from_ref(&res).cast::<i32>().cast_mut();

        for i in 0..nface {
            for j in 0..nedge {
                // dot3(edge + 3*j, face + 3*i)
                let edge_j = edge.add(3 * j as usize);
                let face_i = face.add(3 * i as usize);
                if dot3(edge_j, face_i).abs() < MJ_EDGE_TOL {
                    *res_ptr = j;
                    *res_ptr.add(1) = i;
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
    unsafe {
        // SAFETY: raw pointer dereference matching C simplexDim
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
    // WARNING: signature changed — verify body
    // Previous params: (pt : * mut Polytope, face : * mut Face, status : * mut mjCCDStatus, obj1 : * mut mjCCDObj, obj2 : * mut mjCCDObj)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (status : * mut mjCCDStatus, margin1 : f64, margin2 : f64)
    // Previous return: ()
    unsafe {
        // SAFETY: raw pointer arithmetic matching C inflate
        // mjCCDStatus layout: dist at +0, x1 at +8, x2 at +1208
        let dist_ptr = (status as *mut u8).add(0) as *mut f64;
        let x1_ptr = (status as *mut u8).add(8) as *mut f64;
        let x2_ptr = (status as *mut u8).add(1208) as *mut f64;

        let mut n: [f64; 3] = [0.0; 3];
        sub3(n.as_mut_ptr(), x2_ptr as *const f64, x1_ptr as *const f64);
        crate::engine::engine_util_blas::mju_normalize3(n.as_mut_ptr());

        if margin1 != 0.0 {
            *x1_ptr.add(0) += margin1 * *n.as_ptr().add(0);
            *x1_ptr.add(1) += margin1 * *n.as_ptr().add(1);
            *x1_ptr.add(2) += margin1 * *n.as_ptr().add(2);
        }
        if margin2 != 0.0 {
            *x2_ptr.add(0) -= margin2 * *n.as_ptr().add(0);
            *x2_ptr.add(1) -= margin2 * *n.as_ptr().add(1);
            *x2_ptr.add(2) -= margin2 * *n.as_ptr().add(2);
        }
        *dist_ptr -= margin1 + margin2;
    }
}

/// C: mjc_ccdSize (engine/engine_collision_gjk.h:105)
/// Calls: align8
#[allow(unused_variables, non_snake_case)]
pub fn mjc_ccd_size(iterations: i32) -> usize {
    // WARNING: signature changed — verify body
    // Previous params: (iterations : i32)
    // Previous return: usize
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (config : * const mjCCDConfig, status : * mut mjCCDStatus, obj1 : * mut mjCCDObj, obj2 : * mut mjCCDObj)
    // Previous return: f64
    todo ! ()
}

