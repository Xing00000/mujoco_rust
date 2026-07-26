//! Port of: user/user_mesh.cc
//! IR hash: 73393814548a07d1
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: Fovea (user/user_mesh.cc:83)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn fovea(x: f64, gamma: f64) -> f64 {
    if gamma == 0.0 {
        return x;
    }
    let g = if 1.0 < gamma { 1.0 } else { gamma };
    let g = if 0.0 > g { 0.0 } else { g };
    g * x.powf(5.0) + (1.0 - g) * x
}

/// C: LinSpace (user/user_mesh.cc:93)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn lin_space(lower: f64, upper: f64, n: i32, array: *mut f64) {
    // SAFETY: caller guarantees array has at least n elements
    unsafe {
        let increment = if n > 1 { (upper - lower) / (n - 1) as f64 } else { 0.0 };
        let mut lower = lower;
        let mut ptr = array;
        for _i in 0..n {
            *ptr = lower;
            ptr = ptr.add(1);
            lower += increment;
        }
    }
}

/// C: BinEdges (user/user_mesh.cc:103)
/// Calls: Fovea, LinSpace, mjuu_scalevec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn bin_edges(x_edges: *mut f64, y_edges: *mut f64, size: *mut i32, fov: *mut f64, gamma: f64) {
    // SAFETY: caller guarantees size[0..2], fov[0..2], x_edges[0..size[0]+1], y_edges[0..size[1]+1]
    unsafe {
        let s0 = *size.add(0);
        let s1 = *size.add(1);
        lin_space(-1.0, 1.0, s0 + 1, x_edges);
        lin_space(-1.0, 1.0, s1 + 1, y_edges);
        for i in 0..(s0 + 1) as usize {
            *x_edges.add(i) = fovea(*x_edges.add(i), gamma);
        }
        for i in 0..(s1 + 1) as usize {
            *y_edges.add(i) = fovea(*y_edges.add(i), gamma);
        }
        crate::user::user_util::mjuu_scalevec(
            x_edges, x_edges, *fov.add(0) * std::f64::consts::PI / 180.0, s0 + 1,
        );
        crate::user::user_util::mjuu_scalevec(
            y_edges, y_edges, *fov.add(1) * std::f64::consts::PI / 180.0, s1 + 1,
        );
    }
}

/// C: SphericalToCartesian (user/user_mesh.cc:123)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn spherical_to_cartesian(aer: *const f64, xyz: *mut f32) {
    // SAFETY: caller guarantees aer[0..3] and xyz[0..3]
    unsafe {
        let a = *aer.add(0);
        let e = *aer.add(1);
        let r = *aer.add(2);
        *xyz.add(0) = (r * e.cos() * a.sin()) as f32;
        *xyz.add(1) = (r * e.sin()) as f32;
        *xyz.add(2) = (-r * e.cos() * a.cos()) as f32;
    }
}

/// C: TangentFrame (user/user_mesh.cc:131)
/// Calls: mjuu_copyvec, mjuu_crossvec, mjuu_normvec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn tangent_frame(aer: *const f64, mat: *mut f32) {
    // SAFETY: aer points to 3 doubles, mat points to 9 floats
    unsafe {
        let a = *aer.add(0);
        let e = *aer.add(1);
        let r = *aer.add(2);

        let mut ta: [f64; 3] = [
            r * e.cos() * a.cos(),
            0.0,
            r * e.cos() * a.sin(),
        ];
        let mut te: [f64; 3] = [
            -r * e.sin() * a.sin(),
            r * e.cos(),
            r * e.sin() * a.cos(),
        ];
        let mut n: [f64; 3] = [0.0; 3];

        crate::user::user_util::mjuu_normvec(ta.as_mut_ptr(), 3);
        crate::user::user_util::mjuu_normvec(te.as_mut_ptr(), 3);
        crate::user::user_util::mjuu_copyvec(mat.add(3) as *mut T1, ta.as_ptr() as *const T2, 3);
        crate::user::user_util::mjuu_copyvec(mat.add(6) as *mut T1, te.as_ptr() as *const T2, 3);
        crate::user::user_util::mjuu_crossvec(n.as_mut_ptr(), te.as_ptr(), ta.as_ptr());
        crate::user::user_util::mjuu_copyvec(mat as *mut T1, n.as_ptr() as *const T2, 3);
    }
}

