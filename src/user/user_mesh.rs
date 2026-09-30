//! Port of: user/user_mesh.cc
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: Fovea (user/user_mesh.cc:84)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn fovea(x: f64, gamma: f64) -> f64 {
    todo!() // cxx-internal:user_mesh.cc.o:__ZN12_GLOBAL__N_15FoveaEdd
}

/// C: LinSpace (user/user_mesh.cc:94)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn lin_space(lower: f64, upper: f64, n: i32, array: *mut f64) {
    todo!() // cxx-internal:user_mesh.cc.o:__ZN12_GLOBAL__N_18LinSpaceEddiPd
}

/// C: BinEdges (user/user_mesh.cc:104)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:__ZN12_GLOBAL__N_15FoveaEdd, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:__ZN12_GLOBAL__N_18LinSpaceEddiPd, cxx:__Z13mjuu_scalevecPdPKddi
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn bin_edges(x_edges: *mut f64, y_edges: *mut f64, size: *mut i32, fov: *mut f64, gamma: f64) {
    todo!() // cxx-internal:user_mesh.cc.o:__ZN12_GLOBAL__N_18BinEdgesEPdS0_PiS0_d
}

/// C: SphericalToCartesian (user/user_mesh.cc:119)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn spherical_to_cartesian(aer: *const f64, xyz: *mut f32) {
    todo!() // cxx-internal:user_mesh.cc.o:__ZN12_GLOBAL__N_120SphericalToCartesianEPKdPf
}

/// C: TangentFrame (user/user_mesh.cc:127)
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

/// C: aux_c (user/user_mesh.cc:141)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn aux_c(omega: f64, m: f64) -> f64 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:__ZN12_GLOBAL__N_15aux_cEdd
}

/// C: aux_s (user/user_mesh.cc:144)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn aux_s(omega: f64, m: f64) -> f64 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:__ZN12_GLOBAL__N_15aux_sEdd
}

/// C: triangle (user/user_mesh.cc:150)
/// Calls: cxx:__Z13mjuu_crossvecPdPKdS1_, cxx:__Z9mjuu_dot3PKdS0_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
/// # Safety
/// `v1`, `v2`, and `v3` must each point to three initialized, aligned `f64` values. If `center` is non-null, it must point to three writable, aligned `f64` values. If `normal` is non-null, it must point to three writable, aligned `f64` values. No other thread may make unsynchronized conflicting accesses to these memory locations during the call.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn triangle(normal: *mut f64, center: *mut f64, v1: *const f64, v2: *const f64, v3: *const f64) -> f64 {
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

/// C: ReadFromBuffer (user/user_mesh.cc:186)
#[allow(unused_variables, non_snake_case)]
pub fn read_from_buffer(dst: *mut i32, src: *const i8) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:__ZL14ReadFromBufferIiEvPT_PKc
}

/// C: mjCMesh::PointToLocal (user/user_mesh.cc:259)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_point_to_local(self_ptr: *mut mjCMesh) {
    todo!() // cxx:__ZN7mjCMesh12PointToLocalEv
}

/// C: mjCMesh::NameSpace (user/user_mesh.cc:284)
/// Calls: cxx:__Z13mjuu_stripextNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE, cxx:__Z14mjuu_strippathNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE, cxx:__ZN7mjCBase9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_name_space(self_ptr: *mut mjCMesh, m: *const mjCModel) {
    todo!() // cxx:__ZN7mjCMesh9NameSpaceEPK8mjCModel
}

/// C: mjCMesh::CopyFromSpec (user/user_mesh.cc:296)
/// Calls: cxx:__Z13mjuu_stripextNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE, cxx:__Z14mjuu_strippathNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE, cxx:__ZN7mjCMesh15ProcessVerticesERKNSt3__16vectorIfNS0_9allocatorIfEEEEb, cxx:_mju_free
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_from_spec(self_ptr: *mut mjCMesh) {
    todo!() // cxx:__ZN7mjCMesh12CopyFromSpecEv
}

/// C: mjCMesh::CopyPlugin (user/user_mesh.cc:328)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_plugin(self_ptr: *mut mjCMesh) {
    todo!() // cxx:__ZN7mjCMesh10CopyPluginEv
}

/// C: mjCMesh::LoadSDF (user/user_mesh.cc:340)
/// Calls: cxx:__ZN7mjCMesh15ProcessVerticesERKNSt3__16vectorIfNS0_9allocatorIfEEEEb, cxx:__ZN8mjCModel13ResolvePluginEP7mjCBaseRKNSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEESA_PP9mjCPlugin, cxx:_mjp_getPluginAtSlot
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_load_sdf(self_ptr: *mut mjCMesh) {
    todo!() // cxx:__ZN7mjCMesh7LoadSDFEv
}

/// C: mjCMesh::CacheMesh (user/user_mesh.cc:431)
/// Calls: cxx:__ZN8mjCCache6InsertERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEES8_PK11mjResource_NS0_10shared_ptrIKvEEm, cxx:__ZNK26mjCBoundingVolumeHierarchy4SizeEv, cxx:__ZNK7mjCMesh11npolygonmapEv, cxx:__ZNK7mjCMesh12npolygonvertEv, cxx:__ZNK9mjCOctree4SizeEv, cxx:_mju_malloc
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_cache_mesh(self_ptr: *mut mjCMesh, cache: *mut mjCCache, resource: *const mjResource) {
    todo!() // cxx:__ZN7mjCMesh9CacheMeshEP8mjCCachePK11mjResource_
}

