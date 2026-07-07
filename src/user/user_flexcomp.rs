//! Port of: user/user_flexcomp.cc
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: ReadFromBuffer (user/user_flexcomp.cc:56)
#[allow(unused_variables, non_snake_case)]
pub fn read_from_buffer(dst: *mut T, src: *const i8) {
    extern "C" { fn ReadFromBuffer_impl(dst: *mut T, src: *const i8); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { ReadFromBuffer_impl(dst, src) }
}

/// C: ReadStrFromBuffer (user/user_flexcomp.cc:61)
#[allow(unused_variables, non_snake_case)]
pub fn read_str_from_buffer(dest: *mut i8, src: *const i8, maxlen: i32) {
    extern "C" { fn ReadStrFromBuffer_impl(dest: *mut i8, src: *const i8, maxlen: i32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { ReadStrFromBuffer_impl(dest, src, maxlen) }
}

/// C: IsValidElementOrNodeHeader22 (user/user_flexcomp.cc:65)
#[allow(unused_variables, non_snake_case)]
pub fn is_valid_element_or_node_header22(line: *const std__string) -> bool {
    // WARNING: signature changed — verify body
    // Previous params: (line : * const std__string)
    // Previous return: bool
    extern "C" { fn IsValidElementOrNodeHeader22_impl (line : * const std__string) -> bool ; } unsafe { IsValidElementOrNodeHeader22_impl (line) }
}

/// C: mat2lin (user/user_flexcomp.cc:1103)
#[allow(unused_variables, non_snake_case)]
pub fn mat2lin(ix: i32, iy: i32, iz: i32, count: [i32; 3]) -> i32 {
    extern "C" { fn mat2lin_impl(ix: i32, iy: i32, iz: i32, count: [i32; 3]) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mat2lin_impl(ix, iy, iz, count) }
}

/// C: VecToArray (user/user_flexcomp.cc:1308)
/// Calls: mju_malloc
#[allow(unused_variables, non_snake_case)]
pub fn vec_to_array(vector: *mut i32, clear: bool) -> *mut T {
    extern "C" { fn VecToArray_impl(vector: *mut i32, clear: bool) -> *mut T; }
    // SAFETY: delegates to C implementation
    unsafe { VecToArray_impl(vector, clear) }
}

/// C: findstring (user/user_flexcomp.cc:1426)
#[allow(unused_variables, non_snake_case)]
pub fn findstring(buffer: *const i8, buffer_sz: i32, str: *const i8) -> i32 {
    extern "C" { fn findstring_impl(buffer: *const i8, buffer_sz: i32, str: *const i8) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { findstring_impl(buffer, buffer_sz, str) }
}

/// C: mjCFlexcomp::Make (user/user_flexcomp.h:57)
/// Calls: comperr, mjCFlex::PointToLocal, mjCFlexcomp::MakeBox, mjCFlexcomp::MakeGMSH, mjCFlexcomp::MakeGrid, mjCFlexcomp::MakeMesh, mjCFlexcomp::MakeSquare, mjCModel::AddFlex, mjCModel::Default, mjs_addBody, mjs_addJoint, mjs_getName, mjs_getString, mjs_resolveOrientation, mjs_setDefault, mjs_setName, mjs_setString, mjuu_normvec, mjuu_setvec, mjuu_trnVecPose, mjuu_zerovec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flexcomp_make(self_ptr: *mut mjCFlexcomp, body: *mut mjsBody, error: *mut i8, error_sz: i32, vfs: *const mjVFS) -> bool {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCFlexcomp, body : * mut mjsBody, error : * mut i8, error_sz : i32, vfs : * const mjVFS)
    // Previous return: bool
    todo ! ()
}

/// C: mjCFlexcomp::MakeGrid (user/user_flexcomp.h:59)
/// Calls: comperr
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flexcomp_make_grid(self_ptr: *mut mjCFlexcomp, error: *mut i8, error_sz: i32) -> bool {
    extern "C" { fn mjCFlexcomp_MakeGrid_impl(self_ptr: *mut mjCFlexcomp, error: *mut i8, error_sz: i32) -> bool; }
    // SAFETY: delegates to C implementation
    unsafe { mjCFlexcomp_MakeGrid_impl(self_ptr, error, error_sz) }
}

/// C: mjCFlexcomp::MakeBox (user/user_flexcomp.h:60)
/// Calls: mat2lin, mjCFlexcomp::BoxProject
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flexcomp_make_box(self_ptr: *mut mjCFlexcomp, error: *mut i8, error_sz: i32, dim: i32, open: bool) -> bool {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCFlexcomp, error : * mut i8, error_sz : i32, dim : i32, open : bool)
    // Previous return: bool
    todo ! ()
}

/// C: mjCFlexcomp::MakeSquare (user/user_flexcomp.h:61)
/// Calls: mjCFlexcomp::MakeGrid, mjuu_normvec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flexcomp_make_square(self_ptr: *mut mjCFlexcomp, error: *mut i8, error_sz: i32) -> bool {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCFlexcomp, error : * mut i8, error_sz : i32)
    // Previous return: bool
    todo ! ()
}

/// C: mjCFlexcomp::MakeMesh (user/user_flexcomp.h:62)
/// Calls: comperr, mjCBase::LoadResource, mjCMesh::HasTexcoord, mjCMesh::LoadFromResource, mjs_getString, mju_closeResource, mjuu_crossvec, mjuu_dot3
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flexcomp_make_mesh(self_ptr: *mut mjCFlexcomp, model: *mut mjCModel, compiler: *mut mjsCompiler, error: *mut i8, error_sz: i32, vfs: *const mjVFS) -> bool {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCFlexcomp, model : * mut mjCModel, compiler : * mut mjsCompiler, error : * mut i8, error_sz : i32, vfs : * const mjVFS)
    // Previous return: bool
    todo ! ()
}

