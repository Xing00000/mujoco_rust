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
    // WARNING: signature changed — verify body
    // Previous params: (lambda : * mut f64, s1 : * const f64, s2 : * const f64, s3 : * const f64, s4 : * const f64)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (lambda : * mut f64, s1 : * const f64, s2 : * const f64, s3 : * const f64)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (lambda : * mut f64, s1 : * const f64, s2 : * const f64)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (v : * mut Vertex, obj1 : * mut mjCCDObj, obj2 : * mut mjCCDObj, x_k : * const f64, x_norm : f64)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (pt : * mut Polytope, obj1 : * mut mjCCDObj, obj2 : * mut mjCCDObj, d : * const f64, dnorm : f64)
    // Previous return: i32
    todo ! ()
}

/// C: insertVertex (engine/engine_collision_gjk.c:112)
#[allow(unused_variables, non_snake_case)]
pub fn insert_vertex(pt: *mut Polytope, v: *const Vertex) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (pt : * mut Polytope, v : * const Vertex)
    // Previous return: i32
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (pt : * mut Polytope, v1 : i32, v2 : i32, v3 : i32, adj1 : i32, adj2 : i32, adj3 : i32)
    // Previous return: f64
    todo ! ()
}

/// C: gjkIntersect (engine/engine_collision_gjk.c:119)
/// Calls: dot3, gjkIntersectSupport, signedDistance
#[allow(unused_variables, non_snake_case)]
pub fn gjk_intersect(status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (status : * mut mjCCDStatus, obj1 : * mut mjCCDObj, obj2 : * mut mjCCDObj)
    // Previous return: i32
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (pt : * mut Polytope, status : * mut mjCCDStatus, obj1 : * mut mjCCDObj, obj2 : * mut mjCCDObj)
    // Previous return: i32
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (status : * mut mjCCDStatus, pt : * mut Polytope, obj1 : * mut mjCCDObj, obj2 : * mut mjCCDObj)
    // Previous return: * mut Face
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, v1 : * const f64, v2 : * const f64)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (v : * const f64)
    // Previous return: f64
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (v : * mut Vertex, obj1 : * mut mjCCDObj, obj2 : * mut mjCCDObj, dir : * const f64)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (pt : * mut Polytope, status : * mut mjCCDStatus, v1 : i32, v2 : i32, v3 : i32)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (p0 : * const f64, p1 : * const f64, p2 : * const f64, p3 : * const f64)
    // Previous return: i32
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (p0 : * const f64, p1 : * const f64, p2 : * const f64, p3 : * const f64)
    // Previous return: i32
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (R : * mut f64, axis : * const f64)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (v1 : * const f64, v2 : * const f64, v3 : * const f64, p : * const f64)
    // Previous return: i32
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (pt : * mut Polytope, face : * mut Face, e : i32)
    // Previous return: i32
    todo ! ()
}

/// C: horizon (engine/engine_collision_gjk.c:1303)
/// Calls: addEdge, deleteFace, getEdge, horizonRec
#[allow(unused_variables, non_snake_case)]
pub fn horizon(pt: *mut Polytope, face: *mut Face) {
    // WARNING: signature changed — verify body
    // Previous params: (pt : * mut Polytope, face : * mut Face)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (pt : * const Polytope, face : * const Face, x1 : * mut f64, x2 : * mut f64)
    // Previous return: f64
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (a : * const f64, b : * const f64, c : * const f64, d : * const f64)
    // Previous return: f64
    todo ! ()
}

/// C: next (engine/engine_collision_gjk.c:1520)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn next(polygon: *mut f64, nvert: i32, curr: *mut f64) -> *mut f64 {
    // WARNING: signature changed — verify body
    // Previous params: (polygon : * mut f64, nvert : i32, curr : * mut f64)
    // Previous return: * mut f64
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (res : [* mut f64 ; 4], polygon : * mut f64, nvert : i32)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, v1 : * const f64, v2 : * const f64, n : * const f64)
    // Previous return: f64
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (a : * const f64, n : * const f64, p : * const f64)
    // Previous return: i32
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, pn : * const f64, pd : f64, a : * const f64, b : * const f64)
    // Previous return: f64
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, mat : * const f64, pos : * const f64, l1 : f64, l2 : f64, l3 : f64)
    // Previous return: ()
    todo ! ()
}

/// C: intersect (engine/engine_collision_gjk.c:1759)
#[allow(unused_variables, non_snake_case)]
pub fn intersect(res: [i32; 2], arr1: *const i32, arr2: *const i32, n: i32, m: i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (res : [i32 ; 2], arr1 : * const i32, arr2 : * const i32, n : i32, m : i32)
    // Previous return: i32
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, resind : [i32 ; 3], dim : i32, obj : * mut mjCCDObj, v1 : i32, v2 : i32, v3 : i32)
    // Previous return: i32
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, endverts : * mut f64, dim : i32, obj : * mut mjCCDObj, v1 : * const f64, v2 : * const f64, v1i : i32, v2i : i32)
    // Previous return: i32
    todo ! ()
}

/// C: boxNormals2 (engine/engine_collision_gjk.c:1885)
/// Calls: dot3, globalcoord, scl3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn box_normals2(res: *mut f64, resind: [i32; 3], mat: *const f64, n: *const f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, resind : [i32 ; 3], mat : * const f64, n : * const f64)
    // Previous return: i32
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, resind : [i32 ; 3], dim : i32, obj : * mut mjCCDObj, v1 : i32, v2 : i32, v3 : i32, dir : * const f64)
    // Previous return: i32
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, endverts : * mut f64, dim : i32, obj : * mut mjCCDObj, v1 : * const f64, v2 : * const f64, v1i : i32, v2i : i32)
    // Previous return: i32
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, obj : * mut mjCCDObj, idx : i32)
    // Previous return: i32
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, obj : * mut mjCCDObj, idx : i32)
    // Previous return: i32
    todo ! ()
}

/// C: alignedFaces (engine/engine_collision_gjk.c:2072)
/// Calls: dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn aligned_faces(res: [i32; 2], v: *const f64, nv: i32, w: *const f64, nw: i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (res : [i32 ; 2], v : * const f64, nv : i32, w : * const f64, nw : i32)
    // Previous return: i32
    todo ! ()
}

/// C: alignedFaceEdge (engine/engine_collision_gjk.c:2088)
/// Calls: dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn aligned_face_edge(res: [i32; 2], edge: *const f64, nedge: i32, face: *const f64, nface: i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (res : [i32 ; 2], edge : * const f64, nedge : i32, face : * const f64, nface : i32)
    // Previous return: i32
    todo ! ()
}

/// C: simplexDim (engine/engine_collision_gjk.c:2104)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn simplex_dim(v1i: *mut i32, v2i: *mut i32, v3i: *mut i32, v1: *mut *mut f64, v2: *mut *mut f64, v3: *mut *mut f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (v1i : * mut i32, v2i : * mut i32, v3i : * mut i32, v1 : * mut * mut f64, v2 : * mut * mut f64, v3 : * mut * mut f64)
    // Previous return: i32
    todo ! ()
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

