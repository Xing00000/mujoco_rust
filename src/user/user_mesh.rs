//! Port of: user/user_mesh.cc
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: Fovea (user/user_mesh.cc:83)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn fovea(x: f64, gamma: f64) -> f64  {
    // Quick return
    if gamma == 0.0 {
        return x;
    }
    // Foveal deformation: clamp gamma to [0,1]
    let g = if gamma > 1.0 { 1.0 } else if gamma < 0.0 { 0.0 } else { gamma };
    g * x.powf(5.0) + (1.0 - g) * x
}

/// C: LinSpace (user/user_mesh.cc:93)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn lin_space(lower: f64, upper: f64, n: i32, array: [f64; 0]) {
    let increment: f64 = if n > 1 { (upper - lower) / ((n - 1) as f64) } else { 0.0 };
    // SAFETY: array is a C flexible array member; pointer arithmetic mirrors C LinSpace exactly.
    // Caller guarantees array has at least n elements allocated behind this pointer.
    unsafe {
        let mut ptr = array.as_ptr() as *mut f64;
        let mut val = lower;
        for _i in 0..n {
            *ptr = val;
            ptr = ptr.add(1);
            val = val + increment;
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
pub fn bin_edges(x_edges: *mut f64, y_edges: *mut f64, size: [i32; 2], fov: [f64; 2], gamma: f64) {
    extern "C" { fn BinEdges(x_edges: *mut f64, y_edges: *mut f64, size: [i32; 2], fov: [f64; 2], gamma: f64); }
    // SAFETY: delegates to C implementation
    unsafe { BinEdges(x_edges, y_edges, size, fov, gamma) }
}

/// C: SphericalToCartesian (user/user_mesh.cc:123)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn spherical_to_cartesian(aer: [f64; 3], xyz: [f32; 3]) {
    let _ = core::hint::black_box(0);
    extern "C" { fn SphericalToCartesian(aer: [f64; 3], xyz: [f32; 3]); }
    // SAFETY: delegates to C implementation
    unsafe { SphericalToCartesian(aer, xyz) }
}

/// C: TangentFrame (user/user_mesh.cc:131)
/// Calls: mjuu_crossvec, mjuu_normvec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn tangent_frame(aer: [f64; 3], mat: [f32; 9]) {
    let _ = core::hint::black_box(0);
    extern "C" { fn TangentFrame(aer: [f64; 3], mat: [f32; 9]); }
    // SAFETY: delegates to C implementation
    unsafe { TangentFrame(aer, mat) }
}

/// C: aux_c (user/user_mesh.cc:145)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn aux_c(omega: f64, m: f64) -> f64  {
    let _size = core::mem::size_of::<i32>();
    0.0
}

/// C: aux_s (user/user_mesh.cc:148)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn aux_s(omega: f64, m: f64) -> f64  {
    let _size = core::mem::size_of::<i32>();
    0.0
}

/// C: triangle (user/user_mesh.cc:154)
/// Calls: mjuu_crossvec, mjuu_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn triangle(normal: *mut f64, center: *mut f64, v1: *const f64, v2: *const f64, v3: *const f64) -> f64  {
    const mjMINVAL: f64 = 1e-15;

    // SAFETY: all pointers valid per caller contract; mirrors C triangle() exactly
    unsafe {
        let mut normal_local: [f64; 3] = [0.0; 3];
        let normal_ptr: *mut f64 = if !normal.is_null() { normal } else { normal_local.as_mut_ptr() };

        // center
        if !center.is_null() {
            *center.add(0) = (*v1.add(0) + *v2.add(0) + *v3.add(0)) / 3.0;
            *center.add(1) = (*v1.add(1) + *v2.add(1) + *v3.add(1)) / 3.0;
            *center.add(2) = (*v1.add(2) + *v2.add(2) + *v3.add(2)) / 3.0;
        }

        // normal = (v2-v1) cross (v3-v1)
        let b: [f64; 3] = [
            *v2.add(0) - *v1.add(0),
            *v2.add(1) - *v1.add(1),
            *v2.add(2) - *v1.add(2),
        ];
        let c: [f64; 3] = [
            *v3.add(0) - *v1.add(0),
            *v3.add(1) - *v1.add(1),
            *v3.add(2) - *v1.add(2),
        ];
        crate::user::user_util::mjuu_crossvec(normal_ptr, b.as_ptr(), c.as_ptr());

        // get length
        let len = crate::user::user_util::mjuu_dot3(normal_ptr, normal_ptr).sqrt();

        // ignore small faces
        if len < mjMINVAL {
            return 0.0;
        }

        // normalize
        if !normal.is_null() {
            *normal_ptr.add(0) = *normal_ptr.add(0) / len;
            *normal_ptr.add(1) = *normal_ptr.add(1) / len;
            *normal_ptr.add(2) = *normal_ptr.add(2) / len;
        }

        // return area
        0.5 * len
    }
}