/// C: mjCFlexcomp::MakeGMSH (user/user_flexcomp.h:64)
/// Calls: comperr, mjCBase::LoadResource, mjCFlexcomp::LoadGMSH, mjs_getString, mju_closeResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flexcomp_make_gmsh(self_ptr: *mut mjCFlexcomp, model: *mut mjCModel, compiler: *mut mjsCompiler, error: *mut i8, error_sz: i32, vfs: *const mjVFS) -> bool {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCFlexcomp, model : * mut mjCModel, compiler : * mut mjsCompiler, error : * mut i8, error_sz : i32, vfs : * const mjVFS)
    // Previous return: bool
    todo ! ()
}

/// C: mjCFlexcomp::LoadGMSH (user/user_flexcomp.h:66)
/// Calls: findstring, mjCFlexcomp::LoadGMSH22, mjCFlexcomp::LoadGMSH41, mju_readResource, mju_round
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flexcomp_load_gmsh(self_ptr: *mut mjCFlexcomp, model: *mut mjCModel, resource: *mut mjResource) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCFlexcomp, model : * mut mjCModel, resource : * mut mjResource)
    // Previous return: ()
    todo ! ()
}

/// C: mjCFlexcomp::LoadGMSH41 (user/user_flexcomp.h:67)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flexcomp_load_gmsh41(self_ptr: *mut mjCFlexcomp, buffer: *mut i8, binary: i32, nodeend: i32, nodebegin: i32, elemend: i32, elembegin: i32) {
    extern "C" { fn mjCFlexcomp_LoadGMSH41_impl(self_ptr: *mut mjCFlexcomp, buffer: *mut i8, binary: i32, nodeend: i32, nodebegin: i32, elemend: i32, elembegin: i32); }
    // SAFETY: delegates to C implementation
    unsafe { mjCFlexcomp_LoadGMSH41_impl(self_ptr, buffer, binary, nodeend, nodebegin, elemend, elembegin) }
}

/// C: mjCFlexcomp::LoadGMSH22 (user/user_flexcomp.h:69)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flexcomp_load_gmsh22(self_ptr: *mut mjCFlexcomp, buffer: *mut i8, binary: i32, nodeend: i32, nodebegin: i32, elemend: i32, elembegin: i32) {
    extern "C" { fn mjCFlexcomp_LoadGMSH22_impl(self_ptr: *mut mjCFlexcomp, buffer: *mut i8, binary: i32, nodeend: i32, nodebegin: i32, elemend: i32, elembegin: i32); }
    // SAFETY: delegates to C implementation
    unsafe { mjCFlexcomp_LoadGMSH22_impl(self_ptr, buffer, binary, nodeend, nodebegin, elemend, elembegin) }
}

/// C: mjCFlexcomp::GridID (user/user_flexcomp.h:73)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flexcomp_grid_id(self_ptr: *mut mjCFlexcomp, ix: i32, iy: i32) -> i32 {
    extern "C" { fn mjCFlexcomp_GridID_impl(self_ptr: *mut mjCFlexcomp, ix: i32, iy: i32) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCFlexcomp_GridID_impl(self_ptr, ix, iy) }
}

/// C: mjCFlexcomp::BoxID (user/user_flexcomp.h:75)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flexcomp_box_id(self_ptr: *mut mjCFlexcomp, ix: i32, iy: i32, iz: i32) -> i32 {
    extern "C" { fn mjCFlexcomp_BoxID_impl(self_ptr: *mut mjCFlexcomp, ix: i32, iy: i32, iz: i32) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCFlexcomp_BoxID_impl(self_ptr, ix, iy, iz) }
}

/// C: mjCFlexcomp::BoxProject (user/user_flexcomp.h:76)
/// Calls: mjuu_normvec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flexcomp_box_project(self_ptr: *mut mjCFlexcomp, pos: *mut f64, ix: i32, iy: i32, iz: i32) {
    extern "C" { fn mjCFlexcomp_BoxProject_impl(self_ptr: *mut mjCFlexcomp, pos: *mut f64, ix: i32, iy: i32, iz: i32); }
    // SAFETY: delegates to C implementation
    unsafe { mjCFlexcomp_BoxProject_impl(self_ptr, pos, ix, iy, iz) }
}

/// C: mjCFlexcomp::MarkEmptyCells (user/user_flexcomp.h:123)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flexcomp_mark_empty_cells(self_ptr: *mut mjCFlexcomp, flex: *mut mjCFlex, points: *const f64, npnt: i32, minmax: [f64; 6], nx: i32, ny: i32, nz: i32) {
    extern "C" { fn mjCFlexcomp_MarkEmptyCells_impl(self_ptr: *mut mjCFlexcomp, flex: *mut mjCFlex, points: *const f64, npnt: i32, minmax: [f64; 6], nx: i32, ny: i32, nz: i32); }
    // SAFETY: delegates to C implementation
    unsafe { mjCFlexcomp_MarkEmptyCells_impl(self_ptr, flex, points, npnt, minmax, nx, ny, nz) }
}