/// C: aux_c (user/user_mesh.cc:145)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn aux_c(omega: f64, m: f64) -> f64 {
    todo!() // aux_c
}

/// C: aux_s (user/user_mesh.cc:148)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn aux_s(omega: f64, m: f64) -> f64 {
    todo!() // aux_s
}

/// C: triangle (user/user_mesh.cc:154)
/// Calls: mjuu_crossvec, mjuu_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn triangle(normal: *mut f64, center: *mut f64, v1: *const f64, v2: *const f64, v3: *const f64) -> f64 {
    const MJ_MINVAL: f64 = 1e-15;

    // SAFETY: v1, v2, v3 are valid f64[3] pointers from mujoco mesh data
    unsafe {
        let mut normal_local: [f64; 3] = [0.0; 3];
        let normal_ptr: *mut f64 = if !normal.is_null() { normal } else { normal_local.as_mut_ptr() };

        // center
        if !center.is_null() {
            *center.offset(0) = (*v1.offset(0) + *v2.offset(0) + *v3.offset(0)) / 3.0;
            *center.offset(1) = (*v1.offset(1) + *v2.offset(1) + *v3.offset(1)) / 3.0;
            *center.offset(2) = (*v1.offset(2) + *v2.offset(2) + *v3.offset(2)) / 3.0;
        }

        // normal = (v2-v1) cross (v3-v1)
        let mut b: [f64; 3] = [
            *v2.offset(0) - *v1.offset(0),
            *v2.offset(1) - *v1.offset(1),
            *v2.offset(2) - *v1.offset(2),
        ];
        let mut c: [f64; 3] = [
            *v3.offset(0) - *v1.offset(0),
            *v3.offset(1) - *v1.offset(1),
            *v3.offset(2) - *v1.offset(2),
        ];
        crate::user::user_util::mjuu_crossvec(normal_ptr, b.as_ptr(), c.as_ptr());

        // get length
        let len = f64::sqrt(crate::user::user_util::mjuu_dot3(normal_ptr, normal_ptr));

        // ignore small faces
        if len < MJ_MINVAL {
            return 0.0;
        }

        // normalize
        if !normal.is_null() {
            *normal_ptr.offset(0) /= len;
            *normal_ptr.offset(1) /= len;
            *normal_ptr.offset(2) /= len;
        }

        // return area
        0.5 * len
    }
}

/// C: MeshPolygon::InsertFace (user/user_mesh.cc:2685)
/// Calls: MeshPolygon::CombineIslands
#[allow(unused_variables, non_snake_case)]
pub fn mesh_polygon_insert_face(self_ptr: *mut MeshPolygon, v1: i32, v2: i32, v3: i32) {
    todo!() // MeshPolygon::InsertFace
}

/// C: MeshPolygon::Paths (user/user_mesh.cc:2686)
#[allow(unused_variables, non_snake_case)]
pub fn mesh_polygon_paths(self_ptr: *mut MeshPolygon) -> *const () {
    todo!() // MeshPolygon::Paths
}

/// C: MeshPolygon::Normal (user/user_mesh.cc:2687)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mesh_polygon_normal(self_ptr: *mut MeshPolygon) -> *const f64 {
    // SAFETY: self_ptr is valid; normal_ is [u8; 24] representing double[3] at correct alignment
    unsafe {
        (*self_ptr).normal_.as_ptr() as *const f64
    }
}

