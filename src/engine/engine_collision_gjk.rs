//! Port of: engine/engine_collision_gjk.c
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: align8 (engine/engine_collision_gjk.c:49)
/// Calls: cxx:__ZNK6mujoco4user8FilePath4sizeEv
#[allow(unused_variables, non_snake_case)]
pub fn align8(size: usize) -> usize {
    (size + 7) & !7
}

/// C: subdistance (engine/engine_collision_gjk.c:56)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_S3D, cxx-internal:engine_collision_gjk.c.o:_S1D, cxx-internal:engine_collision_gjk.c.o:_S2D
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
            4 => S3D(lambda, s1, s2, s3, s4),
            3 => S2D(lambda, s1, s2, s3),
            2 => S1D(lambda, s1, s2),
            _ => *lambda.add(0) = 1.0,
        }
    }
}

/// C: S3D (engine/engine_collision_gjk.c:60)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_det3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_dot3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_sameSign2, cxx-internal:engine_collision_gjk.c.o:_S2D, cxx-internal:engine_collision_gjk.c.o:_lincomb
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn s3d(lambda: *mut f64, s1: *const f64, s2: *const f64, s3: *const f64, s4: *const f64) {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_S3D
}

/// C: S2D (engine/engine_collision_gjk.c:62)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_dot3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_sameSign2, cxx-internal:engine_collision_gjk.c.o:_S1D, cxx-internal:engine_collision_gjk.c.o:_lincomb, cxx-internal:engine_collision_gjk.c.o:_projectOriginPlane
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn s2d(lambda: *mut f64, s1: *const f64, s2: *const f64, s3: *const f64) {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_S2D
}

/// C: S1D (engine/engine_collision_gjk.c:63)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_projectOriginLine, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_sameSign2
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn s1d(lambda: *mut f64, s1: *const f64, s2: *const f64) {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_S1D
}

/// C: gjkSupport (engine/engine_collision_gjk.c:66)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_scl3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_support
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn gjk_support(v: *mut Vertex, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj, x_k: *const f64, x_norm: f64) {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_gjkSupport
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
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_scl3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_support
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn epa_support(pt: *mut Polytope, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj, d: *const f64, dnorm: f64) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_epaSupport
}

/// C: insertVertex (engine/engine_collision_gjk.c:112)
#[allow(unused_variables, non_snake_case)]
pub fn insert_vertex(pt: *mut Polytope, v: *const Vertex) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_insertVertex
}

/// C: attachFace (engine/engine_collision_gjk.c:115)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_dot3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_scl3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_sub3, cxx-internal:engine_collision_gjk.c.o:_projectOriginPlane
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn attach_face(pt: *mut Polytope, v1: i32, v2: i32, v3: i32, adj1: i32, adj2: i32, adj3: i32) -> f64 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_attachFace
}

/// C: gjkIntersect (engine/engine_collision_gjk.c:119)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_dot3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_gjkIntersectSupport, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_signedDistance
#[allow(unused_variables, non_snake_case)]
pub fn gjk_intersect(status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_gjkIntersect
}

