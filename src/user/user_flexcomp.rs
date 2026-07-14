//! Port of: user/user_flexcomp.cc
//! IR hash: 8cbd078414266fa8
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: ReadFromBuffer (user/user_flexcomp.cc:56)
#[allow(unused_variables, non_snake_case)]
pub fn read_from_buffer(dst: *mut T, src: *const i8) {
    todo!() // ReadFromBuffer
}

/// C: ReadStrFromBuffer (user/user_flexcomp.cc:61)
#[allow(unused_variables, non_snake_case)]
pub fn read_str_from_buffer(dest: *mut i8, src: *const i8, maxlen: i32) {
    todo!() // ReadStrFromBuffer
}

/// C: IsValidElementOrNodeHeader22 (user/user_flexcomp.cc:65)
#[allow(unused_variables, non_snake_case)]
pub fn is_valid_element_or_node_header22(line: *const std__string) -> bool {
    todo!() // IsValidElementOrNodeHeader22
}

/// C: mat2lin (user/user_flexcomp.cc:1103)
#[allow(unused_variables, non_snake_case)]
pub fn mat2lin(ix: i32, iy: i32, iz: i32, count: *const i32) -> i32 {
    todo!() // mat2lin
}

/// C: VecToArray (user/user_flexcomp.cc:1308)
/// Calls: mju_malloc
#[allow(unused_variables, non_snake_case)]
pub fn vec_to_array(vector: *const (), clear: bool) -> *mut T {
    todo!() // VecToArray
}

/// C: findstring (user/user_flexcomp.cc:1426)
#[allow(unused_variables, non_snake_case)]
pub fn findstring(buffer: *const i8, buffer_sz: i32, str: *const i8) -> i32 {
    todo!() // findstring
}

/// C: mjCFlexcomp::Make (user/user_flexcomp.h:57)
/// Calls: comperr, mjCDef::Flex, mjCFlex::PointToLocal, mjCFlexcomp::GridID, mjCFlexcomp::MakeBox, mjCFlexcomp::MakeGMSH, mjCFlexcomp::MakeGrid, mjCFlexcomp::MakeMesh, mjCFlexcomp::MakeSquare, mjCFlexcomp::MarkEmptyCells, mjCModel::AddFlex, mjCModel::Default, mjs_addBody, mjs_addEquality, mjs_addJoint, mjs_appendString, mjs_getName, mjs_getString, mjs_resolveOrientation, mjs_setDefault, mjs_setDouble, mjs_setFloat, mjs_setInt, mjs_setName, mjs_setString, mjuu_copyvec, mjuu_normvec, mjuu_setvec, mjuu_trnVecPose, mjuu_zerovec, sprintf_arr
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flexcomp_make(self_ptr: *mut mjCFlexcomp, body: *mut mjsBody, error: *mut i8, error_sz: i32, vfs: *const mjVFS) -> bool {
    todo!() // mjCFlexcomp::Make
}

/// C: mjCFlexcomp::MakeGrid (user/user_flexcomp.h:59)
/// Calls: comperr, mjCDef::Flex, mjs_getString
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flexcomp_make_grid(self_ptr: *mut mjCFlexcomp, error: *mut i8, error_sz: i32) -> bool {
    todo!() // mjCFlexcomp::MakeGrid
}

/// C: mjCFlexcomp::MakeBox (user/user_flexcomp.h:60)
/// Calls: mat2lin, mjCFlexcomp::BoxID, mjCFlexcomp::BoxProject, mjs_getString
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flexcomp_make_box(self_ptr: *mut mjCFlexcomp, error: *mut i8, error_sz: i32, dim: i32, open: bool) -> bool {
    todo!() // mjCFlexcomp::MakeBox
}

/// C: mjCFlexcomp::MakeSquare (user/user_flexcomp.h:61)
/// Calls: mjCFlexcomp::MakeGrid, mjuu_normvec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flexcomp_make_square(self_ptr: *mut mjCFlexcomp, error: *mut i8, error_sz: i32) -> bool {
    todo!() // mjCFlexcomp::MakeSquare
}

/// C: mjCFlexcomp::MakeMesh (user/user_flexcomp.h:62)
/// Calls: comperr, mjCBase::LoadResource, mjCMesh::Face, mjCMesh::FaceTexcoord, mjCMesh::HasTexcoord, mjCMesh::IsMSH, mjCMesh::LoadFromResource, mjCMesh::Texcoord, mjCMesh::Vert, mjs_getString, mju_closeResource, mjuu_combinePaths, mjuu_crossvec, mjuu_dot3, mjuu_strippath
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flexcomp_make_mesh(self_ptr: *mut mjCFlexcomp, model: *mut mjCModel, compiler: *mut mjsCompiler, error: *mut i8, error_sz: i32, vfs: *const mjVFS) -> bool {
    todo!() // mjCFlexcomp::MakeMesh
}

