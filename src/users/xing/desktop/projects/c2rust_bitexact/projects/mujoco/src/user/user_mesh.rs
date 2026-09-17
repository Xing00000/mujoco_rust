//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: Fovea (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:83)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn fovea(x: f64, gamma: f64) -> f64 {
    todo!() // cxx-internal:user_mesh.cc.o:__ZN12_GLOBAL__N_15FoveaEdd
}

/// C: LinSpace (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:93)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn lin_space(lower: f64, upper: f64, n: i32, array: *mut f64) {
    todo!() // cxx-internal:user_mesh.cc.o:__ZN12_GLOBAL__N_18LinSpaceEddiPd
}

/// C: BinEdges (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:103)
/// Calls: cxx-internal:user_mesh.cc.o:__ZN12_GLOBAL__N_15FoveaEdd, cxx-internal:user_mesh.cc.o:__ZN12_GLOBAL__N_18LinSpaceEddiPd, cxx:__Z13mjuu_scalevecPdPKddi
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn bin_edges(x_edges: *mut f64, y_edges: *mut f64, size: *mut i32, fov: *mut f64, gamma: f64) {
    todo!() // cxx-internal:user_mesh.cc.o:__ZN12_GLOBAL__N_18BinEdgesEPdS0_PiS0_d
}

/// C: SphericalToCartesian (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:123)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn spherical_to_cartesian(aer: *const f64, xyz: *mut f32) {
    todo!() // cxx-internal:user_mesh.cc.o:__ZN12_GLOBAL__N_120SphericalToCartesianEPKdPf
}

/// C: TangentFrame (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:131)
/// Calls: cxx:__Z13mjuu_crossvecPdPKdS1_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn tangent_frame(aer: *const f64, mat: *mut f32) {
    todo!() // cxx-internal:user_mesh.cc.o:__ZN12_GLOBAL__N_112TangentFrameEPKdPf
}

/// C: aux_c (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:145)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn aux_c(omega: f64, m: f64) -> f64 {
    todo!() // cxx-internal:user_mesh.cc.o:__ZN12_GLOBAL__N_15aux_cEdd
}

/// C: aux_s (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:148)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn aux_s(omega: f64, m: f64) -> f64 {
    todo!() // cxx-internal:user_mesh.cc.o:__ZN12_GLOBAL__N_15aux_sEdd
}

/// C: triangle (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:154)
/// Calls: cxx:__Z13mjuu_crossvecPdPKdS1_, cxx:__Z9mjuu_dot3PKdS0_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn triangle(normal: *mut f64, center: *mut f64, v1: *const f64, v2: *const f64, v3: *const f64) -> f64 {
    todo!() // cxx-internal:user_mesh.cc.o:__ZL8trianglePdS_PKdS1_S1_
}

/// C: MeshPolygon::InsertFace (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:2685)
/// Calls: cxx:__ZN11MeshPolygon14CombineIslandsERiS0_
#[allow(unused_variables, non_snake_case)]
pub fn mesh_polygon_insert_face(self_ptr: *mut MeshPolygon, v1: i32, v2: i32, v3: i32) {
    todo!() // cxx:__ZN11MeshPolygon10InsertFaceEiii
}

/// C: MeshPolygon::Paths (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:2686)
#[allow(unused_variables, non_snake_case)]
pub fn mesh_polygon_paths(self_ptr: *mut MeshPolygon) -> *const () {
    todo!() // cxx:__ZNK11MeshPolygon5PathsEv
}

/// C: MeshPolygon::Normal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:2687)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mesh_polygon_normal__9740693a19f3510e(self_ptr: *mut MeshPolygon) -> *const f64 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { core::ptr::addr_of!((*self_ptr).normal_).cast::<f64>() }
}

/// C: MeshPolygon::Normal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:2688)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mesh_polygon_normal__431e09f81a29c8b8(self_ptr: *mut MeshPolygon, i: i32) -> f64 {
    todo!() // cxx:__ZNK11MeshPolygon6NormalEi
}

/// C: MeshPolygon::CombineIslands (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:2698)
#[allow(unused_variables, non_snake_case)]
pub fn mesh_polygon_combine_islands(self_ptr: *mut MeshPolygon, island1: *mut i32, island2: *mut i32) {
    todo!() // cxx:__ZN11MeshPolygon14CombineIslandsERiS0_
}

/// C: MeshPolygonKey (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:2701)
/// Calls: cxx:__Z13mjuu_crossvecPdPKdS1_, cxx:__Z9mjuu_dot3PKdS0_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mesh_polygon_key(angles: *const (), v1: *const f64, v2: *const f64, v3: *const f64, angle_tol: f64) -> bool {
    todo!() // cxx:__Z14MeshPolygonKeyRNSt3__14pairIddEEPKdS4_S4_d
}