/// C: mjCMesh::ProcessVertices (user/user_mesh.cc:524)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_process_vertices(self_ptr: *mut mjCMesh, vert: *const (), remove_repeated: bool) {
    todo!() // cxx:__ZN7mjCMesh15ProcessVerticesERKNSt3__16vectorIfNS0_9allocatorIfEEEEb
}

/// C: mjCMesh::IsMSH (user/user_mesh.cc:592)
/// Calls: cxx:__ZN7mjCBase19GetAssetContentTypeENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEES4_
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_is_msh__8a61bee78e2aafea(filename: std__string_view, ct: std__string_view) -> bool {
    todo!() // cxx:__ZN7mjCMesh5IsMSHENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEES4_
}

/// C: mjCMesh::IsMSH (user/user_mesh.cc:598)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_is_msh__dc9ef7fb9cd3bbba(self_ptr: *mut mjCMesh) -> bool {
    todo!() // cxx:__ZNK7mjCMesh5IsMSHEv
}

/// C: mjCMesh::LoadFromDecoder (user/user_mesh.cc:603)
/// Calls: cxx:__ZN7mjCMesh15ProcessVerticesERKNSt3__16vectorIfNS0_9allocatorIfEEEEb, cxx:_mj_deleteSpec, cxx:_mjp_findDecoder, cxx:_mjs_asMesh, cxx:_mjs_firstElement
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_load_from_decoder(self_ptr: *mut mjCMesh, resource: *mut mjResource, remove_repeated: bool) {
    todo!() // cxx:__ZN7mjCMesh15LoadFromDecoderEP11mjResource_b
}

/// C: mjCMesh::LoadFromResource (user/user_mesh.cc:630)
/// Calls: cxx:__ZN7mjCBase19GetAssetContentTypeENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEES4_, cxx:__ZN7mjCMesh15LoadFromDecoderEP11mjResource_b, cxx:__ZN7mjCMesh7LoadMSHEP11mjResource_b
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_load_from_resource(self_ptr: *mut mjCMesh, resource: *mut mjResource, remove_repeated: bool) {
    todo!() // cxx:__ZN7mjCMesh16LoadFromResourceEP11mjResource_b
}

/// C: mjCMesh::Compile (user/user_mesh.cc:644)
/// Calls: cxx:__ZN7mjCMesh10TryCompileEPK6mjVFS_, cxx:_mju_closeResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_compile(self_ptr: *mut mjCMesh, vfs: *const mjVFS) {
    todo!() // cxx:__ZN7mjCMesh7CompileEPK6mjVFS_
}

/// C: mjCMesh::TryCompile (user/user_mesh.cc:658)
/// Calls: cxx:__Z14mjuu_strippathNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE, cxx:__ZN7mjCBase12LoadResourceERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEES8_PK6mjVFS_, cxx:__ZN7mjCMesh12CopyFromSpecEv, cxx:__ZN7mjCMesh14LoadCachedMeshEP8mjCCachePK11mjResource_, cxx:__ZN7mjCMesh15ProcessVerticesERKNSt3__16vectorIfNS0_9allocatorIfEEEEb, cxx:__ZN7mjCMesh16LoadFromResourceEP11mjResource_b, cxx:__ZN7mjCMesh7LoadSDFEv, cxx:__ZN7mjCMesh7ProcessEv, cxx:__ZN7mjCMesh9CacheMeshEP8mjCCachePK11mjResource_, cxx:__ZN9mjCOctree11SetMaxDepthEi, cxx:__ZN9mjCOctree12CreateOctreeEPKd, cxx:__ZN9mjCOctree16ComputeSdfCoeffsEPKdiPKiiRK26mjCBoundingVolumeHierarchy, cxx:__ZN9mjCOctree5ClearEv, cxx:__ZN9mjCOctree7SetFaceERKNSt3__16vectorIdNS0_9allocatorIdEEEERKNS1_IiNS2_IiEEEE, cxx:__ZNK6mujoco4user8FilePath3StrEv, cxx:__ZNK7mjCMesh16CheckInitialMeshEv, cxx:__ZNK7mjCMesh5nfaceEv, cxx:__ZNK7mjCMesh5nvertEv, cxx:__ZNK9mjCOctree8NumNodesEv, cxx:_mj_getCache, cxx:_mjs_getString, cxx:_mju_closeResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_try_compile(self_ptr: *mut mjCMesh, vfs: *const mjVFS) {
    todo!() // cxx:__ZN7mjCMesh10TryCompileEPK6mjVFS_
}

/// C: mjCMesh::SetBoundingVolume (user/user_mesh.cc:776)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_set_bounding_volume(self_ptr: *mut mjCMesh, faceid: i32, dvert: *const f64) {
    todo!() // cxx:__ZN7mjCMesh17SetBoundingVolumeEiPKd
}

/// C: mjCMesh::GetPosPtr (user/user_mesh.cc:801)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_get_pos_ptr(self_ptr: *mut mjCMesh) -> *mut f64 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { core::ptr::addr_of_mut!((*self_ptr).pos_).cast::<f64>() }
}

/// C: mjCMesh::GetQuatPtr (user/user_mesh.cc:806)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_get_quat_ptr(self_ptr: *mut mjCMesh) -> *mut f64 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { core::ptr::addr_of_mut!((*self_ptr).quat_).cast::<f64>() }
}