/// C: polytope2 (engine/engine_collision_gjk.c:122)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_add3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_cross3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_insertVertex, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_norm3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_replaceSimplex3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_rotmat, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_scl3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_sub3, cxx-internal:engine_collision_gjk.c.o:_attachFace, cxx-internal:engine_collision_gjk.c.o:_epaSupport, cxx-internal:engine_collision_gjk.c.o:_polytope3, cxx-internal:engine_collision_gjk.c.o:_rayTriangle, cxx:_mju_mulMatVec3
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
        crate::engine::engine_util_blas::mju_mulMatVec3(d2.as_mut_ptr(), R.as_ptr(), d1.as_ptr());
        crate::engine::engine_util_blas::mju_mulMatVec3(d3.as_mut_ptr(), R.as_ptr(), d2.as_ptr());

        let v1i = insertVertex(pt, simplex_base as *const Vertex);
        let v2i = insertVertex(pt, simplex_base.add(SIZEOF_VERTEX) as *const Vertex);
        let v3i = epaSupport(pt, obj1, obj2, d1.as_ptr(), norm3(d1.as_ptr()));
        let v4i = epaSupport(pt, obj1, obj2, d2.as_ptr(), norm3(d2.as_ptr()));
        let v5i = epaSupport(pt, obj1, obj2, d3.as_ptr(), norm3(d3.as_ptr()));

        let v3 = ((*p).verts as *const u8).add(v3i as usize * SIZEOF_VERTEX) as *const f64;
        let v4 = ((*p).verts as *const u8).add(v4i as usize * SIZEOF_VERTEX) as *const f64;
        let v5 = ((*p).verts as *const u8).add(v5i as usize * SIZEOF_VERTEX) as *const f64;

        // build hexahedron
        if attachFace(pt, v1i, v3i, v4i, 1, 3, 2) < MJ_MINDIST2 {
            replaceSimplex3(pt, status, v1i, v3i, v4i);
            return polytope3(pt, status, obj1, obj2);
        }
        if attachFace(pt, v1i, v5i, v3i, 2, 4, 0) < MJ_MINDIST2 {
            replaceSimplex3(pt, status, v1i, v5i, v3i);
            return polytope3(pt, status, obj1, obj2);
        }
        if attachFace(pt, v1i, v4i, v5i, 0, 5, 1) < MJ_MINDIST2 {
            replaceSimplex3(pt, status, v1i, v4i, v5i);
            return polytope3(pt, status, obj1, obj2);
        }
        if attachFace(pt, v2i, v4i, v3i, 5, 0, 4) < MJ_MINDIST2 {
            replaceSimplex3(pt, status, v2i, v4i, v3i);
            return polytope3(pt, status, obj1, obj2);
        }
        if attachFace(pt, v2i, v3i, v5i, 3, 1, 5) < MJ_MINDIST2 {
            replaceSimplex3(pt, status, v2i, v3i, v5i);
            return polytope3(pt, status, obj1, obj2);
        }
        if attachFace(pt, v2i, v5i, v4i, 4, 2, 3) < MJ_MINDIST2 {
            replaceSimplex3(pt, status, v2i, v5i, v4i);
            return polytope3(pt, status, obj1, obj2);
        }

        // check hexahedron is convex
        if rayTriangle(v1 as *const f64, v2 as *const f64, v3, v4, v5) == 0 {
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
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_add3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_cross3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_insertVertex, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_norm3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_scl3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_sub3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_triPointIntersect, cxx-internal:engine_collision_gjk.c.o:_attachFace, cxx-internal:engine_collision_gjk.c.o:_epaSupport, cxx-internal:engine_collision_gjk.c.o:_testTetra
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
        let v1i = insertVertex(pt, simplex_base as *const Vertex);
        let v2i = insertVertex(pt, simplex_base.add(SIZEOF_VERTEX) as *const Vertex);
        let v3i = insertVertex(pt, simplex_base.add(2 * SIZEOF_VERTEX) as *const Vertex);
        let v5i = epaSupport(pt, obj1, obj2, n_neg.as_ptr(), n_norm);
        let v4i = epaSupport(pt, obj1, obj2, n.as_ptr(), n_norm);
        let v4 = ((*p).verts as *const u8).add(v4i as usize * SIZEOF_VERTEX) as *const f64;
        let v5 = ((*p).verts as *const u8).add(v5i as usize * SIZEOF_VERTEX) as *const f64;

        // check v4 not contained in 2-simplex
        if triPointIntersect(v1, v2, v3, v4) != 0 {
            return MJ_EPA_P3_INVALID_V4;
        }
        if triPointIntersect(v1, v2, v3, v5) != 0 {
            return MJ_EPA_P3_INVALID_V5;
        }

        let dist = *(sb.add(STATUS_DIST_OFFSET) as *const f64);
        if dist > 10.0 * MJ_MINVAL && testTetra(v1, v2, v3, v4) == 0 && testTetra(v1, v2, v3, v5) == 0 {
            return MJ_EPA_P3_MISSING_ORIGIN;
        }

        // create hexahedron for EPA (6 faces)
        if attachFace(pt, v4i, v1i, v2i, 1, 3, 2) < MJ_MINDIST3 { return MJ_EPA_P3_ORIGIN_ON_FACE; }
        if attachFace(pt, v4i, v3i, v1i, 2, 4, 0) < MJ_MINDIST3 { return MJ_EPA_P3_ORIGIN_ON_FACE; }
        if attachFace(pt, v4i, v2i, v3i, 0, 5, 1) < MJ_MINDIST3 { return MJ_EPA_P3_ORIGIN_ON_FACE; }
        if attachFace(pt, v5i, v2i, v1i, 5, 0, 4) < MJ_MINDIST3 { return MJ_EPA_P3_ORIGIN_ON_FACE; }
        if attachFace(pt, v5i, v1i, v3i, 3, 1, 5) < MJ_MINDIST3 { return MJ_EPA_P3_ORIGIN_ON_FACE; }
        if attachFace(pt, v5i, v3i, v2i, 4, 2, 3) < MJ_MINDIST3 { return MJ_EPA_P3_ORIGIN_ON_FACE; }

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
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_add3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_insertVertex, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_replaceSimplex3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_scl3, cxx-internal:engine_collision_gjk.c.o:_attachFace, cxx-internal:engine_collision_gjk.c.o:_polytope3, cxx-internal:engine_collision_gjk.c.o:_testTetra
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

        let v1 = insertVertex(pt, simplex_base as *const Vertex);
        let v2 = insertVertex(pt, simplex_base.add(SIZEOF_VERTEX) as *const Vertex);
        let v3 = insertVertex(pt, simplex_base.add(2 * SIZEOF_VERTEX) as *const Vertex);
        let v4 = insertVertex(pt, simplex_base.add(3 * SIZEOF_VERTEX) as *const Vertex);

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
        if attachFace(pt, v1, v2, v3, 1, 3, 2) < MJ_MINDIST4 {
            replaceSimplex3(pt, status, v1, v2, v3);
            return polytope3(pt, status, obj1, obj2);
        }
        if attachFace(pt, v1, v4, v2, 2, 3, 0) < MJ_MINDIST4 {
            replaceSimplex3(pt, status, v1, v4, v2);
            return polytope3(pt, status, obj1, obj2);
        }
        if attachFace(pt, v1, v3, v4, 0, 3, 1) < MJ_MINDIST4 {
            replaceSimplex3(pt, status, v1, v3, v4);
            return polytope3(pt, status, obj1, obj2);
        }
        if attachFace(pt, v4, v3, v2, 2, 0, 1) < MJ_MINDIST4 {
            replaceSimplex3(pt, status, v4, v3, v2);
            return polytope3(pt, status, obj1, obj2);
        }

        // verify origin inside tetrahedron
        if testTetra(p1, p2, p3, p4) == 0 {
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
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_discreteGeoms, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_dot3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_epaWitness, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_horizon, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_maxFaces, cxx-internal:engine_collision_gjk.c.o:_attachFace, cxx-internal:engine_collision_gjk.c.o:_epaSupport, cxx:_mju_warning
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

        let discrete = discreteGeoms(obj1, obj2);
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
            let wi = epaSupport(pt, obj1, obj2, (*face).v.as_ptr(), lower);
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
            if nedges > maxFaces(pt) {
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
            let dist2_0 = attachFace(pt, wi, v2_0, v1_0, nfaces_saved + nedges - 1, hzn_index_0, nfaces_saved + 1);

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
                let dist2_i = attachFace(pt, wi, v2_i, v1_i, cur - 1, hzn_index_i, next);

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
            let dist = epaWitness(pt as *const Polytope, face as *const Face, x1, x2);
            *(status_base.add(STATUS_DIST_OFFSET) as *mut f64) = dist;
            *(status_base.add(STATUS_NX_OFFSET) as *mut i32) = 1;
        } else {
            *(status_base.add(STATUS_NX_OFFSET) as *mut i32) = 0;
            *(status_base.add(STATUS_DIST_OFFSET) as *mut f64) = 0.0;
        }

        face as *mut Face
    }
}