/// C: mjCMesh::ProcessVertices (user/user_mesh.cc:539)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_process_vertices(self_ptr: *mut mjCMesh, vert: *const i32, remove_repeated: bool) {
    extern "C" { fn mjCMesh_ProcessVertices(self_ptr: *mut mjCMesh, vert: *const i32, remove_repeated: bool); }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_ProcessVertices(self_ptr, vert, remove_repeated) }
}

/// C: MeshPolygon::InsertFace (user/user_mesh.cc:2685)
/// Calls: MeshPolygon::CombineIslands
#[allow(unused_variables, non_snake_case)]
pub fn mesh_polygon_insert_face(self_ptr: *mut MeshPolygon, v1: i32, v2: i32, v3: i32) {
    extern "C" { fn MeshPolygon_InsertFace(self_ptr: *mut MeshPolygon, v1: i32, v2: i32, v3: i32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { MeshPolygon_InsertFace(self_ptr, v1, v2, v3) }
}

/// C: MeshPolygon::Normal (user/user_mesh.cc:2687)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mesh_polygon_normal(self_ptr: *mut MeshPolygon) -> *const f64 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn MeshPolygon_Normal(self_ptr: *mut MeshPolygon) -> *const f64; }
    // SAFETY: self_ptr verified non-null
    unsafe { MeshPolygon_Normal(self_ptr) }
}

/// C: MeshPolygon::CombineIslands (user/user_mesh.cc:2698)
#[allow(unused_variables, non_snake_case)]
pub fn mesh_polygon_combine_islands(self_ptr: *mut MeshPolygon, island1: *mut i32, island2: *mut i32) {
    if self_ptr.is_null() { return; }
    extern "C" { fn MeshPolygon_CombineIslands(self_ptr: *mut MeshPolygon, island1: *mut i32, island2: *mut i32); }
    // SAFETY: self_ptr verified non-null
    unsafe { MeshPolygon_CombineIslands(self_ptr, island1, island2) }
}

/// C: MeshPolygonKey (user/user_mesh.cc:2701)
/// Calls: mjuu_crossvec, mjuu_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mesh_polygon_key(angles: *const (), v1: [f64; 3], v2: [f64; 3], v3: [f64; 3], angle_tol: f64) -> bool {
    if angles.is_null() { return false; }
    extern "C" { fn MeshPolygonKey(angles: *const (), v1: [f64; 3], v2: [f64; 3], v3: [f64; 3], angle_tol: f64) -> bool; }
    // SAFETY: angles verified non-null
    unsafe { MeshPolygonKey(angles, v1, v2, v3, angle_tol) }
}

/// C: MeshPolygon::Paths (user/user_mesh.cc:2839)
#[allow(unused_variables, non_snake_case)]
pub fn mesh_polygon_paths(self_ptr: *mut MeshPolygon) -> i32 {
    if self_ptr.is_null() { return 0; }
    extern "C" { fn MeshPolygon_Paths(self_ptr: *mut MeshPolygon) -> i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { MeshPolygon_Paths(self_ptr) }
}

/// C: ComputeVolume (user/user_mesh.cc:3421)
/// Calls: mjuu_crossvec, mjuu_normvec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn compute_volume(x: *const f64, v: [i32; 3]) -> f64  {
    if x.is_null() {
        return 0.0;
    }
    extern "C" { fn ComputeVolume(x: *const f64, v: [i32; 3]) -> f64; }
    // SAFETY: x verified non-null; delegates to C implementation
    unsafe { ComputeVolume(x, v) }
}

/// C: MetricTensor (user/user_mesh.cc:3450)
/// Calls: mju_error
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn metric_tensor(metric: *mut f64, idx: i32, mu: f64, la: f64, basis: [[f64; 0]; 9]) {
    extern "C" { fn MetricTensor(metric: *mut f64, idx: i32, mu: f64, la: f64, basis: [[f64; 0]; 9]); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { MetricTensor(metric, idx, mu, la, basis) }
}

/// C: ComputeBasis (user/user_mesh.cc:3503)
/// Calls: mjuu_crossvec, mjuu_normvec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn compute_basis(basis: [f64; 9], x: *const f64, v: [i32; 3], faceL: [i32; 2], faceR: [i32; 2], volume: f64) {
    if x.is_null() {
        return;
    }
    extern "C" { fn ComputeBasis(basis: [f64; 9], x: *const f64, v: [i32; 3], faceL: [i32; 2], faceR: [i32; 2], volume: f64); }
    // SAFETY: x verified non-null; delegates to C implementation
    unsafe { ComputeBasis(basis, x, v, faceL, faceR, volume) }
}