/// C: mjCMesh::HasTexcoord (user/user_mesh.cc:811)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_has_texcoord(self_ptr: *mut mjCMesh) -> bool {
    todo!() // cxx:__ZNK7mjCMesh11HasTexcoordEv
}

/// C: mjCMesh::CopyVert (user/user_mesh.cc:816)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_vert(self_ptr: *mut mjCMesh, arr: *mut f32) {
    todo!() // cxx:__ZNK7mjCMesh8CopyVertEPf
}

/// C: mjCMesh::CopyNormal (user/user_mesh.cc:821)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_normal(self_ptr: *mut mjCMesh, arr: *mut f32) {
    todo!() // cxx:__ZNK7mjCMesh10CopyNormalEPf
}

/// C: mjCMesh::CopyFace (user/user_mesh.cc:826)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_face(self_ptr: *mut mjCMesh, arr: *mut i32) {
    todo!() // cxx:__ZNK7mjCMesh8CopyFaceEPi
}

/// C: mjCMesh::CopyFaceTexcoord (user/user_mesh.cc:831)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_face_texcoord(self_ptr: *mut mjCMesh, arr: *mut i32) {
    todo!() // cxx:__ZNK7mjCMesh16CopyFaceTexcoordEPi
}

/// C: mjCMesh::CopyFaceNormal (user/user_mesh.cc:836)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_face_normal(self_ptr: *mut mjCMesh, arr: *mut i32) {
    todo!() // cxx:__ZNK7mjCMesh14CopyFaceNormalEPi
}

/// C: mjCMesh::CopyTexcoord (user/user_mesh.cc:841)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_texcoord(self_ptr: *mut mjCMesh, arr: *mut f32) {
    todo!() // cxx:__ZNK7mjCMesh12CopyTexcoordEPf
}

/// C: mjCMesh::CopyGraph (user/user_mesh.cc:846)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_graph__7fa6fcbf55e93b6d(self_ptr: *mut mjCMesh, arr: *mut i32) {
    todo!() // cxx:__ZNK7mjCMesh9CopyGraphEPi
}

/// C: mjCMesh::CopyPolygons (user/user_mesh.cc:851)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_polygons(self_ptr: *mut mjCMesh, verts: *mut i32, adr: *mut i32, num: *mut i32, poly_adr: i32) {
    todo!() // cxx:__ZNK7mjCMesh12CopyPolygonsEPiS0_S0_i
}

/// C: mjCMesh::CopyPolygonMap (user/user_mesh.cc:862)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_polygon_map(self_ptr: *mut mjCMesh, faces: *mut i32, adr: *mut i32, num: *mut i32, poly_adr: i32) {
    todo!() // cxx:__ZNK7mjCMesh14CopyPolygonMapEPiS0_S0_i
}

/// C: mjCMesh::CopyPolygonNormals (user/user_mesh.cc:873)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_polygon_normals(self_ptr: *mut mjCMesh, arr: *mut f64) {
    todo!() // cxx:__ZN7mjCMesh18CopyPolygonNormalsEPd
}

/// C: mjCMesh::DelTexcoord (user/user_mesh.cc:882)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_del_texcoord(self_ptr: *mut mjCMesh) {
    todo!() // cxx:__ZN7mjCMesh11DelTexcoordEv
}

/// C: mjCMesh::FitGeom (user/user_mesh.cc:888)
/// Calls: cxx:__ZN7mjCMesh16GetInertiaBoxPtrEv
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_fit_geom(self_ptr: *mut mjCMesh, geom: *mut mjCGeom, center: *mut f64) {
    todo!() // cxx:__ZN7mjCMesh7FitGeomEP7mjCGeomPd
}

/// C: mjCMesh::LoadCachedMesh (user/user_mesh.cc:965)
/// Calls: cxx:__ZN8mjCCache12PopulateDataERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEPK11mjResource_NS0_8functionIFbPKvEEE, cxx:__ZNK7mjCMesh5nvertEv, cxx:_mju_malloc
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_load_cached_mesh(self_ptr: *mut mjCMesh, cache: *mut mjCCache, resource: *const mjResource) -> bool {
    todo!() // cxx:__ZN7mjCMesh14LoadCachedMeshEP8mjCCachePK11mjResource_
}

/// C: mjCMesh::LoadMSH (user/user_mesh.cc:1033)
/// Calls: cxx:__ZN7mjCMesh15ProcessVerticesERKNSt3__16vectorIfNS0_9allocatorIfEEEEb, cxx:_mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_load_msh(self_ptr: *mut mjCMesh, resource: *mut mjResource, remove_repeated: bool) {
    todo!() // cxx:__ZN7mjCMesh7LoadMSHEP11mjResource_b
}

/// C: mjCMesh::ComputeVolume (user/user_mesh.cc:1129)
/// Calls: cxx-internal:user_mesh.cc.o:__ZL8trianglePdS_PKdS1_S1_, cxx:__Z9mjuu_dot3PKdS0_, cxx:__ZNK7mjCMesh10GraphFacesEv, cxx:__ZNK7mjCMesh5nfaceEv
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_compute_volume(self_ptr: *mut mjCMesh, CoM: *mut f64, facecen: *const f64, dvert: *const f64) -> f64 {
    todo!() // cxx:__ZNK7mjCMesh13ComputeVolumeEPdPKdS2_
}