/// C: MeshPolygon::CombineIslands (user/user_mesh.cc:2698)
#[allow(unused_variables, non_snake_case)]
pub fn mesh_polygon_combine_islands(self_ptr: *mut MeshPolygon, island1: *mut i32, island2: *mut i32) {
    // MeshPolygon::CombineIslands renumbers islands_ vector entries.
    // islands_ is a std::vector<int> at byte offset 24 in MeshPolygon (size 24 bytes).
    // std::vector layout (libc++/libstdc++ ABI): { data_ptr, end_ptr, cap_ptr } (3 x *i32)
    // size = (end_ptr - data_ptr) / sizeof(int)

    // SAFETY: self_ptr, island1, island2 are valid (caller contract).
    // All pointer arithmetic uses verified std::vector ABI layout.
    unsafe {
        let v1 = *island1;
        let v2 = *island2;

        // pick the smaller island
        let (i1, i2) = if v2 < v1 {
            *island1 = v2;
            *island2 = v1;
            (v2, v1)
        } else {
            (v1, v2)
        };

        // islands_ std::vector at offset 24 of MeshPolygon
        // data_ptr at offset 24, end_ptr at offset 32
        let islands_base = (self_ptr as *mut u8).add(24);
        let data_ptr = *(islands_base as *const *mut i32);   // start iterator
        let end_ptr  = *(islands_base.add(8) as *const *mut i32); // past-end iterator

        if data_ptr.is_null() {
            return; // empty vector
        }

        let n = ((end_ptr as usize) - (data_ptr as usize)) / std::mem::size_of::<i32>();

        // renumber the islands
        for k in 0..n {
            let val = data_ptr.add(k);
            if *val == i2 {
                *val = i1;
            } else if *val > i2 {
                *val -= 1;
            }
        }
    }
}

/// C: MeshPolygonKey (user/user_mesh.cc:2701)
/// Calls: mjuu_crossvec, mjuu_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mesh_polygon_key(angles: *const (), v1: *const f64, v2: *const f64, v3: *const f64, angle_tol: f64) -> bool {
    todo!() // MeshPolygonKey
}

/// C: ComputeVolume (user/user_mesh.cc:3421)
/// Calls: mjuu_crossvec, mjuu_normvec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn compute_volume(x: *const f64, v: *const i32) -> f64 {
    // SAFETY: caller guarantees x points to vertex array, v points to array of 3 vertex indices
    unsafe {
        let mut normal: [f64; 3] = [0.0; 3];
        let v0 = *v.add(0) as usize;
        let v1 = *v.add(1) as usize;
        let v2 = *v.add(2) as usize;
        let edge1: [f64; 3] = [
            *x.add(3 * v1) - *x.add(3 * v0),
            *x.add(3 * v1 + 1) - *x.add(3 * v0 + 1),
            *x.add(3 * v1 + 2) - *x.add(3 * v0 + 2),
        ];
        let edge2: [f64; 3] = [
            *x.add(3 * v2) - *x.add(3 * v0),
            *x.add(3 * v2 + 1) - *x.add(3 * v0 + 1),
            *x.add(3 * v2 + 2) - *x.add(3 * v0 + 2),
        ];

        crate::user::user_util::mjuu_crossvec(normal.as_mut_ptr(), edge1.as_ptr(), edge2.as_ptr());
        let dot = crate::user::user_util::mjuu_dot3(normal.as_ptr(), normal.as_ptr());
        dot.sqrt() / 2.0
    }
}