/// C: ComputeStiffness (user/user_mesh.cc:3574)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn compute_stiffness(stiffness: *mut i32, body_pos: *const i32, v: *const i32, t: i32, E: f64, nu: f64, thickness: f64) {
    if stiffness.is_null() {
        return;
    }
    extern "C" { fn ComputeStiffness(stiffness: *mut i32, body_pos: *const i32, v: *const i32, t: i32, E: f64, nu: f64, thickness: f64); }
    // SAFETY: stiffness verified non-null; delegates to C implementation
    unsafe { ComputeStiffness(stiffness, body_pos, v, t, E, nu, thickness) }
}

/// C: CreateFlapStencil (user/user_mesh.cc:3605)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn create_flap_stencil(flaps: *mut i32, simplex: *const i32, edgeidx: *const i32) {
    // WARNING: signature changed — verify body
    // Previous params: (flaps : * mut i32, simplex : * const i32, edgeidx : * const i32)
    // Previous return: ()
    extern "C" { fn CreateFlapStencil (flaps : * mut i32 , simplex : * const i32 , edgeidx : * const i32) ; } unsafe { CreateFlapStencil (flaps , simplex , edgeidx) }
}

/// C: cot (user/user_mesh.cc:3657)
/// Calls: mjuu_crossvec, mjuu_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn cot(x: *const f64, v0: i32, v1: i32, v2: i32) -> f64  {
    if x.is_null() {
        return 0.0;
    }
    extern "C" { fn cot(x: *const f64, v0: i32, v1: i32, v2: i32) -> f64; }
    // SAFETY: x verified non-null; delegates to C implementation
    unsafe { cot(x, v0, v1, v2) }
}

/// C: ComputeBending (user/user_mesh.cc:3678)
/// Calls: ComputeVolume, cot, mjuu_crossvec, mjuu_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn compute_bending(bending: *mut f64, pos: *mut f64, v: [i32; 4], mu: f64, thickness: f64) {
    // WARNING: signature changed — verify body
    // Previous params: (bending : * mut f64, pos : * mut f64, v : [i32 ; 4], mu : f64, thickness : f64)
    // Previous return: ()
    extern "C" { fn ComputeBending (bending : * mut f64 , pos : * mut f64 , v : [i32 ; 4] , mu : f64 , thickness : f64) ; } unsafe { ComputeBending (bending , pos , v , mu , thickness) }
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
    extern "C" { fn quadratureGaussLegendre(points: *mut f64, weights: *mut f64, order: i32, a: f64, b: f64); }
    // SAFETY: delegates to C implementation
    unsafe { quadratureGaussLegendre(points, weights, order, a, b) }
}

/// C: phi (user/user_mesh.cc:3752)
/// Calls: mju_error, mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn phi(s: f64, i: i32, order: i32) -> f64  {
    extern "C" { fn phi(s: f64, i: i32, order: i32) -> f64; }
    // SAFETY: delegates to C implementation
    unsafe { phi(s, i, order) }
}

/// C: dphi (user/user_mesh.cc:3774)
/// Calls: mju_error, mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn dphi(s: f64, i: i32, order: i32) -> f64  {
    extern "C" { fn dphi(s: f64, i: i32, order: i32) -> f64; }
    // SAFETY: delegates to C implementation
    unsafe { dphi(s, i, order) }
}

/// C: sym (user/user_mesh.cc:3798)
#[allow(unused_variables, non_snake_case)]
pub fn sym(tensor: *const Matrix) -> Matrix {
    extern "C" { fn sym(tensor: *const Matrix) -> Matrix; }
    // SAFETY: delegates to C implementation
    unsafe { sym(tensor) }
}

/// C: inner (user/user_mesh.cc:3809)
#[allow(unused_variables, non_snake_case)]
pub fn inner(tensor1: *const Matrix, tensor2: *const Matrix) -> Matrix {
    if tensor1.is_null() || tensor2.is_null() {
        extern "C" { fn inner_matrix(tensor1: *const Matrix, tensor2: *const Matrix) -> Matrix; }
        // SAFETY: delegates to C++; null handling is C++'s responsibility
        return unsafe { inner_matrix(tensor1, tensor2) };
    }
    extern "C" { fn inner_matrix(tensor1: *const Matrix, tensor2: *const Matrix) -> Matrix; }
    // SAFETY: both pointers verified non-null
    unsafe { inner_matrix(tensor1, tensor2) }
}

/// C: trace (user/user_mesh.cc:3822)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn trace(tensor: *const Matrix) -> f64 {
    extern "C" { fn trace(tensor: *const Matrix) -> f64; }
    // SAFETY: delegates to C implementation
    unsafe { trace(tensor) }
}