/// C: mjCMesh::ComputeSurfaceArea (user/user_mesh.cc:1169)
/// Calls: cxx-internal:user_mesh.cc.o:__ZL8trianglePdS_PKdS1_S1_, cxx:__ZNK7mjCMesh5nfaceEv
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_compute_surface_area(self_ptr: *mut mjCMesh, CoM: *mut f64, facecen: *const f64, dvert: *const f64) -> f64 {
    todo!() // cxx:__ZNK7mjCMesh18ComputeSurfaceAreaEPdPKdS2_
}

/// C: mjCMesh::ApplyTransformations (user/user_mesh.cc:1198)
/// Calls: cxx:__Z13mjuu_quat2matPdPKd, cxx:__Z15mjuu_mulvecmatTPdPKdS1_, cxx:__ZNK7mjCMesh5nvertEv, cxx:__ZNK7mjCMesh7nnormalEv
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_apply_transformations(self_ptr: *mut mjCMesh, dvert: *mut f64) {
    todo!() // cxx:__ZN7mjCMesh20ApplyTransformationsEPd
}

/// C: mjCMesh::ComputeFaceCentroid (user/user_mesh.cc:1265)
/// Calls: cxx-internal:user_mesh.cc.o:__ZL8trianglePdS_PKdS1_S1_, cxx:__ZNK7mjCMesh5nfaceEv
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_compute_face_centroid(self_ptr: *mut mjCMesh, arg0: *mut f64, dvert: *const f64) -> f64 {
    todo!() // cxx:__ZNK7mjCMesh19ComputeFaceCentroidEPdPKd
}

/// C: mjCMesh::Process (user/user_mesh.cc:1293)
/// Calls: cxx-internal:user_mesh.cc.o:__ZL8trianglePdS_PKdS1_S1_, cxx:__Z9mjuu_eig3PdS_S_PKd, cxx:__ZN26mjCBoundingVolumeHierarchy23AllocateBoundingVolumesEi, cxx:__ZN26mjCBoundingVolumeHierarchy9CreateBVHEP8mjCModelPK7mjCBase, cxx:__ZN7mjCMesh10MakeCenterEPKd, cxx:__ZN7mjCMesh10MakeNormalEPKd, cxx:__ZN7mjCMesh12MakePolygonsEPKd, cxx:__ZN7mjCMesh17SetBoundingVolumeEiPKd, cxx:__ZN7mjCMesh18MakePolygonNormalsEPKd, cxx:__ZN7mjCMesh20ApplyTransformationsEPd, cxx:__ZN7mjCMesh6RotateEPdS0_, cxx:__ZN7mjCMesh9MakeGraphEPKd, cxx:__ZN9mjCOctree11SetMaxDepthEi, cxx:__ZN9mjCOctree12CreateOctreeEPKd, cxx:__ZN9mjCOctree16ComputeSdfCoeffsEPKdiPKiiRK26mjCBoundingVolumeHierarchy, cxx:__ZN9mjCOctree7SetFaceERKNSt3__16vectorIdNS0_9allocatorIdEEEERKNS1_IiNS2_IiEEEE, cxx:__ZNK26mjCBoundingVolumeHierarchy3BvhEv, cxx:__ZNK7mjCMesh12GetVolumeRefEv, cxx:__ZNK7mjCMesh13ComputeVolumeEPdPKdS2_, cxx:__ZNK7mjCMesh14ComputeInertiaEPdPKdS2_, cxx:__ZNK7mjCMesh18ComputeSurfaceAreaEPdPKdS2_, cxx:__ZNK7mjCMesh19ComputeFaceCentroidEPdPKd, cxx:__ZNK7mjCMesh5nfaceEv, cxx:__ZNK7mjCMesh5nvertEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_process(self_ptr: *mut mjCMesh) {
    todo!() // cxx:__ZN7mjCMesh7ProcessEv
}

/// C: mjCMesh::ComputeInertia (user/user_mesh.cc:1500)
/// Calls: cxx-internal:user_mesh.cc.o:__ZL8trianglePdS_PKdS1_S1_, cxx:__Z9mjuu_dot3PKdS0_, cxx:__ZNK7mjCMesh10GraphFacesEv, cxx:__ZNK7mjCMesh5nfaceEv, cxx:__ZNK7mjCMesh5nvertEv
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_compute_inertia(self_ptr: *mut mjCMesh, inert: *mut f64, CoM: *const f64, dvert: *const f64) -> f64 {
    todo!() // cxx:__ZNK7mjCMesh14ComputeInertiaEPdPKdS2_
}

/// C: mjCMesh::Rotate (user/user_mesh.cc:1571)
/// Calls: cxx:__Z13mjuu_quat2matPdPKd, cxx:__Z14mjuu_mulvecmatPdPKdS1_, cxx:__ZNK7mjCMesh5nvertEv, cxx:__ZNK7mjCMesh7nnormalEv
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_rotate(self_ptr: *mut mjCMesh, quat: *mut f64, dvert: *mut f64) {
    todo!() // cxx:__ZN7mjCMesh6RotateEPdS0_
}

/// C: mjCMesh::CheckInitialMesh (user/user_mesh.cc:1597)
/// Calls: cxx:__ZNK7mjCMesh5nvertEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_check_initial_mesh(self_ptr: *mut mjCMesh) {
    todo!() // cxx:__ZNK7mjCMesh16CheckInitialMeshEv
}