/// C: MetricTensor (user/user_mesh.cc:3450)
/// Calls: mju_error
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn metric_tensor(metric: *mut f64, idx: i32, mu: f64, la: f64, basis: *const [f64; 9]) {
    // Template instantiation: Stencil3D with kNumEdges = 6
    const NUM_EDGES: usize = 6;

    // SAFETY: metric points to array with at least 21*(idx+1) elements,
    // basis points to array of NUM_EDGES [f64; 9] arrays.
    unsafe {
        let mut trE: [f64; NUM_EDGES] = [0.0; NUM_EDGES];
        let mut trEE: [f64; NUM_EDGES * NUM_EDGES] = [0.0; NUM_EDGES * NUM_EDGES];
        let mut k: [f64; NUM_EDGES * NUM_EDGES] = [0.0; NUM_EDGES * NUM_EDGES];

        // compute first invariant i.e. trace(strain)
        for e in 0..NUM_EDGES {
            for i in 0..3usize {
                trE[e] += (*basis.add(e))[4 * i];
            }
        }

        // compute second invariant i.e. trace(strain^2)
        for ed1 in 0..NUM_EDGES {
            for ed2 in 0..NUM_EDGES {
                for i in 0..3usize {
                    for j in 0..3usize {
                        trEE[NUM_EDGES * ed1 + ed2] +=
                            (*basis.add(ed1))[3 * i + j] * (*basis.add(ed2))[3 * j + i];
                    }
                }
            }
        }

        // assembly of strain metric tensor
        for ed1 in 0..NUM_EDGES {
            for ed2 in 0..NUM_EDGES {
                k[NUM_EDGES * ed1 + ed2] = mu * trEE[NUM_EDGES * ed1 + ed2] +
                                           la * trE[ed2] * trE[ed1];
            }
        }

        // copy to triangular representation
        let mut id: i32 = 0;
        for ed1 in 0..NUM_EDGES {
            for ed2 in ed1..NUM_EDGES {
                *metric.add((21 * idx + id) as usize) = k[NUM_EDGES * ed1 + ed2];
                id += 1;
            }
        }

        if id != (NUM_EDGES * (NUM_EDGES + 1) / 2) as i32 {
            crate::engine::engine_util_errmem::mju_error(
                b"incorrect stiffness matrix size\0".as_ptr() as *const i8,
            );
        }
    }
}

/// C: ComputeBasis (user/user_mesh.cc:3503)
/// Calls: mjuu_crossvec, mjuu_normvec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn compute_basis(basis: *mut f64, x: *const f64, v: *const i32, faceL: *const i32, faceR: *const i32, volume: f64) {
    // SAFETY: caller guarantees all pointers are valid and arrays are properly sized.
    // basis points to array of 9 f64; v points to vertex indices; faceL/faceR index into v.
    unsafe {
        let mut basisL: [f64; 3] = [0.0; 3];
        let mut basisR: [f64; 3] = [0.0; 3];
        let mut normal: [f64; 3] = [0.0; 3];

        let xL0 = x.add(3 * (*v.add(*faceL.add(0) as usize) as usize));
        let xL1 = x.add(3 * (*v.add(*faceL.add(1) as usize) as usize));
        let xR0 = x.add(3 * (*v.add(*faceR.add(0) as usize) as usize));
        let xR1 = x.add(3 * (*v.add(*faceR.add(1) as usize) as usize));

        let edgesL: [f64; 3] = [
            *xL0.add(0) - *xL1.add(0),
            *xL0.add(1) - *xL1.add(1),
            *xL0.add(2) - *xL1.add(2),
        ];
        let edgesR: [f64; 3] = [
            *xR1.add(0) - *xR0.add(0),
            *xR1.add(1) - *xR0.add(1),
            *xR1.add(2) - *xR0.add(2),
        ];

        crate::user::user_util::mjuu_crossvec(normal.as_mut_ptr(), edgesR.as_ptr(), edgesL.as_ptr());
        crate::user::user_util::mjuu_normvec(normal.as_mut_ptr(), 3);
        crate::user::user_util::mjuu_crossvec(basisL.as_mut_ptr(), normal.as_ptr(), edgesL.as_ptr());
        crate::user::user_util::mjuu_crossvec(basisR.as_mut_ptr(), edgesR.as_ptr(), normal.as_ptr());

        for i in 0..3usize {
            for j in 0..3usize {
                *basis.add(3 * i + j) = (basisL[i] * basisR[j] +
                                          basisR[i] * basisL[j]) / (8.0 * volume * volume);
            }
        }
    }
}

/// C: ComputeStiffness (user/user_mesh.cc:3574)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn compute_stiffness(stiffness: *const (), body_pos: *const (), v: *const i32, t: i32, E: f64, nu: f64, thickness: f64) {
    todo!() // ComputeStiffness
}

/// C: CreateFlapStencil (user/user_mesh.cc:3605)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn create_flap_stencil(flaps: *const (), simplex: *const (), edgeidx: *const ()) {
    todo!() // CreateFlapStencil
}