/// C: add3 (engine/engine_collision_gjk.c:133)
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

/// C: sub3 (engine/engine_collision_gjk.c:138)
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

/// C: dot3 (engine/engine_collision_gjk.c:143)
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

/// C: norm3 (engine/engine_collision_gjk.c:148)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn norm3(v: *const f64) -> f64 {
    dot3(v, v).sqrt()
}

/// C: copy3 (engine/engine_collision_gjk.c:153)
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

/// C: scl3 (engine/engine_collision_gjk.c:158)
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

/// C: addScl3 (engine/engine_collision_gjk.c:163)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_scl3(res: *mut f64, v1: *const f64, v2: *const f64, s: f64) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_addScl3
}

/// C: cross3 (engine/engine_collision_gjk.c:168)
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

/// C: det3 (engine/engine_collision_gjk.c:175)
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

/// C: discreteGeoms (engine/engine_collision_gjk.c:186)
#[allow(unused_variables, non_snake_case)]
pub fn discrete_geoms(obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_discreteGeoms
}

/// C: gjk (engine/engine_collision_gjk.c:198)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_discreteGeoms, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_dot3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_gjkIntersect, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_norm3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_sub3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_subdistance, cxx-internal:engine_collision_gjk.c.o:_gjkSupport, cxx-internal:engine_collision_gjk.c.o:_lincomb
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
        let epsilon = if discreteGeoms(obj1, obj2) != 0 { 0.0 } else { 0.5 * tol2 };
        let min_norm2 = if discreteGeoms(obj1, obj2) != 0 { MJ_MINVAL2 } else { tol2 };

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
            gjkSupport(sv_n, obj1, obj2, x_k.as_ptr(), x_norm);
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
                let ret = gjkIntersect(status, obj1, obj2);
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