/// C: mjCMesh::GetInertiaBoxPtr (user/user_mesh.cc:1628)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_get_inertia_box_ptr(self_ptr: *mut mjCMesh) -> *mut f64 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { core::ptr::addr_of_mut!((*self_ptr).boxsz_).cast::<f64>() }
}

/// C: mjCMesh::GetVolumeRef (user/user_mesh.cc:1634)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_get_volume_ref(self_ptr: *mut mjCMesh) -> f64 {
    todo!() // cxx:__ZNK7mjCMesh12GetVolumeRefEv
}

/// C: mjCMesh::MakeGraph (user/user_mesh.cc:1640)
/// Calls: cxx:__Z10mjuu_dist3PKdS0_, cxx:__Z13mjuu_crossvecPdPKdS1_, cxx:__Z9mjuu_dot3PKdS0_, cxx:__ZNK7mjCMesh5nvertEv, cxx:_mju_error, cxx:_mju_free, cxx:_mju_malloc, cxx:_mju_warning
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_graph(self_ptr: *mut mjCMesh, dvert: *const f64) {
    todo!() // cxx:__ZN7mjCMesh9MakeGraphEPKd
}

/// C: mjCMesh::CopyGraph (user/user_mesh.cc:1886)
/// Calls: cxx:__ZNK7mjCMesh5nfaceEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_graph__8a22f29b846e120f(self_ptr: *mut mjCMesh) {
    todo!() // cxx:__ZN7mjCMesh9CopyGraphEv
}

/// C: mjCMesh::MakeHemisphere (user/user_mesh.cc:1908)
/// Calls: cxx:_mjs_setFloat, cxx:_mjs_setInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_hemisphere(self_ptr: *mut mjCMesh, res: i32, make_faces: bool, make_cap: bool) {
    todo!() // cxx:__ZN7mjCMesh14MakeHemisphereEibb
}

/// C: mjCMesh::MakeSphere (user/user_mesh.cc:2033)
/// Calls: cxx:_mjs_setFloat, cxx:_mjs_setInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_sphere(self_ptr: *mut mjCMesh, subdiv: i32, make_faces: bool) {
    todo!() // cxx:__ZN7mjCMesh10MakeSphereEib
}

/// C: mjCMesh::MakeSupersphere (user/user_mesh.cc:2120)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:__ZN12_GLOBAL__N_15aux_cEdd, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:__ZN12_GLOBAL__N_15aux_sEdd, cxx:_mjs_setFloat, cxx:_mjs_setInt
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_supersphere(self_ptr: *mut mjCMesh, res: i32, e: f64, n: f64) {
    todo!() // cxx:__ZN7mjCMesh15MakeSupersphereEidd
}

/// C: mjCMesh::MakeTorus (user/user_mesh.cc:2181)
/// Calls: cxx:_mjs_setFloat, cxx:_mjs_setInt
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_torus(self_ptr: *mut mjCMesh, res: i32, radius: f64) {
    todo!() // cxx:__ZN7mjCMesh9MakeTorusEid
}

/// C: mjCMesh::MakeSupertorus (user/user_mesh.cc:2233)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:__ZN12_GLOBAL__N_15aux_cEdd, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:__ZN12_GLOBAL__N_15aux_sEdd, cxx:_mjs_setFloat, cxx:_mjs_setInt
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_supertorus(self_ptr: *mut mjCMesh, res: i32, radius: f64, s: f64, t: f64) {
    todo!() // cxx:__ZN7mjCMesh14MakeSupertorusEiddd
}

/// C: mjCMesh::MakeWedge (user/user_mesh.cc:2285)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:__ZN12_GLOBAL__N_112TangentFrameEPKdPf, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:__ZN12_GLOBAL__N_120SphericalToCartesianEPKdPf, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:__ZN12_GLOBAL__N_18BinEdgesEPdS0_PiS0_d, cxx:_mjs_setFloat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_wedge(self_ptr: *mut mjCMesh, resolution: *mut i32, fov: *mut f64, gamma: f64) {
    todo!() // cxx:__ZN7mjCMesh9MakeWedgeEPiPdd
}

/// C: mjCMesh::MakeRect (user/user_mesh.cc:2309)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:__ZN12_GLOBAL__N_18LinSpaceEddiPd, cxx:_mjs_setFloat, cxx:_mjs_setInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_rect(self_ptr: *mut mjCMesh, resolution: *mut i32) {
    todo!() // cxx:__ZN7mjCMesh8MakeRectEPi
}

/// C: mjCMesh::MakeCone (user/user_mesh.cc:2349)
/// Calls: cxx:_mjs_setFloat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_cone(self_ptr: *mut mjCMesh, nedge: i32, radius: f64) {
    todo!() // cxx:__ZN7mjCMesh8MakeConeEid
}

/// C: mjCMesh::MakeNormal (user/user_mesh.cc:2376)
/// Calls: cxx:__Z13mjuu_crossvecPdPKdS1_, cxx:__Z9mjuu_dot3PKdS0_, cxx:__ZNK7mjCMesh5nfaceEv, cxx:__ZNK7mjCMesh5nvertEv, cxx:__ZNK7mjCMesh7nnormalEv, cxx:_mju_free, cxx:_mju_malloc
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_normal(self_ptr: *mut mjCMesh, dvert: *const f64) {
    todo!() // cxx:__ZN7mjCMesh10MakeNormalEPKd
}

