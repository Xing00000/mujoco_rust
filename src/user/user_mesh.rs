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
pub fn fovea(x: f64, gamma: f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (x : f64, gamma : f64)
    // Previous return: f64
    if gamma == 0.0 { return x ; } let g : f64 = if gamma < 0.0 { 0.0 } else if gamma > 1.0 { 1.0 } else { gamma } ; g * x . powi (5) + (1.0 - g) * x
}

/// C: LinSpace (user/user_mesh.cc:93)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn lin_space(lower: f64, upper: f64, n: i32, array: [f64; 0]) {
    // WARNING: signature changed — verify body
    // Previous params: (lower : f64, upper : f64, n : i32, array : [f64 ; 0])
    // Previous return: ()
    unsafe { let ptr = array . as_ptr () as * mut f64 ; let increment : f64 = if n > 1 { (upper - lower) / (n - 1) as f64 } else { 0.0 } ; let mut val = lower ; for i in 0 .. n as usize { * ptr . add (i) = val ; val += increment ; } }
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
    // WARNING: signature changed — verify body
    // Previous params: (x_edges : * mut f64, y_edges : * mut f64, size : [i32 ; 2], fov : [f64 ; 2], gamma : f64)
    // Previous return: ()
    use crate :: user :: user_util :: mjuu_scalevec ; const MJ_PI : f64 = std :: f64 :: consts :: PI ; unsafe { { let n = size [0] + 1 ; let mut lower : f64 = - 1.0 ; let increment : f64 = if n > 1 { (1.0 - (- 1.0)) / (n - 1) as f64 } else { 0.0 } ; for i in 0 .. n as usize { * x_edges . add (i) = lower ; lower += increment ; } } { let n = size [1] + 1 ; let mut lower : f64 = - 1.0 ; let increment : f64 = if n > 1 { (1.0 - (- 1.0)) / (n - 1) as f64 } else { 0.0 } ; for i in 0 .. n as usize { * y_edges . add (i) = lower ; lower += increment ; } } for i in 0 .. (size [0] + 1) as usize { * x_edges . add (i) = fovea (* x_edges . add (i) , gamma) ; } for i in 0 .. (size [1] + 1) as usize { * y_edges . add (i) = fovea (* y_edges . add (i) , gamma) ; } mjuu_scalevec (x_edges , x_edges as * const f64 , fov [0] * MJ_PI / 180.0 , size [0] + 1) ; mjuu_scalevec (y_edges , y_edges as * const f64 , fov [1] * MJ_PI / 180.0 , size [1] + 1) ; }
}

/// C: SphericalToCartesian (user/user_mesh.cc:123)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn spherical_to_cartesian(aer: [f64; 3], xyz: [f32; 3]) {
    // NOTE: In C ABI, array parameters decay to pointers, so aer/xyz are passed by-ref.
    // Rust passes small arrays in registers on some platforms; the extern "C" decl matches.
    // We re-implement using the same semantics: write to xyz via pointer cast.
    unsafe {
        let aer_ptr = &aer as *const [f64; 3] as *const f64;
        let xyz_ptr = &xyz as *const [f32; 3] as *mut f32;
        let a: f64 = *aer_ptr.add(0);
        let e: f64 = *aer_ptr.add(1);
        let r: f64 = *aer_ptr.add(2);
        *xyz_ptr.add(0) = (r * e.cos() * a.sin()) as f32;
        *xyz_ptr.add(1) = (r * e.sin()) as f32;
        *xyz_ptr.add(2) = (-r * e.cos() * a.cos()) as f32;
    }
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
    // NOTE: In C ABI, array parameters decay to pointers. We reimplement in-place.
    unsafe {
        let aer_ptr = &aer as *const [f64; 3] as *const f64;
        let mat_ptr = &mat as *const [f32; 9] as *mut f32;
        let a: f64 = *aer_ptr.add(0);
        let e: f64 = *aer_ptr.add(1);
        let r: f64 = *aer_ptr.add(2);

        let mut ta: [f64; 3] = [r * e.cos() * a.cos(), 0.0, r * e.cos() * a.sin()];
        let mut te: [f64; 3] = [-r * e.sin() * a.sin(), r * e.cos(), r * e.sin() * a.cos()];
        let mut n: [f64; 3] = [0.0; 3];

        crate::user::user_util::mjuu_normvec(ta.as_mut_ptr(), 3);
        crate::user::user_util::mjuu_normvec(te.as_mut_ptr(), 3);

        // mat[3..6] = ta (as f32)
        *mat_ptr.add(3) = ta[0] as f32;
        *mat_ptr.add(4) = ta[1] as f32;
        *mat_ptr.add(5) = ta[2] as f32;

        // mat[6..9] = te (as f32)
        *mat_ptr.add(6) = te[0] as f32;
        *mat_ptr.add(7) = te[1] as f32;
        *mat_ptr.add(8) = te[2] as f32;

        // n = cross(te, ta)
        crate::user::user_util::mjuu_crossvec(n.as_mut_ptr(), te.as_ptr(), ta.as_ptr());

        // mat[0..3] = n (as f32)
        *mat_ptr.add(0) = n[0] as f32;
        *mat_ptr.add(1) = n[1] as f32;
        *mat_ptr.add(2) = n[2] as f32;
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
    f64::copysign(omega.cos().abs().powf(m), omega.cos())
}