/// C: support (engine/engine_collision_gjk.c:336)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_sub3
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

/// C: gjkIntersectSupport (engine/engine_collision_gjk.c:398)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_support
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn gjk_intersect_support(v: *mut Vertex, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj, dir: *const f64) {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_gjkIntersectSupport
}

/// C: signedDistance (engine/engine_collision_gjk.c:406)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_cross3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_dot3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_scl3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn signed_distance(normal: *mut f64, v1: *const Vertex, v2: *const Vertex, v3: *const Vertex) -> f64 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_signedDistance
}

/// C: projectOriginPlane (engine/engine_collision_gjk.c:512)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_cross3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_dot3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_scl3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn project_origin_plane(res: *mut f64, v1: *const f64, v2: *const f64, v3: *const f64) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_projectOriginPlane
}

/// C: projectOriginLine (engine/engine_collision_gjk.c:549)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_dot3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn project_origin_line(res: *mut f64, v1: *const f64, v2: *const f64) {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_projectOriginLine
}

/// C: sameSign2 (engine/engine_collision_gjk.c:561)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn same_sign2(a: f64, b: f64) -> i32 {
    if a > 0.0 && b > 0.0 {
        1
    } else if a < 0.0 && b < 0.0 {
        -1
    } else {
        0
    }
}

/// C: replaceSimplex3 (engine/engine_collision_gjk.c:855)
#[allow(unused_variables, non_snake_case)]
pub fn replace_simplex3(pt: *mut Polytope, status: *mut mjCCDStatus, v1: i32, v2: i32, v3: i32) {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_replaceSimplex3
}