/// C: ComputeVolume (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:3421)
/// Calls: cxx:__Z13mjuu_crossvecPdPKdS1_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn compute_volume__1339de89b34dd356(x: *const f64, v: *const i32) -> f64 {
    todo!() // cxx:__Z13ComputeVolumeI9Stencil2DEdPKdPKi
}

/// C: ComputeVolume (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:3434)
/// Calls: cxx:__Z13mjuu_crossvecPdPKdS1_, cxx:__Z9mjuu_dot3PKdS0_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn compute_volume__cc1eb5f46965fb16(x: *const f64, v: *const i32) -> f64 {
    todo!() // cxx:__Z13ComputeVolumeI9Stencil3DEdPKdPKi
}

/// C: MetricTensor (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:3450)
/// Calls: cxx:_mju_error
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn metric_tensor__0e7e12881271b16f(metric: *mut f64, idx: i32, mu: f64, la: f64, basis: *const [f64; 9]) {
    todo!() // cxx:__Z12MetricTensorI9Stencil2DEvPdiddPA9_Kd
}

/// C: MetricTensor (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:3450)
/// Calls: cxx:_mju_error
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn metric_tensor__cf7940da5202a7ae(metric: *mut f64, idx: i32, mu: f64, la: f64, basis: *const [f64; 9]) {
    todo!() // cxx:__Z12MetricTensorI9Stencil3DEvPdiddPA9_Kd
}

/// C: ComputeBasis (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:3503)
/// Calls: cxx:__Z13mjuu_crossvecPdPKdS1_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn compute_basis__68a2b16626b0d417(basis: *mut f64, x: *const f64, v: *const i32, faceL: *const i32, faceR: *const i32, volume: f64) {
    todo!() // cxx:__Z12ComputeBasisI9Stencil2DEvPdPKdPKiS5_S5_d
}

/// C: ComputeBasis (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:3538)
/// Calls: cxx:__Z13mjuu_crossvecPdPKdS1_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn compute_basis__c4df668648a713c9(basis: *mut f64, x: *const f64, v: *const i32, faceL: *const i32, faceR: *const i32, volume: f64) {
    todo!() // cxx:__Z12ComputeBasisI9Stencil3DEvPdPKdPKiS5_S5_d
}

/// C: ComputeStiffness (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:3574)
/// Calls: cxx:__ZNK7mjCMesh13ComputeVolumeEPdPKdS2_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn compute_stiffness__f6cd5519a10fcd20(stiffness: *const (), body_pos: *const (), v: *const i32, t: i32, E: f64, nu: f64, thickness: f64) {
    todo!() // cxx:__Z16ComputeStiffnessI9Stencil2DEvRNSt3__16vectorIdNS1_9allocatorIdEEEERKS5_PKiiddd
}

/// C: ComputeStiffness (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:3574)
/// Calls: cxx:__ZNK7mjCMesh13ComputeVolumeEPdPKdS2_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn compute_stiffness__d2985b49caf84fef(stiffness: *const (), body_pos: *const (), v: *const i32, t: i32, E: f64, nu: f64, thickness: f64) {
    todo!() // cxx:__Z16ComputeStiffnessI9Stencil3DEvRNSt3__16vectorIdNS1_9allocatorIdEEEERKS5_PKiiddd
}

/// C: CreateFlapStencil (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:3605)
/// Calls: cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn create_flap_stencil(flaps: *const (), simplex: *const (), edgeidx: *const ()) {
    todo!() // cxx-internal:user_mesh.cc.o:__ZL17CreateFlapStencilRNSt3__16vectorI11StencilFlapNS_9allocatorIS1_EEEERKNS0_IiNS2_IiEEEES9_
}

/// C: cot (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:3657)
/// Calls: cxx:__Z13mjuu_crossvecPdPKdS1_, cxx:__Z9mjuu_dot3PKdS0_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn cot(x: *const f64, v0: i32, v1: i32, v2: i32) -> f64 {
    todo!() // cxx:__Z3cotPKdiii
}

/// C: ComputeVolume (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:3667)
/// Calls: cxx:__Z13mjuu_crossvecPdPKdS1_, cxx:__Z9mjuu_dot3PKdS0_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn compute_volume__5a1eb3f925804cb6(x: *const f64, v: *const i32) -> f64 {
    todo!() // cxx:__Z13ComputeVolumePKdPKi
}

/// C: ComputeBending (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:3678)
/// Calls: cxx:__Z13mjuu_crossvecPdPKdS1_, cxx:__Z3cotPKdiii, cxx:__Z9mjuu_dot3PKdS0_, cxx:__ZNK7mjCMesh13ComputeVolumeEPdPKdS2_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn compute_bending(bending: *mut f64, pos: *mut f64, v: *const i32, mu: f64, thickness: f64) {
    todo!() // cxx:__Z14ComputeBendingI11StencilFlapEvPdS1_PKidd
}