/// C: cot (user/user_mesh.cc:3657)
/// Calls: mjuu_crossvec, mjuu_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn cot(x: *const f64, v0: i32, v1: i32, v2: i32) -> f64 {
    // SAFETY: caller guarantees x points to vertex array with valid indices v0, v1, v2
    unsafe {
        let mut normal: [f64; 3] = [0.0; 3];
        let edge1: [f64; 3] = [
            *x.add(3 * v1 as usize) - *x.add(3 * v0 as usize),
            *x.add(3 * v1 as usize + 1) - *x.add(3 * v0 as usize + 1),
            *x.add(3 * v1 as usize + 2) - *x.add(3 * v0 as usize + 2),
        ];
        let edge2: [f64; 3] = [
            *x.add(3 * v2 as usize) - *x.add(3 * v0 as usize),
            *x.add(3 * v2 as usize + 1) - *x.add(3 * v0 as usize + 1),
            *x.add(3 * v2 as usize + 2) - *x.add(3 * v0 as usize + 2),
        ];

        crate::user::user_util::mjuu_crossvec(normal.as_mut_ptr(), edge1.as_ptr(), edge2.as_ptr());
        let dot_e1e2 = crate::user::user_util::mjuu_dot3(edge1.as_ptr(), edge2.as_ptr());
        let dot_nn = crate::user::user_util::mjuu_dot3(normal.as_ptr(), normal.as_ptr());
        dot_e1e2 / dot_nn.sqrt()
    }
}

/// C: ComputeBending (user/user_mesh.cc:3678)
/// Calls: ComputeVolume, cot, mjuu_crossvec, mjuu_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn compute_bending(bending: *mut f64, pos: *mut f64, v: *const i32, mu: f64, thickness: f64) {
    // Template instantiated with kNumVerts = 4 (quad elements).
    // Uses cotangent operator from Wardetzky et al., "Discrete Quadratic Curvature Energies".

    // SAFETY: bending[17], pos[3*max_v], v[4] are valid (caller contract).
    unsafe {
        let v0 = *v.add(0);
        let v1 = *v.add(1);
        let v2 = *v.add(2);
        let v3 = *v.add(3);

        // skip boundary edges
        if v3 == -1 {
            return;
        }

        let vadj: [i32; 3] = [v1, v0, v3];

        // cotangent coefficients
        let a01 = cot(pos, v0, v1, v2);
        let a02 = cot(pos, v0, v3, v1);
        let a03 = cot(pos, v1, v2, v0);
        let a04 = cot(pos, v1, v0, v3);
        let c: [f64; 4] = [a03 + a04, a01 + a02, -(a01 + a03), -(a02 + a04)];

        let vol1 = compute_volume(pos, v);
        let vol2 = compute_volume(pos, vadj.as_ptr());
        let volume = vol1 + vol2;
        let stiffness = 3.0 * mu * thickness.powi(3) / (24.0 * volume);

        // edge vectors
        let p0 = pos.add(3 * v0 as usize);
        let p1 = pos.add(3 * v1 as usize);
        let p2 = pos.add(3 * v2 as usize);
        let p3 = pos.add(3 * v3 as usize);
        let e0: [f64; 3] = [*p1 - *p0, *p1.add(1) - *p0.add(1), *p1.add(2) - *p0.add(2)];
        let e1: [f64; 3] = [*p2 - *p0, *p2.add(1) - *p0.add(1), *p2.add(2) - *p0.add(2)];
        let e2: [f64; 3] = [*p3 - *p0, *p3.add(1) - *p0.add(1), *p3.add(2) - *p0.add(2)];
        let e3: [f64; 3] = [*p2 - *p1, *p2.add(1) - *p1.add(1), *p2.add(2) - *p1.add(2)];
        let e4: [f64; 3] = [*p3 - *p1, *p3.add(1) - *p1.add(1), *p3.add(2) - *p1.add(2)];

        let t0: [f64; 3] = [
            -(a03 * e1[0] + a01 * e3[0]),
            -(a03 * e1[1] + a01 * e3[1]),
            -(a03 * e1[2] + a01 * e3[2]),
        ];
        let t1: [f64; 3] = [
            -(a04 * e2[0] + a02 * e4[0]),
            -(a04 * e2[1] + a02 * e4[1]),
            -(a04 * e2[2] + a02 * e4[2]),
        ];

        let sqr = crate::user::user_util::mjuu_dot3(e0.as_ptr(), e0.as_ptr());
        let cos_theta = -crate::user::user_util::mjuu_dot3(t0.as_ptr(), t1.as_ptr()) / sqr;

        // kNumVerts = 4
        for vi1 in 0..4usize {
            for vi2 in 0..4usize {
                *bending.add(4 * vi1 + vi2) += c[vi1] * c[vi2] * cos_theta * stiffness;
            }
        }

        let mut n: [f64; 3] = [0.0; 3];
        crate::user::user_util::mjuu_crossvec(n.as_mut_ptr(), e0.as_ptr(), e1.as_ptr());
        *bending.add(16) = crate::user::user_util::mjuu_dot3(n.as_ptr(), e2.as_ptr())
            * (a01 - a03) * (a04 - a02) * stiffness / (sqr * sqr.sqrt());
    }
}