/// C: sameSide (engine/engine_collision_gjk.c:870)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_cross3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_dot3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_scl3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn same_side(p0: *const f64, p1: *const f64, p2: *const f64, p3: *const f64) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_sameSide
}

/// C: testTetra (engine/engine_collision_gjk.c:889)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_sameSide
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn test_tetra(p0: *const f64, p1: *const f64, p2: *const f64, p3: *const f64) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_testTetra
}

/// C: rotmat (engine/engine_collision_gjk.c:899)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_norm3
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

/// C: rayTriangle (engine/engine_collision_gjk.c:917)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_det3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_triangle(v1: *const f64, v2: *const f64, v3: *const f64, v4: *const f64, v5: *const f64) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_rayTriangle
}

/// C: triAffineCoord (engine/engine_collision_gjk.c:1022)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn tri_affine_coord(lambda: *mut f64, v1: *const f64, v2: *const f64, v3: *const f64, p: *const f64) {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_triAffineCoord
}

/// C: triPointIntersect (engine/engine_collision_gjk.c:1067)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_norm3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_sub3, cxx-internal:engine_collision_gjk.c.o:_triAffineCoord
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn tri_point_intersect(v1: *const f64, v2: *const f64, v3: *const f64, p: *const f64) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_triPointIntersect
}

/// C: deleteFace (engine/engine_collision_gjk.c:1222)
#[allow(unused_variables, non_snake_case)]
pub fn delete_face(pt: *mut Polytope, face: *mut Face) {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_deleteFace
}

/// C: maxFaces (engine/engine_collision_gjk.c:1232)
#[allow(unused_variables, non_snake_case)]
pub fn max_faces(pt: *mut Polytope) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_maxFaces
}

/// C: addEdge (engine/engine_collision_gjk.c:1269)
#[allow(unused_variables, non_snake_case)]
pub fn add_edge(pt: *mut Polytope, index: i32, edge: i32) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_addEdge
}

/// C: getEdge (engine/engine_collision_gjk.c:1276)
#[allow(unused_variables, non_snake_case)]
pub fn get_edge(face: *mut Face, vertex: i32) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_getEdge
}

/// C: horizonRec (engine/engine_collision_gjk.c:1285)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_addEdge, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_deleteFace, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_dot3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_getEdge
#[allow(unused_variables, non_snake_case)]
pub fn horizon_rec(pt: *mut Polytope, face: *mut Face, e: i32) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_horizonRec
}

/// C: horizon (engine/engine_collision_gjk.c:1309)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_addEdge, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_deleteFace, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_getEdge, cxx-internal:engine_collision_gjk.c.o:_horizonRec
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

        deleteFace(pt, face);

        let verts_packed = (*f).verts;
        let verts = [
            verts_packed & 0x3FF,
            (verts_packed >> 10) & 0x3FF,
            (verts_packed >> 20) & 0x3FF,
        ];

        let faces_base = (*p).faces as *mut FaceRepr;

        // first edge
        let adj_face = faces_base.add((*f).adj[0] as usize) as *mut Face;
        let adj_edge = getEdge(adj_face, verts[1]);
        if horizonRec(pt, adj_face, adj_edge) == 0 {
            addEdge(pt, (*f).adj[0], adj_edge);
        }

        // second edge
        let adj_face = faces_base.add((*f).adj[1] as usize) as *mut Face;
        let adj_face_repr = faces_base.add((*f).adj[1] as usize);
        let adj_edge = getEdge(adj_face, verts[2]);
        if (*adj_face_repr).index > -2 && horizonRec(pt, adj_face, adj_edge) == 0 {
            addEdge(pt, (*f).adj[1], adj_edge);
        }

        // third edge
        let adj_face = faces_base.add((*f).adj[2] as usize) as *mut Face;
        let adj_face_repr = faces_base.add((*f).adj[2] as usize);
        let adj_edge = getEdge(adj_face, verts[0]);
        if (*adj_face_repr).index > -2 && horizonRec(pt, adj_face, adj_edge) == 0 {
            addEdge(pt, (*f).adj[2], adj_edge);
        }
    }
}