/// C: mjCMesh::MakeCenter (user/user_mesh.cc:2473)
/// Calls: cxx:__Z13mjuu_crossvecPdPKdS1_, cxx:__Z9mjuu_dot3PKdS0_, cxx:__ZNK7mjCMesh5nfaceEv, cxx:_mju_malloc
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_center(self_ptr: *mut mjCMesh, dvert: *const f64) {
    todo!() // cxx:__ZN7mjCMesh10MakeCenterEPKd
}

/// C: mjCMesh::MakePolygonNormals (user/user_mesh.cc:2513)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_polygon_normals(self_ptr: *mut mjCMesh, dvert: *const f64) {
    todo!() // cxx:__ZN7mjCMesh18MakePolygonNormalsEPKd
}

/// C: MeshPolygon::InsertFace (user/user_mesh.cc:2545)
/// Calls: cxx:__ZN11MeshPolygon14CombineIslandsERiS0_
#[allow(unused_variables, non_snake_case)]
pub fn mesh_polygon_insert_face(self_ptr: *mut MeshPolygon, v1: i32, v2: i32, v3: i32) {
    todo!() // cxx:__ZN11MeshPolygon10InsertFaceEiii
}

/// C: MeshPolygon::Paths (user/user_mesh.cc:2546)
#[allow(unused_variables, non_snake_case)]
pub fn mesh_polygon_paths(self_ptr: *mut MeshPolygon) -> *const () {
    todo!() // cxx:__ZNK11MeshPolygon5PathsEv
}

/// C: MeshPolygon::Normal (user/user_mesh.cc:2547)
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

/// C: MeshPolygon::Normal (user/user_mesh.cc:2548)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mesh_polygon_normal__431e09f81a29c8b8(self_ptr: *mut MeshPolygon, i: i32) -> f64 {
    todo!() // cxx:__ZNK11MeshPolygon6NormalEi
}

/// C: MeshPolygon::CombineIslands (user/user_mesh.cc:2558)
#[allow(unused_variables, non_snake_case)]
pub fn mesh_polygon_combine_islands(self_ptr: *mut MeshPolygon, island1: *mut i32, island2: *mut i32) {
    todo!() // cxx:__ZN11MeshPolygon14CombineIslandsERiS0_
}

/// C: MeshPolygonKey (user/user_mesh.cc:2561)
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

/// C: mjCMesh::MakePolygons (user/user_mesh.cc:2765)
/// Calls: cxx:__Z14MeshPolygonKeyRNSt3__14pairIddEEPKdS4_S4_d, cxx:__ZN11MeshPolygon10InsertFaceEiii, cxx:__ZNK11MeshPolygon5PathsEv, cxx:__ZNK7mjCMesh10GraphFacesEv, cxx:__ZNK7mjCMesh5nvertEv
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_polygons(self_ptr: *mut mjCMesh, dvert: *const f64) {
    todo!() // cxx:__ZN7mjCMesh12MakePolygonsEPKd
}

/// C: mjCSkin::PointToLocal (user/user_mesh.cc:2869)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_point_to_local(self_ptr: *mut mjCSkin) {
    todo!() // cxx:__ZN7mjCSkin12PointToLocalEv
}

/// C: mjCSkin::NameSpace (user/user_mesh.cc:2895)
/// Calls: cxx:__Z13mjuu_stripextNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE, cxx:__Z14mjuu_strippathNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_name_space(self_ptr: *mut mjCSkin, m: *const mjCModel) {
    todo!() // cxx:__ZN7mjCSkin9NameSpaceEPK8mjCModel
}

/// C: mjCSkin::CopyFromSpec (user/user_mesh.cc:2905)
/// Calls: cxx:__Z13mjuu_stripextNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE, cxx:__Z14mjuu_strippathNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_copy_from_spec(self_ptr: *mut mjCSkin) {
    todo!() // cxx:__ZN7mjCSkin12CopyFromSpecEv
}

/// C: mjCSkin::ResolveReferences (user/user_mesh.cc:2942)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_resolve_references(self_ptr: *mut mjCSkin, m: *const mjCModel) {
    todo!() // cxx:__ZN7mjCSkin17ResolveReferencesEPK8mjCModel
}

/// C: mjCSkin::Compile (user/user_mesh.cc:2954)
/// Calls: cxx:__Z11mjuu_getextNSt3__117basic_string_viewIcNS_11char_traitsIcEEEE, cxx:__Z14mjuu_strippathNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE, cxx:__ZN7mjCBase12LoadResourceERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEES8_PK6mjVFS_, cxx:__ZN7mjCSkin12CopyFromSpecEv, cxx:__ZN7mjCSkin17ResolveReferencesEPK8mjCModel, cxx:__ZN7mjCSkin7LoadSKNEP11mjResource_, cxx:__ZNK6mujoco4user8FilePath3StrEv, cxx:_mjs_getString, cxx:_mju_closeResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_compile(self_ptr: *mut mjCSkin, vfs: *const mjVFS) {
    todo!() // cxx:__ZN7mjCSkin7CompileEPK6mjVFS_
}

/// C: mjCSkin::LoadSKN (user/user_mesh.cc:3088)
/// Calls: cxx:_mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_load_skn(self_ptr: *mut mjCSkin, resource: *mut mjResource) {
    todo!() // cxx:__ZN7mjCSkin7LoadSKNEP11mjResource_
}