/// C: quadratureGaussLegendre (user/user_mesh.cc:3727)
/// Calls: mju_error
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn quadrature_gauss_legendre(points: *mut f64, weights: *mut f64, order: i32, a: f64, b: f64) {
    // Gauss-Legendre quadrature on [a,b] mapped from [-1,1].
    // SAFETY: points and weights are valid arrays sized appropriately for order.
    unsafe {
        if order > 3 {
            crate::engine::engine_util_errmem::mju_error(
                b"Integration order > 3 not yet supported.\0".as_ptr() as *const i8);
        }
        let p0 = (a + b) / 2.0;
        let dpdx = (b - a) / 2.0;
        if order == 2 {
            *points.add(0) = -dpdx / (3.0f64).sqrt() + p0;
            *points.add(1) =  dpdx / (3.0f64).sqrt() + p0;
            *weights.add(0) = dpdx;
            *weights.add(1) = dpdx;
        } else {
            *points.add(0) = p0;
            *points.add(1) = -dpdx / (3.0f64 / 5.0).sqrt() + p0;
            *points.add(2) =  dpdx / (3.0f64 / 5.0).sqrt() + p0;
            *weights.add(0) = 8.0 / 9.0 * dpdx;
            *weights.add(1) = 5.0 / 9.0 * dpdx;
            *weights.add(2) = 5.0 / 9.0 * dpdx;
        }
    }
}

/// C: phi (user/user_mesh.cc:3752)
/// Calls: mju_error, mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn phi(s: f64, i: i32, order: i32) -> f64 {
    if order == 1 {
        if i == 0 { 1.0 - s } else { s }
    } else if order == 2 {
        match i {
            0 => 2.0 * s * s - 3.0 * s + 1.0,
            1 => 4.0 * (s - s * s),
            2 => 2.0 * s * s - s,
            _ => {
                crate::engine::engine_util_errmem::mju_error(
                    b"invalid index %d\0".as_ptr() as *const i8);
                0.0
            }
        }
    } else {
        crate::engine::engine_util_errmem::mju_error(
            b"Order must be 1 or 2.\0".as_ptr() as *const i8);
        0.0
    }
}

/// C: dphi (user/user_mesh.cc:3774)
/// Calls: mju_error, mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn dphi(s: f64, i: i32, order: i32) -> f64 {
    if order == 1 {
        if i == 0 { -1.0 } else { 1.0 }
    } else if order == 2 {
        match i {
            0 => 4.0 * s - 3.0,
            1 => 4.0 * (1.0 - 2.0 * s),
            2 => 4.0 * s - 1.0,
            _ => {
                crate::engine::engine_util_errmem::mju_error(
                    b"invalid index %d, must be 0, 1, or 2\0".as_ptr() as *const i8);
                0.0
            }
        }
    } else {
        crate::engine::engine_util_errmem::mju_error(
            b"Order must be 1 or 2.\0".as_ptr() as *const i8);
        0.0
    }
}

/// C: sym (user/user_mesh.cc:3798)
#[allow(unused_variables, non_snake_case)]
pub fn sym(tensor: *const Matrix) -> Matrix {
    todo!() // sym
}