/// C: epaWitness (engine/engine_collision_gjk.c:1337)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_lincomb, cxx-internal:engine_collision_gjk.c.o:_triAffineCoord
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn epa_witness(pt: *const Polytope, face: *const Face, x1: *mut f64, x2: *mut f64) -> f64 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_epaWitness
}

/// C: area4 (engine/engine_collision_gjk.c:1511)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_cross3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_norm3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn area4(hull: *const f64, a: i32, b: i32, c: i32, d: i32) -> f64 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_area4
}

/// C: hull4 (engine/engine_collision_gjk.c:1521)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_area4
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn hull4(res: *mut i32, hull: *const f64, nhull: i32) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_hull4
}

/// C: planeNormal (engine/engine_collision_gjk.c:1568)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_add3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_cross3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_dot3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_sub3, cxx:_mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn plane_normal(res: *mut f64, v1: *const f64, v2: *const f64, n: *const f64) -> f64 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_planeNormal
}

/// C: halfspace (engine/engine_collision_gjk.c:1583)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_dot3
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

/// C: witnessOnFace (engine/engine_collision_gjk.c:1590)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_addScl3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_copy3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn witness_on_face(w1: *mut f64, w2: *mut f64, v: *const f64, p: *const f64, n: *const f64, dir: *const f64) -> f64 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_witnessOnFace
}

/// C: polygonClip (engine/engine_collision_gjk.c:1601)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_addScl3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_copy3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_dot3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_halfspace, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_hull4, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_planeNormal, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_sub3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_witnessOnFace
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn polygon_clip(status: *mut mjCCDStatus, face1: *const f64, nface1: i32, face2: *const f64, nface2: i32, n: *const f64, dir: *const f64, buffer: *mut f64, npolygonmax: i32) {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_polygonClip
}

/// C: globalcoord (engine/engine_collision_gjk.c:1737)
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

/// C: intersect (engine/engine_collision_gjk.c:1752)
#[allow(unused_variables, non_snake_case)]
pub fn intersect(res: *mut i32, arr1: *const i32, arr2: *const i32, n: i32, m: i32) -> i32 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_intersect
}

/// C: meshNormals (engine/engine_collision_gjk.c:1767)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_globalcoord, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_intersect
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mesh_normals(res: *mut f64, resind: *mut i32, dim: i32, obj: *mut mjCCDObj, vi: *const i32) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_meshNormals
}

/// C: meshEdgeNormals (engine/engine_collision_gjk.c:1832)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_copy3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_globalcoord, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_sub3, cxx:_mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mesh_edge_normals(res: *mut f64, endverts: *mut f64, dim: i32, obj: *mut mjCCDObj, v: *const f64, v1i: i32) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_meshEdgeNormals
}

/// C: cylinderNormals (engine/engine_collision_gjk.c:1878)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_globalcoord
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn cylinder_normals(res: *mut f64, resind: *mut i32, dim: i32, obj: *mut mjCCDObj, vi: *const i32, dir: *const f64) -> i32 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_cylinderNormals
}

/// C: boxNormals2 (engine/engine_collision_gjk.c:1890)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_dot3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_globalcoord, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_scl3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn box_normals2(res: *mut f64, resind: *mut i32, mat: *const f64, n: *const f64) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_boxNormals2
}

/// C: boxNormals (engine/engine_collision_gjk.c:1916)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_globalcoord, cxx-internal:engine_collision_gjk.c.o:_boxNormals2
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn box_normals(res: *mut f64, resind: *mut i32, dim: i32, obj: *mut mjCCDObj, vi: *const i32, dir: *const f64) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_boxNormals
}