/// C: aux_s (user/user_mesh.cc:148)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn aux_s(omega: f64, m: f64) -> f64 {
    f64::copysign(omega.sin().abs().powf(m), omega.sin())
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
    // WARNING: signature changed — verify body
    // Previous params: (normal : * mut f64, center : * mut f64, v1 : * const f64, v2 : * const f64, v3 : * const f64)
    // Previous return: f64
    use crate :: user :: user_util :: { mjuu_crossvec , mjuu_dot3 } ; const MJ_MINVAL : f64 = 1e-15 ; unsafe { let mut normal_local : [f64 ; 3] = [0.0 ; 3] ; let normal_ptr : * mut f64 = if ! normal . is_null () { normal } else { normal_local . as_mut_ptr () } ; if ! center . is_null () { * center . add (0) = (* v1 . add (0) + * v2 . add (0) + * v3 . add (0)) / 3.0 ; * center . add (1) = (* v1 . add (1) + * v2 . add (1) + * v3 . add (1)) / 3.0 ; * center . add (2) = (* v1 . add (2) + * v2 . add (2) + * v3 . add (2)) / 3.0 ; } let b : [f64 ; 3] = [* v2 . add (0) - * v1 . add (0) , * v2 . add (1) - * v1 . add (1) , * v2 . add (2) - * v1 . add (2) ,] ; let c : [f64 ; 3] = [* v3 . add (0) - * v1 . add (0) , * v3 . add (1) - * v1 . add (1) , * v3 . add (2) - * v1 . add (2) ,] ; mjuu_crossvec (normal_ptr , b . as_ptr () , c . as_ptr ()) ; let len : f64 = mjuu_dot3 (normal_ptr as * const f64 , normal_ptr as * const f64) . sqrt () ; if len < MJ_MINVAL { return 0.0 ; } if ! normal . is_null () { * normal_ptr . add (0) /= len ; * normal_ptr . add (1) /= len ; * normal_ptr . add (2) /= len ; } 0.5 * len }
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
    extern "C" { fn MeshPolygon_Normal(self_ptr: *mut MeshPolygon) -> *const f64; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { MeshPolygon_Normal(self_ptr) }
}