/// C: mjCFlexcomp::MakeGMSH (user/user_flexcomp.h:64)
/// Calls: comperr, mjCBase::LoadResource, mjCFlexcomp::LoadGMSH, mjs_getString, mju_closeResource, mjuu_combinePaths, mjuu_strippath
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flexcomp_make_gmsh(self_ptr: *mut mjCFlexcomp, model: *mut mjCModel, compiler: *mut mjsCompiler, error: *mut i8, error_sz: i32, vfs: *const mjVFS) -> bool {
    todo!() // mjCFlexcomp::MakeGMSH
}

/// C: mjCFlexcomp::LoadGMSH (user/user_flexcomp.h:66)
/// Calls: findstring, mjCFlexcomp::LoadGMSH22, mjCFlexcomp::LoadGMSH41, mju_readResource, mju_round
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flexcomp_load_gmsh(self_ptr: *mut mjCFlexcomp, model: *mut mjCModel, resource: *mut mjResource) {
    todo!() // mjCFlexcomp::LoadGMSH
}

/// C: mjCFlexcomp::LoadGMSH41 (user/user_flexcomp.h:67)
/// Calls: ReadFromBuffer
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flexcomp_load_gmsh41(self_ptr: *mut mjCFlexcomp, buffer: *mut i8, binary: i32, nodeend: i32, nodebegin: i32, elemend: i32, elembegin: i32) {
    todo!() // mjCFlexcomp::LoadGMSH41
}

/// C: mjCFlexcomp::LoadGMSH22 (user/user_flexcomp.h:69)
/// Calls: IsValidElementOrNodeHeader22, ReadFromBuffer, ReadStrFromBuffer
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flexcomp_load_gmsh22(self_ptr: *mut mjCFlexcomp, buffer: *mut i8, binary: i32, nodeend: i32, nodebegin: i32, elemend: i32, elembegin: i32) {
    todo!() // mjCFlexcomp::LoadGMSH22
}

/// C: mjCFlexcomp::GridID (user/user_flexcomp.h:73)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flexcomp_grid_id(self_ptr: *mut mjCFlexcomp, ix: i32, iy: i32) -> i32 {
    // SAFETY: self_ptr is a valid mjCFlexcomp pointer. count is [i32; 3] stored as [u8; 12].
    unsafe {
        let count = (*self_ptr).count.as_ptr() as *const i32;
        ix * *count.add(1) + iy
    }
}

/// C: mjCFlexcomp::BoxID (user/user_flexcomp.h:75)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flexcomp_box_id(self_ptr: *mut mjCFlexcomp, ix: i32, iy: i32, iz: i32) -> i32 {
    // SAFETY: self_ptr is a valid mjCFlexcomp pointer. count is [i32; 3] stored as [u8; 12].
    unsafe {
        let count = (*self_ptr).count.as_ptr() as *const i32;
        let c0 = *count.add(0);
        let c1 = *count.add(1);
        let c2 = *count.add(2);

        // side iz=0
        if iz == 0 {
            ix * c1 + iy + 1
        }
        // side iz=max
        else if iz == c2 - 1 {
            c0 * c1 + ix * c1 + iy + 1
        }
        // side iy=0
        else if iy == 0 {
            2 * c0 * c1 + ix * (c2 - 2) + iz - 1 + 1
        }
        // side iy=max
        else if iy == c1 - 1 {
            2 * c0 * c1 + c0 * (c2 - 2) + ix * (c2 - 2) + iz - 1 + 1
        }
        // side ix=0
        else if ix == 0 {
            2 * c0 * c1 + 2 * c0 * (c2 - 2) + (iy - 1) * (c2 - 2) + iz - 1 + 1
        }
        // side ix=max
        else {
            2 * c0 * c1 + 2 * c0 * (c2 - 2) + (c1 - 2) * (c2 - 2) +
                (iy - 1) * (c2 - 2) + iz - 1 + 1
        }
    }
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
    todo!() // mjCFlexcomp::BoxProject
}

/// C: mjCFlexcomp::MarkEmptyCells (user/user_flexcomp.h:123)
/// Calls: mjCFlex::ComputeCellEmpty
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flexcomp_mark_empty_cells(self_ptr: *mut mjCFlexcomp, flex: *mut mjCFlex, points: *const f64, npnt: i32, minmax: *const f64, nx: i32, ny: i32, nz: i32) {
    todo!() // mjCFlexcomp::MarkEmptyCells
}