/// C: quadratureGaussLegendre (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:3727)
/// Calls: cxx:_mju_error
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn quadrature_gauss_legendre(points: *mut f64, weights: *mut f64, order: i32, a: f64, b: f64) {
    todo!() // cxx:__Z23quadratureGaussLegendrePdS_idd
}

/// C: phi (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:3752)
/// Calls: cxx:_mju_error, cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn phi(s: f64, i: i32, order: i32) -> f64 {
    todo!() // cxx:__Z3phidii
}

/// C: dphi (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:3774)
/// Calls: cxx:_mju_error, cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn dphi(s: f64, i: i32, order: i32) -> f64 {
    todo!() // cxx:__Z4dphidii
}

/// C: sym (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:3798)
#[allow(unused_variables, non_snake_case)]
pub fn sym(tensor: *const Matrix) -> Matrix {
    todo!() // cxx:__Z3symRKNSt3__15arrayINS0_IdLm3EEELm3EEE
}

/// C: inner (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:3809)
#[allow(unused_variables, non_snake_case)]
pub fn inner(tensor1: *const Matrix, tensor2: *const Matrix) -> Matrix {
    todo!() // cxx:__Z5innerRKNSt3__15arrayINS0_IdLm3EEELm3EEES4_
}

/// C: trace (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:3822)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn trace(tensor: *const Matrix) -> f64 {
    todo!() // cxx:__Z5traceRKNSt3__15arrayINS0_IdLm3EEELm3EEE
}

/// C: ComputeLinearStiffness (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:3826)
/// Calls: cxx:__Z23quadratureGaussLegendrePdS_idd, cxx:__Z3phidii, cxx:__Z3symRKNSt3__15arrayINS0_IdLm3EEELm3EEE, cxx:__Z4dphidii, cxx:__Z5innerRKNSt3__15arrayINS0_IdLm3EEELm3EEES4_, cxx:__Z5traceRKNSt3__15arrayINS0_IdLm3EEELm3EEE
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn compute_linear_stiffness(K: *const (), pos: *const f64, E: f64, nu: f64, order: i32) {
    todo!() // cxx:__Z22ComputeLinearStiffnessRNSt3__16vectorIdNS_9allocatorIdEEEEPKdddi
}

/// C: ComputeLinearStiffness2D (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:3914)
/// Calls: cxx:__Z23quadratureGaussLegendrePdS_idd, cxx:__Z3phidii, cxx:__Z3symRKNSt3__15arrayINS0_IdLm3EEELm3EEE, cxx:__Z4dphidii, cxx:__Z5innerRKNSt3__15arrayINS0_IdLm3EEELm3EEES4_, cxx:__Z5traceRKNSt3__15arrayINS0_IdLm3EEELm3EEE
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn compute_linear_stiffness2d(K: *const (), pos: *const f64, E: f64, nu: f64, order: i32, thickness: f64, normal_axis: i32) {
    todo!() // cxx:__Z24ComputeLinearStiffness2DRNSt3__16vectorIdNS_9allocatorIdEEEEPKdddidi
}

/// C: ComputeWarpMode (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:4007)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn compute_warp_mode(warp: *mut f64, pos: *const f64, npe: i32, order: i32, normal_axis: i32) {
    todo!() // cxx-internal:user_mesh.cc.o:__ZL15ComputeWarpModePdPKdiii
}

/// C: ComputeWarpStiffness (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:4104)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn compute_warp_stiffness(pos: *const f64, npe: i32, normal_axis: i32, E: f64, nu: f64, thickness: f64) -> f64 {
    todo!() // cxx-internal:user_mesh.cc.o:__ZL20ComputeWarpStiffnessPKdiiddd
}

/// C: EigendecomposeStiffness (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:4130)
/// Calls: cxx:__Z19mjuu_eigendecomposePdS_S_i
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn eigendecompose_stiffness(K_cell_data: *const f64, out: *mut f64, ndof: i32) -> i32 {
    todo!() // cxx-internal:user_mesh.cc.o:__ZL23EigendecomposeStiffnessPKdPdi
}

/// C: ComputeInterpBending (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:4391)
/// Calls: cxx:__Z13mjuu_crossvecPdPKdS1_, cxx:__Z3phidii, cxx:__Z4dphidii
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn compute_interp_bending(bending: *const (), nodexpos_local: *const (), order: i32, cellcount: *const i32, young: f64, poisson: f64, thickness: f64) {
    todo!() // cxx-internal:user_mesh.cc.o:__ZL20ComputeInterpBendingRNSt3__16vectorIdNS_9allocatorIdEEEERKS3_iPKiddd
}