/// C: ComputeVolume (user/user_mesh.cc:3291)
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

/// C: ComputeVolume (user/user_mesh.cc:3303)
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

/// C: MetricTensor (user/user_mesh.cc:3318)
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

/// C: MetricTensor (user/user_mesh.cc:3318)
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

/// C: ComputeBasis (user/user_mesh.cc:3368)
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

/// C: ComputeBasis (user/user_mesh.cc:3403)
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

/// C: ComputeStiffness (user/user_mesh.cc:3448)
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

/// C: ComputeStiffness (user/user_mesh.cc:3448)
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

/// C: CreateFlapStencil (user/user_mesh.cc:3489)
/// Calls: cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn create_flap_stencil(flaps: *const (), simplex: *const (), edgeidx: *const ()) {
    todo!() // cxx-internal:user_mesh.cc.o:__ZL17CreateFlapStencilRNSt3__16vectorI11StencilFlapNS_9allocatorIS1_EEEERKNS0_IiNS2_IiEEEES9_
}

/// C: cot (user/user_mesh.cc:3539)
/// Calls: cxx:__Z13mjuu_crossvecPdPKdS1_, cxx:__Z9mjuu_dot3PKdS0_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
/// # Safety
/// `x` must point to an initialized, aligned vertex array, and `v0`, `v1`, and `v2` must be nonnegative indices whose three-coordinate ranges lie within that array. No other thread may make unsynchronized conflicting accesses to the array during the call.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn cot(x: *const f64, v0: i32, v1: i32, v2: i32) -> f64 {
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

/// C: ComputeVolume (user/user_mesh.cc:3553)
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

/// C: ComputeBending (user/user_mesh.cc:3568)
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

/// C: quadratureGaussLegendre (user/user_mesh.cc:3621)
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

/// C: phi (user/user_mesh.cc:3645)
/// Calls: cxx:_mju_error, cxx:_mju_message
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

/// C: dphi (user/user_mesh.cc:3667)
/// Calls: cxx:_mju_error, cxx:_mju_message
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

/// C: sym (user/user_mesh.cc:3691)
#[allow(unused_variables, non_snake_case)]
pub fn sym(tensor: *const Matrix) -> Matrix {
    todo!() // cxx:__Z3symRKNSt3__15arrayINS0_IdLm3EEELm3EEE
}

/// C: inner (user/user_mesh.cc:3700)
#[allow(unused_variables, non_snake_case)]
pub fn inner(tensor1: *const Matrix, tensor2: *const Matrix) -> Matrix {
    todo!() // cxx:__Z5innerRKNSt3__15arrayINS0_IdLm3EEELm3EEES4_
}

/// C: trace (user/user_mesh.cc:3713)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn trace(tensor: *const Matrix) -> f64 {
    todo!() // cxx:__Z5traceRKNSt3__15arrayINS0_IdLm3EEELm3EEE
}

/// C: ComputeLinearStiffness (user/user_mesh.cc:3717)
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

/// C: ComputeLinearStiffness2D (user/user_mesh.cc:3804)
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

/// C: ComputeWarpMode (user/user_mesh.cc:3897)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn compute_warp_mode(warp: *mut f64, pos: *const f64, npe: i32, order: i32, normal_axis: i32) {
    todo!() // cxx-internal:user_mesh.cc.o:__ZL15ComputeWarpModePdPKdiii
}

/// C: ComputeWarpStiffness (user/user_mesh.cc:3991)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn compute_warp_stiffness(pos: *const f64, npe: i32, normal_axis: i32, E: f64, nu: f64, thickness: f64) -> f64 {
    todo!() // cxx-internal:user_mesh.cc.o:__ZL20ComputeWarpStiffnessPKdiiddd
}

/// C: EigendecomposeStiffness (user/user_mesh.cc:4017)
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

/// C: mjCFlex::PointToLocal (user/user_mesh.cc:4089)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_point_to_local(self_ptr: *mut mjCFlex) {
    todo!() // cxx:__ZN7mjCFlex12PointToLocalEv
}

/// C: mjCFlex::NameSpace (user/user_mesh.cc:4111)
/// Calls: cxx:__ZN7mjCBase9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_name_space(self_ptr: *mut mjCFlex, m: *const mjCModel) {
    todo!() // cxx:__ZN7mjCFlex9NameSpaceEPK8mjCModel
}

/// C: mjCFlex::CopyFromSpec (user/user_mesh.cc:4121)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_copy_from_spec(self_ptr: *mut mjCFlex) {
    todo!() // cxx:__ZN7mjCFlex12CopyFromSpecEv
}

/// C: mjCFlex::HasTexcoord (user/user_mesh.cc:4142)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_has_texcoord(self_ptr: *mut mjCFlex) -> bool {
    todo!() // cxx:__ZNK7mjCFlex11HasTexcoordEv
}

/// C: mjCFlex::DelTexcoord (user/user_mesh.cc:4147)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_del_texcoord(self_ptr: *mut mjCFlex) {
    todo!() // cxx:__ZN7mjCFlex11DelTexcoordEv
}

/// C: mjCFlex::ResolveReferences (user/user_mesh.cc:4152)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_resolve_references(self_ptr: *mut mjCFlex, m: *const mjCModel) {
    todo!() // cxx:__ZN7mjCFlex17ResolveReferencesEPK8mjCModel
}