/// C: boxEdgeNormals (engine/engine_collision_gjk.c:1971)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_copy3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_globalcoord, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_sub3, cxx:_mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn box_edge_normals(res: *mut f64, endverts: *mut f64, dim: i32, obj: *mut mjCCDObj, v: *const f64, v1i: i32) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_boxEdgeNormals
}

/// C: cylinderEdgeNormals (engine/engine_collision_gjk.c:2010)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_addScl3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn cylinder_edge_normals(res: *mut f64, endverts: *mut f64, dim: i32, obj: *mut mjCCDObj, v: *const f64, v1i: i32) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_cylinderEdgeNormals
}

/// C: cylinderFace (engine/engine_collision_gjk.c:2027)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_globalcoord
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn cylinder_face(res: *mut f64, obj: *mut mjCCDObj, idx: i32) -> i32 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_cylinderFace
}

/// C: boxFace (engine/engine_collision_gjk.c:2053)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_globalcoord
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn box_face(res: *mut f64, obj: *mut mjCCDObj, idx: i32) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_boxFace
}

/// C: meshFace (engine/engine_collision_gjk.c:2103)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_globalcoord
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mesh_face(res: *mut f64, obj: *mut mjCCDObj, idx: i32) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_meshFace
}

/// C: alignedFaces (engine/engine_collision_gjk.c:2120)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn aligned_faces(res: *mut i32, v: *const f64, nv: i32, w: *const f64, nw: i32) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_alignedFaces
}

/// C: alignedFaceEdge (engine/engine_collision_gjk.c:2136)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn aligned_face_edge(res: *mut i32, edge: *const f64, nedge: i32, face: *const f64, nface: i32, dir: *const f64) -> i32 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_alignedFaceEdge
}

/// C: simplexDim (engine/engine_collision_gjk.c:2157)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_copy3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn simplex_dim(vi: *mut i32, v: *mut f64) -> i32 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_simplexDim
}

/// C: multicontact (engine/engine_collision_gjk.c:2169)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_align8, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_alignedFaceEdge, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_alignedFaces, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_copy3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_cylinderFace, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_cylinderNormals, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_meshFace, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_scl3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_simplexDim, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_sub3, cxx-internal:engine_collision_gjk.c.o:_boxEdgeNormals, cxx-internal:engine_collision_gjk.c.o:_boxFace, cxx-internal:engine_collision_gjk.c.o:_boxNormals, cxx-internal:engine_collision_gjk.c.o:_cylinderEdgeNormals, cxx-internal:engine_collision_gjk.c.o:_meshEdgeNormals, cxx-internal:engine_collision_gjk.c.o:_meshNormals, cxx-internal:engine_collision_gjk.c.o:_polygonClip
#[allow(unused_variables, non_snake_case)]
pub fn multicontact(nmeshdegmax: i32, npolygonmax: i32, buffer: *mut u8, pt: *mut Polytope, face: *mut Face, status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_multicontact
}

/// C: inflate (engine/engine_collision_gjk.c:2335)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_sub3, cxx:_mju_normalize3
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

/// C: mjc_ccdSize (engine/engine_collision_gjk.c:2354)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_align8
#[allow(unused_variables, non_snake_case)]
pub fn mjc_ccd_size(npolygonmax: i32, nmeshdegmax: i32, iterations: i32) -> usize {
    todo!() // cxx:_mjc_ccdSize
}

/// C: mjc_ccd (engine/engine_collision_gjk.c:2381)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_align8, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:_inflate, cxx-internal:engine_collision_gjk.c.o:_epa, cxx-internal:engine_collision_gjk.c.o:_gjk, cxx-internal:engine_collision_gjk.c.o:_multicontact, cxx-internal:engine_collision_gjk.c.o:_polytope2, cxx-internal:engine_collision_gjk.c.o:_polytope3, cxx-internal:engine_collision_gjk.c.o:_polytope4
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_ccd(config: *const mjCCDConfig, status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> f64 {
    todo!() // cxx:_mjc_ccd
}