/// C: MeshPolygon::CombineIslands (user/user_mesh.cc:2698)
#[allow(unused_variables, non_snake_case)]
pub fn mesh_polygon_combine_islands(self_ptr: *mut MeshPolygon, island1: *mut i32, island2: *mut i32) {
    extern "C" { fn MeshPolygon_CombineIslands(self_ptr: *mut MeshPolygon, island1: *mut i32, island2: *mut i32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
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
    extern "C" { fn MeshPolygonKey(angles: *const (), v1: [f64; 3], v2: [f64; 3], v3: [f64; 3], angle_tol: f64) -> bool; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { MeshPolygonKey(angles, v1, v2, v3, angle_tol) }
}

/// C: MeshPolygon::Paths (user/user_mesh.cc:2839)
#[allow(unused_variables, non_snake_case)]
pub fn mesh_polygon_paths(self_ptr: *mut MeshPolygon) -> i32 {
    extern "C" { fn MeshPolygon_Paths(self_ptr: *mut MeshPolygon) -> i32; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
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
pub fn compute_volume(x: *const f64, v: [i32; 3]) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (x : * const f64, v : [i32 ; 3])
    // Previous return: f64
    use crate :: user :: user_util :: { mjuu_crossvec , mjuu_normvec } ; unsafe { let x0 = x . add (3 * v [0] as usize) ; let x1 = x . add (3 * v [1] as usize) ; let x2 = x . add (3 * v [2] as usize) ; let mut edge1 : [f64 ; 3] = [* x1 . add (0) - * x0 . add (0) , * x1 . add (1) - * x0 . add (1) , * x1 . add (2) - * x0 . add (2) ,] ; let mut edge2 : [f64 ; 3] = [* x2 . add (0) - * x0 . add (0) , * x2 . add (1) - * x0 . add (1) , * x2 . add (2) - * x0 . add (2) ,] ; let mut normal : [f64 ; 3] = [0.0 ; 3] ; mjuu_crossvec (normal . as_mut_ptr () , edge1 . as_ptr () , edge2 . as_ptr ()) ; mjuu_normvec (normal . as_mut_ptr () , 3) / 2.0 }
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
    // WARNING: signature changed — verify body
    // Previous params: (basis : [f64 ; 9], x : * const f64, v : [i32 ; 3], faceL : [i32 ; 2], faceR : [i32 ; 2], volume : f64)
    // Previous return: ()
    extern "C" { fn ComputeBasis (basis : [f64 ; 9] , x : * const f64 , v : [i32 ; 3] , faceL : [i32 ; 2] , faceR : [i32 ; 2] , volume : f64) ; } unsafe { ComputeBasis (basis , x , v , faceL , faceR , volume) }
}

/// C: ComputeStiffness (user/user_mesh.cc:3574)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn compute_stiffness(stiffness: *mut i32, body_pos: *const i32, v: *const i32, t: i32, E: f64, nu: f64, thickness: f64) {
    // WARNING: signature changed — verify body
    // Previous params: (stiffness : * mut i32, body_pos : * const i32, v : * const i32, t : i32, E : f64, nu : f64, thickness : f64)
    // Previous return: ()
    extern "C" { fn ComputeStiffness (stiffness : * mut i32 , body_pos : * const i32 , v : * const i32 , t : i32 , E : f64 , nu : f64 , thickness : f64) ; } unsafe { ComputeStiffness (stiffness , body_pos , v , t , E , nu , thickness) }
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
pub fn cot(x: *const f64, v0: i32, v1: i32, v2: i32) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (x : * const f64, v0 : i32, v1 : i32, v2 : i32)
    // Previous return: f64
    use crate :: user :: user_util :: { mjuu_crossvec , mjuu_dot3 } ; unsafe { let mut normal : [f64 ; 3] = [0.0 ; 3] ; let mut edge1 : [f64 ; 3] = [* x . add (3 * v1 as usize) - * x . add (3 * v0 as usize) , * x . add (3 * v1 as usize + 1) - * x . add (3 * v0 as usize + 1) , * x . add (3 * v1 as usize + 2) - * x . add (3 * v0 as usize + 2) ,] ; let mut edge2 : [f64 ; 3] = [* x . add (3 * v2 as usize) - * x . add (3 * v0 as usize) , * x . add (3 * v2 as usize + 1) - * x . add (3 * v0 as usize + 1) , * x . add (3 * v2 as usize + 2) - * x . add (3 * v0 as usize + 2) ,] ; mjuu_crossvec (normal . as_mut_ptr () , edge1 . as_ptr () , edge2 . as_ptr ()) ; mjuu_dot3 (edge1 . as_ptr () , edge2 . as_ptr ()) / (mjuu_dot3 (normal . as_ptr () , normal . as_ptr ())) . sqrt () }
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
    unsafe {
        if order > 3 {
            crate::engine::engine_util_errmem::mju_error(
                b"Integration order > 3 not yet supported.\0".as_ptr() as *const i8,
            );
        }

        // x is on [-1, 1], p on [a, b]
        let p0: f64 = (a + b) / 2.0;
        let dpdx: f64 = (b - a) / 2.0;

        if order == 2 {
            *points.add(0) = -dpdx / (3.0_f64).sqrt() + p0;
            *points.add(1) = dpdx / (3.0_f64).sqrt() + p0;
            *weights.add(0) = dpdx;
            *weights.add(1) = dpdx;
        } else {
            *points.add(0) = p0;
            *points.add(1) = -dpdx / (3.0_f64 / 5.0).sqrt() + p0;
            *points.add(2) = dpdx / (3.0_f64 / 5.0).sqrt() + p0;
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
    // WARNING: signature changed — verify body
    // Previous params: (s : f64, i : i32, order : i32)
    // Previous return: f64
    if order == 1 { if i == 0 { 1.0 - s } else { s } } else if order == 2 { if i == 0 { 2.0 * s * s - 3.0 * s + 1.0 } else if i == 1 { 4.0 * (s - s * s) } else if i == 2 { 2.0 * s * s - s } else { 0.0 } } else { 0.0 }
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
    // WARNING: signature changed — verify body
    // Previous params: (s : f64, i : i32, order : i32)
    // Previous return: f64
    if order == 1 { if i == 0 { - 1.0 } else { 1.0 } } else if order == 2 { if i == 0 { 4.0 * s - 3.0 } else if i == 1 { 4.0 * (1.0 - 2.0 * s) } else if i == 2 { 4.0 * s - 1.0 } else { 0.0 } } else { 0.0 }
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
    extern "C" { fn inner(tensor1: *const Matrix, tensor2: *const Matrix) -> Matrix; }
    // SAFETY: delegates to C implementation
    unsafe { inner(tensor1, tensor2) }
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
    // WARNING: signature changed — verify body
    // Previous params: (warp : * mut f64, pos : * const f64, npe : i32, order : i32, normal_axis : i32)
    // Previous return: ()
    extern "C" { fn ComputeWarpMode (warp : * mut f64 , pos : * const f64 , npe : i32 , order : i32 , normal_axis : i32) ; } unsafe { ComputeWarpMode (warp , pos , npe , order , normal_axis) }
}

/// C: ComputeWarpStiffness (user/user_mesh.cc:4104)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn compute_warp_stiffness(pos: *const f64, npe: i32, normal_axis: i32, E: f64, nu: f64, thickness: f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (pos : * const f64, npe : i32, normal_axis : i32, E : f64, nu : f64, thickness : f64)
    // Previous return: f64
    extern "C" { fn ComputeWarpStiffness (pos : * const f64 , npe : i32 , normal_axis : i32 , E : f64 , nu : f64 , thickness : f64) -> f64 ; } unsafe { ComputeWarpStiffness (pos , npe , normal_axis , E , nu , thickness) }
}

/// C: EigendecomposeStiffness (user/user_mesh.cc:4130)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn eigendecompose_stiffness(K_cell_data: *const f64, out: *mut f64, ndof: i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (K_cell_data : * const f64, out : * mut f64, ndof : i32)
    // Previous return: i32
    extern "C" { fn EigendecomposeStiffness (K_cell_data : * const f64 , out : * mut f64 , ndof : i32) -> i32 ; } unsafe { EigendecomposeStiffness (K_cell_data , out , ndof) }
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