/// C: inner (user/user_mesh.cc:3809)
#[allow(unused_variables, non_snake_case)]
pub fn inner(tensor1: *const Matrix, tensor2: *const Matrix) -> Matrix {
    todo!() // inner
}

/// C: trace (user/user_mesh.cc:3822)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn trace(tensor: *const Matrix) -> f64 {
    todo!() // trace
}

/// C: ComputeLinearStiffness (user/user_mesh.cc:3826)
/// Calls: dphi, inner, mjuu_zerovec, phi, quadratureGaussLegendre, sym, trace
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn compute_linear_stiffness(K: *const (), pos: *const f64, E: f64, nu: f64, order: i32) {
    todo!() // ComputeLinearStiffness
}

/// C: ComputeLinearStiffness2D (user/user_mesh.cc:3914)
/// Calls: dphi, inner, mjuu_zerovec, phi, quadratureGaussLegendre, sym, trace
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn compute_linear_stiffness2d(K: *const (), pos: *const f64, E: f64, nu: f64, order: i32, thickness: f64, normal_axis: i32) {
    todo!() // ComputeLinearStiffness2D
}

/// C: ComputeWarpMode (user/user_mesh.cc:4007)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn compute_warp_mode(warp: *mut f64, pos: *const f64, npe: i32, order: i32, normal_axis: i32) {
    // Uses a dynamic-size rigid matrix. Max npe in practice is ~25 (order=4).
    // ndof = 3*npe, rigid matrix = 6*ndof.
    // SAFETY: warp[3*npe], pos[3*npe] are valid (caller contract).
    unsafe {
        let npe = npe as usize;
        let ndof = 3 * npe;
        let nbasis = (order + 1) as usize;

        // zero out warp
        for k in 0..ndof { *warp.add(k) = 0.0; }

        // evaluate warp pattern at each node
        for b0 in 0..nbasis {
            for b1 in 0..nbasis {
                let node = b0 * nbasis + b1;
                let s = b0 as f64 / (nbasis as f64 - 1.0);
                let t = b1 as f64 / (nbasis as f64 - 1.0);
                *warp.add(3 * node + normal_axis as usize) = (1.0 - 2.0 * s) * (1.0 - 2.0 * t);
            }
        }

        // compute centroid
        let mut centroid = [0.0f64; 3];
        for n in 0..npe {
            for k in 0..3 {
                centroid[k] += *pos.add(3 * n + k);
            }
        }
        for k in 0..3 { centroid[k] /= npe as f64; }

        // build rigid body modes (6 * ndof)
        let mut rigid = vec![0.0f64; 6 * ndof];

        // translations
        for n in 0..npe {
            rigid[0 * ndof + 3 * n + 0] = 1.0;
            rigid[1 * ndof + 3 * n + 1] = 1.0;
            rigid[2 * ndof + 3 * n + 2] = 1.0;
        }

        // rotations about centroid
        for n in 0..npe {
            let rx = *pos.add(3 * n + 0) - centroid[0];
            let ry = *pos.add(3 * n + 1) - centroid[1];
            let rz = *pos.add(3 * n + 2) - centroid[2];
            rigid[3 * ndof + 3 * n + 1] = -rz;
            rigid[3 * ndof + 3 * n + 2] =  ry;
            rigid[4 * ndof + 3 * n + 0] =  rz;
            rigid[4 * ndof + 3 * n + 2] = -rx;
            rigid[5 * ndof + 3 * n + 0] = -ry;
            rigid[5 * ndof + 3 * n + 1] =  rx;
        }

        // orthonormalize rigid modes via modified Gram-Schmidt
        for i in 0..6 {
            let ri_start = i * ndof;
            for j in 0..i {
                let rj_start = j * ndof;
                let mut dot = 0.0f64;
                for k in 0..ndof { dot += rigid[ri_start + k] * rigid[rj_start + k]; }
                for k in 0..ndof { rigid[ri_start + k] -= dot * rigid[rj_start + k]; }
            }
            let mut norm2 = 0.0f64;
            for k in 0..ndof { norm2 += rigid[ri_start + k] * rigid[ri_start + k]; }
            if norm2 > 1e-20 {
                let inv_norm = 1.0 / norm2.sqrt();
                for k in 0..ndof { rigid[ri_start + k] *= inv_norm; }
            }
        }

        // project warp against rigid modes
        for i in 0..6 {
            let ri_start = i * ndof;
            let mut dot = 0.0f64;
            for k in 0..ndof { dot += *warp.add(k) * rigid[ri_start + k]; }
            for k in 0..ndof { *warp.add(k) -= dot * rigid[ri_start + k]; }
        }

        // normalize
        let mut norm2 = 0.0f64;
        for k in 0..ndof { norm2 += *warp.add(k) * *warp.add(k); }
        if norm2 > 1e-20 {
            let inv_norm = 1.0 / norm2.sqrt();
            for k in 0..ndof { *warp.add(k) *= inv_norm; }
        }
    }
}