/// C: mjCFlex::ComputeStiffnessCacheKey (user/user_mesh.cc:4188)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_compute_stiffness_cache_key(self_ptr: *mut mjCFlex) -> std__string {
    todo!() // cxx:__ZNK7mjCFlex24ComputeStiffnessCacheKeyEv
}

/// C: mjCFlex::LoadCachedStiffness (user/user_mesh.cc:4229)
/// Calls: cxx:__ZN8mjCCache12PopulateDataERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEPK11mjResource_NS0_8functionIFbPKvEEE, cxx:__ZNK7mjCFlex24ComputeStiffnessCacheKeyEv, cxx:_mj_getCache
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_load_cached_stiffness(self_ptr: *mut mjCFlex) -> bool {
    todo!() // cxx:__ZN7mjCFlex19LoadCachedStiffnessEv
}

/// C: mjCFlex::CacheStiffness (user/user_mesh.cc:4249)
/// Calls: cxx:__ZN8mjCCache6InsertERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEES8_PK11mjResource_NS0_10shared_ptrIKvEEm, cxx:__ZNK7mjCFlex24ComputeStiffnessCacheKeyEv, cxx:_mj_getCache
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_cache_stiffness(self_ptr: *mut mjCFlex) {
    todo!() // cxx:__ZN7mjCFlex14CacheStiffnessEv
}

/// C: ComputeInterpBending (user/user_mesh.cc:4274)
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

/// C: mjCFlex::Compile (user/user_mesh.cc:4516)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:__ZL15ComputeWarpModePdPKdiii, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:__ZL17CreateFlapStencilRNSt3__16vectorI11StencilFlapNS_9allocatorIS1_EEEERKNS0_IiNS2_IiEEEES9_, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:__ZL20ComputeInterpBendingRNSt3__16vectorIdNS_9allocatorIdEEEERKS3_iPKiddd, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:__ZL20ComputeWarpStiffnessPKdiiddd, cxx-internal:user_mesh.cc.o:__ZL23EigendecomposeStiffnessPKdPdi, cxx:__Z13mjuu_addtovecPdPKdi, cxx:__Z13mjuu_crossvecPdPKdS1_, cxx:__Z14ComputeBendingI11StencilFlapEvPdS1_PKidd, cxx:__Z14mjuu_mulvecmatPdPKdS1_, cxx:__Z15mjuu_rotVecQuatPdPKdS1_, cxx:__Z22ComputeLinearStiffnessRNSt3__16vectorIdNS_9allocatorIdEEEEPKdddi, cxx:__Z24ComputeLinearStiffness2DRNSt3__16vectorIdNS_9allocatorIdEEEEPKdddidi, cxx:__Z9mjuu_dot3PKdS0_, cxx:__ZN7mjCFlex12CopyFromSpecEv, cxx:__ZN7mjCFlex15CreateShellPairEv, cxx:__ZN7mjCFlex16ComputeCellEmptyEPKdPKiiiiS1_, cxx:__ZN7mjCFlex17ResolveReferencesEPK8mjCModel, cxx:__ZN7mjCFlex19LoadCachedStiffnessEv, cxx:__ZN7mjCFlex9CreateBVHEv, cxx:__ZNK26mjCBoundingVolumeHierarchy3BvhEv, cxx:__ZNK7mjCFlex29ComputeUnrotatedNodePositionsERKNSt3__16vectorIdNS0_9allocatorIdEEEEPd, cxx:__ZNK8mjCModel10EqualitiesEv, cxx:__ZNK8mjCModel6BodiesEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_compile(self_ptr: *mut mjCFlex, vfs: *const mjVFS) {
    todo!() // cxx:__ZN7mjCFlex7CompileEPK6mjVFS_
}

/// C: mjCFlex::ComputeUnrotatedNodePositions (user/user_mesh.cc:5071)
/// Calls: cxx:__Z14mjuu_mulvecmatPdPKdS1_, cxx:__Z9mjuu_dot3PKdS0_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_compute_unrotated_node_positions(self_ptr: *mut mjCFlex, nodexpos: *const (), R0_out: *mut f64) -> *const () {
    todo!() // cxx:__ZNK7mjCFlex29ComputeUnrotatedNodePositionsERKNSt3__16vectorIdNS0_9allocatorIdEEEEPd
}

/// C: mjCFlex::ComputeCellEmpty (user/user_mesh.cc:5156)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_compute_cell_empty(self_ptr: *mut mjCFlex, vpos: *const f64, elems: *const i32, nv: i32, ne: i32, fdim: i32, bbox: *const f64) {
    todo!() // cxx:__ZN7mjCFlex16ComputeCellEmptyEPKdPKiiiiS1_
}

/// C: mjCFlex::CreateBVH (user/user_mesh.cc:5276)
/// Calls: cxx:__ZN26mjCBoundingVolumeHierarchy21RemoveInactiveVolumesEi, cxx:__ZN26mjCBoundingVolumeHierarchy23AllocateBoundingVolumesEi, cxx:__ZN26mjCBoundingVolumeHierarchy9CreateBVHEP8mjCModelPK7mjCBase
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_create_bvh(self_ptr: *mut mjCFlex) {
    todo!() // cxx:__ZN7mjCFlex9CreateBVHEv
}

/// C: mjCFlex::CreateShellPair (user/user_mesh.cc:5323)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_create_shell_pair(self_ptr: *mut mjCFlex) {
    todo!() // cxx:__ZN7mjCFlex15CreateShellPairEv
}