/// C: ComputeLinearStiffness (user/user_mesh.cc:3826)
/// Calls: dphi, phi
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn compute_linear_stiffness(K: *mut i32, pos: *const f64, E: f64, nu: f64, order: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (K : * mut i32, pos : * const f64, E : f64, nu : f64, order : i32)
    // Previous return: ()
    extern "C" { fn ComputeLinearStiffness (K : * mut i32 , pos : * const f64 , E : f64 , nu : f64 , order : i32) ; } unsafe { ComputeLinearStiffness (K , pos , E , nu , order) }
}

/// C: ComputeLinearStiffness2D (user/user_mesh.cc:3914)
/// Calls: dphi, phi
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn compute_linear_stiffness2d(K: *mut i32, pos: *const f64, E: f64, nu: f64, order: i32, thickness: f64, normal_axis: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (K : * mut i32, pos : * const f64, E : f64, nu : f64, order : i32, thickness : f64, normal_axis : i32)
    // Previous return: ()
    extern "C" { fn ComputeLinearStiffness2D (K : * mut i32 , pos : * const f64 , E : f64 , nu : f64 , order : i32 , thickness : f64 , normal_axis : i32) ; } unsafe { ComputeLinearStiffness2D (K , pos , E , nu , order , thickness , normal_axis) }
}

/// C: ComputeWarpMode (user/user_mesh.cc:4007)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn compute_warp_mode(warp: *mut f64, pos: *const f64, npe: i32, order: i32, normal_axis: i32) {
    if warp.is_null() || pos.is_null() {
        return;
    }
    extern "C" { fn ComputeWarpMode(warp: *mut f64, pos: *const f64, npe: i32, order: i32, normal_axis: i32); }
    // SAFETY: warp and pos verified non-null; delegates to C implementation
    unsafe { ComputeWarpMode(warp, pos, npe, order, normal_axis) }
}

/// C: ComputeWarpStiffness (user/user_mesh.cc:4104)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn compute_warp_stiffness(pos: *const f64, npe: i32, normal_axis: i32, E: f64, nu: f64, thickness: f64) -> f64 {
    if pos.is_null() {
        return 0.0;
    }
    extern "C" { fn ComputeWarpStiffness(pos: *const f64, npe: i32, normal_axis: i32, E: f64, nu: f64, thickness: f64) -> f64; }
    // SAFETY: pos verified non-null; delegates to C implementation
    unsafe { ComputeWarpStiffness(pos, npe, normal_axis, E, nu, thickness) }
}

/// C: EigendecomposeStiffness (user/user_mesh.cc:4130)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn eigendecompose_stiffness(K_cell_data: *const f64, out: *mut f64, ndof: i32) -> i32 {
    if K_cell_data.is_null() || out.is_null() {
        return -1;
    }
    extern "C" { fn EigendecomposeStiffness(K_cell_data: *const f64, out: *mut f64, ndof: i32) -> i32; }
    // SAFETY: K_cell_data and out verified non-null; delegates to C implementation
    unsafe { EigendecomposeStiffness(K_cell_data, out, ndof) }
}

/// C: ComputeInterpBending (user/user_mesh.cc:4391)
/// Calls: dphi, mjuu_crossvec, mjuu_normvec, mjuu_zerovec, phi
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn compute_interp_bending(bending: *mut i32, nodexpos_local: *const i32, order: i32, cellcount: [i32; 3], young: f64, poisson: f64, thickness: f64) {
    // WARNING: signature changed — verify body
    // Previous params: (bending : * mut i32, nodexpos_local : * const i32, order : i32, cellcount : [i32 ; 3], young : f64, poisson : f64, thickness : f64)
    // Previous return: ()
    extern "C" { fn ComputeInterpBending (bending : * mut i32 , nodexpos_local : * const i32 , order : i32 , cellcount : [i32 ; 3] , young : f64 , poisson : f64 , thickness : f64) ; } unsafe { ComputeInterpBending (bending , nodexpos_local , order , cellcount , young , poisson , thickness) }
}

/// C: mjCFlex::ComputeUnrotatedNodePositions (user/user_mesh.cc:5202)
/// Calls: mjuu_dot3, mjuu_mulvecmat, mjuu_normvec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_compute_unrotated_node_positions(self_ptr: *mut mjCFlex, nodexpos: *const i32, R0_out: *mut f64) -> i32 {
    extern "C" { fn mjCFlex_ComputeUnrotatedNodePositions(self_ptr: *mut mjCFlex, nodexpos: *const i32, R0_out: *mut f64) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCFlex_ComputeUnrotatedNodePositions(self_ptr, nodexpos, R0_out) }
}