/// C: ComputeWarpStiffness (user/user_mesh.cc:4104)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn compute_warp_stiffness(pos: *const f64, npe: i32, normal_axis: i32, E: f64, nu: f64, thickness: f64) -> f64 {
    // SAFETY: pos is a valid f64 array of length 3*npe, npe >= 2 (caller contract).
    unsafe {
        let axis0 = ((normal_axis + 1) % 3) as usize;
        let axis1 = ((normal_axis + 2) % 3) as usize;
        let d0 = (*pos.add(3 * (npe as usize - 1) + axis0) - *pos.add(axis0)).abs();
        let d1 = (*pos.add(3 * (npe as usize - 1) + axis1) - *pos.add(axis1)).abs();
        if d0 < 1e-30 || d1 < 1e-30 { return 0.0; }
        // plate bending rigidity: D = E*t³ / (12*(1-ν²))
        let D = E * thickness * thickness * thickness / (12.0 * (1.0 - nu * nu));
        // warp stiffness: D*(1-ν)*4 / (d0*d1)
        D * (1.0 - nu) * 4.0 / (d0 * d1)
    }
}

/// C: EigendecomposeStiffness (user/user_mesh.cc:4130)
/// Calls: mjuu_eigendecompose
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn eigendecompose_stiffness(K_cell_data: *const f64, out: *mut f64, ndof: i32) -> i32 {
    // SAFETY: K_cell_data[ndof*ndof], out[1+ndof*ndof] are valid (caller contract).
    unsafe {
        let ndof = ndof as usize;
        let n2 = ndof * ndof;

        // copy K_cell for in-place decomposition
        let mut mat = vec![0.0f64; n2];
        std::ptr::copy_nonoverlapping(K_cell_data, mat.as_mut_ptr(), n2);
        let mut eigval = vec![0.0f64; ndof];
        let mut eigvec = vec![0.0f64; n2];

        crate::user::user_util::mjuu_eigendecompose(mat.as_mut_ptr(), eigval.as_mut_ptr(), eigvec.as_mut_ptr(), ndof as i32);

        // K_stored = -K_physical, so physical eigenvalue = -eigval[i]
        let mut max_eigval: f64 = 0.0;
        for i in 0..ndof {
            let abs_v = eigval[i].abs();
            if abs_v > max_eigval { max_eigval = abs_v; }
        }
        let threshold = max_eigval * 1e-8;

        let mut neig: i32 = 0;
        for i in 0..ndof {
            let lambda_phys = -eigval[i];
            if lambda_phys > threshold {
                let scale = lambda_phys.sqrt();
                let w = out.add(1 + neig as usize * ndof);
                for j in 0..ndof {
                    *w.add(j) = scale * eigvec[j * ndof + i];
                }
                neig += 1;
            }
        }

        *out.add(0) = neig as f64;
        neig
    }
}

/// C: ComputeInterpBending (user/user_mesh.cc:4391)
/// Calls: dphi, mjuu_copyvec, mjuu_crossvec, mjuu_normvec, mjuu_zerovec, phi
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn compute_interp_bending(bending: *const (), nodexpos_local: *const (), order: i32, cellcount: *const i32, young: f64, poisson: f64, thickness: f64) {
    todo!() // ComputeInterpBending
}

